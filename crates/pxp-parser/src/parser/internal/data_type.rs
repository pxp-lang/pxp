use crate::expected_token;
use crate::lexer::token::TokenKind;
use crate::parser::ast::data_type::Type;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::internal::utils;
use crate::parser::state::State;
use crate::peek_token;

pub fn data_type(state: &mut State) -> ParseResult<Type> {
    if state.stream.current().kind == TokenKind::Question {
        return nullable(state);
    }

    // (A|B|..)&C.. or (A&B&..)|C..
    if state.stream.current().kind == TokenKind::LeftParen {
        return dnf(state);
    }

    let ty = simple_data_type(state)?;

    if state.stream.current().kind == TokenKind::Pipe {
        return union(state, ty, false);
    }

    if state.stream.current().kind == TokenKind::Ampersand
        && !matches!(
            state.stream.peek().kind,
            TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
        )
    {
        return intersection(state, ty, false);
    }

    Ok(ty)
}

pub fn optional_data_type(state: &mut State) -> ParseResult<Option<Type>> {
    if state.stream.current().kind == TokenKind::Question {
        return nullable(state).map(Some);
    }

    // (A|B|..)&C.. or (A&B&..)|C..
    if state.stream.current().kind == TokenKind::LeftParen {
        return dnf(state).map(Some);
    }

    let ty = optional_simple_data_type(state)?;

    match ty {
        Some(ty) => {
            if state.stream.current().kind == TokenKind::Pipe {
                return union(state, ty, false).map(Some);
            }

            if state.stream.current().kind == TokenKind::Ampersand
                && !matches!(
                    state.stream.peek().kind,
                    TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                )
            {
                return intersection(state, ty, false).map(Some);
            }

            Ok(Some(ty))
        }
        None => Ok(None),
    }
}

fn dnf(state: &mut State) -> ParseResult<Type> {
    // (A|B|..)&C.. or (A&B&..)|C..
    state.stream.next();
    let ty = simple_data_type(state)?;
    peek_token!([
        TokenKind::Pipe => {
            let union = union(state, ty, true)?;

            utils::skip_right_parenthesis(state)?;

            intersection(state, union, false)
        },
        TokenKind::Ampersand => {
            let intersection = intersection(state, ty, true)?;

            utils::skip_right_parenthesis(state)?;

            union(state, intersection, false)
        },
    ], state, ["|", "&"])
}

fn optional_simple_data_type(state: &mut State) -> ParseResult<Option<Type>> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Array => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::Array(span)))
        }
        TokenKind::Callable => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::Callable(span)))
        }
        TokenKind::Null => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::Null(span)))
        }
        TokenKind::True => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::True(span)))
        }
        TokenKind::False => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::False(span)))
        }
        TokenKind::Static => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::StaticReference(span)))
        }
        TokenKind::Self_ => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::SelfReference(span)))
        }
        TokenKind::Parent => {
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::ParentReference(span)))
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;
            let name = current.to_string().into();

            state.stream.next();

            Ok(Some(Type::Named(span, name)))
        }
        TokenKind::Identifier => {
            let id = current.value.clone();
            let span = current.span;
            state.stream.next();

            let name = &id[..];
            let lowered_name = name.to_ascii_lowercase();
            match lowered_name.as_slice() {
                b"void" => Ok(Some(Type::Void(span))),
                b"never" => Ok(Some(Type::Never(span))),
                b"float" => Ok(Some(Type::Float(span))),
                b"bool" => Ok(Some(Type::Boolean(span))),
                b"int" => Ok(Some(Type::Integer(span))),
                b"string" => Ok(Some(Type::String(span))),
                b"object" => Ok(Some(Type::Object(span))),
                b"mixed" => Ok(Some(Type::Mixed(span))),
                b"iterable" => Ok(Some(Type::Iterable(span))),
                b"null" => Ok(Some(Type::Null(span))),
                b"true" => Ok(Some(Type::True(span))),
                b"false" => Ok(Some(Type::False(span))),
                b"array" => Ok(Some(Type::Array(span))),
                b"callable" => Ok(Some(Type::Callable(span))),
                _ => Ok(Some(Type::Named(span, name.into()))),
            }
        }
        TokenKind::QualifiedIdentifier | TokenKind::FullyQualifiedIdentifier => {
            let name = current.value.clone();
            let span = current.span;
            state.stream.next();

            Ok(Some(Type::Named(span, name)))
        }
        _ => Ok(None),
    }
}

