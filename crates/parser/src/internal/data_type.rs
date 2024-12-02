use crate::state::State;
use crate::{internal::utils, ParserDiagnostic};
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;
use pxp_type::Type;

pub fn data_type(state: &mut State) -> DataType {
    let start = state.current().span;

    let kind = if state.is_in_docblock() {
        docblock_type(state)
    } else if state.current().kind == TokenKind::Question {
        nullable(state)
    } else if state.current().kind == TokenKind::LeftParen {
        dnf(state)
    } else {
        let ty = simple_data_type(state);

        if state.current().kind == TokenKind::Pipe {
            union(state, ty, false)
        } else if state.current().kind == TokenKind::Ampersand
            && !matches!(
                state.peek().kind,
                TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
            )
        {
            intersection(state, ty, false)
        } else {
            ty
        }
    };

    let end = state.previous().span;

    DataType::new(state.id(), kind, Span::new(start.start, end.end))
}

pub fn optional_data_type(state: &mut State) -> Option<DataType> {
    let start = state.current().span;
    let kind = if state.is_in_docblock() {
        docblock_type(state)
    } else if state.current().kind == TokenKind::Question {
        nullable(state)
    } else if state.current().kind == TokenKind::LeftParen {
        dnf(state)
    } else {
        let ty = optional_simple_data_type(state);

        match ty {
            Some(ty) => {
                if state.current().kind == TokenKind::Pipe {
                    union(state, ty, false)
                } else if state.current().kind == TokenKind::Ampersand
                    && !matches!(
                        state.peek().kind,
                        TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                    )
                {
                    intersection(state, ty, false)
                } else {
                    ty
                }
            }
            None => return None,
        }
    };

    let end = state.previous().span;

    Some(DataType::new(
        state.id(),
        kind,
        Span::new(start.start, end.end),
    ))
}

// Special type parsing logic for DocBlock comments, heavily based on the phpstan/phpdoc-parser package.
fn docblock_type(state: &mut State) -> Type<Name> {
    let current = state.current();

    if current.kind == TokenKind::Question {
        return nullable(state);
    }
    
    let atomic = docblock_atomic(state);

    if atomic == Type::Missing {
        return atomic;
    }

    let current = state.current();

    if current.kind == TokenKind::Pipe {
        return union(state, atomic, false);
    }

    if current.kind == TokenKind::Ampersand {
        return intersection(state, atomic, false);
    }

    atomic
}

fn docblock_atomic(state: &mut State) -> Type<Name> {
    let current = state.current();
    let peek = state.peek();

    if current.kind == TokenKind::LeftParen {
        return dnf(state);
    }

    if current.kind == TokenKind::Variable && peek.kind == TokenKind::Identifier && peek.symbol.as_ref().is_some_and(|sym| sym == b"is") {
        todo!()
    }

    let ty = optional_simple_data_type(state);

    match ty {
        Some(ty) => ty,
        None => {
            state.diagnostic(
                ParserDiagnostic::MissingType,
                Severity::Error,
                state.current().span,
            );

            Type::Missing
        }
    }
}

fn docblock_subparse(state: &mut State) -> Type<Name> {
    todo!()
}

fn dnf(state: &mut State) -> Type<Name> {
    // (A|B|..)&C.. or (A&B&..)|C..
    state.next();
    let ty = simple_data_type(state);

    match state.current().kind {
        TokenKind::Pipe => {
            let union = union(state, ty, true);

            utils::skip_right_parenthesis(state);

            intersection(state, union, false)
        }
        TokenKind::Ampersand => {
            let intersection = intersection(state, ty, true);

            utils::skip_right_parenthesis(state);

            union(state, intersection, false)
        }
        _ => {
            state.diagnostic(
                ParserDiagnostic::UnexpectedToken {
                    token: state.current().clone(),
                },
                Severity::Error,
                state.current().span,
            );

            Type::Missing
        }
    }
}

