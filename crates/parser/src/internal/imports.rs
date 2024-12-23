use pxp_ast::{Name, NodeId, UseKind};
use pxp_bytestring::ByteStr;
use pxp_token::{Token, TokenKind};

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn add_import(&mut self, kind: &UseKind, name: &ByteStr, alias: Option<&ByteStr>) {
        // We first need to check if the alias has been provided, and if not, create a new
        // symbol using the last part of the name.
        let alias = match alias {
            Some(alias) => alias,
            None => name.after_last(b'\\'),
        };

        // Then we can insert the import into the hashmap.
        self.imports
            .get_mut(kind)
            .unwrap()
            .insert(alias.to_bytestring(), name.to_bytestring());
    }

    pub(crate) fn add_prefixed_import(
        &mut self,
        kind: &UseKind,
        prefix: &ByteStr,
        name: &ByteStr,
        alias: Option<&ByteStr>,
    ) {
        let coagulated = prefix.coagulate(&[name], b'\\');

        self.add_import(kind, coagulated.as_bytestr(), alias);
    }

    pub(crate) fn maybe_resolve_identifier(
        &self,
        id: NodeId,
        token: &Token,
        kind: UseKind,
    ) -> Name {
        let part = match &token.kind {
            TokenKind::Identifier | TokenKind::Enum | TokenKind::From => token.symbol,
            TokenKind::QualifiedIdentifier => token.symbol.before_first(b'\\'),
            _ if self.is_soft_reserved_identifier(token.kind) => token.symbol,
            _ => unreachable!("{:?}", token.kind),
        };

        let map = self.imports.get(&kind).unwrap();

        // We found an import that matches the first part of the identifier, so we can resolve it.
        if let Some(imported) = map.get(&part.to_bytestring()) {
            match &token.kind {
                TokenKind::Identifier | TokenKind::From | TokenKind::Enum => Name::resolved(
                    id,
                    imported.clone(),
                    token.symbol.to_bytestring(),
                    token.span,
                ),
                TokenKind::QualifiedIdentifier => {
                    // Qualified identifiers might be aliased, so we need to take the full un-aliased import and
                    // concatenate that with everything after the first part of the qualified identifier.
                    let rest = token.symbol.after_first(b'\\');
                    let coagulated = imported.as_bytestr().coagulate(&[rest], b'\\');

                    Name::resolved(id, coagulated, token.symbol.to_bytestring(), token.span)
                }
                _ => unreachable!(),
            }
        // We didn't find an import, but since we're trying to resolve the name of a class like, we can
        // follow PHP's name resolution rules and just prepend the current namespace.
        //
        // Additionally, if the name we're trying to resolve is qualified, then PHP's name resolution rules say that
        // we should just prepend the current namespace if the import map doesn't contain the first part.
        } else if kind == UseKind::Normal || token.kind == TokenKind::QualifiedIdentifier {
            Name::resolved(
                id,
                self.join_with_namespace(&token.symbol.to_bytestring()),
                token.symbol.to_bytestring(),
                token.span,
            )
        // Unqualified names in the global namespace can be resolved without any imports, since we can
        // only be referencing something else inside of the global namespace.
        } else if (kind == UseKind::Function || kind == UseKind::Const)
            && token.kind == TokenKind::Identifier
            && self.namespace().is_none()
        {
            Name::resolved(
                id,
                token.symbol.to_bytestring(),
                token.symbol.to_bytestring(),
                token.span,
            )
        } else {
            Name::unresolved(
                id,
                token.symbol.to_bytestring(),
                token.kind.into(),
                token.span,
            )
        }
    }
}
