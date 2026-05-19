use std::sync::Arc;
use crate::core::lexer::lexer::Lexer;
use crate::core::operations::division::DivisionOperation;
use crate::core::operations::minus::MinusOperation;
use crate::core::operations::multiply::MultiplyOperation;
use crate::core::operations::sum::SumOperation;
use crate::core::parser::parser::Parser;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;
use crate::core::registries::operation_registry::OperationRegistry;

pub struct CoreInitializer {
    operation_registry: OperationRegistry,
    identifiers_registry: IdentifiersRegistry,
    lexer: Lexer,
}

impl CoreInitializer {
    pub fn new() -> CoreInitializer {
        let mut operation_registry = OperationRegistry::new();
        operation_registry.register(Arc::new(SumOperation));
        operation_registry.register(Arc::new(MultiplyOperation));
        operation_registry.register(Arc::new(MinusOperation));
        operation_registry.register(Arc::new(DivisionOperation));

        CoreInitializer {
            operation_registry,
            identifiers_registry: IdentifiersRegistry::new(),
            lexer: Lexer,
        }
    }

    pub fn build_parser(&self) -> Parser<'_> {
        Parser::new(&self.operation_registry, &self.lexer)
    }

    pub fn identifiers_registry(&self) -> &IdentifiersRegistry {
        &self.identifiers_registry
    }

    pub fn identifiers_registry_mut(&mut self) -> &mut IdentifiersRegistry {
        &mut self.identifiers_registry
    }
}