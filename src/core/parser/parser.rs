use crate::core::lexer::lexer::Lexer;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::parsable::Parsable;
use crate::core::lexer::raw_token::RawToken;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::registries::operation_registry::OperationRegistry;

pub struct Parser<'a> {
    operation_registry: &'a OperationRegistry,
    identifiers_registry: &'a IdentifiersRegistry<'a>,
    lexer: &'a Lexer
}

impl Parser<'_> {
    pub fn new<'a>(operation_registry: &'a OperationRegistry,
                   identifiers_registry: &'a IdentifiersRegistry<'a>,
                   lexer: &'a Lexer
    ) -> Parser<'a> {
        Parser {
            operation_registry,
            identifiers_registry,
            lexer
        }
    }

    pub fn parse_expression(&self, expression: &str) -> Vec<Token> {
        let sliced = self.lexer.slice_input_string(expression.trim());

        let mut tokens: Vec<Token> = Vec::new();

        for i in &sliced {
            let parsed_token = self.parse_raw_token(i)
                .expect(&format!("Failed to parse raw token: {:?}", i));

            tokens.push(parsed_token);
        }

        tokens
    }

    fn parse_raw_token(&self, raw_token: &RawToken) -> Option<Token> {
        match raw_token {
            RawToken::Number(body) => {
                let num_body = NumberBody::parse(body)?;
                Some(Token::Number(num_body))
            },
            RawToken::Operator(body) => {
                let operation = self.operation_registry.get(body)?;
                Some(Token::Operation(operation))
            },
            RawToken::Identifier(body) => {
                let identifier = self.identifiers_registry.get_identifier(body)?;
                Some(Token::Variable(identifier.to_string()))
            },
            RawToken::Eof => Some(Token::Eof),
            _ => unreachable!()
        }
    }
}