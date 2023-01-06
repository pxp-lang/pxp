use pxp_parser::parser::ast::{Expression, Statement, classes::{ClassMember, AnonymousClassMember}, traits::TraitMember, interfaces::InterfaceMember, properties::{Property, VariableProperty}};

pub mod short_match;
pub mod type_alias;
pub mod multi_line_closures;
pub mod range;

pub trait Transpiler {
    fn transpile_statement(&mut self, statement: &mut Statement) {}
    fn transpile_expression(&mut self, expression: &mut Expression) {}
    fn transpile_class_member(&mut self, member: &mut ClassMember) {}
    fn transpile_anonymous_class_member(&mut self, member: &mut AnonymousClassMember) {}
    fn transpile_trait_member(&mut self, member: &mut TraitMember) {}
    fn transpile_interface_member(&mut self, member: &mut InterfaceMember) {}
    fn transpile_property(&mut self, property: &mut Property) {}
    fn transpile_variable_property(&mut self, property: &mut VariableProperty) {}
}
