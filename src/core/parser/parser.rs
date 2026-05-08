use crate::core::error_display::{ErrorContext, Located};
use crate::core::lexer::lexer::Lexer;
use crate::core::lexer::raw_token::RawToken;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::parsable::Parsable;
use crate::core::parser::parse_error::ParseError;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::registries::operation_registry::OperationRegistry;

pub struct Parser<'a> {
    operation_registry: &'a OperationRegistry,
    identifiers_registry: &'a IdentifiersRegistry<'a>,
    lexer: &'a Lexer,
}

impl Parser<'_> {
    pub fn new<'a>(
        operation_registry: &'a OperationRegistry,
        identifiers_registry: &'a IdentifiersRegistry<'a>,
        lexer: &'a Lexer,
    ) -> Parser<'a> {
        Parser { operation_registry, identifiers_registry, lexer }
    }

    pub fn parse_expression(&self, expression: &str) -> Result<Vec<Token>, Located<ParseError>> {
        let sliced = self.lexer.slice_input_string(expression.trim());

        let display_tokens: Vec<String> = sliced.iter()
            .filter(|t| !matches!(t, RawToken::Eof))
            .map(|t| t.to_string())
            .collect();

        let mut tokens: Vec<Token> = Vec::new();

        for i in 0..sliced.len() {
            match self.parse_raw_token(&sliced[i]) {
                Ok(t) => tokens.push(t),
                Err(err) => {
                    let ctx = ErrorContext::new(display_tokens, i);
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
            RawToken::Operator(body) => self.operation_registry.get(body)
                .map(Token::Operation)
                .ok_or_else(|| ParseError::UnknownOperator(body.clone())),
            RawToken::Identifier(body) => self.identifiers_registry.get_identifier(body)
                .map(|x| Token::Variable(x.to_string()))
                .ok_or_else(|| ParseError::UnknownIdentifier(body.clone())),
            RawToken::Eof => Ok(Token::Eof),
            _ => unreachable!(),
        }
    }
}