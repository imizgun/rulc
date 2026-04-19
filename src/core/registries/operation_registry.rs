use std::collections::HashMap;
use crate::core::operations::operation::Operation;

pub struct OperationRegistry {
    registered_operations: HashMap<String, Box<dyn Operation>>
}

impl OperationRegistry {
    pub fn new() -> OperationRegistry {
        OperationRegistry {registered_operations: HashMap::new()}
    }
    
    pub fn register(&mut self, operation: Box<dyn Operation>) {
        self.registered_operations.insert(operation.get_sign().to_string(), operation);
    }
    
    pub fn get(&self, operation_name: &str) -> Option<&Box<dyn Operation>> {
        self.registered_operations.get(operation_name)
    }
}