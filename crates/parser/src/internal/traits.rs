use crate::internal::diagnostics::ParserDiagnostic;
use crate::Parser;
use pxp_ast::StatementKind;
use pxp_ast::TraitUsageAdaptation;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_trait_usage(&mut self) -> TraitUsage {
        let span = self.skip(TokenKind::Use);

        let mut traits = Vec::new();

        while self.current_kind() != TokenKind::SemiColon
            && self.current_kind() != TokenKind::LeftBrace
        {
            let t = self.parse_full_name(UseKind::Normal);
            traits.push(t);

            if self.current_kind() == TokenKind::Comma {
                if self.peek_kind() == TokenKind::SemiColon {
                    // will fail with unexpected token `,`
                    // as `use` doesn't allow for trailing commas.
                    self.skip_semicolon();
                } else if self.peek_kind() == TokenKind::LeftBrace {
                    // will fail with unexpected token `{`
                    // as `use` doesn't allow for trailing commas.
                    self.skip_left_brace();
                } else {
                    self.next();
                }
            } else {
                break;
            }
        }

        let mut adaptations = Vec::new();
        if self.current_kind() == TokenKind::LeftBrace {
            self.skip_left_brace();

            while self.current_kind() != TokenKind::RightBrace {
                let (r#trait, method): (Option<Name>, SimpleIdentifier) = match self.peek_kind() {
                    TokenKind::DoubleColon => {
                        let r#trait = self.parse_full_name_including_self();
                        self.next();
                        let method = self.parse_identifier();
                        (Some(r#trait), method)
                    }
                    _ => (None, self.parse_identifier()),
                };

                while !self.is_eof()
                    && !matches!(self.current_kind(), TokenKind::As | TokenKind::Insteadof)
                {
                    let token = self.current().to_owned();
                    let span = token.span;

                    self.next();

                    self.diagnostic(
                        ParserDiagnostic::ExpectedToken {
                            expected: vec![TokenKind::As, TokenKind::Insteadof],
                            found: token,
                        },
                        Severity::Error,
                        span,
                    );
                }

                match self.current_kind() {
                    TokenKind::As => {
                        self.next();

                        match self.current_kind() {
                            TokenKind::Public | TokenKind::Protected | TokenKind::Private => {
                                let span = self.current_span();

                                let visibility = match self.current_kind() {
                                    TokenKind::Public => VisibilityModifier::Public(span),
                                    TokenKind::Protected => VisibilityModifier::Protected(span),
                                    TokenKind::Private => VisibilityModifier::Private(span),
                                    _ => unreachable!(),
                                };

                                self.next();

                                if self.current_kind() == TokenKind::SemiColon {
                                    let span = if r#trait.is_some() {
                                        Span::combine(r#trait.span(), visibility.span())
                                    } else {
                                        Span::combine(method.span, visibility.span())
                                    };
                                    adaptations.push(TraitUsageAdaptation {
                                        id: self.id(),
                                        span,
                                        kind: TraitUsageAdaptationKind::Visibility(
                                            TraitUsageAdaptationVisibility {
                                                id: self.id(),
                                                span,
                                                r#trait,
                                                method,
                                                visibility,
                                            },
                                        ),
                                    });
                                } else {
                                    let alias: SimpleIdentifier = self.parse_name_identifier();
                                    let span = if r#trait.is_some() {
                                        Span::combine(r#trait.span(), visibility.span())
                                    } else {
                                        Span::combine(method.span, visibility.span())
                                    };

                                    adaptations.push(TraitUsageAdaptation {
                                        id: self.id(),
                                        span,
                                        kind: TraitUsageAdaptationKind::Alias(
                                            TraitUsageAdaptationAlias {
                                                id: self.id(),
                                                span,
                                                r#trait,
                                                method,
                                                alias,
                                                visibility: Some(visibility),
                                            },
                                        ),
                                    });
                                }
                            }
                            _ => {
                                let alias: SimpleIdentifier = self.parse_name_identifier();
                                let span = if r#trait.is_some() {
                                    Span::combine(r#trait.span(), alias.span())
                                } else {
                                    Span::combine(method.span, alias.span())
                                };

                                adaptations.push(TraitUsageAdaptation {
                                    id: self.id(),
                                    span,
                                    kind: TraitUsageAdaptationKind::Alias(
                                        TraitUsageAdaptationAlias {
                                            id: self.id(),
                                            span,
                                            r#trait,
                                            method,
                                            alias,
                                            visibility: None,
                                        },
                                    ),
                                });
                            }
                        }
                    }
                    TokenKind::Insteadof => {
                        self.next();

                        let mut insteadof = vec![self.parse_full_type_name_identifier()];

                        if self.current_kind() == TokenKind::Comma {
                            if self.peek_kind() == TokenKind::SemiColon {
                                // will fail with unexpected token `,`
                                // as `insteadof` doesn't allow for trailing commas.
                                self.skip_semicolon();
                            }

                            self.next();

                            while self.current_kind() != TokenKind::SemiColon {
                                insteadof.push(self.parse_full_type_name_identifier());

                                if self.current_kind() == TokenKind::Comma {
                                    if self.peek_kind() == TokenKind::SemiColon {
                                        // will fail with unexpected token `,`
                                        // as `insteadof` doesn't allow for trailing commas.
                                        self.skip_semicolon();
                                    } else {
                                        self.next();
                                    }
                                } else {
                                    break;
                                }
                            }
                        }

                        let span = if r#trait.is_some() {
                            Span::combine(r#trait.span(), insteadof.span())
                        } else {
                            Span::combine(method.span, insteadof.span())
                        };

                        adaptations.push(TraitUsageAdaptation {
                            id: self.id(),
                            span,
                            kind: TraitUsageAdaptationKind::Precedence(
                                TraitUsageAdaptationPrecedence {
                                    id: self.id(),
                                    span,
                                    r#trait,
                                    method,
                                    insteadof,
                                },
                            ),
                        });
                    }
                    _ => unreachable!("{:?}", self.current()),
                };

                self.skip_semicolon();
            }

            self.skip_right_brace();
        } else {
            self.skip_semicolon();
        }

        TraitUsage {
            id: self.id(),
            span: Span::combine(span, adaptations.span()),
            r#use: span,
            traits,
            adaptations,
        }
    }

    pub fn parse_trait(&mut self) -> StatementKind {
        let span = self.skip(TokenKind::Trait);
        let name = self.parse_type_name();
        let attributes = self.get_attributes();

        let left_brace = self.skip_left_brace();
        let members = {
            let mut members = Vec::new();
            while self.current_kind() != TokenKind::RightBrace && !self.is_eof() {
                members.push(self.parse_classish_member(true));
            }
            members
        };
        let right_brace = self.skip_right_brace();

        let body = TraitBody {
            id: self.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            members,
            right_brace,
        };

        StatementKind::Trait(TraitStatement {
            id: self.id(),
            span: Span::combine(span, body.span),
            r#trait: span,
            name,
            attributes,
            body,
        })
    }
}
