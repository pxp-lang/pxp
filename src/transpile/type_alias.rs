use pxp_parser::{
    lexer::{byte_string::ByteString, token::Span},
    parser::ast::{
        classes::{AnonymousClassMember, ClassMember},
        comments::{Comment, CommentFormat},
        data_type::Type,
        functions::{
            AbstractConstructor, AbstractMethod, ArrowFunction, Closure, ConcreteConstructor,
            ConcreteMethod, ConstructorParameter, FunctionParameter, FunctionStatement, ReturnType,
        },
        properties::{Property, VariableProperty},
        traits::TraitMember,
        Expression, Statement,
    },
};

use super::Transpiler;

pub struct TypeAliasTranspiler {
    aliases: Vec<(ByteString, Type)>,
}

impl TypeAliasTranspiler {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
        }
    }

    fn has_alias(&self, name: &ByteString) -> bool {
        self.aliases.iter().any(|(alias, _)| alias == name)
    }

    fn get_alias(&self, name: &ByteString) -> Option<&Type> {
        self.aliases
            .iter()
            .find(|(alias, _)| alias == name)
            .map(|(_, r#type)| r#type)
    }

    fn maybe_change_data_type(&self, data_type: &mut Type) {
        if let Type::Named(_, name) = data_type {
            if self.has_alias(name) {
                *data_type = self.get_alias(name).unwrap().clone();
            }
        }
    }
}

impl Transpiler for TypeAliasTranspiler {
    fn transpile_statement(&mut self, statement: &mut Statement) {
        match statement {
            Statement::TypeAlias {
                name,
                r#type,
                ..
            } => {
                self.aliases.push((name.value.clone(), r#type.clone()));

                // Replace the statement with a noop.
                *statement = Statement::Comment(Comment {
                    span: Span::default(),
                    format: CommentFormat::SingleLine,
                    content: format!(
                        "Type alias `{} = {}` removed",
                        name.value,
                        r#type
                    )
                    .into(),
                })
            }
            Statement::Function(FunctionStatement {
                parameters,
                return_type,
                ..
            }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }

                if let Some(ReturnType { data_type, .. }) = return_type {
                    self.maybe_change_data_type(data_type);
                }
            }
            _ => {},
        }
    }

    fn transpile_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::Closure(Closure {
                parameters,
                return_type,
                ..
            })
            | Expression::ArrowFunction(ArrowFunction {
                parameters,
                return_type,
                ..
            }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }

                if let Some(ReturnType { data_type, .. }) = return_type {
                    self.maybe_change_data_type(data_type);
                }
            }
            _ => {}
        }
    }

    fn transpile_class_member(&mut self, member: &mut ClassMember) {
        match member {
            ClassMember::AbstractConstructor(AbstractConstructor { parameters, .. }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }
            }
            ClassMember::ConcreteConstructor(ConcreteConstructor { parameters, .. }) => {
                for ConstructorParameter { data_type, .. } in parameters.parameters.inner.iter_mut()
                {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }
            }
            ClassMember::AbstractMethod(AbstractMethod {
                parameters,
                return_type,
                ..
            })
            | ClassMember::ConcreteMethod(ConcreteMethod {
                parameters,
                return_type,
                ..
            }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }

                if let Some(ReturnType { data_type, .. }) = return_type {
                    self.maybe_change_data_type(data_type);
                }
            }
            _ => {}
        }
    }

    fn transpile_anonymous_class_member(&mut self, member: &mut AnonymousClassMember) {
        match member {
            AnonymousClassMember::ConcreteConstructor(ConcreteConstructor {
                parameters, ..
            }) => {
                for ConstructorParameter { data_type, .. } in parameters.parameters.inner.iter_mut()
                {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }
            }
            AnonymousClassMember::ConcreteMethod(ConcreteMethod {
                parameters,
                return_type,
                ..
            }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }

                if let Some(ReturnType { data_type, .. }) = return_type {
                    self.maybe_change_data_type(data_type);
                }
            }
            _ => {}
        }
    }

    fn transpile_trait_member(&mut self, member: &mut TraitMember) {
        match member {
            TraitMember::AbstractConstructor(AbstractConstructor { parameters, .. }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }
            }
            TraitMember::ConcreteConstructor(ConcreteConstructor { parameters, .. }) => {
                for ConstructorParameter { data_type, .. } in parameters.parameters.inner.iter_mut()
                {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }
            }
            TraitMember::AbstractMethod(AbstractMethod {
                parameters,
                return_type,
                ..
            })
            | TraitMember::ConcreteMethod(ConcreteMethod {
                parameters,
                return_type,
                ..
            }) => {
                for FunctionParameter { data_type, .. } in parameters.parameters.inner.iter_mut() {
                    if let Some(data_type) = data_type {
                        self.maybe_change_data_type(data_type);
                    }
                }

                if let Some(ReturnType { data_type, .. }) = return_type {
                    self.maybe_change_data_type(data_type);
                }
            }
            _ => {}
        }
    }

    fn transpile_property(&mut self, property: &mut Property) {
        if let Some(data_type) = &mut property.r#type {
            self.maybe_change_data_type(data_type);
        }
    }

    fn transpile_variable_property(&mut self, property: &mut VariableProperty) {
        if let Some(data_type) = &mut property.r#type {
            self.maybe_change_data_type(data_type);
        }
    }
}
