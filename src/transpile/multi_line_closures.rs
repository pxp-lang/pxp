use pxp_parser::{
    lexer::token::Span,
    parser::ast::{
        comments::CommentGroup,
        functions::{
            ArrowFunction, ArrowFunctionBody, Closure, ClosureUse, ClosureUseVariable,
            FunctionBody, FunctionParameterList,
        },
        utils::CommaSeparated,
        variables::SimpleVariable,
        Expression,
    },
    traverser::Visitor,
};

use crate::visitors::VariableFinderVisitor;

use super::Transpiler;

pub struct MultiLineClosuresTranspiler;

impl MultiLineClosuresTranspiler {
    pub fn new() -> Self {
        Self
    }

    fn variable_is_parameter(
        &self,
        variable: &SimpleVariable,
        parameters: &FunctionParameterList,
    ) -> bool {
        parameters
            .parameters
            .inner
            .iter()
            .any(|parameter| parameter.name.name == variable.name)
    }
}

impl Transpiler for MultiLineClosuresTranspiler {
    fn transpile_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::ArrowFunction(ArrowFunction {
                comments,
                r#static,
                ampersand,
                attributes,
                parameters,
                return_type,
                body,
                ..
            }) => match body {
                ArrowFunctionBody::Block { ref statements, .. } => {
                    let found_variables = VariableFinderVisitor::find(body, false);
                    let mut variables = Vec::new();
                    for variable in found_variables.iter() {
                        if self.variable_is_parameter(variable, parameters) {
                            continue;
                        }

                        variables.push(ClosureUseVariable {
                            comments: CommentGroup { comments: vec![] },
                            ampersand: None,
                            variable: variable.clone(),
                        });
                    }

                    let uses = if variables.is_empty() {
                        None
                    } else {
                        Some(ClosureUse {
                            comments: CommentGroup { comments: vec![] },
                            r#use: Span::default(),
                            left_parenthesis: Span::default(),
                            variables: CommaSeparated {
                                inner: variables,
                                commas: vec![],
                            },
                            right_parenthesis: Span::default(),
                        })
                    };

                    *expression = Expression::Closure(Closure {
                        comments: comments.clone(),
                        attributes: attributes.clone(),
                        r#static: r#static.clone(),
                        function: Span::default(),
                        ampersand: ampersand.clone(),
                        parameters: parameters.clone(),
                        uses,
                        return_type: return_type.clone(),
                        body: FunctionBody {
                            comments: CommentGroup { comments: vec![] },
                            left_brace: Span::default(),
                            statements: statements.clone(),
                            right_brace: Span::default(),
                        },
                    })
                }
                _ => return,
            },
            _ => return,
        }
    }
}
