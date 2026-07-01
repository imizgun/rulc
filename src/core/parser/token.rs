use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::operations::operation::Operation;
use crate::core::parser::identifier_value::{BuiltinValue, IdentifierValue};
use crate::core::parser::numeric::number_body::NumberBody;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub enum Token {
    Number(NumberBody),
    Variable(String),
    FunctionCall { name: String, args: Vec<Vec<Token>> },
    Operation(Arc<dyn Operation>),
    OpenParen,
    CloseParen,
    Eof,
}

impl Token {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Token::Number(body) => Some(body.decimal_value),
            _ => None,
        }
    }
}

fn eval_args(args: &[Vec<Token>], evaluator: &mut Evaluator) -> Result<Vec<f64>, EvaluationError> {
    let mut values = Vec::with_capacity(args.len());
    for arg_tokens in args {
        match Evaluator::new(arg_tokens, evaluator.identifier_registry)
            .run()
            .map_err(|e| e.error)?
        {
            Value::Numeric(n) => values.push(n),
            other => return Err(EvaluationError::InvalidTokenPlace(other.to_string())),
        }
    }
    Ok(values)
}

impl EvaluationRule for Token {
    fn nud(&self, evaluator: &mut Evaluator) -> Result<Token, EvaluationError> {
        match self {
            Token::Number(n) => Ok(Token::Number(n.clone())),
            Token::Operation(op) => op.nud(evaluator),
            Token::OpenParen => {
                let result = evaluator.evaluate(0).map_err(|e| e.error)?;
                evaluator.consume();
                match result {
                    Value::Numeric(n) => Ok(Token::Number(NumberBody::from(n))),
                    other => Err(EvaluationError::InvalidTokenPlace(other.to_string())),
                }
            }
            Token::Variable(name) => match evaluator.identifier_registry.get_identifier(name) {
                Some(IdentifierValue::Number(num)) => Ok(Token::Number(NumberBody::from(num))),
                Some(IdentifierValue::Builtin(BuiltinValue::Constant(n))) => {
                    Ok(Token::Number(NumberBody::from(n)))
                }
                Some(IdentifierValue::Function(_))
                | Some(IdentifierValue::Builtin(BuiltinValue::Function { .. })) => {
                    Err(EvaluationError::InvalidTokenPlace(format!(
                        "'{}' is a function, use {}(...) to call it",
                        name, name
                    )))
                }
                None => Err(EvaluationError::UnknownIdentifier(name.clone())),
            },
            Token::FunctionCall { name, args } => {
                match evaluator.identifier_registry.get_identifier(name) {
                    Some(IdentifierValue::Function(func)) => {
                        let mut arg_values = eval_args(args, evaluator)?;
                        func.value(&arg_values, evaluator.identifier_registry)
                            .map_err(|e| e.error)
                            .map(|n| Token::Number(NumberBody::from(n)))
                    }
                    Some(IdentifierValue::Builtin(BuiltinValue::Function { arity, func })) => {
                        if args.len() != arity {
                            return Err(EvaluationError::ArityMismatch(arity, args.len()));
                        }
                        let arg_values = eval_args(args, evaluator)?;
                        Ok(Token::Number(NumberBody::from(func(&arg_values))))
                    }
                    Some(IdentifierValue::Number(_))
                    | Some(IdentifierValue::Builtin(BuiltinValue::Constant(_))) => {
                        Err(EvaluationError::InvalidTokenPlace(format!(
                            "'{}' is a number, not a function",
                            name
                        )))
                    }
                    None => Err(EvaluationError::UnknownIdentifier(name.clone())),
                }
            }
            Token::CloseParen | Token::Eof => {
                Err(EvaluationError::InvalidTokenPlace(self.to_string()))
            }
        }
    }

    fn led(&self, evaluator: &mut Evaluator, left: &Token) -> Result<Token, EvaluationError> {
        match self {
            Token::Operation(op) => op.default_led(evaluator, left),
            _ => Err(EvaluationError::MissingOperand),
        }
    }

    fn lbp(&self) -> u32 {
        match self {
            Token::Operation(op) => op.lbp(),
            _ => 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(b) => write!(f, "{}", b.raw),
            Token::Variable(v) => write!(f, "{}", v),
            Token::FunctionCall { name, args } => {
                let args_str: Vec<String> = args
                    .iter()
                    .map(|arg| {
                        arg.iter()
                            .filter(|t| !matches!(t, Token::Eof))
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
            Token::Operation(op) => write!(f, "{}", op.get_sign()),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
