use crate::lexer::token::TokenKind;
use crate::parser::ast::functions::AbstractConstructor;
use crate::parser::ast::functions::AbstractMethod;
use crate::parser::ast::functions::ArrowFunctionExpression;
use crate::parser::ast::functions::ClosureExpression;
use crate::parser::ast::functions::ClosureUse;
use crate::parser::ast::functions::ClosureUseVariable;
use crate::parser::ast::functions::ConcreteConstructor;
use crate::parser::ast::functions::ConcreteMethod;
use crate::parser::ast::functions::FunctionBody;
use crate::parser::ast::functions::FunctionStatement;
use crate::parser::ast::functions::MethodBody;
use crate::parser::ast::functions::ReturnType;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::modifiers::MethodModifierGroup;
use crate::parser::ast::Expression;
use crate::parser::ast::Statement;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::blocks;
use crate::parser::internal::data_type;
use crate::parser::internal::identifiers;
use crate::parser::internal::parameters;
use crate::parser::internal::utils;
use crate::parser::internal::variables;
use crate::parser::state::State;

pub enum MethodType {
    Abstract,
    Concrete,
    DependingOnModifiers,
}

pub enum Method {
    Abstract(AbstractMethod),
    Concrete(ConcreteMethod),
    AbstractConstructor(AbstractConstructor),
    ConcreteConstructor(ConcreteConstructor),
}

