use crate::core::operations::operation::Operation;
use std::collections::HashMap;
use std::sync::Arc;

pub struct OperationRegistry {
    registered_operations: HashMap<String, Arc<dyn Operation>>,
}

impl OperationRegistry {
    pub fn new() -> OperationRegistry {
        OperationRegistry {
            registered_operations: HashMap::new(),
        }
    }

    pub fn register(&mut self, operation: Arc<dyn Operation>) {
        self.registered_operations
            .insert(operation.get_sign().to_string(), operation);
    }

    pub fn get(&self, operation_name: &str) -> Option<Arc<dyn Operation>> {
        self.registered_operations.get(operation_name).cloned()
    }
}
