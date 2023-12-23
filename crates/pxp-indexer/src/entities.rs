use std::fmt::{Debug, Formatter};

use pxp_ast::{modifiers::Visibility, enums::BackedEnumType};
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

#[derive(Debug, Clone, Default)]
pub struct ParameterEntity {
    pub name: Symbol,
    pub reference: bool,
    pub variadic: bool,
    pub optional: bool,
    pub r#type: Type,
    pub location: Location,
}

#[derive(Debug, Clone, Default)]
pub struct ClassLikeEntity {
    pub name: Symbol,
    pub short_name: Symbol,
    pub is_class: bool,
    pub is_interface: bool,
    pub is_enum: bool,
    pub is_trait: bool,
    pub backing_type: BackedEnumType,
    // This needs to be a Vec<Symbol> because we need to be able to
    // represent interfaces that extend multiple other interfaces.
    pub extends: Vec<Symbol>,
    pub implements: Vec<Symbol>,
    pub constants: Vec<ClassishConstantEntity>,
    pub properties: Vec<PropertyEntity>,
    pub methods: Vec<MethodEntity>,
    pub uses: Vec<Symbol>,
    pub cases: Vec<Symbol>,
    pub r#final: bool,
    pub r#abstract: bool,
    pub r#readonly: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Default)]
pub struct ClassishConstantEntity {
    pub name: Symbol,
    pub r#type: Type,
    pub visibility: Visibility,
    pub r#final: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyEntity {
    pub name: Symbol,
    pub r#static: bool,
    pub visibility: Visibility,
    pub r#type: Type,
    pub default: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MethodEntity {
    pub name: Symbol,
    pub visibility: Visibility,
    pub parameters: Vec<ParameterEntity>,
    pub r#static: bool,
    pub r#final: bool,
    pub r#abstract: bool,
    pub r#virtual: bool,
    pub return_type: Type,
}

type DebuggableEntityDebuggerCallback<'a, T> = dyn Fn(&T, &'a SymbolTable, &mut Formatter) -> std::fmt::Result;

pub struct DebuggableEntityWithSymbolTable<'a, T> {
    entity: T,
    symbol_table: &'a SymbolTable,
    debugger: Box<DebuggableEntityDebuggerCallback<'a, T>>,
}

impl<'a, T> Debug for DebuggableEntityWithSymbolTable<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.debugger)(&self.entity, self.symbol_table, f)
    }
}

pub fn debuggable_entity<'a, T>(entity: T, symbol_table: &'a SymbolTable, debugger: Box<DebuggableEntityDebuggerCallback<'a, T>>) -> DebuggableEntityWithSymbolTable<'a, T> {
    DebuggableEntityWithSymbolTable {
        entity,
        symbol_table,
        debugger
    }
}
