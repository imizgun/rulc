use std::sync::Arc;
use crate::core::lexer::lexer::Lexer;
use crate::core::operations::division::DivisionOperation;
use crate::core::operations::minus::MinusOperation;
use crate::core::operations::multiply::MultiplyOperation;
use crate::core::operations::pow::PowOperation;
use crate::core::operations::sum::SumOperation;
use crate::core::parser::identifier_value::{BuiltinValue, IdentifierValue};
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
        operation_registry.register(Arc::new(PowOperation));

        let mut identifiers_registry = IdentifiersRegistry::new();

        let fn1 = |f: fn(&[f64]) -> f64| IdentifierValue::Builtin(BuiltinValue::Function { arity: 1, func: f });

        identifiers_registry.register_identifier("sin",   &fn1(|a| a[0].sin()));
        identifiers_registry.register_identifier("cos",   &fn1(|a| a[0].cos()));
        identifiers_registry.register_identifier("tan",   &fn1(|a| a[0].tan()));
        identifiers_registry.register_identifier("asin",  &fn1(|a| a[0].asin()));
        identifiers_registry.register_identifier("acos",  &fn1(|a| a[0].acos()));
        identifiers_registry.register_identifier("atan",  &fn1(|a| a[0].atan()));
        identifiers_registry.register_identifier("sqrt",  &fn1(|a| a[0].sqrt()));
        identifiers_registry.register_identifier("ln",    &fn1(|a| a[0].ln()));
        identifiers_registry.register_identifier("log",   &fn1(|a| a[0].log10()));
        identifiers_registry.register_identifier("abs",   &fn1(|a| a[0].abs()));
        identifiers_registry.register_identifier("ceil",  &fn1(|a| a[0].ceil()));
        identifiers_registry.register_identifier("floor", &fn1(|a| a[0].floor()));

        identifiers_registry.register_identifier("pi", &IdentifierValue::Builtin(BuiltinValue::Constant(std::f64::consts::PI)));
        identifiers_registry.register_identifier("e",  &IdentifierValue::Builtin(BuiltinValue::Constant(std::f64::consts::E)));

        CoreInitializer {
            operation_registry,
            identifiers_registry,
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