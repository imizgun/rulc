use std::collections::HashMap;
use crate::core::operations::calculation_handler::CalculationHandler;

struct OperationRegistry {
    registered_operations: HashMap<String, Box<dyn CalculationHandler>>
}