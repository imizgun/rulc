use crate::core::error_display::{ErrorContext, Located};
use crate::core::lexer::lexer::Lexer;
use crate::core::lexer::raw_token::RawToken;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::parsable::Parsable;
use crate::core::parser::parse_error::ParseError;
use crate::core::parser::statement::Statement;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::registries::operation_registry::OperationRegistry;

pub struct Parser<'a> {
    operation_registry: &'a OperationRegistry,
    identifiers_registry: &'a IdentifiersRegistry,
    lexer: &'a Lexer,
}

impl Parser<'_> {
    pub fn new<'a>(
        operation_registry: &'a OperationRegistry,
        identifiers_registry: &'a IdentifiersRegistry,
        lexer: &'a Lexer,
    ) -> Parser<'a> {
        Parser { operation_registry, identifiers_registry, lexer }
    }

    pub fn parse(&self, input: &str) -> Result<Statement, Located<ParseError>> {
        let sliced = self.lexer.slice_input_string(input.trim());

        let display_tokens: Vec<String> = sliced.iter()
            .filter(|t| !matches!(t, RawToken::Eof))
            .map(|t| t.to_string())
            .collect();

        if let (Some(RawToken::Identifier(name)), Some(RawToken::Operator(op))) =
            (sliced.get(0), sliced.get(1))
        {
            if op == "=" {
                let name = name.clone();
                let rhs = &sliced[2..];
                let tokens = self.parse_raw_tokens(rhs, &display_tokens, 2)?;
                return Ok(Statement::Assignment { name, tokens });
            }
            else if op.ends_with("=") { 
                let base_operation_sign = op[..op.len() - 1].trim();
                // x += 12 -> x = x + 12
                if let Some(base_op) = self.operation_registry.get(base_operation_sign) {
                    let expanded_rhs = &mut sliced[2..].to_vec();
                    expanded_rhs.insert(0, RawToken::Operator(base_operation_sign.to_string()));
                    expanded_rhs.insert(0, RawToken::Identifier(name.clone()));
                    
                    let tokens = self.parse_raw_tokens(expanded_rhs, &display_tokens, 2)?;
                    
                    return Ok(Statement::Assignment {name: name.clone(), tokens})
                }
            }
        }

        let tokens = self.parse_raw_tokens(&sliced, &display_tokens, 0)?;
        Ok(Statement::Expression(tokens))
    }

    fn parse_raw_tokens(
        &self,
        raw_tokens: &[RawToken],
        display_tokens: &[String],
        offset: usize,
    ) -> Result<Vec<Token>, Located<ParseError>> {
        let mut tokens = Vec::new();
        for (i, raw) in raw_tokens.iter().enumerate() {
            match self.parse_raw_token(raw) {
                Ok(t) => tokens.push(t),
                Err(err) => {
                    let ctx = ErrorContext::new(display_tokens.to_vec(), offset + i);
                    return Err(Located::new(err, ctx));
                }
            }
        }
        Ok(tokens)
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
            RawToken::Identifier(body) => {
                if self.identifiers_registry.get_identifier(body).is_some() {
                    Ok(Token::Variable(body.clone()))
                } else {
                    Err(ParseError::UnknownIdentifier(body.clone()))
                }
            }
            RawToken::Eof => Ok(Token::Eof),
        }
    }
}