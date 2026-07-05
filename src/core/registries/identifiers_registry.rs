use crate::core::parser::identifier_value::IdentifierValue;
use std::collections::HashMap;

#[derive(Clone)]
pub struct IdentifiersRegistry {
    identifiers: HashMap<String, IdentifierValue>,
}

impl IdentifiersRegistry {
    pub fn new() -> IdentifiersRegistry {
        IdentifiersRegistry {
            identifiers: HashMap::new(),
        }
    }

    pub fn get_identifier(&self, identifier: &str) -> Option<IdentifierValue> {
        self.identifiers.get(identifier).cloned()
    }

    pub fn register_identifier(&mut self, identifier: &str, value: &IdentifierValue) {
        self.identifiers
            .insert(identifier.to_owned(), value.clone());
    }

    pub fn remove_identifier(&mut self, identifier: &str) -> Option<IdentifierValue> {
        self.identifiers.remove(identifier)
    }

    pub fn clear_user_identifiers(&mut self) {
        self.identifiers
            .retain(|_, v| matches!(v, IdentifierValue::Builtin(_)));
    }

    pub fn user_entries(&self) -> Vec<(&String, &IdentifierValue)> {
        let mut entries: Vec<_> = self
            .identifiers
            .iter()
            .filter(|x| !matches!(x.1, IdentifierValue::Builtin(_)))
            .collect();

        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }
}
