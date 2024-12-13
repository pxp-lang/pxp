use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::Parser;
use crate::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::TraitUsageAdaptation;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::Token;
use pxp_token::TokenKind;

use super::classes::parse_classish_member;
use super::names;

impl<'a> Parser<'a> {
    pub fn parse_trait_usage(&mut self) -> TraitUsage {
        let span = self.skip(TokenKind::Use);

        let mut traits = Vec::new();

        while self.current().kind != TokenKind::SemiColon
            && self.current().kind != TokenKind::LeftBrace
        {
            let t = names::parse_full_name(state, UseKind::Normal);
            traits.push(t);

            if self.current().kind == TokenKind::Comma {
                if state.peek().kind == TokenKind::SemiColon {
                    // will fail with unexpected token `,`
                    // as `use` doesn't allow for trailing commas.
                    utils::skip_semicolon();
                } else if state.peek().kind == TokenKind::LeftBrace {
                    // will fail with unexpected token `{`
                    // as `use` doesn't allow for trailing commas.
                    utils::skip_left_brace();
                } else {
                    self.next();
                }
            } else {
                break;
            }
        }

        let mut adaptations = Vec::new();
        if self.current().kind == TokenKind::LeftBrace {
            utils::skip_left_brace();

            while self.current().kind != TokenKind::RightBrace {
                let (r#trait, method): (Option<Name>, SimpleIdentifier) = match state.peek().kind {
                    TokenKind::DoubleColon => {
                        let r#trait = names::parse_full_name_including_self();
                        self.next();
                        let method = identifiers::parse_identifier();
                        (Some(r#trait), method)
                    }
                    _ => (None, identifiers::parse_identifier()),
                };

                while !state.is_eof()
                    && !matches!(self.current().kind, TokenKind::As | TokenKind::Insteadof)
                {
                    let token = self.current();
                    self.next();

                    self.diagnostic(
                        ParserDiagnostic::ExpectedToken {
                            expected: vec![TokenKind::As, TokenKind::Insteadof],
                            found: token.clone(),
                        },
                        Severity::Error,
                        token.span,
                    );
                }

                match self.current().kind {
                    TokenKind::As => {
                        self.next();

                        match self.current() {
                            Token {
                                kind: TokenKind::Public | TokenKind::Protected | TokenKind::Private,
                                span,
                                ..
                            } => {
                                let visibility = match self.current().kind {
                                    TokenKind::Public => VisibilityModifier::Public(*span),
                                    TokenKind::Protected => VisibilityModifier::Protected(*span),
                                    TokenKind::Private => VisibilityModifier::Private(*span),
                                    _ => unreachable!(),
                                };

                                self.next();

                                if self.current().kind == TokenKind::SemiColon {
                                    let span = if r#trait.is_some() {
                                        Span::combine(r#trait.span(), visibility.span())
                                    } else {
                                        Span::combine(method.span, visibility.span())
                                    };
                                    adaptations.push(TraitUsageAdaptation {
                                        id: self.state.id(),
                                        span,
                                        kind: TraitUsageAdaptationKind::Visibility(
                                            TraitUsageAdaptationVisibility {
                                                id: self.state.id(),
                                                span,
                                                r#trait,
                                                method,
                                                visibility,
                                            },
                                        ),
                                    });
                                } else {
                                    let alias: SimpleIdentifier =
                                        identifiers::parse_name_identifier();
                                    let span = if r#trait.is_some() {
                                        Span::combine(r#trait.span(), visibility.span())
                                    } else {
                                        Span::combine(method.span, visibility.span())
                                    };

                                    adaptations.push(TraitUsageAdaptation {
                                        id: self.state.id(),
                                        span,
                                        kind: TraitUsageAdaptationKind::Alias(
                                            TraitUsageAdaptationAlias {
                                                id: self.state.id(),
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
                                let alias: SimpleIdentifier =
                                    identifiers::parse_name_identifier();
                                let span = if r#trait.is_some() {
                                    Span::combine(r#trait.span(), alias.span())
                                } else {
                                    Span::combine(method.span, alias.span())
                                };

                                adaptations.push(TraitUsageAdaptation {
                                    id: self.state.id(),
                                    span,
                                    kind: TraitUsageAdaptationKind::Alias(
                                        TraitUsageAdaptationAlias {
                                            id: self.state.id(),
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

                        let mut insteadof =
                            vec![identifiers::parse_full_type_name_identifier()];

                        if self.current().kind == TokenKind::Comma {
                            if state.peek().kind == TokenKind::SemiColon {
                                // will fail with unexpected token `,`
                                // as `insteadof` doesn't allow for trailing commas.
                                utils::skip_semicolon();
                            }

                            self.next();

                            while self.current().kind != TokenKind::SemiColon {
                                insteadof.push(identifiers::parse_full_type_name_identifier());

                                if self.current().kind == TokenKind::Comma {
                                    if state.peek().kind == TokenKind::SemiColon {
                                        // will fail with unexpected token `,`
                                        // as `insteadof` doesn't allow for trailing commas.
                                        utils::skip_semicolon();
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
                            id: self.state.id(),
                            span,
                            kind: TraitUsageAdaptationKind::Precedence(
                                TraitUsageAdaptationPrecedence {
                                    id: self.state.id(),
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

                utils::skip_semicolon();
            }

            utils::skip_right_brace();
        } else {
            utils::skip_semicolon();
        }

        TraitUsage {
            id: self.state.id(),
            span: Span::combine(span, adaptations.span()),
            r#use: span,
            traits,
            adaptations,
        }
    }

    pub fn parse_trait(&mut self) -> StatementKind {
        let span = self.skip(TokenKind::Trait);
        let name = names::parse_type_name();
        let attributes = state.get_attributes();

        let left_brace = utils::skip_left_brace();
        let members = {
            let mut members = Vec::new();
            while self.current().kind != TokenKind::RightBrace && !state.is_eof() {
                members.push(parse_classish_member(state, true));
            }
            members
        };
        let right_brace = utils::skip_right_brace();

        let body = TraitBody {
            id: self.state.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            members,
            right_brace,
        };

        StatementKind::Trait(TraitStatement {
            id: self.state.id(),
            span: Span::combine(span, body.span),
            r#trait: span,
            name,
            attributes,
            body,
        })
    }
}
