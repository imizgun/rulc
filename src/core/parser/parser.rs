use crate::core::parser::parser_state::ParserState;
use crate::core::parser::raw_token::RawToken;
use crate::core::parser::token::Token;
use crate::core::registry::operation_registry::OperationRegistry;

pub struct Parser {
    // operation_registry: &'a OperationRegistry
}

impl Parser {
    // pub fn new(registry: &OperationRegistry ) -> Parser {
    //     Parser {
    //         operation_registry: registry
    //     }
    // }

    pub fn parse_expression(expression: &str) -> Vec<Token> {
        let mut tokens : Vec<Token> = Vec::new();

        let sliced = Self::slice_input_string(expression);

        println!("{:?}", sliced);

        tokens
    }

    // 32 + 3 * 2
    pub fn slice_input_string(expression: &str) -> Vec<RawToken> {
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
                raw.push(Self::classify_raw_token(&state, &buffer));
                buffer.clear();
            }

            state = next_state;

            if state != ParserState::Idle {
                buffer.push(ch);
            }
        }

        if !buffer.is_empty() {
            raw.push(Self::classify_raw_token(&state, &buffer));
        }

        raw
    }

    fn classify_raw_token(state: &ParserState, buffer: &str) -> RawToken {
        match state {
            ParserState::Number => RawToken::Number(buffer.trim().to_string()),
            ParserState::Identifier => RawToken::Identifier(buffer.trim().to_string()),
            ParserState::Operator => RawToken::Operator(buffer.trim().to_string()),
            _ => unreachable!()
        }
    }
}