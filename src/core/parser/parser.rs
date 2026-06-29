use crate::core::error_display::{ErrorContext, Located};
use crate::core::lexer::lexer::Lexer;
use crate::core::lexer::raw_token::RawToken;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::parsable::Parsable;
use crate::core::parser::parse_error::ParseError;
use crate::core::parser::statement::Statement;
use crate::core::parser::token::Token;
use crate::core::registries::operation_registry::OperationRegistry;

pub struct Parser<'a> {
    operation_registry: &'a OperationRegistry,
    lexer: &'a Lexer,
}

impl Parser<'_> {
    pub fn new<'a>(
        operation_registry: &'a OperationRegistry,
        lexer: &'a Lexer,
    ) -> Parser<'a> {
        Parser { operation_registry, lexer }
    }

    pub fn parse(&self, input: &str) -> Result<Statement, Located<ParseError>> {
        let sliced = self.lexer.slice_input_string(input.trim());

        let display_tokens: Vec<String> = sliced.iter()
            .filter(|t| !matches!(t, RawToken::Eof))
            .map(|t| t.to_string())
            .collect();

        // Draw command: draw f from <expr> to <expr>
        if let Some(RawToken::Identifier(keyword)) = sliced.get(0) {
            if keyword == "draw" {
                return self.parse_draw(&sliced, &display_tokens);
            }
        }

        // Function definition: f(x, y) = body
        if let (Some(RawToken::Identifier(name)), Some(RawToken::Operator(paren))) =
            (sliced.get(0), sliced.get(1))
        {
            if paren == "(" {
                if let Some(stmt) = self.try_parse_function_def(name, &sliced, &display_tokens)? {
                    return Ok(stmt);
                }
            }
        }

        // Variable assignment: x = expr  or  x += expr
        if let (Some(RawToken::Identifier(name)), Some(RawToken::Operator(op))) =
            (sliced.get(0), sliced.get(1))
        {
            if op == "=" {
                let name = name.clone();
                let tokens = self.parse_raw_tokens(&sliced[2..], &display_tokens, 2)?;
                return Ok(Statement::Assignment { name, tokens });
            } else if op.ends_with("=") {
                let base_sign = op[..op.len() - 1].trim();
                if self.operation_registry.get(base_sign).is_some() {
                    let mut expanded = sliced[2..].to_vec();
                    expanded.insert(0, RawToken::Operator(base_sign.to_string()));
                    expanded.insert(0, RawToken::Identifier(name.clone()));
                    let tokens = self.parse_raw_tokens(&expanded, &display_tokens, 2)?;
                    return Ok(Statement::Assignment { name: name.clone(), tokens });
                }
            }
        }

        let tokens = self.parse_raw_tokens(&sliced, &display_tokens, 0)?;
        Ok(Statement::Expression(tokens))
    }

    fn parse_draw(
        &self,
        sliced: &[RawToken],
        display_tokens: &[String],
    ) -> Result<Statement, Located<ParseError>> {
        let syntax_err = || Located::unlocated(ParseError::InvalidSyntax(
            "expected: draw <func> from <expr> to <expr>".to_string()
        ));

        let function_name = match sliced.get(1) {
            Some(RawToken::Identifier(name)) => name.clone(),
            _ => return Err(syntax_err()),
        };

        match sliced.get(2) {
            Some(RawToken::Identifier(kw)) if kw == "from" => {}
            _ => return Err(syntax_err()),
        }

        let to_pos = sliced[3..].iter()
            .position(|t| matches!(t, RawToken::Identifier(kw) if kw == "to"))
            .map(|p| p + 3)
            .ok_or_else(syntax_err)?;

        let mut from_tokens = self.parse_raw_tokens(&sliced[3..to_pos], display_tokens, 3)?;
        from_tokens.push(Token::Eof);
        let to_tokens = self.parse_raw_tokens(&sliced[to_pos + 1..], display_tokens, to_pos + 1)?;

        Ok(Statement::DrawCommand { function_name, from_tokens, to_tokens })
    }

    fn try_parse_function_def(
        &self,
        name: &str,
        sliced: &[RawToken],
        display_tokens: &[String],
    ) -> Result<Option<Statement>, Located<ParseError>> {
        // sliced[0] = Identifier(name), sliced[1] = Operator("(")
        // Find first ")" — params are flat identifiers, no nesting
        let Some(close_pos) = sliced[2..].iter()
            .position(|t| matches!(t, RawToken::Operator(op) if op == ")"))
            .map(|p| p + 2)
        else {
            return Ok(None);
        };

        // Must be followed by "="
        let Some(RawToken::Operator(eq)) = sliced.get(close_pos + 1) else {
            return Ok(None);
        };
        if eq != "=" {
            return Ok(None);
        }

        // Extract parameter names from sliced[2..close_pos]
        let mut params = Vec::new();
        for token in &sliced[2..close_pos] {
            match token {
                RawToken::Identifier(p) => params.push(p.clone()),
                RawToken::Operator(op) if op == "," => {}
                _ => return Ok(None),
            }
        }

        let body = self.parse_raw_tokens(&sliced[close_pos + 2..], display_tokens, close_pos + 2)?;
        Ok(Some(Statement::FunctionDefinition { name: name.to_string(), params, body }))
    }

    fn parse_raw_tokens(
        &self,
        raw_tokens: &[RawToken],
        display_tokens: &[String],
        offset: usize,
    ) -> Result<Vec<Token>, Located<ParseError>> {
        let mut tokens = Vec::new();
        let mut i = 0;

        while i < raw_tokens.len() {
            // Function call: Identifier followed by "("
            if let (RawToken::Identifier(name), Some(RawToken::Operator(paren))) =
                (&raw_tokens[i], raw_tokens.get(i + 1))
            {
                if paren == "(" {
                    let args_start = i + 2;
                    let (args, consumed) = self.parse_call_args(
                        &raw_tokens[args_start..],
                        display_tokens,
                        offset + args_start,
                    )?;
                    tokens.push(Token::FunctionCall { name: name.clone(), args });
                    i += 2 + consumed;
                    continue;
                }
            }

            match self.parse_raw_token(&raw_tokens[i]) {
                Ok(t) => tokens.push(t),
                Err(err) => {
                    let ctx = ErrorContext::new(display_tokens.to_vec(), offset + i);
                    return Err(Located::new(err, ctx));
                }
            }
            i += 1;
        }

        Ok(tokens)
    }

    // Returns (parsed args, tokens consumed including the closing ")")
    fn parse_call_args(
        &self,
        raw_tokens: &[RawToken],
        display_tokens: &[String],
        offset: usize,
    ) -> Result<(Vec<Vec<Token>>, usize), Located<ParseError>> {
        let mut args: Vec<Vec<Token>> = Vec::new();
        let mut current: Vec<RawToken> = Vec::new();
        let mut depth = 0usize;

        for (i, token) in raw_tokens.iter().enumerate() {
            match token {
                RawToken::Operator(op) if op == "(" => {
                    depth += 1;
                    current.push(token.clone());
                }
                RawToken::Operator(op) if op == ")" => {
                    if depth == 0 {
                        if !current.is_empty() {
                            let mut arg_tokens = self.parse_raw_tokens(&current, display_tokens, offset)?;
                            arg_tokens.push(Token::Eof);
                            args.push(arg_tokens);
                        }
                        return Ok((args, i + 1));
                    }
                    depth -= 1;
                    current.push(token.clone());
                }
                RawToken::Operator(op) if op == "," && depth == 0 => {
                    let mut arg_tokens = self.parse_raw_tokens(&current, display_tokens, offset)?;
                    arg_tokens.push(Token::Eof);
                    args.push(arg_tokens);
                    current.clear();
                }
                _ => current.push(token.clone()),
            }
        }

        Err(Located::new(
            ParseError::UnmatchedParen,
            ErrorContext::new(display_tokens.to_vec(), offset),
        ))
    }

    fn parse_raw_token(&self, raw_token: &RawToken) -> Result<Token, ParseError> {
        match raw_token {
            RawToken::Number(body) => NumberBody::parse(body)
                .map(Token::Number)
                .ok_or_else(|| ParseError::InvalidNumber(body.clone())),
            RawToken::Operator(body) if body == "(" => Ok(Token::OpenParen),
            RawToken::Operator(body) if body == ")" => Ok(Token::CloseParen),
            RawToken::Operator(body) => self.operation_registry.get(body)
                .map(Token::Operation)
                .ok_or_else(|| ParseError::UnknownOperator(body.clone())),
            RawToken::Identifier(body) => Ok(Token::Variable(body.clone())),
            RawToken::Eof => Ok(Token::Eof),
        }
    }
}