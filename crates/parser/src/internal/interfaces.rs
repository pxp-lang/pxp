use crate::Parser;
use pxp_ast::StatementKind;
use pxp_ast::UseKind;
use pxp_ast::*;
use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

use super::diagnostics::ParserDiagnostic;

impl<'a> Parser<'a> {
    pub fn parse_interface(&mut self) -> StatementKind {
        let span = self.skip(TokenKind::Interface);

        let name = self.parse_type_name();

        let current = self.current();
        let extends = if current.kind == TokenKind::Extends {
            let span = current.span;

            self.next();

            let parents = self.at_least_one_comma_separated_no_trailing::<Name>(|parser| {
                parser.parse_full_name(UseKind::Normal)
            });

            Some(InterfaceExtends {
                id: self.id(),
                span: Span::combine(span, parents.span()),
                extends: span,
                parents,
            })
        } else {
            None
        };

        let attributes = self.get_attributes();

        let left_brace = self.skip_left_brace();
        let members = {
            let mut members = Vec::new();

            while !self.is_eof() && self.current_kind() != TokenKind::RightBrace {
                let member = self.parse_classish_member(true);

                match member {
                    ClassishMember::TraitUsage(TraitUsage { span, .. }) => {
                        self.diagnostic(
                            ParserDiagnostic::InterfaceCannotUseTraits,
                            Severity::Error,
                            span,
                        );
                    }
                    ClassishMember::Method(ref method) if method.is_concrete() => {
                        self.diagnostic(
                            ParserDiagnostic::InterfaceCannotContainConcreteMethods,
                            Severity::Error,
                            span,
                        );
                    }
                    ClassishMember::Property(ref property) if !property.is_public() => {
                        self.diagnostic(
                            ParserDiagnostic::InterfaceMembersMustBePublic,
                            Severity::Error,
                            property.span(),
                        );
                    }
                    _ => {}
                };

                members.push(member);
            }

            members
        };
        let right_brace = self.skip_right_brace();

        let body = InterfaceBody {
            id: self.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            members,
            right_brace,
        };

        StatementKind::Interface(Box::new(InterfaceStatement {
            id: self.id(),
            span: Span::combine(span, body.span),
            interface: span,
            name,
            attributes,
            extends,
            body,
        }))
    }
}
