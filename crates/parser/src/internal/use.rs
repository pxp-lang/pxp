use pxp_ast::{
    CommentGroup, GroupUseStatement, Name, SimpleIdentifier, Statement, StatementKind, Use,
    UseKind, UseStatement,
};
use pxp_bytestring::ByteStr;
use pxp_span::Span;
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_use(&mut self) -> Statement {
        let r#use = self.expect(TokenKind::Use);
        let kind = self.parse_use_kind();

        match self.peek_kind() {
            // use (const|function)? Foo\{ ... };
            TokenKind::LeftBrace => self.parse_group_use(r#use, kind),
            // use (const|function)? Foo;
            _ => self.parse_single_use(r#use, kind),
        }
    }

    fn parse_single_use(&mut self, r#use: Span, kind: UseKind) -> Statement {
        let mut uses = Vec::new();

        while !self.is_eof() && self.current_kind() != TokenKind::SemiColon {
            let name = self.parse_use_name();
            let alias = self.parse_use_alias();
            let span = name.span.maybe_join(alias.as_ref().map(|alias| alias.span));

            self.add_import(
                kind,
                name.symbol().as_ref(),
                alias.as_ref().map(|alias| alias.symbol.as_ref()),
            );

            uses.push(Use {
                id: self.id(),
                name,
                span,
                alias,
                kind,
            });

            self.optional_comma();
        }

        let semi_colon = self.semi_colon();
        let span = r#use.join(semi_colon);

        Statement::new(
            self.id(),
            StatementKind::Use(UseStatement {
                id: self.id(),
                span,
                kind,
                uses,
            }),
            span,
            CommentGroup::default(),
        )
    }

    fn parse_group_use(&mut self, r#use: Span, kind: UseKind) -> Statement {
        let prefix = self.parse_identifier();
        self.expect(TokenKind::LeftBrace);

        let mut uses: Vec<Use> = Vec::new();

        while !self.is_eof() && self.current_kind() != TokenKind::RightBrace {
            let use_kind = self.parse_use_kind();

            if use_kind != UseKind::Normal && kind != UseKind::Normal {
                self.mixed_import_types();
            }

            let name = self.parse_prefixed_use_name(prefix.symbol.as_ref());
            let alias = self.parse_use_alias();
            let span = name.span.maybe_join(alias.as_ref().map(|alias| alias.span));

            self.add_import(
                use_kind,
                name.symbol().as_ref(),
                alias.as_ref().map(|alias| alias.symbol.as_ref()),
            );

            uses.push(Use {
                id: self.id(),
                name,
                span,
                alias,
                kind,
            });

            self.optional_comma();
        }

        self.expect(TokenKind::RightBrace);

        let semi_colon = self.semi_colon();
        let span = r#use.join(semi_colon);

        Statement::new(
            self.id(),
            StatementKind::GroupUse(GroupUseStatement {
                id: self.id(),
                span,
                prefix,
                kind,
                uses,
            }),
            span,
            CommentGroup::default(),
        )
    }

    fn parse_prefixed_use_name(&mut self, prefix: &ByteStr) -> Name {
        let identifier = self.parse_full_type_identifier();

        if identifier.is_missing() {
            return Name::missing(self.id(), identifier.span);
        }

        Name::resolved(
            self.id(),
            prefix.coagulate(&[identifier.symbol.as_ref()], b'\\'),
            identifier.symbol,
            identifier.span,
        )
    }

    fn parse_use_name(&mut self) -> Name {
        let identifier = self.parse_full_type_identifier();

        if identifier.is_missing() {
            return Name::missing(self.id(), identifier.span);
        }

        Name::resolved(
            self.id(),
            identifier.symbol.clone(),
            identifier.symbol,
            identifier.span,
        )
    }

    fn parse_use_alias(&mut self) -> Option<SimpleIdentifier> {
        if self.current_kind() != TokenKind::As {
            return None;
        }

        self.expect(TokenKind::As);

        Some(self.parse_type_identifier())
    }

    fn parse_use_kind(&mut self) -> UseKind {
        match self.current_kind() {
            TokenKind::Function => self.next_but_first(|_| UseKind::Function),
            TokenKind::Const => self.next_but_first(|_| UseKind::Const),
            _ => UseKind::Normal,
        }
    }
}
