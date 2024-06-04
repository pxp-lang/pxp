use crate::{internal::utils, ParserDiagnostic};
use crate::state::State;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;
use pxp_type::Type;

pub fn data_type(state: &mut State) -> DataType {
    let start = state.stream.current().span;

    let kind = if state.stream.current().kind == TokenKind::Question {
        nullable(state)
    } else if state.stream.current().kind == TokenKind::LeftParen {
        dnf(state)
    } else {
        let ty = simple_data_type(state);

        if state.stream.current().kind == TokenKind::Pipe {
            union(state, ty, false)
        } else if state.stream.current().kind == TokenKind::Ampersand
            && !matches!(
                state.stream.peek().kind,
                TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
            )
        {
            intersection(state, ty, false)
        } else {
            ty
        }
    };
    
    let end = state.stream.previous().span;
    
    DataType::new(kind, Span::new(start.start, end.end))
}

pub fn optional_data_type(state: &mut State) -> Option<DataType> {
    let start = state.stream.current().span;
    let kind = if state.stream.current().kind == TokenKind::Question {
        nullable(state)
    } else if state.stream.current().kind == TokenKind::LeftParen {
        dnf(state)
    } else {
        let ty = optional_simple_data_type(state);

        match ty {
            Some(ty) => {
                if state.stream.current().kind == TokenKind::Pipe {
                    union(state, ty, false)
                } else if state.stream.current().kind == TokenKind::Ampersand
                    && !matches!(
                        state.stream.peek().kind,
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

    let end = state.stream.previous().span;

    Some(DataType::new(kind, Span::new(start.start, end.end)))
}

fn dnf(state: &mut State) -> Type<Name> {
    // (A|B|..)&C.. or (A&B&..)|C..
    state.stream.next();
    let ty = simple_data_type(state);

    match state.stream.current().kind {
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
        _ => {
            state.diagnostic(
                ParserDiagnostic::UnexpectedToken { token: *state.stream.current() },
                Severity::Error,
                state.stream.current().span,
            );

            Type::Missing
        }
    }
}

fn optional_simple_data_type(state: &mut State) -> Option<Type<Name>> {
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Array => {
            state.stream.next();

            Some(Type::Array)
        }
        TokenKind::Callable => {
            state.stream.next();

            Some(Type::Callable)
        }
        TokenKind::Null => {
            state.stream.next();

            Some(Type::Null)
        }
        TokenKind::True => {
            state.stream.next();

            Some(Type::True)
        }
        TokenKind::False => {
            state.stream.next();

            Some(Type::False)
        }
        TokenKind::Static => {
            state.stream.next();

            Some(Type::StaticReference)
        }
        TokenKind::Self_ => {
            state.stream.next();

            Some(Type::SelfReference)
        }
        TokenKind::Parent => {
            state.stream.next();

            Some(Type::ParentReference)
        }
        TokenKind::Enum | TokenKind::From => {
            state.stream.next();

            Some(Type::Named(state.maybe_resolve_identifier(*current, UseKind::Normal)))
        }
        TokenKind::Identifier => {
            let id = state.symbol_table.resolve(current.symbol.unwrap()).unwrap();
            state.stream.next();

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
                _ => Some(Type::Named(state.maybe_resolve_identifier(*current, UseKind::Normal))),
            }
        }
        TokenKind::FullyQualifiedIdentifier => {
            state.stream.next();

            let symbol = current.symbol.unwrap();

            Some(Type::Named(Name::resolved(symbol, symbol, current.span)))
        },
        TokenKind::QualifiedIdentifier => {
            state.stream.next();

            let name = state.maybe_resolve_identifier(*current, UseKind::Normal);

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
                state.stream.current().span,
            );

            Type::Missing
        }
    }
}

fn nullable(state: &mut State) -> Type<Name> {
    let current = state.stream.current();

    state.stream.next();

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
                    ParserDiagnostic::NestedDisjunctiveNormalFormType,
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
                    ParserDiagnostic::StandaloneTypeUsedInUnionType,
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

fn intersection(state: &mut State, other: Type<Name>, within_dnf: bool) -> Type<Name> {
    if other.standalone() {
        state.diagnostic(
            ParserDiagnostic::StandaloneTypeUsedInIntersectionType,
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
                    ParserDiagnostic::NestedDisjunctiveNormalFormType,
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
                    ParserDiagnostic::StandaloneTypeUsedInIntersectionType,
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
