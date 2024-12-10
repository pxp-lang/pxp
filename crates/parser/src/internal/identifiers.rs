use pxp_ast::SimpleIdentifier;
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_unqualified_identifier(&mut self) -> SimpleIdentifier {
        if self.current_kind() != TokenKind::Identifier {
            self.expected_token(TokenKind::Identifier);

            return SimpleIdentifier::missing(self.id(), self.current_span());
        }

        self.next_but_first(|parser| SimpleIdentifier {
            id: parser.id(),
            symbol: parser.current_symbol().to_bytestring(),
            span: parser.current_span(),
        })
    }

    pub(crate) fn parse_unqualified_or_qualified_identifier(&mut self) -> SimpleIdentifier {
        if !matches!(
            self.current_kind(),
            TokenKind::Identifier | TokenKind::QualifiedIdentifier
        ) {
            self.expected_any_of_tokens(&[TokenKind::Identifier, TokenKind::QualifiedIdentifier]);

            return SimpleIdentifier::missing(self.id(), self.current_span());
        }

        self.next_but_first(|parser| SimpleIdentifier {
            id: parser.id(),
            symbol: parser.current_symbol().to_bytestring(),
            span: parser.current_span(),
        })
    }

    pub(crate) fn parse_type_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => self.next_but_first(|parser| SimpleIdentifier::new(
                parser.id(),
                parser.current_symbol_as_bytestring(),
                parser.current_span(),
            )),
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.next_but_first(|parser| {
                    parser.cannot_use_reserved_keyword_as_type_name();

                    SimpleIdentifier::new(
                        parser.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            },
            t if self.is_reserved_identifier(t) => {
                self.next_but_first(|parser| {
                    parser.cannot_use_reserved_keyword_as_type_name();

                    SimpleIdentifier::new(
                        parser.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            },
            _ => {
                self.expected_token(TokenKind::Identifier);

                SimpleIdentifier::missing(self.id(), self.current_span())
            }
        }
    }

    pub(crate) fn parse_full_type_identifier(&mut self) -> SimpleIdentifier {
        match self.current_kind() {
            TokenKind::Identifier
            | TokenKind::QualifiedIdentifier
            | TokenKind::FullyQualifiedIdentifier
            | TokenKind::Enum
            | TokenKind::From => self.next_but_first(|parser| {
                SimpleIdentifier::new(
                    parser.id(),
                    parser.current_symbol_as_bytestring(),
                    parser.current_span(),
                )
            }),
            TokenKind::Self_ | TokenKind::Static | TokenKind::Parent => {
                self.next_but_first(|parser| {
                    parser.cannot_use_reserved_keyword_as_type_name();

                    SimpleIdentifier::new(
                        parser.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            },
            t if self.is_reserved_identifier(t) => {
                self.next_but_first(|parser| {
                    parser.cannot_use_reserved_keyword_as_type_name();

                    SimpleIdentifier::new(
                        parser.id(),
                        parser.current_symbol_as_bytestring(),
                        parser.current_span(),
                    )
                })
            },
            _ => {
                self.expected_token(TokenKind::Identifier);

                SimpleIdentifier::missing(self.id(), self.current_span())
            }
        }
    }

    pub(crate) fn is_soft_reserved_identifier(&self, kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Parent
                | TokenKind::Self_
                | TokenKind::True
                | TokenKind::False
                | TokenKind::List
                | TokenKind::Null
                | TokenKind::Enum
                | TokenKind::From
                | TokenKind::Readonly
        )
    }

    pub(crate) fn is_reserved_identifier(&self, kind: TokenKind) -> bool {
        if self.is_soft_reserved_identifier(kind) {
            return true;
        }

        matches!(
            kind,
            TokenKind::Static
                | TokenKind::Abstract
                | TokenKind::Final
                | TokenKind::For
                | TokenKind::Private
                | TokenKind::Protected
                | TokenKind::Public
                | TokenKind::Include
                | TokenKind::IncludeOnce
                | TokenKind::Eval
                | TokenKind::Require
                | TokenKind::RequireOnce
                | TokenKind::LogicalOr
                | TokenKind::LogicalXor
                | TokenKind::LogicalAnd
                | TokenKind::Instanceof
                | TokenKind::New
                | TokenKind::Clone
                | TokenKind::Exit
                | TokenKind::Die
                | TokenKind::If
                | TokenKind::ElseIf
                | TokenKind::Else
                | TokenKind::EndIf
                | TokenKind::Echo
                | TokenKind::Do
                | TokenKind::While
                | TokenKind::EndWhile
                | TokenKind::EndFor
                | TokenKind::Foreach
                | TokenKind::EndForeach
                | TokenKind::Declare
                | TokenKind::EndDeclare
                | TokenKind::As
                | TokenKind::Try
                | TokenKind::Catch
                | TokenKind::Finally
                | TokenKind::Throw
                | TokenKind::Use
                | TokenKind::Insteadof
                | TokenKind::Global
                | TokenKind::Var
                | TokenKind::Unset
                | TokenKind::Isset
                | TokenKind::Empty
                | TokenKind::Continue
                | TokenKind::Goto
                | TokenKind::Function
                | TokenKind::Const
                | TokenKind::Return
                | TokenKind::Print
                | TokenKind::Yield
                | TokenKind::List
                | TokenKind::Switch
                | TokenKind::EndSwitch
                | TokenKind::Case
                | TokenKind::Default
                | TokenKind::Break
                | TokenKind::Array
                | TokenKind::Callable
                | TokenKind::Extends
                | TokenKind::Implements
                | TokenKind::Namespace
                | TokenKind::Trait
                | TokenKind::Interface
                | TokenKind::Class
                | TokenKind::ClassConstant
                | TokenKind::TraitConstant
                | TokenKind::FunctionConstant
                | TokenKind::MethodConstant
                | TokenKind::LineConstant
                | TokenKind::FileConstant
                | TokenKind::DirConstant
                | TokenKind::NamespaceConstant
                | TokenKind::HaltCompiler
                | TokenKind::CompilerHaltOffsetConstant
                | TokenKind::Fn
                | TokenKind::Match
        )
    }
}
