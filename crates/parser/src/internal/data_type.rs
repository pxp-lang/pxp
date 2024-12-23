use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;
use pxp_type::{CallableParameter, ShapeItem, ShapeItemKey, ShapeUnsealedType, Type};

impl<'a> Parser<'a> {
    pub fn parse_data_type(&mut self) -> DataType {
        let kind = if self.is_in_docblock() {
            self.parse_docblock_type()
        } else if self.current_kind() == TokenKind::Question {
            self.parse_nullable_type()
        } else if self.current_kind() == TokenKind::LeftParen {
            self.parse_dnf_type()
        } else {
            let ty = self.parse_simple_data_type();

            if self.current_kind() == TokenKind::Pipe {
                self.parse_union_type(ty, false)
            } else if self.current_kind() == TokenKind::Ampersand
                && !matches!(
                    self.peek_kind(),
                    TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                )
            {
                self.parse_intersection_type(ty, false)
            } else {
                ty
            }
        };

        // FIXME: We need to create spans for types, but we don't have access to the previous token anymore.
        let span = Span::missing();

        DataType::new(self.id(), kind, span)
    }

    pub fn parse_optional_data_type(&mut self) -> Option<DataType> {
        let kind = if self.is_in_docblock() {
            self.parse_docblock_type()
        } else if self.current_kind() == TokenKind::Question {
            self.parse_nullable_type()
        } else if self.current_kind() == TokenKind::LeftParen {
            self.parse_dnf_type()
        } else {
            let ty = self.parse_optional_simple_data_type();

            match ty {
                Some(ty) => {
                    if self.current_kind() == TokenKind::Pipe {
                        self.parse_union_type(ty, false)
                    } else if self.current_kind() == TokenKind::Ampersand
                        && !matches!(
                            self.peek_kind(),
                            TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                        )
                    {
                        self.parse_intersection_type(ty, false)
                    } else {
                        ty
                    }
                }
                None => return None,
            }
        };

        // FIXME: We need to create spans for types, but we don't have access to the previous token anymore.
        let span = Span::missing();

        Some(DataType::new(self.id(), kind, span))
    }

