use pxp_parser::parser::ast::{
    classes::{AnonymousClassMember, ClassMember},
    interfaces::InterfaceMember,
    properties::{Property, VariableProperty},
    traits::TraitMember,
    Expression, Statement,
};

pub mod multi_line_closures;
pub mod multi_line_match;
pub mod range;
pub mod short_match;
pub mod type_alias;

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
