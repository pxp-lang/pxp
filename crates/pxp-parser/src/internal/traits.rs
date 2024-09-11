use crate::internal::identifiers;
use crate::internal::utils;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::TraitUsageAdaptation;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::Token;
use pxp_token::TokenKind;

use super::classes::member;
use super::names;

pub fn usage(state: &mut State) -> TraitUsage {
    let span = utils::skip(state, TokenKind::Use);

    let mut traits = Vec::new();

    while state.current().kind != TokenKind::SemiColon
        && state.current().kind != TokenKind::LeftBrace
    {
        let t = names::full_name(state, UseKind::Normal);
        traits.push(t);

        if state.current().kind == TokenKind::Comma {
            if state.peek().kind == TokenKind::SemiColon {
                // will fail with unexpected token `,`
                // as `use` doesn't allow for trailing commas.
                utils::skip_semicolon(state);
            } else if state.peek().kind == TokenKind::LeftBrace {
                // will fail with unexpected token `{`
                // as `use` doesn't allow for trailing commas.
                utils::skip_left_brace(state);
            } else {
                state.next();
            }
        } else {
            break;
        }
    }

    let mut adaptations = Vec::new();
    if state.current().kind == TokenKind::LeftBrace {
        utils::skip_left_brace(state);

        while state.current().kind != TokenKind::RightBrace {
            let (r#trait, method): (Option<Name>, SimpleIdentifier) = match state.peek().kind
            {
                TokenKind::DoubleColon => {
                    let r#trait = names::full_name_including_self(state);
                    state.next();
                    let method = identifiers::identifier(state);
                    (Some(r#trait), method)
                }
                _ => (None, identifiers::identifier(state)),
            };

            while !state.is_eof()
                && !matches!(
                    state.current().kind,
                    TokenKind::As | TokenKind::Insteadof
                )
            {
                let token = state.current();
                state.next();

                state.diagnostic(
                    ParserDiagnostic::ExpectedToken {
                        expected: vec![TokenKind::As, TokenKind::Insteadof],
                        found: token.clone(),
                    },
                    Severity::Error,
                    token.span,
                );
            }

            match state.current().kind {
                TokenKind::As => {
                    state.next();

                    match state.current() {
                        Token {
                            kind: TokenKind::Public | TokenKind::Protected | TokenKind::Private,
                            span,
                            ..
                        } => {
                            let visibility = match state.current().kind {
                                TokenKind::Public => VisibilityModifier::Public(*span),
                                TokenKind::Protected => VisibilityModifier::Protected(*span),
                                TokenKind::Private => VisibilityModifier::Private(*span),
                                _ => unreachable!(),
                            };

                            state.next();

                            if state.current().kind == TokenKind::SemiColon {
                                let span = if r#trait.is_some() {
                                    Span::combine(r#trait.span(), visibility.span())
                                } else {
                                    Span::combine(method.span, visibility.span())
                                };
                                adaptations.push(TraitUsageAdaptation {
                                    id: state.id(),
                                    span,
                                    kind: TraitUsageAdaptationKind::Visibility(
                                        TraitUsageAdaptationVisibility {
                                            id: state.id(),
                                            span,
                                            r#trait,
                                            method,
                                            visibility,
                                        },
                                    ),
                                });
                            } else {
                                let alias: SimpleIdentifier = identifiers::name(state);
                                let span = if r#trait.is_some() {
                                    Span::combine(r#trait.span(), visibility.span())
                                } else {
                                    Span::combine(method.span, visibility.span())
                                };

                                adaptations.push(TraitUsageAdaptation {
                                    id: state.id(),
                                    span,
                                    kind: TraitUsageAdaptationKind::Alias(
                                        TraitUsageAdaptationAlias {
                                            id: state.id(),
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
                            let alias: SimpleIdentifier = identifiers::name(state);
                            let span = if r#trait.is_some() {
                                Span::combine(r#trait.span(), alias.span())
                            } else {
                                Span::combine(method.span, alias.span())
                            };

                            adaptations.push(TraitUsageAdaptation {
                                id: state.id(),
                                span,
                                kind: TraitUsageAdaptationKind::Alias(TraitUsageAdaptationAlias {
                                    id: state.id(),
                                    span,
                                    r#trait,
                                    method,
                                    alias,
                                    visibility: None,
                                }),
                            });
                        }
                    }
                }
                TokenKind::Insteadof => {
                    state.next();

                    let mut insteadof = vec![identifiers::full_type_name(state)];

                    if state.current().kind == TokenKind::Comma {
                        if state.peek().kind == TokenKind::SemiColon {
                            // will fail with unexpected token `,`
                            // as `insteadof` doesn't allow for trailing commas.
                            utils::skip_semicolon(state);
                        }

                        state.next();

                        while state.current().kind != TokenKind::SemiColon {
                            insteadof.push(identifiers::full_type_name(state));

                            if state.current().kind == TokenKind::Comma {
                                if state.peek().kind == TokenKind::SemiColon {
                                    // will fail with unexpected token `,`
                                    // as `insteadof` doesn't allow for trailing commas.
                                    utils::skip_semicolon(state);
                                } else {
                                    state.next();
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
                        id: state.id(),
                        span,
                        kind: TraitUsageAdaptationKind::Precedence(
                            TraitUsageAdaptationPrecedence {
                                id: state.id(),
                                span,
                                r#trait,
                                method,
                                insteadof,
                            },
                        ),
                    });
                }
                _ => unreachable!("{:?}", state.current()),
            };

            utils::skip_semicolon(state);
        }

        utils::skip_right_brace(state);
    } else {
        utils::skip_semicolon(state);
    }

    TraitUsage {
        id: state.id(),
        span: Span::combine(span, adaptations.span()),
        r#use: span,
        traits,
        adaptations,
    }
}

pub fn parse(state: &mut State) -> StatementKind {
    let span = utils::skip(state, TokenKind::Trait);
    let name = names::type_name(state);
    let attributes = state.get_attributes();

    let left_brace = utils::skip_left_brace(state);
    let members = {
        let mut members = Vec::new();
        while state.current().kind != TokenKind::RightBrace && !state.is_eof() {
            members.push(member(state, true));
        }
        members
    };
    let right_brace = utils::skip_right_brace(state);

    let body = TraitBody {
        id: state.id(),
        span: Span::combine(left_brace, right_brace),
        left_brace,
        members,
        right_brace,
    };

    StatementKind::Trait(TraitStatement {
        id: state.id(),
        span: Span::combine(span, body.span),
        r#trait: span,
        name,
        attributes,
        body,
    })
}
