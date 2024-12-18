use crate::Parser;
use crate::internal::diagnostics::ParserDiagnostic;
use pxp_ast::StatementKind;
use pxp_ast::*;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_enum(&mut self) -> StatementKind {
        let span = self.skip(TokenKind::Enum);

        let name = self.parse_type_name();

        let backed_type: Option<(Span, BackedEnumType)> = if self.current_kind() == TokenKind::Colon
        {
            let colon = self.skip_colon();

            match self.current_kind() {
                TokenKind::Identifier => {
                    let symbol = self.current_symbol();

                    Some(match &symbol[..] {
                        b"string" => {
                            let span = self.next();
                            (colon, BackedEnumType::String(span))
                        }
                        b"int" => {
                            let span = self.next();
                            (colon, BackedEnumType::Int(span))
                        }
                        _ => {
                            let span = self.next();

                            self.diagnostic(
                                ParserDiagnostic::InvalidBackedEnumType,
                                Severity::Error,
                                span,
                            );

                            (colon, BackedEnumType::Invalid)
                        }
                    })
                }
                TokenKind::LeftBrace => {
                    self.diagnostic(
                        ParserDiagnostic::UnexpectedToken {
                            token: self.current().to_owned(),
                        },
                        Severity::Error,
                        self.current_span(),
                    );

                    Some((colon, BackedEnumType::Invalid))
                }
                _ => {
                    let span = self.next();

                    self.diagnostic(
                        ParserDiagnostic::InvalidBackedEnumType,
                        Severity::Error,
                        span,
                    );

                    Some((colon, BackedEnumType::Invalid))
                }
            }
        } else {
            None
        };

        let mut implements = Vec::new();
        if self.current_kind() == TokenKind::Implements {
            self.next();

            while self.current_kind() != TokenKind::LeftBrace {
                implements.push(self.parse_full_name(UseKind::Normal));

                if self.current_kind() == TokenKind::Comma {
                    self.next();
                } else {
                    break;
                }
            }
        }

        let attributes = self.get_attributes();
        if let Some((colon, backed_type)) = backed_type {
            let left_brace = self.skip_left_brace();
            let members = {
                let mut members = Vec::new();
                while self.current_kind() != TokenKind::RightBrace {
                    if let Some(member) = self.parse_backed_member() {
                        members.push(member);
                    }
                }

                members
            };
            let right_brace = self.skip_right_brace();

            let body = BackedEnumBody {
                id: self.id(),
                span: Span::combine(left_brace, right_brace),
                left_brace,
                members,
                right_brace,
            };

            StatementKind::BackedEnum(BackedEnumStatement {
                id: self.id(),
                span: Span::combine(span, body.span),
                r#enum: span,
                name,
                colon,
                backed_type,
                attributes,
                implements,
                body,
            })
        } else {
            let left_brace = self.skip_left_brace();
            let members = {
                let mut members = Vec::new();
                while self.current_kind() != TokenKind::RightBrace {
                    if let Some(member) = self.parse_unit_member() {
                        members.push(member);
                    }
                }

                members
            };
            let right_brace = self.skip_right_brace();

            let body = UnitEnumBody {
                id: self.id(),
                span: Span::combine(left_brace, right_brace),
                left_brace,
                members,
                right_brace,
            };

            StatementKind::UnitEnum(UnitEnumStatement {
                id: self.id(),
                span: Span::combine(span, body.span),
                r#enum: span,
                name,
                attributes,
                implements,
                body,
            })
        }
    }

    fn parse_unit_member(&mut self) -> Option<UnitEnumMember> {
        let _has_attributes = self.gather_attributes();

        if self.current_kind() == TokenKind::Case {
            let attributes = self.get_attributes();

            let start = self.next();

            let name = self.parse_identifier_maybe_reserved();

            let current = self.current();
            if current.kind == TokenKind::Equals {
                // parse the value, but don't do anything with it.
                let equals = self.skip(TokenKind::Equals);
                let expression = self.parse_expression();
                self.skip_semicolon();

                self.diagnostic(
                    ParserDiagnostic::UnitEnumsCannotHaveCaseValues,
                    Severity::Error,
                    Span::new(equals.start, expression.span.end),
                );

                return None;
            }

            let end = self.skip_semicolon();

            return Some(UnitEnumMember::Case(UnitEnumCase {
                id: self.id(),
                span: Span::combine(start, end),
                start,
                end,
                name,
                attributes,
            }));
        }

        Some(UnitEnumMember::Classish(self.parse_classish_member(false)))
    }

    fn parse_backed_member(&mut self) -> Option<BackedEnumMember> {
        let _has_attributes = self.gather_attributes();

        if self.current_kind() == TokenKind::Case {
            let attributes = self.get_attributes();

            let case = self.next();
            let name = self.parse_identifier_maybe_reserved();

            if self.current_kind() == TokenKind::SemiColon {
                // parse the semicolon, but don't do anything with it.
                let semi = self.skip_semicolon();

                self.diagnostic(
                    ParserDiagnostic::BackedEnumCaseMustHaveValue,
                    Severity::Error,
                    Span::new(case.start, semi.end),
                );

                return None;
            }

            let equals = self.skip(TokenKind::Equals);
            let value = self.parse_expression();
            let semicolon = self.skip_semicolon();

            return Some(BackedEnumMember::Case(BackedEnumCase {
                id: self.id(),
                span: Span::combine(case, semicolon),
                attributes,
                case,
                name,
                equals,
                value,
                semicolon,
            }));
        }

        Some(BackedEnumMember::Classish(
            self.parse_classish_member(false),
        ))
    }
}
