use crate::expect_token;
use crate::internal::identifiers;
use crate::internal::utils;
use crate::peek_token;
use crate::state::State;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::modifiers::VisibilityModifier;
use pxp_ast::traits::TraitBody;
use pxp_ast::traits::TraitStatement;
use pxp_ast::traits::TraitUsage;
use pxp_ast::traits::TraitUsageAdaptation;
use pxp_ast::StatementKind;
use pxp_token::Token;
use pxp_token::TokenKind;

use super::classes::member;

pub fn usage(state: &mut State) -> TraitUsage {
    let span = utils::skip(state, TokenKind::Use);

    let mut traits = Vec::new();

    while state.stream.current().kind != TokenKind::SemiColon
        && state.stream.current().kind != TokenKind::LeftBrace
    {
        let t = identifiers::full_type_name(state);
        traits.push(t);

        if state.stream.current().kind == TokenKind::Comma {
            if state.stream.peek().kind == TokenKind::SemiColon {
                // will fail with unexpected token `,`
                // as `use` doesn't allow for trailing commas.
                utils::skip_semicolon(state);
            } else if state.stream.peek().kind == TokenKind::LeftBrace {
                // will fail with unexpected token `{`
                // as `use` doesn't allow for trailing commas.
                utils::skip_left_brace(state);
            } else {
                state.stream.next();
            }
        } else {
            break;
        }
    }

    let mut adaptations = Vec::new();
    if state.stream.current().kind == TokenKind::LeftBrace {
        utils::skip_left_brace(state);

        while state.stream.current().kind != TokenKind::RightBrace {
            let (r#trait, method): (Option<SimpleIdentifier>, SimpleIdentifier) =
                match state.stream.peek().kind {
                    TokenKind::DoubleColon => {
                        let r#trait = identifiers::full_type_name(state);
                        state.stream.next();
                        let method = identifiers::identifier(state);
                        (Some(r#trait), method)
                    }
                    _ => (None, identifiers::identifier(state)),
                };

            expect_token!([
                    TokenKind::As => {
                        match state.stream.current() {
                            Token { kind: TokenKind::Public | TokenKind::Protected | TokenKind::Private, span, .. }=> {
                                let visibility = peek_token!([
                                    TokenKind::Public => VisibilityModifier::Public(*span),
                                    TokenKind::Protected => VisibilityModifier::Protected(*span),
                                    TokenKind::Private => VisibilityModifier::Private(*span),
                                ], state, ["`private`", "`protected`", "`public`"]);

                                state.stream.next();

                                if state.stream.current().kind == TokenKind::SemiColon {
                                    adaptations.push(TraitUsageAdaptation::Visibility {
                                        r#trait,
                                        method,
                                        visibility,
                                    });
                                } else {
                                    let alias: SimpleIdentifier = identifiers::name(state);
                                    adaptations.push(TraitUsageAdaptation::Alias {
                                        r#trait,
                                        method,
                                        alias,
                                        visibility: Some(visibility),
                                    });
                                }
                            }
                            _ => {
                                let alias: SimpleIdentifier = identifiers::name(state);
                                adaptations.push(TraitUsageAdaptation::Alias {
                                    r#trait,
                                    method,
                                    alias,
                                    visibility: None,
                                });
                            }
                        }
                    },
                    TokenKind::Insteadof => {
                        let mut insteadof = vec![
                            identifiers::full_type_name(state)
                        ];

                        if state.stream.current().kind == TokenKind::Comma {
                            if state.stream.peek().kind == TokenKind::SemiColon {
                                // will fail with unexpected token `,`
                                // as `insteadof` doesn't allow for trailing commas.
                                utils::skip_semicolon(state);
                            }

                            state.stream.next();

                            while state.stream.current().kind != TokenKind::SemiColon {
                                insteadof.push(identifiers::full_type_name(state));

                                if state.stream.current().kind == TokenKind::Comma {
                                    if state.stream.peek().kind == TokenKind::SemiColon {
                                        // will fail with unexpected token `,`
                                        // as `insteadof` doesn't allow for trailing commas.
                                        utils::skip_semicolon(state);
                                    } else {
                                        state.stream.next();
                                    }
                                } else {
                                    break;
                                }
                            }
                        }

                        adaptations.push(TraitUsageAdaptation::Precedence {
                            r#trait,
                            method,
                            insteadof,
                        });
                    }
                ], state, ["`as`", "`insteadof`"]);

            utils::skip_semicolon(state);
        }

        utils::skip_right_brace(state);
    } else {
        utils::skip_semicolon(state);
    }

    TraitUsage {
        r#use: span,
        traits,
        adaptations,
    }
}

pub fn parse(state: &mut State) -> StatementKind {
    let span = utils::skip(state, TokenKind::Trait);
    let name = identifiers::type_identifier(state);
    let attributes = state.get_attributes();

    let body = TraitBody {
        left_brace: utils::skip_left_brace(state),
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace && !state.stream.is_eof() {
                members.push(member(state, true, &name));
            }
            members
        },
        right_brace: utils::skip_right_brace(state),
    };

    StatementKind::Trait(TraitStatement {
        r#trait: span,
        name,
        attributes,
        body,
    })
}