    // Special type parsing logic for DocBlock comments, heavily based on the phpstan/phpdoc-parser package.
    fn parse_docblock_type(&mut self) -> Type<Name> {
        match self.current_kind() {
            TokenKind::Question => self.parse_docblock_nullable(),
            _ => {
                let r#type = self.parse_docblock_atomic();

                if r#type == Type::Missing {
                    return Type::Missing;
                }

                match self.current_kind() {
                    TokenKind::Pipe => self.parse_docblock_union(r#type),
                    TokenKind::Ampersand => self.parse_docblock_intersection(r#type),
                    _ => r#type,
                }
            }
        }
    }

    fn parse_docblock_nullable(&mut self) -> Type<Name> {
        self.next();

        let inner = self.parse_docblock_atomic();

        if inner == Type::Missing {
            return Type::Missing;
        }

        Type::Nullable(Box::new(inner))
    }

    fn parse_docblock_union(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Pipe = self.current_kind() {
            self.next();

            // FIXME: Warn about invalid types inside of union.
            types.push(self.parse_docblock_atomic());
        }

        Type::Union(types)
    }

    fn parse_docblock_subparse_union(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Pipe = self.current_kind() {
            self.next();

            self.skip_doc_eol();
            // FIXME: Warn about invalid types inside of union.
            types.push(self.parse_docblock_atomic());
            self.skip_doc_eol();
        }

        Type::Union(types)
    }

    fn parse_docblock_intersection(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Ampersand = self.current_kind() {
            self.next();

            // FIXME: Warn about invalid types inside of intersection.
            types.push(self.parse_docblock_atomic());
        }

        Type::Intersection(types)
    }

    fn parse_docblock_subparse_intersection(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut types = vec![lhs];

        while let TokenKind::Ampersand = self.current_kind() {
            self.next();

            self.skip_doc_eol();
            // FIXME: Warn about invalid types inside of intersection.
            types.push(self.parse_docblock_atomic());
            self.skip_doc_eol();
        }

        Type::Intersection(types)
    }

    fn parse_docblock_missing_type(&mut self) -> Type<Name> {
        self.diagnostic(
            ParserDiagnostic::MissingType,
            Severity::Warning,
            self.current_span(),
        );

        Type::Missing
    }

    fn parse_docblock_atomic(&mut self) -> Type<Name> {
        match self.current_kind() {
            TokenKind::LeftParen => {
                self.next();
                self.skip_doc_eol();

                let inner = self.parse_docblock_subparse();

                if inner == Type::Missing {
                    return self.parse_docblock_missing_type();
                }

                self.skip_doc_eol();

                if self.current_kind() != TokenKind::RightParen {
                    self.diagnostic(
                        ParserDiagnostic::ExpectedTokenExFound {
                            expected: vec![TokenKind::RightParen],
                        },
                        Severity::Warning,
                        self.current_span(),
                    );
                } else {
                    self.next();
                }

                if self.current_kind() == TokenKind::LeftBracket {
                    self.parse_docblock_array_or_offset_access(inner)
                } else {
                    inner
                }
            }
            TokenKind::Variable if self.current_symbol() == b"$this" => {
                self.next();

                if self.current_kind() == TokenKind::LeftBracket {
                    self.parse_docblock_array_or_offset_access(Type::This)
                } else {
                    Type::This
                }
            }
            _ => {
                let r#type = self
                    .parse_optional_simple_data_type()
                    .unwrap_or_else(|| self.parse_docblock_missing_type());

                if r#type == Type::Missing {
                    return Type::Missing;
                }

                // FIXME: Check for ! T:: here.
                let current = self.current();

                if current.kind == TokenKind::LessThan {
                    let mut r#type = self.parse_docblock_generic(r#type);

                    if self.current_kind() == TokenKind::LeftBracket {
                        r#type = self.parse_docblock_array_or_offset_access(r#type);
                    }

                    r#type
                } else if current.kind == TokenKind::LeftParen {
                    self.parse_docblock_callable(r#type)
                } else if current.kind == TokenKind::LeftBracket {
                    self.parse_docblock_array_or_offset_access(r#type)
                } else if matches!(r#type, Type::Array | Type::Object)
                    && current.kind == TokenKind::LeftBrace
                {
                    self.parse_docblock_array_shape(r#type)
                } else {
                    r#type
                }
            }
        }
    }

    fn parse_docblock_array_shape(&mut self, lhs: Type<Name>) -> Type<Name> {
        self.expect(TokenKind::LeftBrace);
        self.skip_doc_eol();

        let mut items = Vec::new();
        let mut sealed = true;
        let mut unsealed_type = None;

        while !self.is_eof() && self.current_kind() != TokenKind::RightBrace {
            self.skip_doc_eol();

            if self.current_kind() == TokenKind::Ellipsis {
                self.next();

                sealed = false;

                self.skip_doc_eol();

                if self.current_kind() == TokenKind::LessThan {
                    if lhs == Type::Array {
                        unsealed_type =
                            Some(Box::new(self.parse_docblock_array_shape_unsealed_type()));
                    } else {
                        unsealed_type =
                            Some(Box::new(self.parse_docblock_list_shape_unsealed_type()));
                    }
                }

                self.skip_doc_eol();

                if self.current_kind() == TokenKind::Comma {
                    self.next();
                }

                break;
            }

            items.push(self.parse_docblock_array_shape_item());

            self.skip_doc_eol();

            if self.current_kind() == TokenKind::Comma {
                self.next();
            }
        }

        self.skip_doc_eol();
        self.expect(TokenKind::RightBrace);

        Type::Shaped {
            base: Box::new(lhs),
            items,
            sealed,
            unsealed_type,
        }
    }

    fn parse_docblock_array_shape_unsealed_type(&mut self) -> ShapeUnsealedType<Name> {
        self.expect(TokenKind::LessThan);
        self.skip_doc_eol();

        let mut value_type = self.parse_docblock_type();
        self.skip_doc_eol();

        let mut key_type = None;
        if self.current_kind() == TokenKind::Comma {
            self.next();
            self.skip_doc_eol();

            key_type = Some(value_type);
            value_type = self.parse_docblock_type();

            self.skip_doc_eol();
        }

        self.expect(TokenKind::GreaterThan);

        ShapeUnsealedType {
            key_type,
            value_type,
        }
    }

    fn parse_docblock_list_shape_unsealed_type(&mut self) -> ShapeUnsealedType<Name> {
        self.expect(TokenKind::LessThan);
        self.skip_doc_eol();

        let value_type = self.parse_docblock_type();

        self.skip_doc_eol();
        self.expect(TokenKind::GreaterThan);

        ShapeUnsealedType {
            key_type: None,
            value_type,
        }
    }

    fn parse_docblock_array_shape_item(&mut self) -> ShapeItem<Name> {
        let (key_name, optional) = self.parse_docblock_array_shape_key();
        self.skip_doc_eol();
        let value_type = self.parse_docblock_type();

        ShapeItem {
            key_name,
            value_type,
            optional,
        }
    }

    fn parse_docblock_array_shape_key(&mut self) -> (Option<ShapeItemKey>, bool) {
        if !matches!(self.peek_kind(), TokenKind::Colon | TokenKind::Question) {
            return (None, false);
        }

        let key = match self.current_kind() {
            TokenKind::LiteralInteger => self.next_but_first(|parser| {
                Some(ShapeItemKey::Integer(parser.current_symbol_as_bytestring()))
            }),
            TokenKind::LiteralSingleQuotedString | TokenKind::LiteralDoubleQuotedString => self
                .next_but_first(|parser| {
                    Some(ShapeItemKey::String(parser.current_symbol_as_bytestring()))
                }),
            _ => self.next_but_first(|parser| {
                Some(ShapeItemKey::String(parser.current_symbol_as_bytestring()))
            }),
        };

        let optional = if self.current_kind() == TokenKind::Question {
            self.next();
            true
        } else {
            false
        };

        self.expect(TokenKind::Colon);

        (key, optional)
    }

    fn parse_docblock_generic(&mut self, lhs: Type<Name>) -> Type<Name> {
        self.next();
        let mut generic_types = vec![];
        let mut is_first = true;

        while is_first || self.current_kind() == TokenKind::Comma {
            if self.current_kind() == TokenKind::Comma {
                self.next();
            }

            self.skip_doc_eol();

            if !is_first && self.current_kind() == TokenKind::GreaterThan {
                break;
            }

            is_first = false;

            // FIXME: Parse variance keywords.
            if self.current_kind() == TokenKind::Asterisk {
                self.next();

                generic_types.push(Type::Mixed);
            } else {
                generic_types.push(self.parse_docblock_type());
            }

            self.skip_doc_eol();
        }

        if self.current_kind() == TokenKind::GreaterThan {
            self.next();
        } else {
            self.diagnostic(
                ParserDiagnostic::ExpectedTokenExFound {
                    expected: vec![TokenKind::GreaterThan],
                },
                Severity::Warning,
                self.current_span(),
            );
        }

        Type::Generic(Box::new(lhs), generic_types)
    }

    fn parse_docblock_callable(&mut self, lhs: Type<Name>) -> Type<Name> {
        self.skip(TokenKind::LeftParen);
        self.skip_doc_eol();

        let mut parameters = vec![];

        while self.current_kind() != TokenKind::RightParen {
            parameters.push(self.parse_docblock_callable_parameter());

            self.skip_doc_eol();

            if self.current_kind() == TokenKind::Comma {
                self.next();
            }

            self.skip_doc_eol();
        }

        self.skip(TokenKind::RightParen);

        let return_type = if self.current_kind() == TokenKind::Colon {
            self.next();
            self.skip_doc_eol();

            self.parse_docblock_type()
        } else {
            Type::Void
        };

        Type::CallableSignature(Box::new(lhs), parameters, Box::new(return_type))
    }

    fn parse_docblock_callable_parameter(&mut self) -> CallableParameter<Name> {
        // This isn't where we should be checking for variadics, but some projects
        // incorrectly place them before the type, so it's best to support it.
        let ellipsis = if self.current_kind() == TokenKind::Ellipsis {
            Some(self.next())
        } else {
            None
        };

        let r#type = self.parse_docblock_type();

        self.skip_doc_eol();

        let ampersand = if self.current_kind() == TokenKind::Ampersand {
            Some(self.next())
        } else {
            None
        };

        self.skip_doc_eol();

        let ellipsis = if ellipsis.is_some() {
            ellipsis
        } else if self.current_kind() == TokenKind::Ellipsis {
            Some(self.next())
        } else {
            None
        };

        self.skip_doc_eol();

        let name = if self.current_kind() == TokenKind::Variable {
            let name = self.current_symbol_as_bytestring();

            self.next();

            Some(name)
        } else {
            None
        };

        self.skip_doc_eol();

        let equal = if self.current_kind() == TokenKind::Equals {
            Some(self.next())
        } else {
            None
        };

        self.skip_doc_eol();

        CallableParameter {
            r#type,
            ellipsis,
            ampersand,
            equal,
            name,
        }
    }

    fn parse_docblock_array_or_offset_access(&mut self, lhs: Type<Name>) -> Type<Name> {
        let mut r#type = lhs;

        while let TokenKind::LeftBracket = self.current_kind() {
            self.next();

            // FIXME: Add offset type parsing here.

            if self.current_kind() == TokenKind::RightBracket {
                self.next();

                r#type = Type::TypedArray(Box::new(Type::array_key_types()), Box::new(r#type));
            }
        }

        r#type
    }

    fn parse_docblock_conditional(&mut self, lhs: Type<Name>) -> Type<Name> {
        self.skip(TokenKind::PhpDocIs);
        self.skip_doc_eol();

        let negated = if self.current_kind() == TokenKind::PhpDocNot {
            self.next();
            true
        } else {
            false
        };

        let target_type = self.parse_docblock_type();

        self.skip_doc_eol();

        self.skip(TokenKind::Question);

        self.skip_doc_eol();

        let if_type = self.parse_docblock_type();

        self.skip_doc_eol();

        self.skip(TokenKind::Colon);

        self.skip_doc_eol();

        let else_type = self.parse_docblock_subparse();

        Type::Conditional {
            subject: Box::new(lhs),
            negated,
            target: Box::new(target_type),
            then: Box::new(if_type),
            otherwise: Box::new(else_type),
        }
    }

    fn parse_docblock_conditional_for_parameter(&mut self) -> Type<Name> {
        let parameter = self.current_symbol_as_bytestring();

        self.next();
        self.skip_doc_eol();

        self.skip(TokenKind::PhpDocIs);

        self.skip_doc_eol();

        let negated = if self.current_kind() == TokenKind::PhpDocNot {
            self.next();
            self.skip_doc_eol();

            true
        } else {
            false
        };

        let target_type = self.parse_docblock_type();

        self.skip_doc_eol();

        self.skip(TokenKind::Question);

        self.skip_doc_eol();

        let if_type = self.parse_docblock_type();

        self.skip_doc_eol();

        self.skip(TokenKind::Colon);

        self.skip_doc_eol();

        let else_type = self.parse_docblock_subparse();

        Type::ConditionalForParameter {
            parameter,
            negated,
            target: Box::new(target_type),
            then: Box::new(if_type),
            otherwise: Box::new(else_type),
        }
    }

    fn parse_docblock_subparse(&mut self) -> Type<Name> {
        match self.current_kind() {
            TokenKind::Question => self.parse_docblock_nullable(),
            TokenKind::Variable => self.parse_docblock_conditional_for_parameter(),
            _ => {
                let r#type = self.parse_docblock_atomic();

                if r#type == Type::Missing {
                    return Type::Missing;
                }

                self.skip_doc_eol();

                if self.current_kind() == TokenKind::PhpDocIs {
                    self.parse_docblock_conditional(r#type)
                } else if self.current_kind() == TokenKind::Pipe {
                    self.parse_docblock_subparse_union(r#type)
                } else if self.current_kind() == TokenKind::Ampersand {
                    self.parse_docblock_subparse_intersection(r#type)
                } else {
                    r#type
                }
            }
        }
    }

    fn parse_dnf_type(&mut self) -> Type<Name> {
        // (A|B|..)&C.. or (A&B&..)|C..
        self.next();
        let ty = self.parse_simple_data_type();

        match self.current_kind() {
            TokenKind::Pipe => {
                let union = self.parse_union_type(ty, true);

                self.skip_right_parenthesis();

                self.parse_intersection_type(union, false)
            }
            TokenKind::Ampersand => {
                let intersection = self.parse_intersection_type(ty, true);

                self.skip_right_parenthesis();

                self.parse_union_type(intersection, false)
            }
            _ => {
                self.diagnostic(
                    ParserDiagnostic::UnexpectedToken {
                        token: self.current().to_owned(),
                    },
                    Severity::Error,
                    self.current_span(),
                );

                Type::Missing
            }
        }
    }

    fn parse_optional_simple_data_type(&mut self) -> Option<Type<Name>> {
        match self.current_kind() {
            TokenKind::Array => {
                self.next();

                Some(Type::Array)
            }
            TokenKind::Callable => {
                self.next();

                Some(Type::Callable)
            }
            TokenKind::Null => {
                self.next();

                Some(Type::Null)
            }
            TokenKind::True => {
                self.next();

                Some(Type::True)
            }
            TokenKind::False => {
                self.next();

                Some(Type::False)
            }
            TokenKind::Static => {
                self.next();

                Some(Type::StaticReference)
            }
            TokenKind::Self_ => {
                self.next();

                Some(Type::SelfReference)
            }
            TokenKind::Parent => {
                self.next();

                Some(Type::ParentReference)
            }
            TokenKind::Enum | TokenKind::From => {
                self.next();

                let id = self.id();

                Some(Type::Named(self.maybe_resolve_identifier(
                    id,
                    &self.current(),
                    UseKind::Normal,
                )))
            }
            TokenKind::Identifier => {
                self.next_but_first(|parser| match parser.current_symbol().as_ref() {
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
                    b"array-key" if parser.is_in_docblock() => Some(Type::ArrayKey),
                    b"value-of" if parser.is_in_docblock() => Some(Type::ValueOf),
                    b"class-string" if parser.is_in_docblock() => Some(Type::ClassString),
                    _ => {
                        let id = parser.id();

                        Some(Type::Named(parser.maybe_resolve_identifier(
                            id,
                            &parser.current(),
                            UseKind::Normal,
                        )))
                    }
                })
            }
            TokenKind::FullyQualifiedIdentifier => {
                let symbol = self.current_symbol_as_bytestring();
                let resolved = self.strip_leading_namespace_qualifier(&symbol);
                let span = self.next();

                Some(Type::Named(Name::resolved(
                    self.id(),
                    resolved,
                    symbol,
                    span,
                )))
            }
            TokenKind::QualifiedIdentifier => {
                let id = self.id();
                let name = self.maybe_resolve_identifier(id, &self.current(), UseKind::Normal);
                self.next();

                Some(Type::Named(name))
            }
            _ => None,
        }
    }

    fn parse_simple_data_type(&mut self) -> Type<Name> {
        match self.parse_optional_simple_data_type() {
            Some(ty) => ty,
            None => {
                self.diagnostic(
                    ParserDiagnostic::MissingType,
                    Severity::Error,
                    self.current_span(),
                );

                Type::Missing
            }
        }
    }

    fn parse_nullable_type(&mut self) -> Type<Name> {
        let span = self.next();

        let ty = self.parse_simple_data_type();

        if ty.standalone() {
            self.diagnostic(
                ParserDiagnostic::StandaloneTypeUsedInNullableType,
                Severity::Error,
                span,
            );
        }

        Type::Nullable(Box::new(ty))
    }

    fn parse_union_type(&mut self, other: Type<Name>, within_dnf: bool) -> Type<Name> {
        if other.standalone() {
            self.diagnostic(
                ParserDiagnostic::StandaloneTypeUsedInUnionType,
                Severity::Error,
                self.current_span(),
            );
        }

        let mut types = vec![other];
        let mut last_pipe = self.skip(TokenKind::Pipe);

        loop {
            let current = self.current();
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
                    self.diagnostic(
                        ParserDiagnostic::NestedDisjunctiveNormalFormType,
                        Severity::Error,
                        current.span,
                    );
                }

                self.next();

                let other = self.parse_simple_data_type();
                let ty = self.parse_intersection_type(other, true);

                self.skip_right_parenthesis();

                ty
            } else {
                let ty = self.parse_simple_data_type();
                if ty.standalone() {
                    self.diagnostic(
                        ParserDiagnostic::StandaloneTypeUsedInUnionType,
                        Severity::Error,
                        last_pipe,
                    );
                }

                ty
            };

            types.push(ty);

            if self.current_kind() == TokenKind::Pipe {
                last_pipe = self.skip(TokenKind::Pipe);
            } else {
                break;
            }
        }

        Type::Union(types)
    }

    fn parse_intersection_type(&mut self, other: Type<Name>, within_dnf: bool) -> Type<Name> {
        if other.standalone() {
            self.diagnostic(
                ParserDiagnostic::StandaloneTypeUsedInIntersectionType,
                Severity::Error,
                self.current_span(),
            );
        }

        let mut types = vec![other];

        let mut last_ampersand = self.skip(TokenKind::Ampersand);

        loop {
            let current = self.current();
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

                    self.diagnostic(
                        ParserDiagnostic::NestedDisjunctiveNormalFormType,
                        Severity::Error,
                        current.span,
                    );
                }

                self.next();

                let other = self.parse_simple_data_type();
                let ty = self.parse_union_type(other, true);

                self.skip_right_parenthesis();

                ty
            } else {
                let ty = self.parse_simple_data_type();
                if ty.standalone() {
                    self.diagnostic(
                        ParserDiagnostic::StandaloneTypeUsedInIntersectionType,
                        Severity::Error,
                        last_ampersand,
                    );
                }

                ty
            };

            types.push(ty);

            if self.current_kind() == TokenKind::Ampersand
                && !matches!(
                    self.peek_kind(),
                    TokenKind::Variable | TokenKind::Ellipsis | TokenKind::Ampersand
                )
            {
                last_ampersand = self.skip(TokenKind::Ampersand);
            } else {
                break;
            }
        }

        Type::Intersection(types)
    }
}