pub fn anonymous_function(state: &mut State) -> ParseResult<Expression> {
    let comments = state.stream.comments();
    let attributes = state.get_attributes();
    let current = state.stream.current();
    let r#static = if current.kind == TokenKind::Static {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let function = utils::skip(state, TokenKind::Function)?;

    let current = state.stream.current();
    let ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let parameters = parameters::function_parameter_list(state)?;

    let current = state.stream.current();
    let uses = if current.kind == TokenKind::Use {
        state.stream.next();

        Some(ClosureUse {
            comments: state.stream.comments(),
            r#use: current.span,
            left_parenthesis: utils::skip_left_parenthesis(state)?,
            variables: utils::comma_separated::<ClosureUseVariable>(
                state,
                &|state| {
                    let use_comments = state.stream.comments();
                    let current = state.stream.current();
                    let use_ampersand = if current.kind == TokenKind::Ampersand {
                        state.stream.next();

                        Some(current.span)
                    } else {
                        None
                    };

                    let var = variables::simple_variable(state)?;

                    Ok(ClosureUseVariable {
                        comments: use_comments,
                        variable: var,
                        ampersand: use_ampersand,
                    })
                },
                TokenKind::RightParen,
            )?,
            right_parenthesis: utils::skip_right_parenthesis(state)?,
        })
    } else {
        None
    };

    let return_type = if state.stream.current().kind == TokenKind::Colon {
        Some(ReturnType {
            colon: utils::skip_colon(state)?,
            data_type: data_type::data_type(state)?,
        })
    } else {
        None
    };

    let body = FunctionBody {
        comments: state.stream.comments(),
        left_brace: utils::skip_left_brace(state)?,
        statements: blocks::multiple_statements_until(state, &TokenKind::RightBrace)?,
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(Expression::Closure(ClosureExpression {
        comments,
        function,
        attributes,
        parameters,
        uses,
        return_type,
        body,
        r#static,
        ampersand,
    }))
}

pub fn arrow_function(state: &mut State) -> ParseResult<Expression> {
    let comments = state.stream.comments();
    let current = state.stream.current();
    let r#static = if current.kind == TokenKind::Static {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let r#fn = utils::skip(state, TokenKind::Fn)?;

    let current = state.stream.current();
    let ampersand = if state.stream.current().kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let attributes = state.get_attributes();
    let parameters = parameters::function_parameter_list(state)?;
    let return_type = if state.stream.current().kind == TokenKind::Colon {
        Some(ReturnType {
            colon: utils::skip_colon(state)?,
            data_type: data_type::data_type(state)?,
        })
    } else {
        None
    };

    let double_arrow = utils::skip(state, TokenKind::DoubleArrow)?;

    let body = Box::new(expressions::create(state)?);

    Ok(Expression::ArrowFunction(ArrowFunctionExpression {
        comments,
        attributes,
        r#static,
        r#fn,
        ampersand,
        parameters,
        return_type,
        double_arrow,
        body,
    }))
}

pub fn function(state: &mut State) -> ParseResult<Statement> {
    let comments = state.stream.comments();

    let function = utils::skip(state, TokenKind::Function)?;

    let current = state.stream.current();
    let ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let name = identifiers::identifier_maybe_soft_reserved(state)?;

    // get attributes before processing parameters, otherwise
    // parameters will steal attributes of this function.
    let attributes = state.get_attributes();

    let parameters = parameters::function_parameter_list(state)?;
    let return_type = if state.stream.current().kind == TokenKind::Colon {
        Some(ReturnType {
            colon: utils::skip_colon(state)?,
            data_type: data_type::data_type(state)?,
        })
    } else {
        None
    };

    let body = FunctionBody {
        comments: state.stream.comments(),
        left_brace: utils::skip_left_brace(state)?,
        statements: blocks::multiple_statements_until(state, &TokenKind::RightBrace)?,
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(Statement::Function(FunctionStatement {
        comments,
        function,
        name,
        attributes,
        parameters,
        return_type,
        body,
        ampersand,
    }))
}

pub fn method(
    state: &mut State,
    r#type: MethodType,
    modifiers: MethodModifierGroup,
    class: Option<&SimpleIdentifier>,
) -> ParseResult<Method> {
    let comments = state.stream.comments();
    let attributes = state.get_attributes();
    let function = utils::skip(state, TokenKind::Function)?;

    let current = state.stream.current();
    let ampersand = if current.kind == TokenKind::Ampersand {
        state.stream.next();

        Some(current.span)
    } else {
        None
    };

    let name = identifiers::identifier_maybe_reserved(state)?;
    let has_body = match r#type {
        MethodType::Abstract => false,
        MethodType::Concrete => true,
        MethodType::DependingOnModifiers => !modifiers.has_abstract(),
    };

    if name.to_string().to_lowercase() == "__construct" {
        return if has_body {
            let parameters = parameters::constructor_parameter_list(state, class)?;
            let body = MethodBody {
                comments: state.stream.comments(),
                left_brace: utils::skip_left_brace(state)?,
                statements: blocks::multiple_statements_until(state, &TokenKind::RightBrace)?,
                right_brace: utils::skip_right_brace(state)?,
            };

            Ok(Method::ConcreteConstructor(ConcreteConstructor {
                comments,
                attributes,
                modifiers,
                function,
                ampersand,
                name,
                parameters,
                body,
            }))
        } else {
            let parameters = parameters::function_parameter_list(state)?;
            let semicolon = utils::skip_semicolon(state)?;

            Ok(Method::AbstractConstructor(AbstractConstructor {
                comments,
                attributes,
                modifiers,
                function,
                ampersand,
                name,
                parameters,
                semicolon,
            }))
        };
    }

    let parameters = parameters::function_parameter_list(state)?;
    let return_type = if state.stream.current().kind == TokenKind::Colon {
        Some(ReturnType {
            colon: utils::skip_colon(state)?,
            data_type: data_type::data_type(state)?,
        })
    } else {
        None
    };

    if has_body {
        Ok(Method::Concrete(ConcreteMethod {
            comments,
            attributes,
            modifiers,
            function,
            ampersand,
            name,
            parameters,
            return_type,
            body: MethodBody {
                comments: state.stream.comments(),
                left_brace: utils::skip_left_brace(state)?,
                statements: blocks::multiple_statements_until(state, &TokenKind::RightBrace)?,
                right_brace: utils::skip_right_brace(state)?,
            },
        }))
    } else {
        Ok(Method::Abstract(AbstractMethod {
            comments,
            attributes,
            modifiers,
            function,
            ampersand,
            name,
            parameters,
            return_type,
            semicolon: utils::skip_semicolon(state)?,
        }))
    }
}