fn simple_data_type(state: &mut State) -> ParseResult<Type> {
    // TODO(azjezz): add a better error message here.
    optional_simple_data_type(state)?.ok_or_else(|| expected_token!(["a type"], state))
}

fn nullable(state: &mut State) -> ParseResult<Type> {
    let current = state.stream.current();

    state.stream.next();

    let ty = simple_data_type(state)?;

    if ty.standalone() {
        state.record(error::standalone_type_used_as_nullable(&ty, current.span));
    }

    Ok(Type::Nullable(current.span, Box::new(ty)))
}

fn union(state: &mut State, other: Type, within_dnf: bool) -> ParseResult<Type> {
    if other.standalone() {
        state.record(error::standalone_type_used_in_union(
            &other,
            state.stream.current().span,
        ));
    }

    let mut types = vec![other];

    let mut last_pipe = utils::skip(state, TokenKind::Pipe)?;

    loop {
        let current = state.stream.current();
        let ty = if current.kind == TokenKind::LeftParen {
            if within_dnf {
                // don't allow nesting.
                //
                // examples on how we got here:
                //
                // v-- get_intersection_type: within_dnf = false
                //     v-- get_union_type: within_dnf = true
                //      v-- error
                // F&(A|(D&S))
                //
                // v-- get_intersection_type: within_dnf = false
                //     v-- get_union_type: within_dnf = true
                //        v-- error
                // F&(A|B|(D&S))
                state.record(error::nested_disjunctive_normal_form_types(current.span));
            }

            state.stream.next();

            let other = simple_data_type(state)?;
            let ty = intersection(state, other, true)?;

            utils::skip_right_parenthesis(state)?;

            ty
        } else {
            let ty = simple_data_type(state)?;
            if ty.standalone() {
                state.record(error::standalone_type_used_in_union(&ty, last_pipe));
            }

            ty
        };

        types.push(ty);

        if state.stream.current().kind == TokenKind::Pipe {
            last_pipe = utils::skip(state, TokenKind::Pipe)?;
        } else {
            break;
        }
    }

    Ok(Type::Union(types))
}

fn intersection(state: &mut State, other: Type, within_dnf: bool) -> ParseResult<Type> {
    if other.standalone() {
        state.record(error::standalone_type_used_in_intersection(
            &other,
            state.stream.current().span,
        ));
    }

    let mut types = vec![other];

    let mut last_ampersand = utils::skip(state, TokenKind::Ampersand)?;

    loop {
        let current = state.stream.current();
        let ty = if current.kind == TokenKind::LeftParen {
            if within_dnf {
                // don't allow nesting.
                //
                // examples on how we got here:
                //
                //  v-- get_union_type: within_dnf = false
                //     v-- get_intersection_type: within_dnf = true
                //      v-- error
                // F|(A&(D|S))
                //
                //  v-- get_union_type: within_dnf = false
                //     v-- get_intersection_type: within_dnf = true
                //        v-- error
                // F|(A&B&(D|S))
                state.record(error::nested_disjunctive_normal_form_types(current.span));
            }

            state.stream.next();

            let other = simple_data_type(state)?;
            let ty = union(state, other, true)?;

            utils::skip_right_parenthesis(state)?;

            ty
        } else {
            let ty = simple_data_type(state)?;
            if ty.standalone() {
                state.record(error::standalone_type_used_in_intersection(
                    &ty,
                    last_ampersand,
                ));
            }

            ty
        };

        types.push(ty);

        if state.stream.current().kind == TokenKind::Ampersand
            && !matches!(
                state.stream.peek().kind,
                TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
            )
        {
            last_ampersand = utils::skip(state, TokenKind::Ampersand)?;
        } else {
            break;
        }
    }

    Ok(Type::Intersection(types))
}
