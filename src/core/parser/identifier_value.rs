use crate::core::error_display::Located;
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;

#[derive(Clone)]
pub enum IdentifierValue {
    Number(f64),
    Function(FunctionIdentifier)
}

#[derive(Clone)]
pub struct FunctionIdentifier {
    pub function_body: Vec<Token>,
    pub parameters: Vec<String>,
}

impl FunctionIdentifier {
    pub fn new(parameters: Vec<String>, function_body: Vec<Token>) -> FunctionIdentifier {
        FunctionIdentifier { function_body, parameters }
    }

    pub fn value(&self, args: &[f64], global: &IdentifiersRegistry) -> Result<f64, Located<EvaluationError>> {
        if self.parameters.len() != args.len() {
            return Err(Located::unlocated(
                EvaluationError::ArityMismatch(self.parameters.len(), args.len())
            ));
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