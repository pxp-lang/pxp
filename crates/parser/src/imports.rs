use std::collections::HashMap;

use pxp_ast::UseKind;
use pxp_bytestring::{ByteStr, ByteString};

use crate::Parser;

#[derive(Debug, Default)]
pub(crate) struct ImportMap {
    normal: HashMap<ByteString, ByteString>,
    function: HashMap<ByteString, ByteString>,
    r#const: HashMap<ByteString, ByteString>,
}

impl ImportMap {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn map(&mut self, kind: UseKind) -> &mut HashMap<ByteString, ByteString> {
        match kind {
            UseKind::Normal => &mut self.normal,
            UseKind::Function => &mut self.function,
            UseKind::Const => &mut self.r#const,
        }
    }

    pub(crate) fn insert(&mut self, kind: UseKind, name: &ByteStr, alias: Option<&ByteStr>) {
        let key = match alias {
            Some(alias) => alias.to_bytestring(),
            None => {
                let parts = name.split(|&c| c == b'\\').collect::<Vec<_>>();
                let last = parts.last().unwrap();

                ByteString::from(last)
            }
        };

        self.map(kind).insert(key, name.to_bytestring());
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn add_import(&mut self, kind: UseKind, name: &ByteStr, alias: Option<&ByteStr>) {
        self.imports.insert(kind, name, alias);
    }
}
