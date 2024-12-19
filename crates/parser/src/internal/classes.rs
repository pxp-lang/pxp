use crate::internal::diagnostics::ParserDiagnostic;
use crate::internal::functions::Method;
use crate::Parser;
use pxp_ast::Expression;
use pxp_ast::StatementKind;
use pxp_ast::UseKind;
use pxp_ast::*;
use pxp_ast::{ExpressionKind, NewExpression};

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_span::Spanned;
use pxp_token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_class(&mut self) -> StatementKind {
        let attributes = self.get_attributes();

        let modifiers = self.collect_modifiers();
        let modifiers = self.parse_class_group(modifiers);
        let class = self.skip(TokenKind::Class);
        let name = self.parse_type_name();

        let extends = if self.current_kind() == TokenKind::Extends {
            let span = self.next();
            let parent = self.parse_full_name(UseKind::Normal);

            Some(ClassExtends {
                id: self.id(),
                span: Span::combine(span, parent.span),
                extends: span,
                parent,
            })
        } else {
            None
        };

        let implements = if self.current_kind() == TokenKind::Implements {
            let span = self.next();

            let interfaces = self.at_least_one_comma_separated_no_trailing::<Name>(|parser| {
                parser.parse_full_name(UseKind::Normal)
            });

            Some(ClassImplements {
                id: self.id(),
                span: Span::combine(span, interfaces.span()),
                implements: span,
                interfaces,
            })
        } else {
            None
        };

        let has_abstract = modifiers.has_abstract();
        let left_brace = self.skip_left_brace();

        let members = {
            let mut members = Vec::new();

            while self.current_kind() != TokenKind::RightBrace {
                if self.is_eof() {
                    break;
                }

                members.push(self.parse_classish_member(has_abstract));
            }

            members
        };

        let right_brace = self.skip_right_brace();

        let body = ClassBody {
            id: self.id(),
            span: Span::combine(left_brace, right_brace),
            left_brace,
            members,
            right_brace,
        };

        let span = if !modifiers.is_empty() {
            Span::combine(modifiers.span(), body.span)
        } else {
            Span::combine(class, body.span)
        };

        StatementKind::Class(ClassStatement {
            id: self.id(),
            span,
            class,
            name,
            modifiers,
            extends,
            implements,
            attributes,
            body,
        })
    }

    pub fn parse_anonymous_class(&mut self, span: Option<Span>) -> Expression {
        let new = match span {
            Some(span) => span,
            None => self.skip(TokenKind::New),
        };

        self.gather_attributes();

        let attributes = self.get_attributes();
        let class = self.skip(TokenKind::Class);
        let class_span = class;

        let arguments = if self.current_kind() == TokenKind::LeftParen {
            Some(self.parse_argument_list())
        } else {
            None
        };

        let extends = if self.current_kind() == TokenKind::Extends {
            let extends = self.next();
            let parent = self.parse_full_name(UseKind::Normal);

            Some(ClassExtends {
                id: self.id(),
                span: Span::combine(extends, parent.span),
                extends,
                parent,
            })
        } else {
            None
        };

        let implements = if self.current_kind() == TokenKind::Implements {
            let implements = self.next();
            let interfaces = self.at_least_one_comma_separated_no_trailing::<Name>(|parser| {
                parser.parse_full_name(UseKind::Normal)
            });

            Some(ClassImplements {
                id: self.id(),
                span: Span::combine(implements, interfaces.span()),
                implements,
                interfaces,
            })
        } else {
            None
        };

        let left_brace = self.skip_left_brace();
        let members = {
            let mut members = Vec::new();
            while self.current_kind() != TokenKind::RightBrace {
                members.push(self.parse_classish_member(false));
            }
            members
        };
        let right_brace = self.skip_right_brace();
        let span = Span::combine(left_brace, right_brace);

        let body = AnonymousClassBody {
            id: self.id(),
            span,
            left_brace,
            members,
            right_brace,
        };

        let end_span = body.right_brace;

        let anonymous_class = Expression::new(
            self.id(),
            ExpressionKind::AnonymousClass(Box::new(AnonymousClassExpression {
                id: self.id(),
                span: Span::combine(class, body.span),
                class,
                extends,
                implements,
                attributes,
                body: Box::new(body),
            })),
            Span::new(class_span.start, end_span.end),
            CommentGroup::default(),
        );

        let span = Span::combine(new, arguments.span());

        Expression::new(
            self.id(),
            ExpressionKind::New(Box::new(NewExpression {
                id: self.id(),
                span,
                target: Box::new(anonymous_class),
                new,
                arguments,
            })),
            span,
            CommentGroup::default(),
        )
    }

    pub fn parse_classish_member(&mut self, has_abstract: bool) -> ClassishMember {
        let has_attributes = self.gather_attributes();

        if !has_attributes && self.current_kind() == TokenKind::Use {
            return ClassishMember::TraitUsage(self.parse_trait_usage());
        }

        if self.current_kind() == TokenKind::Var {
            return ClassishMember::VariableProperty(self.parse_var_property());
        }

        let modifiers = self.collect_modifiers();

        if modifiers.is_empty()
            && !matches!(self.current_kind(), TokenKind::Const | TokenKind::Function)
        {
            self.diagnostic(
                ParserDiagnostic::UnexpectedToken {
                    token: self.current().to_owned(),
                },
                Severity::Error,
                self.current_span(),
            );

            let span = self.next();

            return ClassishMember::Missing(MissingClassishMember {
                id: self.id(),
                span,
            });
        }

        if self.current_kind() == TokenKind::Const {
            let modifiers = self.parse_constant_group(modifiers);
            return ClassishMember::Constant(self.parse_classish_constant(modifiers));
        }

        if self.current_kind() == TokenKind::Function {
            let modifiers = self.parse_method_group(modifiers);
            let method = self.parse_method(modifiers);

            return match method {
                Method::Abstract(method) => {
                    if !has_abstract && method.modifiers.has_abstract() {
                        self.diagnostic(
                            ParserDiagnostic::AbstractMethodInNonAbstractClass,
                            Severity::Error,
                            method.modifiers.get_abstract().unwrap().span(),
                        );
                    }

                    ClassishMember::AbstractMethod(method)
                }
                Method::Concrete(method) => ClassishMember::ConcreteMethod(method),
                Method::AbstractConstructor(ctor) => {
                    if !has_abstract {
                        self.diagnostic(
                            ParserDiagnostic::AbstractMethodInNonAbstractClass,
                            Severity::Error,
                            ctor.span(),
                        );
                    }

                    ClassishMember::AbstractConstructor(ctor)
                }
                Method::ConcreteConstructor(ctor) => ClassishMember::ConcreteConstructor(ctor),
            };
        }

        // e.g: public static
        let modifiers = self.parse_property_group(modifiers);

        ClassishMember::Property(self.parse_property(modifiers))
    }
}
