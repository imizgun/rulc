use std::collections::HashMap;

pub struct IdentifiersRegistry {
    identifiers: HashMap<String, f64>
}

impl IdentifiersRegistry {
    pub fn new() -> IdentifiersRegistry {
        IdentifiersRegistry {
            identifiers: HashMap::new()
        }
    }

    pub fn get_identifier(&self, identifier: &str) -> Option<f64> {
        self.identifiers.get(identifier).cloned()
    }

    pub fn register_identifier(&mut self, identifier: &str, value: f64) {
        self.identifiers.insert(identifier.to_owned(), value);
    }
}