use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::evaluator::evaluator_result::Value::Numeric;
use crate::core::parser::identifier_value::{FunctionIdentifier, IdentifierValue};
use crate::core::parser::statement::Statement;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::repl_output::ReplOutput;
use crate::core::runtime_error::RuntimeError;

const STEP: f64 = 0.5;

pub struct EvaluateService {
    core: CoreInitializer,
}

impl EvaluateService {
    pub fn new() -> EvaluateService {
        EvaluateService {
            core: CoreInitializer::new(),
        }
    }

    pub fn identifiers_registry(&self) -> &IdentifiersRegistry {
        self.core.identifiers_registry()
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
            },
            Statement::DrawCommand {function_name, from_tokens, to_tokens} => {
                let func_opt = self.core.identifiers_registry().get_identifier(&function_name);

                if let Some(func) = func_opt {
                    match func {
                        IdentifierValue::Function(func) => {
                            let mut from_evaluator = Evaluator::new(&from_tokens, self.core.identifiers_registry());
                            let mut to_evaluator = Evaluator::new(&to_tokens, self.core.identifiers_registry());
                            let from_val = from_evaluator.run()?;
                            let to_val = to_evaluator.run()?;

                            let mut points = Vec::<(f64, f64)>::new();

                            match (from_val, to_val) {
                                (Numeric(from), Numeric(to)) => {
                                    let mut x = from;
                                    while x <= to {
                                        points.push((x, func.value(&[x], self.core.identifiers_registry())?));
                                        x += STEP;
                                    }
                                    return Ok(ReplOutput::FuncPoints { points });
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => Err(RuntimeError::from(
                            EvaluationError::InvalidTokenPlace(format!("'{}' is not a function", function_name)))
                        )
                    }
                }
                else {
                    return Err(RuntimeError::from(EvaluationError::UnknownIdentifier(function_name)));
                }
            }
        }
    }
}