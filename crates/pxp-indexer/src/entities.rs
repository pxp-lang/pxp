use std::fmt::{Debug, Formatter};

use pxp_span::Span;
use pxp_symbol::{Symbol, SymbolTable};
use pxp_type::Type;

use crate::Location;

#[derive(Debug, Clone, Default)]
pub struct FunctionEntity {
    pub name: Symbol,
    pub short_name: Symbol,
    pub parameters: Vec<ParameterEntity>,
    pub return_type: Type,
    pub location: Location,
}

// impl Debug for FunctionEntity {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}({}): {} -> {}@{}:{}", )
//     }
// }

#[derive(Debug, Clone, Default)]
pub struct ParameterEntity {
    pub name: Symbol,
    pub reference: bool,
    pub variadic: bool,
    pub optional: bool,
    pub r#type: Type,
    pub location: Location,
}

pub struct DebuggableEntityWithSymbolTable<'a, T> {
    entity: T,
    symbol_table: &'a SymbolTable,
    debugger: Box<dyn Fn(&T, &'a SymbolTable, &mut Formatter)>,
}

pub fn debuggable_entity<T>(entity: T, symbol_table: &SymbolTable, debugger: Box<dyn Fn(&T, &SymbolTable, &mut Formatter)>) -> DebuggableEntityWithSymbolTable<T> {
    DebuggableEntityWithSymbolTable {
        entity,
        symbol_table,
        debugger
    }
}
