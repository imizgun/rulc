use crate::core::lexer::lexer_state::LexerState;
use crate::core::lexer::raw_token::RawToken;

pub struct Lexer;

impl Lexer {
    pub fn slice_input_string(&self, expression: &str) -> Vec<RawToken> {
        let mut raw = Vec::<RawToken>::new();
        let mut state: LexerState = LexerState::Idle;
        let mut buffer = String::new();

        for ch in expression.chars() {
            if matches!(ch, '(' | ')' | '[' | ']' | ',') {
                if !buffer.is_empty() {
                    raw.push(self.classify_raw_token(&state, &buffer));
                    buffer.clear();
                    state = LexerState::Idle;
                }
                raw.push(RawToken::Operator(ch.to_string()));
                continue;
            }

            let next_state = match ch {
                '0'..='9' | '.' => LexerState::Number,
                c if c.is_alphabetic() => LexerState::Identifier,
                ' ' => LexerState::Idle,
                _ => LexerState::Operator,
            };

            if state != next_state && !buffer.is_empty() {
                raw.push(self.classify_raw_token(&state, &buffer));
                buffer.clear();
            }

            state = next_state;

            if state != LexerState::Idle {
                buffer.push(ch);
            }
        }

        if !buffer.is_empty() {
            raw.push(self.classify_raw_token(&state, &buffer));
        }

        raw.push(RawToken::Eof);

        raw
    }

    fn classify_raw_token(&self, state: &LexerState, buffer: &str) -> RawToken {
        match state {
            LexerState::Number => RawToken::Number(buffer.trim().to_string()),
            LexerState::Identifier => RawToken::Identifier(buffer.trim().to_string()),
            LexerState::Operator => RawToken::Operator(buffer.trim().to_string()),
            _ => unreachable!(),
        }
    }
}