fn optional_simple_data_type(state: &mut State) -> Option<Type<Name>> {
    let current = state.current();

    match &current.kind {
        TokenKind::Array => {
            state.next();

            Some(Type::Array)
        }
        TokenKind::Callable => {
            state.next();

            Some(Type::Callable)
        }
        TokenKind::Null => {
            state.next();

            Some(Type::Null)
        }
        TokenKind::True => {
            state.next();

            Some(Type::True)
        }
        TokenKind::False => {
            state.next();

            Some(Type::False)
        }
        TokenKind::Static => {
            state.next();

            Some(Type::StaticReference)
        }
        TokenKind::Self_ => {
            state.next();

            Some(Type::SelfReference)
        }
        TokenKind::Parent => {
            state.next();

            Some(Type::ParentReference)
        }
        TokenKind::Enum | TokenKind::From => {
            state.next();

            Some(Type::Named(
                state.maybe_resolve_identifier(current, UseKind::Normal),
            ))
        }
        TokenKind::Identifier => {
            let id = current.symbol.as_ref().unwrap();
            state.next();

            let name = &id[..];
            let lowered_name = name.to_ascii_lowercase();
            match lowered_name.as_slice() {
                b"void" => Some(Type::Void),
                b"never" => Some(Type::Never),
                b"float" => Some(Type::Float),
                b"bool" => Some(Type::Boolean),
                b"int" => Some(Type::Integer),
                b"string" => Some(Type::String),
                b"object" => Some(Type::Object),
                b"mixed" => Some(Type::Mixed),
                b"iterable" => Some(Type::Iterable),
                b"null" => Some(Type::Null),
                b"true" => Some(Type::True),
                b"false" => Some(Type::False),
                b"array" => Some(Type::Array),
                b"callable" => Some(Type::Callable),
                _ => Some(Type::Named(
                    state.maybe_resolve_identifier(current, UseKind::Normal),
                )),
            }
        }
        TokenKind::FullyQualifiedIdentifier => {
            state.next();

            let symbol = current.symbol.as_ref().unwrap();
            let resolved = state.strip_leading_namespace_qualifier(symbol);

            Some(Type::Named(Name::resolved(
                state.id(),
                resolved,
                symbol.clone(),
                current.span,
            )))
        }
        TokenKind::QualifiedIdentifier => {
            state.next();

            let name = state.maybe_resolve_identifier(current, UseKind::Normal);

            Some(Type::Named(name))
        }
        _ => None,
    }
}

fn simple_data_type(state: &mut State) -> Type<Name> {
    match optional_simple_data_type(state) {
        Some(ty) => ty,
        None => {
            state.diagnostic(
                ParserDiagnostic::MissingType,
                Severity::Error,
                state.current().span,
            );

            Type::Missing
        }
    }
}

fn nullable(state: &mut State) -> Type<Name> {
    let current = state.current();

    state.next();

    let ty = simple_data_type(state);

    if ty.standalone() {
        state.diagnostic(
            ParserDiagnostic::StandaloneTypeUsedInNullableType,
            Severity::Error,
            current.span,
        );
    }

    Type::Nullable(Box::new(ty))
}

fn union(state: &mut State, other: Type<Name>, within_dnf: bool) -> Type<Name> {
    if other.standalone() {
        state.diagnostic(
            ParserDiagnostic::StandaloneTypeUsedInUnionType,
            Severity::Error,
            state.current().span,
        );
    }

    let mut types = vec![other];
    let mut last_pipe = utils::skip(state, TokenKind::Pipe);

    loop {
        let current = state.current();
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
                    ParserDiagnostic::NestedDisjunctiveNormalFormType,
                    Severity::Error,
                    current.span,
                );
            }

            state.next();

            let other = simple_data_type(state);
            let ty = intersection(state, other, true);

            utils::skip_right_parenthesis(state);

            ty
        } else {
            let ty = simple_data_type(state);
            if ty.standalone() {
                state.diagnostic(
                    ParserDiagnostic::StandaloneTypeUsedInUnionType,
                    Severity::Error,
                    last_pipe,
                );
            }

            ty
        };

        types.push(ty);

        if state.current().kind == TokenKind::Pipe {
            last_pipe = utils::skip(state, TokenKind::Pipe);
        } else {
            break;
        }
    }

    Type::Union(types)
}

fn intersection(state: &mut State, other: Type<Name>, within_dnf: bool) -> Type<Name> {
    if other.standalone() {
        state.diagnostic(
            ParserDiagnostic::StandaloneTypeUsedInIntersectionType,
            Severity::Error,
            state.current().span,
        );
    }

    let mut types = vec![other];

    let mut last_ampersand = utils::skip(state, TokenKind::Ampersand);

    loop {
        let current = state.current();
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
                    ParserDiagnostic::NestedDisjunctiveNormalFormType,
                    Severity::Error,
                    current.span,
                );
            }

            state.next();

            let other = simple_data_type(state);
            let ty = union(state, other, true);

            utils::skip_right_parenthesis(state);

            ty
        } else {
            let ty = simple_data_type(state);
            if ty.standalone() {
                state.diagnostic(
                    ParserDiagnostic::StandaloneTypeUsedInIntersectionType,
                    Severity::Error,
                    last_ampersand,
                );
            }

            ty
        };

        types.push(ty);

        if state.current().kind == TokenKind::Ampersand
            && !matches!(
                state.peek().kind,
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
