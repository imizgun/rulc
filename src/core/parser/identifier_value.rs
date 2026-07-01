use crate::core::error_display::Located;
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum BuiltinValue {
    Constant(f64),
    Function {
        arity: usize,
        func: fn(&[f64]) -> f64,
    },
}

#[derive(Clone)]
pub enum IdentifierValue {
    Number(f64),
    Function(FunctionIdentifier),
    Builtin(BuiltinValue),
}

impl Display for IdentifierValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierValue::Number(n) => write!(f, "{n}"),
            IdentifierValue::Function(func) => write!(
                f,
                "fn({}) {}",
                func.parameters.join(", "),
                func.function_body
                    .iter()
                    .filter(|t| !matches!(t, Token::Eof))
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            IdentifierValue::Builtin(BuiltinValue::Constant(n)) => write!(f, "{n}"),
            IdentifierValue::Builtin(BuiltinValue::Function { arity, .. }) => {
                write!(f, "<builtin/{arity}>")
            }
        }
    }
}

#[derive(Clone)]
pub struct FunctionIdentifier {
    pub function_body: Vec<Token>,
    pub parameters: Vec<String>,
}

impl FunctionIdentifier {
    pub fn new(parameters: Vec<String>, function_body: Vec<Token>) -> FunctionIdentifier {
        FunctionIdentifier {
            function_body,
            parameters,
        }
    }

    pub fn value(
        &self,
        args: &[f64],
        global: &IdentifiersRegistry,
    ) -> Result<f64, Located<EvaluationError>> {
        if self.parameters.len() != args.len() {
            return Err(Located::unlocated(EvaluationError::ArityMismatch(
                self.parameters.len(),
                args.len(),
            )));
        }

        let mut local = global.clone();
        for (name, &val) in self.parameters.iter().zip(args.iter()) {
            local.register_identifier(name, &IdentifierValue::Number(val));
        }

        let Value::Numeric(n) = Evaluator::new(&self.function_body, &local).run()? else {
            unreachable!("boolean expressions are not yet implemented")
        };
        Ok(n)
    }
}
