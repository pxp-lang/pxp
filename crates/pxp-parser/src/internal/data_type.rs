use crate::expected_token;
use crate::internal::utils;
use crate::peek_token;
use crate::state::State;
use pxp_diagnostics::{DiagnosticKind, Severity};
use pxp_token::TokenKind;
use pxp_type::Type;

pub fn data_type(state: &mut State) -> Type {
    if state.stream.current().kind == TokenKind::Question {
        return nullable(state);
    }

    // (A|B|..)&C.. or (A&B&..)|C..
    if state.stream.current().kind == TokenKind::LeftParen {
        return dnf(state);
    }

    let ty = simple_data_type(state);

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

    ty
}

pub fn optional_data_type(state: &mut State) -> Option<Type> {
    if state.stream.current().kind == TokenKind::Question {
        return Some(nullable(state));
    }

    // (A|B|..)&C.. or (A&B&..)|C..
    if state.stream.current().kind == TokenKind::LeftParen {
        return Some(dnf(state));
    }

    let ty = optional_simple_data_type(state);

    match ty {
        Some(ty) => {
            if state.stream.current().kind == TokenKind::Pipe {
                return Some(union(state, ty, false));
            }

            if state.stream.current().kind == TokenKind::Ampersand
                && !matches!(
                    state.stream.peek().kind,
                    TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                )
            {
                return Some(intersection(state, ty, false));
            }

            Some(ty)
        }
        None => None,
    }
}

fn dnf(state: &mut State) -> Type {
    // (A|B|..)&C.. or (A&B&..)|C..
    state.stream.next();
    let ty = simple_data_type(state);
    peek_token!([
        TokenKind::Pipe => {
            let union = union(state, ty, true);

            utils::skip_right_parenthesis(state);

            intersection(state, union, false)
        },
        TokenKind::Ampersand => {
            let intersection = intersection(state, ty, true);

            utils::skip_right_parenthesis(state);

            union(state, intersection, false)
        },
    ], state, ["|", "&"])
}

fn optional_simple_data_type(state: &mut State) -> Option<Type> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Array => {
            let span = current.span;
            state.stream.next();

            Some(Type::Array(span))
        }
        TokenKind::Callable => {
            let span = current.span;
            state.stream.next();

            Some(Type::Callable(span))
        }
        TokenKind::Null => {
            let span = current.span;
            state.stream.next();

            Some(Type::Null(span))
        }
        TokenKind::True => {
            let span = current.span;
            state.stream.next();

            Some(Type::True(span))
        }
        TokenKind::False => {
            let span = current.span;
            state.stream.next();

            Some(Type::False(span))
        }
        TokenKind::Static => {
            let span = current.span;
            state.stream.next();

            Some(Type::StaticReference(span))
        }
        TokenKind::Self_ => {
            let span = current.span;
            state.stream.next();

            Some(Type::SelfReference(span))
        }
        TokenKind::Parent => {
            let span = current.span;
            state.stream.next();

            Some(Type::ParentReference(span))
        }
        TokenKind::Enum | TokenKind::From => {
            let span = current.span;

            state.stream.next();

            Some(Type::Named(span, current.symbol.unwrap()))
        }
        TokenKind::Identifier => {
            let id = state.symbol_table.resolve(current.symbol.unwrap()).unwrap();
            let span = current.span;
            state.stream.next();

            let name = &id[..];
            let lowered_name = name.to_ascii_lowercase();
            match lowered_name.as_slice() {
                b"void" => Some(Type::Void(span)),
                b"never" => Some(Type::Never(span)),
                b"float" => Some(Type::Float(span)),
                b"bool" => Some(Type::Boolean(span)),
                b"int" => Some(Type::Integer(span)),
                b"string" => Some(Type::String(span)),
                b"object" => Some(Type::Object(span)),
                b"mixed" => Some(Type::Mixed(span)),
                b"iterable" => Some(Type::Iterable(span)),
                b"null" => Some(Type::Null(span)),
                b"true" => Some(Type::True(span)),
                b"false" => Some(Type::False(span)),
                b"array" => Some(Type::Array(span)),
                b"callable" => Some(Type::Callable(span)),
                _ => Some(Type::Named(span, current.symbol.unwrap())),
            }
        }
        TokenKind::QualifiedIdentifier | TokenKind::FullyQualifiedIdentifier => {
            let span = current.span;
            state.stream.next();

            Some(Type::Named(span, current.symbol.unwrap()))
        }
        _ => None,
    }
}

fn simple_data_type(state: &mut State) -> Type {
    match optional_simple_data_type(state) {
        Some(ty) => ty,
        None => {
            state.diagnostic(
                DiagnosticKind::MissingType,
                Severity::Error,
                state.stream.current().span,
            );

            Type::Missing(state.stream.current().span)
        },
    }
}

fn nullable(state: &mut State) -> Type {
    let current = state.stream.current();

    state.stream.next();

    let ty = simple_data_type(state);

    if ty.standalone() {
        state.diagnostic(
            DiagnosticKind::StandaloneTypeUsedInNullableType,
            Severity::Error,
            current.span,
        );
    }

    Type::Nullable(current.span, Box::new(ty))
}

fn union(state: &mut State, other: Type, within_dnf: bool) -> Type {
    if other.standalone() {
        state.diagnostic(
            DiagnosticKind::StandaloneTypeUsedInUnionType,
            Severity::Error,
            state.stream.current().span,
        );
    }

    let mut types = vec![other];
    let mut last_pipe = utils::skip(state, TokenKind::Pipe);

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
                state.diagnostic(
                    DiagnosticKind::NestedDisjunctiveNormalFormType,
                    Severity::Error,
                    current.span,
                );
            }

            state.stream.next();

            let other = simple_data_type(state);
            let ty = intersection(state, other, true);

            utils::skip_right_parenthesis(state);

            ty
        } else {
            let ty = simple_data_type(state);
            if ty.standalone() {
                state.diagnostic(
                    DiagnosticKind::StandaloneTypeUsedInUnionType,
                    Severity::Error,
                    last_pipe,
                );
            }

            ty
        };

        types.push(ty);

        if state.stream.current().kind == TokenKind::Pipe {
            last_pipe = utils::skip(state, TokenKind::Pipe);
        } else {
            break;
        }
    }

    Type::Union(types)
}

fn intersection(state: &mut State, other: Type, within_dnf: bool) -> Type {
    if other.standalone() {
        state.diagnostic(
            DiagnosticKind::StandaloneTypeUsedInIntersectionType,
            Severity::Error,
            state.stream.current().span,
        );
    }

    let mut types = vec![other];

    let mut last_ampersand = utils::skip(state, TokenKind::Ampersand);

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
                
                state.diagnostic(
                    DiagnosticKind::NestedDisjunctiveNormalFormType,
                    Severity::Error,
                    current.span,
                );
            }

            state.stream.next();

            let other = simple_data_type(state);
            let ty = union(state, other, true);

            utils::skip_right_parenthesis(state);

            ty
        } else {
            let ty = simple_data_type(state);
            if ty.standalone() {
                state.diagnostic(
                    DiagnosticKind::StandaloneTypeUsedInIntersectionType,
                    Severity::Error,
                    last_ampersand,
                );
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
            last_ampersand = utils::skip(state, TokenKind::Ampersand);
        } else {
            break;
        }
    }

    Type::Intersection(types)
}
