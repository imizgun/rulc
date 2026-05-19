use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::parser::identifier_value::IdentifierValue;
use crate::core::parser::statement::Statement;
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

    pub fn evaluate(&mut self, string: &str) -> Result<f64, RuntimeError> {
        let statement = {
            let parser = self.core.build_parser();
            parser.parse(string)?
        };

        match statement {
            Statement::Expression(tokens) => {
                let mut evaluator = Evaluator::new(&tokens, self.core.identifiers_registry());
                Ok(evaluator.run()?)
            }
            Statement::Assignment { name, tokens } => {
                let value = {
                    let mut evaluator = Evaluator::new(&tokens, self.core.identifiers_registry());
                    IdentifierValue::Number(evaluator.run()?)
                };
                self.core.identifiers_registry_mut().register_identifier(&name, &value);
                Ok(match value { 
                    IdentifierValue::Number(value) => value,
                    IdentifierValue::Function(function) => function.value(self.core.identifiers_registry())?
                })
            }
        }
    }
}