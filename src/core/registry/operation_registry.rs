use std::collections::HashMap;
use crate::core::evaluator::evaluation_rule::EvaluationRule;

pub struct OperationRegistry {
    registered_operations: HashMap<String, Box<dyn EvaluationRule>>
}