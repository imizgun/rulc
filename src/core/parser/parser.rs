use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::parsable::Parsable;
use crate::core::parser::parser_state::ParserState;
use crate::core::parser::raw_token::RawToken;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::registries::operation_registry::OperationRegistry;

pub struct Parser<'a> {
    operation_registry: &'a OperationRegistry,
    identifiers_registry: &'a IdentifiersRegistry<'a>
}

impl Parser<'_> {
    pub fn new<'a>(operation_registry: &'a OperationRegistry, identifiers_registry: &'a IdentifiersRegistry<'a>) -> Parser<'a> {
        Parser {
            operation_registry,
            identifiers_registry
        }
    }

    pub fn parse_expression(&self, expression: &str) -> Vec<Token> {
        let sliced = self.slice_input_string(expression.trim());

        let mut tokens: Vec<Token> = Vec::new();

        for i in &sliced {
            let parsed_token = self.parse_raw_token(i)
                .expect(&format!("Failed to parse raw token: {:?}", i));

            tokens.push(parsed_token);
        }

        println!("{:?}", sliced);

        tokens
    }

    // 32 + 3 * 2
    pub fn slice_input_string(&self, expression: &str) -> Vec<RawToken> {
        let mut raw = Vec::<RawToken>::new();
        let mut state: ParserState = ParserState::Idle;
        let mut buffer = String::new();

        for ch in expression.chars() {
            let next_state = match ch {
                '0'..='9' | '.' => ParserState::Number,
                'a'..='z' | 'A'..='Z' => ParserState::Identifier,
                ' ' => ParserState::Idle,
                _ => ParserState::Operator
            };

            if state != next_state && !buffer.is_empty() {
                raw.push(self.classify_raw_token(&state, &buffer));
                buffer.clear();
            }

            state = next_state;

            if state != ParserState::Idle {
                buffer.push(ch);
            }
        }

        if !buffer.is_empty() {
            raw.push(self.classify_raw_token(&state, &buffer));
        }
        
        raw.push(RawToken::Eof);

        raw
    }

    fn classify_raw_token(&self, state: &ParserState, buffer: &str) -> RawToken {
        match state {
            ParserState::Number => RawToken::Number(buffer.trim().to_string()),
            ParserState::Identifier => RawToken::Identifier(buffer.trim().to_string()),
            ParserState::Operator => RawToken::Operator(buffer.trim().to_string()),
            _ => unreachable!()
        }
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