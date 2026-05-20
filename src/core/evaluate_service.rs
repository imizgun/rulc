use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::parser::identifier_value::{FunctionIdentifier, IdentifierValue};
use crate::core::parser::statement::Statement;
use crate::core::repl_output::ReplOutput;
use crate::core::runtime_error::RuntimeError;

pub struct EvaluateService {
    core: CoreInitializer,
}

impl EvaluateService {
    pub fn new() -> EvaluateService {
        EvaluateService {
            core: CoreInitializer::new(),
        }
    }

    pub fn evaluate(&mut self, string: &str) -> Result<ReplOutput, RuntimeError> {
        let statement = {
            let parser = self.core.build_parser();
            parser.parse(string)?
        };

        match statement {
            Statement::Expression(tokens) => {
                let mut evaluator = Evaluator::new(&tokens, self.core.identifiers_registry());
                Ok(ReplOutput::Value(evaluator.run()?))
            }
            Statement::Assignment { name, tokens } => {
                let mut evaluator = Evaluator::new(&tokens, self.core.identifiers_registry());
                let Value::Numeric(num) = evaluator.run()? else {
                    unreachable!("boolean expressions are not yet implemented")
                };
                self.core.identifiers_registry_mut()
                    .register_identifier(&name, &IdentifierValue::Number(num));
                Ok(ReplOutput::Message(format!("{} = {}", name, num)))
            }
            Statement::FunctionDefinition { name, params, body } => {
                let func = FunctionIdentifier::new(params, body);
                self.core.identifiers_registry_mut()
                    .register_identifier(&name, &IdentifierValue::Function(func));
                Ok(ReplOutput::Message(format!("function '{}' defined", name)))
            }
        }
    }
}