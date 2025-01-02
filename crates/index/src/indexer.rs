use pxp_ast::{
    visitor::Visitor, ClassStatement, ClassishMember, FunctionParameterList, FunctionStatement,
    Method, MethodParameterList, Name, ReturnType,
};
use pxp_type::Type;

use crate::{
    entities::{ClassEntity, ClassEntityKind, FunctionEntity, MethodEntity, Parameter, Parameters},
    location::Location,
    FileId, Index,
};

pub struct IndexingVisitor<'a> {
    file_id: FileId,
    index: &'a mut Index,
}

impl<'a> IndexingVisitor<'a> {
    pub fn new(file_id: FileId, index: &'a mut Index) -> Self {
        Self { file_id, index }
    }

    fn transform_function_parameter_list(&self, node: &FunctionParameterList) -> Parameters {
        let mut parameters = Vec::new();

        for parameter in node.parameters.iter() {
            parameters.push(Parameter {
                name: parameter.name.clone(),
                r#type: parameter
                    .data_type
                    .as_ref()
                    .map(|data_type| data_type.get_type().clone()),
                optional: parameter.default.is_some(),
                variadic: parameter.ellipsis.is_some(),
                location: Location::new(self.file_id, parameter.span),
            })
        }

        Parameters::new(parameters)
    }

    fn transform_method_parameter_list(&self, node: &MethodParameterList) -> Parameters {
        let mut parameters = Vec::new();

        for parameter in node.parameters.iter() {
            parameters.push(Parameter {
                name: parameter.name.clone(),
                r#type: parameter
                    .data_type
                    .as_ref()
                    .map(|data_type| data_type.get_type().clone()),
                optional: parameter.default.is_some(),
                variadic: parameter.ellipsis.is_some(),
                location: Location::new(self.file_id, parameter.span),
            })
        }

        Parameters::new(parameters)
    }

    fn transform_return_type(&self, node: Option<&ReturnType>) -> Option<Type<Name>> {
        node.map(|return_type| return_type.data_type.get_type().clone())
    }

    fn transform_method(&self, node: &Method) -> MethodEntity {
        MethodEntity {
            name: node.name.clone(),
            parameters: self.transform_method_parameter_list(&node.parameters),
            return_type: self.transform_return_type(node.return_type.as_ref()),
            returns_reference: node.ampersand.is_some(),
            modifiers: node.modifiers.clone(),
            location: Location::new(self.file_id, node.span),
        }
    }

    fn transform_classish_members(&self, nodes: &[ClassishMember]) -> (Vec<MethodEntity>, ()) {
        let mut methods = Vec::new();

        for member in nodes.iter() {
            match member {
                ClassishMember::Method(method) => methods.push(self.transform_method(method)),
                _ => {}
            }
        }

        (methods, ())
    }
}

impl<'a> Visitor for IndexingVisitor<'a> {
    fn visit_class_statement(&mut self, node: &ClassStatement) {
        let (methods, properties) = self.transform_classish_members(&node.body.members);

        self.index.entities.add_class(ClassEntity {
            name: node.name.to_resolved().clone(),
            kind: ClassEntityKind::Class,
            methods,
            location: Location::new(self.file_id, node.span),
        })
    }

    fn visit_function_statement(&mut self, node: &FunctionStatement) {
        self.index.entities.add_function(FunctionEntity {
            name: node.name.to_resolved().clone(),
            parameters: self.transform_function_parameter_list(&node.parameters),
            return_type: self.transform_return_type(node.return_type.as_ref()),
            returns_reference: node.ampersand.is_some(),
            location: Location::new(self.file_id, node.span),
        });
    }
}
