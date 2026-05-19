use crate::core::evaluator::evaluator::Evaluator;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::runtime_error::RuntimeError;

#[derive(Clone)]
pub enum IdentifierValue {
    Number(f64),
    Function(FunctionIdentifier)
}

#[derive(Clone)]
pub struct FunctionIdentifier {
    pub function_body: Vec<Token>
}

impl FunctionIdentifier {
    pub fn new(function_body: Vec<Token>) -> FunctionIdentifier {
        FunctionIdentifier {
            function_body
        }
    }

    pub fn value(&self, identifiers_registry: &IdentifiersRegistry) -> Result<f64, RuntimeError> {
        let mut evaluator = Evaluator::new(& self.function_body, identifiers_registry);

         match evaluator.run() {
             Ok(value) => Ok(value),
             Err(err) => Err(RuntimeError::from(err))
         }
    }
}