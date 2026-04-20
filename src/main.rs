mod core;

use std::sync::Arc;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::operations::multiply::MultiplyOperation;
use crate::core::operations::sum::SumOperation;
use crate::core::parser::parser::Parser;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::registries::operation_registry::OperationRegistry;

fn main() {
    let mut str = String::new();

    _ = std::io::stdin().read_line(&mut str).unwrap();
    
    let sum_operation = SumOperation {};
    let multiply_operation = MultiplyOperation {};

    let mut operation_registry = OperationRegistry::new();
    operation_registry.register(Arc::new(sum_operation));
    operation_registry.register(Arc::new(multiply_operation));
    
    let identifiers_registry = IdentifiersRegistry::new();
    let parser = Parser::new(&operation_registry, &identifiers_registry);

    let tokens = parser.parse_expression(&str);
    
    let mut evaluator = Evaluator::new(tokens);
    
    let res = evaluator.evaluate(0);
    
    println!("{}", res);
}