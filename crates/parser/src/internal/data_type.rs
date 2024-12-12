use crate::state::State;
use crate::Parser;
use crate::{internal::utils, ParserDiagnostic};
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;
use pxp_type::Type;

impl<'a> Parser<'a> {
    pub fn parse_data_type(&mut self) -> DataType {
        let start = state.current().span;

        let kind = if state.is_in_docblock() {
            parse_docblock_type(state)
        } else if state.current().kind == TokenKind::Question {
            parse_nullable_type(state)
        } else if state.current().kind == TokenKind::LeftParen {
            parse_dnf_type(state)
        } else {
            let ty = parse_simple_data_type(state);

            if state.current().kind == TokenKind::Pipe {
                parse_union_type(state, ty, false)
            } else if state.current().kind == TokenKind::Ampersand
                && !matches!(
                    state.peek().kind,
                    TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                )
            {
                parse_intersection_type(state, ty, false)
            } else {
                ty
            }
        };

        let end = state.previous().span;

        DataType::new(state.id(), kind, Span::new(start.start, end.end))
    }

    pub fn parse_optional_data_type(&mut self) -> Option<DataType> {
        let start = state.current().span;
        let kind = if state.is_in_docblock() {
            parse_docblock_type(state)
        } else if state.current().kind == TokenKind::Question {
            parse_nullable_type(state)
        } else if state.current().kind == TokenKind::LeftParen {
            parse_dnf_type(state)
        } else {
            let ty = parse_optional_simple_data_type(state);

            match ty {
                Some(ty) => {
                    if state.current().kind == TokenKind::Pipe {
                        parse_union_type(state, ty, false)
                    } else if state.current().kind == TokenKind::Ampersand
                        && !matches!(
                            state.peek().kind,
                            TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                        )
                    {
                        parse_intersection_type(state, ty, false)
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
    fn parse_docblock_type(&mut self) -> Type<Name> {
        let current = state.current();

        match current.kind {
            TokenKind::Question => parse_docblock_nullable(state),
            _ => {
                let r#type = parse_docblock_atomic(state);

                if r#type == Type::Missing {
                    return Type::Missing;
                }

                let current = state.current();

                match current.kind {
                    TokenKind::Pipe => parse_docblock_union(state, r#type),
                    TokenKind::Ampersand => parse_docblock_intersection(state, r#type),
                    _ => r#type,
                }
            }
        }
    }

    fn parse_docblock_nullable(&mut self) -> Type<Name> {
        state.next();

        let inner = parse_docblock_atomic(state);

        if inner == Type::Missing {
            return Type::Missing;
        }

        Type::Nullable(Box::new(inner))
    }

    fn parse_docblock_union(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Pipe = state.current().kind {
            state.next();

            // FIXME: Warn about invalid types inside of union.
            types.push(parse_docblock_atomic(state));
        }

        Type::Union(types)
    }

    fn parse_docblock_subparse_union(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Pipe = state.current().kind {
            state.next();

            state.skip_doc_eol();
            // FIXME: Warn about invalid types inside of union.
            types.push(parse_docblock_atomic(state));
            state.skip_doc_eol();
        }

        Type::Union(types)
    }

    fn parse_docblock_intersection(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Ampersand = state.current().kind {
            state.next();

            // FIXME: Warn about invalid types inside of intersection.
            types.push(parse_docblock_atomic(state));
        }

        Type::Intersection(types)
    }

    fn parse_docblock_subparse_intersection(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Ampersand = state.current().kind {
            state.next();

            state.skip_doc_eol();
            // FIXME: Warn about invalid types inside of intersection.
            types.push(parse_docblock_atomic(state));
            state.skip_doc_eol();
        }

        Type::Intersection(types)
    }

    fn parse_docblock_missing_type(&mut self) -> Type<Name> {
        state.diagnostic(
            ParserDiagnostic::MissingType,
            Severity::Warning,
            state.current().span,
        );

        Type::Missing
    }

    fn parse_docblock_atomic(&mut self) -> Type<Name> {
        let current = state.current();

        match current.kind {
            TokenKind::LeftParen => {
                state.next();
                state.skip_doc_eol();

                let inner = parse_docblock_subparse(state);

                if inner == Type::Missing {
                    return parse_docblock_missing_type(state);
                }

                state.skip_doc_eol();

                if state.current().kind != TokenKind::RightParen {
                    state.diagnostic(
                        ParserDiagnostic::ExpectedTokenExFound {
                            expected: vec![TokenKind::RightParen],
                        },
                        Severity::Warning,
                        state.current().span,
                    );
                } else {
                    state.next();
                }

                if state.current().kind == TokenKind::LeftBracket {
                    parse_docblock_array_or_offset_access(state, inner)
                } else {
                    inner
                }
            }
            TokenKind::Variable if current.symbol.as_ref().is_some_and(|sym| sym == b"$this") => {
                state.next();

                if state.current().kind == TokenKind::LeftBracket {
                    parse_docblock_array_or_offset_access(state, Type::This)
                } else {
                    Type::This
                }
            }
            _ => {
                let r#type = parse_optional_simple_data_type(state)
                    .unwrap_or_else(|| parse_docblock_missing_type(state));

                if r#type == Type::Missing {
                    return Type::Missing;
                }

                // FIXME: Check for ! T:: here.
                let current = state.current();

                if current.kind == TokenKind::LessThan {
                    let mut r#type = parse_docblock_generic(state, r#type);

                    if state.current().kind == TokenKind::LeftBracket {
                        r#type = parse_docblock_array_or_offset_access(state, r#type);
                    }

                    r#type
                } else if current.kind == TokenKind::LeftParen {
                    todo!("parse docblock callable type");
                } else if current.kind == TokenKind::LeftBracket {
                    parse_docblock_array_or_offset_access(state, r#type)
                } else {
                    r#type
                }
            }
        }
    }

    fn parse_docblock_generic(&mut self, lhs: Type<Name>) -> Type<Name> {
        state.next();
        let mut generic_types = vec![];
        let mut is_first = true;

        while is_first || state.current().kind == TokenKind::Comma {
            if state.current().kind == TokenKind::Comma {
                state.next();
            }

            state.skip_doc_eol();

            if !is_first && state.current().kind == TokenKind::GreaterThan {
                break;
            }

            is_first = false;

            // FIXME: Parse variance keywords and wildcards here too.
            generic_types.push(parse_docblock_type(state));

            state.skip_doc_eol();
        }

        if state.current().kind == TokenKind::GreaterThan {
            state.next();
        } else {
            state.diagnostic(
                ParserDiagnostic::ExpectedTokenExFound {
                    expected: vec![TokenKind::GreaterThan],
                },
                Severity::Warning,
                state.current().span,
            );
        }

        Type::NamedWithGenerics(Box::new(lhs), generic_types)
    }

    fn parse_docblock_array_or_offset_access(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut r#type = lhs;

        while let TokenKind::LeftBracket = state.current().kind {
            state.next();

            // FIXME: Add offset type parsing here.

            if state.current().kind == TokenKind::RightBracket {
                state.next();

                r#type = Type::TypedArray(Box::new(Type::array_key_types()), Box::new(r#type));
            }
        }

        r#type
    }

    fn parse_docblock_subparse(&mut self) -> Type<Name> {
        let current = state.current();

        match &current.kind {
            TokenKind::Question => parse_docblock_nullable(state),
            TokenKind::Variable => todo!(),
            _ => {
                let r#type = parse_docblock_atomic(state);

                if r#type == Type::Missing {
                    return Type::Missing;
                }

                let current = state.current();

                if current.kind == TokenKind::Identifier
                    && current.symbol.as_ref().is_some_and(|sym| sym == b"is")
                {
                    todo!("parse docblock conditional type");
                }

                state.skip_doc_eol();

                let current = state.current();

                if current.kind == TokenKind::Pipe {
                    parse_docblock_subparse_union(state, r#type)
                } else if current.kind == TokenKind::Ampersand {
                    parse_docblock_subparse_intersection(state, r#type)
                } else {
                    r#type
                }
            }
        }
    }

    fn parse_dnf_type(&mut self) -> Type<Name> {
        // (A|B|..)&C.. or (A&B&..)|C..
        state.next();
        let ty = parse_simple_data_type(state);

        match state.current().kind {
            TokenKind::Pipe => {
                let union = parse_union_type(state, ty, true);

                utils::skip_right_parenthesis(state);

                parse_intersection_type(state, union, false)
            }
            TokenKind::Ampersand => {
                let intersection = parse_intersection_type(state, ty, true);

                utils::skip_right_parenthesis(state);

                parse_union_type(state, intersection, false)
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

    fn parse_optional_simple_data_type(&mut self) -> Option<Type<Name>> {
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

    fn parse_simple_data_type(&mut self) -> Type<Name> {
        match parse_optional_simple_data_type(state) {
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

    fn parse_nullable_type(&mut self) -> Type<Name> {
        let current = state.current();

        state.next();

        let ty = parse_simple_data_type(state);

        if ty.standalone() {
            state.diagnostic(
                ParserDiagnostic::StandaloneTypeUsedInNullableType,
                Severity::Error,
                current.span,
            );
        }

        Type::Nullable(Box::new(ty))
    }

    fn parse_union_type(&mut self, other: Type<Name>, within_dnf: bool) -> Type<Name> {
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

                let other = parse_simple_data_type(state);
                let ty = parse_intersection_type(state, other, true);

                utils::skip_right_parenthesis(state);

                ty
            } else {
                let ty = parse_simple_data_type(state);
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

    fn parse_intersection_type(&mut self, other: Type<Name>, within_dnf: bool) -> Type<Name> {
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

                let other = parse_simple_data_type(state);
                let ty = parse_union_type(state, other, true);

                utils::skip_right_parenthesis(state);

                ty
            } else {
                let ty = parse_simple_data_type(state);
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
}
