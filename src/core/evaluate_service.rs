use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value::Numeric;
use crate::core::parser::identifier_value::{BuiltinValue, FunctionIdentifier, IdentifierValue};
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

    fn guard_builtin(&self, name: &str) -> Result<(), RuntimeError> {
        if matches!(
            self.core.identifiers_registry().get_identifier(name),
            Some(IdentifierValue::Builtin(_))
        ) {
            return Err(RuntimeError::from(EvaluationError::InvalidTokenPlace(
                format!("'{}' is a builtin and cannot be redefined", name),
            )));
        }
        Ok(())
    }

    fn eval_scalar(
        &self,
        tokens: &[crate::core::parser::token::Token],
    ) -> Result<f64, RuntimeError> {
        let Numeric(n) = Evaluator::new(tokens, self.core.identifiers_registry()).run()? else {
            unreachable!()
        };
        Ok(n)
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
                self.guard_builtin(&name)?;
                let mut evaluator = Evaluator::new(&tokens, self.core.identifiers_registry());
                let Numeric(num) = evaluator.run()? else {
                    unreachable!("boolean expressions are not yet implemented")
                };
                self.core
                    .identifiers_registry_mut()
                    .register_identifier(&name, &IdentifierValue::Number(num));
                Ok(ReplOutput::Message(format!("{} = {}", name, num)))
            }
            Statement::FunctionDefinition { name, params, body } => {
                self.guard_builtin(&name)?;
                let func = FunctionIdentifier::new(params, body);
                self.core
                    .identifiers_registry_mut()
                    .register_identifier(&name, &IdentifierValue::Function(func));
                Ok(ReplOutput::Message(format!("function '{}' defined", name)))
            }
            Statement::DrawCommand {
                function_name,
                from_tokens,
                to_tokens,
            } => {
                let from = self.eval_scalar(&from_tokens)?;
                let to = self.eval_scalar(&to_tokens)?;

                let ident = self
                    .core
                    .identifiers_registry()
                    .get_identifier(&function_name)
                    .ok_or_else(|| {
                        RuntimeError::from(EvaluationError::UnknownIdentifier(
                            function_name.clone(),
                        ))
                    })?;

                let mut points = Vec::<(f64, f64)>::new();
                let mut x = from;
                while x <= to {
                    let y = match &ident {
                        IdentifierValue::Function(func) => {
                            let call_args: &[f64] = if func.parameters.is_empty() {
                                &[]
                            } else {
                                &[x]
                            };
                            func.value(call_args, self.core.identifiers_registry())
                                .map_err(RuntimeError::from)?
                        }
                        IdentifierValue::Builtin(BuiltinValue::Function { arity: 1, func }) => {
                            func(&[x])
                        }
                        IdentifierValue::Builtin(BuiltinValue::Function { arity, .. }) => {
                            return Err(RuntimeError::from(EvaluationError::ArityMismatch(
                                *arity, 1,
                            )));
                        }
                        _ => {
                            return Err(RuntimeError::from(EvaluationError::InvalidTokenPlace(
                                format!("'{}' is not a function", function_name),
                            )));
                        }
                    };
                    points.push((x, y));
                    x += STEP;
                }
                Ok(ReplOutput::FuncPoints { points })
            }
        }
    }
}
