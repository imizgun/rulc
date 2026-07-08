use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value::Numeric;
use crate::core::parser::identifier_value::{BuiltinValue, FunctionIdentifier, IdentifierValue};
use crate::core::parser::statement::Clear;
use crate::core::parser::statement::Clear::ClearVariable;
use crate::core::parser::statement::Statement;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::repl_output::{ReplClearOutput, ReplOutput};
use crate::core::runtime_error::RuntimeError;

const STEP: f64 = 0.01;

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

    // Collects every bare variable name referenced in a function body (including
    // inside call arguments), so a definition can be rejected if a declared
    // parameter is never actually used — e.g. `f(y) = x ^ 2`
    fn referenced_variables(tokens: &[Token]) -> std::collections::HashSet<String> {
        let mut names = std::collections::HashSet::new();
        Self::collect_referenced_variables(tokens, &mut names);
        names
    }

    fn collect_referenced_variables(
        tokens: &[Token],
        names: &mut std::collections::HashSet<String>,
    ) {
        for token in tokens {
            match token {
                Token::Variable(name) => {
                    names.insert(name.clone());
                }
                Token::FunctionCall { args, .. } => {
                    for arg in args {
                        Self::collect_referenced_variables(arg, names);
                    }
                }
                _ => {}
            }
        }
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

    fn eval_scalar(&self, tokens: &[Token]) -> Result<f64, RuntimeError> {
        let Numeric(n) = Evaluator::new(tokens, self.core.identifiers_registry()).run()? else {
            unreachable!()
        };
        Ok(n)
    }

    fn get_func_value_in(
        &self,
        func_ident: &IdentifierValue,
        from: f64,
        to: f64,
    ) -> Result<Vec<(f64, f64)>, RuntimeError> {
        let mut points = Vec::<(f64, f64)>::new();
        // step count computed once and x derived from it directly (from + i * STEP)
        // instead of repeatedly adding STEP, so rounding error doesn't accumulate
        // across thousands of samples and make the last x fall short of `to`
        let steps = ((to - from) / STEP).round() as u64;

        for i in 0..=steps {
            let x = from + i as f64 * STEP;
            let y = match &func_ident {
                IdentifierValue::Function(func) => {
                    let call_args: &[f64] = if func.parameters.is_empty() {
                        &[]
                    } else {
                        &[x]
                    };
                    match func.value(call_args, self.core.identifiers_registry()) {
                        Ok(y) => y,
                        // point-local evaluation failure (e.g. division by zero) —
                        // the function just isn't defined at this x, skip the sample
                        Err(_) => continue,
                    }
                }
                IdentifierValue::Builtin(BuiltinValue::Function { arity: 1, func }) => func(&[x]),
                IdentifierValue::Builtin(BuiltinValue::Function { arity, .. }) => {
                    return Err(RuntimeError::from(EvaluationError::ArityMismatch(
                        *arity, 1,
                    )));
                }
                _ => {
                    return Err(RuntimeError::from(EvaluationError::InvalidTokenPlace(
                        format!("'{}' is not a function", *func_ident),
                    )));
                }
            };
            points.push((x, y));
        }

        Ok(Self::filter_outliers(points))
    }

    // Drops points whose y sits far outside the bulk of the sampled values (Tukey's fences).
    // Cheap stand-in for proper domain analysis: near an asymptote (e.g. 1/x close to x=0)
    // a handful of samples blow up to huge-but-finite values and stretch the shared axis
    // until everything else on the plot flattens into a line.
    fn filter_outliers(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let mut ys: Vec<f64> = points
            .iter()
            .map(|(_, y)| *y)
            .filter(|y| y.is_finite())
            .collect();

        if ys.len() < 4 {
            return points;
        }

        ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q1 = ys[ys.len() / 4];
        let q3 = ys[ys.len() * 3 / 4];
        let iqr = q3 - q1;

        if iqr == 0.0 {
            return points;
        }

        let lower = q1 - 3.0 * iqr;
        let upper = q3 + 3.0 * iqr;

        points
            .into_iter()
            .filter(|(_, y)| y.is_finite() && *y >= lower && *y <= upper)
            .collect()
    }

    fn handle_drawing(
        &self,
        function_name: String,
        from: f64,
        to: f64,
    ) -> Result<ReplOutput, RuntimeError> {
        if from > to {
            return Err(RuntimeError::from(EvaluationError::InvalidInterval(
                from, to,
            )));
        }

        let ident = self
            .core
            .identifiers_registry()
            .get_identifier(&function_name)
            .ok_or_else(|| {
                RuntimeError::from(EvaluationError::UnknownIdentifier(function_name.clone()))
            })?;

        let points = self.get_func_value_in(&ident, from, to)?;
        Ok(ReplOutput::FuncPoints { points })
    }

    fn handle_intersection(
        &self,
        left_function_name: String,
        right_function_name: String,
        from: f64,
        to: f64,
    ) -> Result<ReplOutput, RuntimeError> {
        if from > to {
            return Err(RuntimeError::from(EvaluationError::InvalidInterval(
                from, to,
            )));
        }

        let left_ident = self
            .core
            .identifiers_registry()
            .get_identifier(&left_function_name)
            .ok_or_else(|| {
                RuntimeError::from(EvaluationError::UnknownIdentifier(
                    left_function_name.clone(),
                ))
            })?;

        let right_ident = self
            .core
            .identifiers_registry()
            .get_identifier(&right_function_name)
            .ok_or_else(|| {
                RuntimeError::from(EvaluationError::UnknownIdentifier(
                    right_function_name.clone(),
                ))
            })?;

        let left_points = self.get_func_value_in(&left_ident, from, to)?;
        let right_points = self.get_func_value_in(&right_ident, from, to)?;

        let points = left_points
            .windows(2)
            .zip(right_points.windows(2))
            .filter_map(|(l, r)| {
                let (x0, left_y0) = l[0];
                let (x1, left_y1) = l[1];
                let (_, right_y0) = r[0];
                let (_, right_y1) = r[1];

                let d0 = left_y0 - right_y0;
                let d1 = left_y1 - right_y1;

                if d0.is_nan() || d1.is_nan() {
                    return None;
                }
                if d0 == 0.0 {
                    return Some((x0, left_y0));
                }
                if d0.signum() == d1.signum() {
                    return None;
                }

                let t = d0 / (d0 - d1);
                Some((x0 + t * (x1 - x0), left_y0 + t * (left_y1 - left_y0)))
            })
            .collect();

        Ok(ReplOutput::IntersectionPoints { points })
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
                let used = Self::referenced_variables(&body);
                if let Some(unused) = params.iter().find(|p| !used.contains(*p)) {
                    return Err(RuntimeError::from(EvaluationError::UnusedParameter(
                        unused.clone(),
                    )));
                }
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

                self.handle_drawing(function_name, from, to)
            }
            Statement::IntersectionCommand {
                left_function_name,
                right_function_name,
                from_tokens,
                to_tokens,
            } => {
                let from = self.eval_scalar(&from_tokens)?;
                let to = self.eval_scalar(&to_tokens)?;

                self.handle_intersection(left_function_name, right_function_name, from, to)
            }
            Statement::ClearCommand(com) => match com {
                Clear::ClearAll => Ok(ReplOutput::Clear(ReplClearOutput::ClearAll)),
                Clear::ClearPlots => Ok(ReplOutput::Clear(ReplClearOutput::ClearPlots)),
                Clear::ClearOutput => Ok(ReplOutput::Clear(ReplClearOutput::ClearHistory)),
                Clear::ClearMemory => {
                    self.core
                        .identifiers_registry_mut()
                        .clear_user_identifiers();
                    Ok(ReplOutput::Clear(ReplClearOutput::ClearMemory))
                }
                ClearVariable(var) => {
                    let is_user_defined = matches!(
                        self.core.identifiers_registry().get_identifier(&var),
                        Some(IdentifierValue::Number(_)) | Some(IdentifierValue::Function(_))
                    );
                    if !is_user_defined {
                        return Err(RuntimeError::from(EvaluationError::UnknownClearTarget(var)));
                    }
                    self.core.identifiers_registry_mut().remove_identifier(&var);
                    Ok(ReplOutput::Clear(ReplClearOutput::ClearVariable(var)))
                }
            },
            Statement::Help => Ok(ReplOutput::Help),
        }
    }
}
