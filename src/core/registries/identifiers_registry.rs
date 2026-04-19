use std::collections::HashMap;

pub struct IdentifiersRegistry<'a> {
    identifiers: HashMap<String, &'a str>
}

impl IdentifiersRegistry<'_> {
    pub fn new() -> IdentifiersRegistry<'static> {
        IdentifiersRegistry {
            identifiers: HashMap::new()
        }
    }

    pub fn register_identifier(&mut self, identifier: String) {}

    pub fn get_identifier(&self, identifier: &str) -> Option<&str> {
        todo!()
    }
}