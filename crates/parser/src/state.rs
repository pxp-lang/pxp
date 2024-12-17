use std::collections::{HashMap, VecDeque};

use pxp_ast::*;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Diagnostic;

use crate::ParserDiagnostic;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NamespaceType {
    Braced,
    Unbraced,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scope {
    Namespace(ByteString),
    BracedNamespace(Option<ByteString>),
}

#[derive(Debug)]
pub struct State {
    // Scope Tracking
    pub stack: VecDeque<Scope>,
    pub imports: HashMap<UseKind, HashMap<ByteString, ByteString>>,
    pub namespace_type: Option<NamespaceType>,
    pub attributes: Vec<AttributeGroup>,
    docblock: bool,

    // Diagnostics
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
}

impl State {
    pub fn new() -> Self {
        let mut imports = HashMap::new();
        imports.insert(UseKind::Normal, HashMap::new());
        imports.insert(UseKind::Function, HashMap::new());
        imports.insert(UseKind::Const, HashMap::new());

        Self {
            stack: VecDeque::with_capacity(32),
            namespace_type: None,
            attributes: vec![],
            imports,
            docblock: false,

            diagnostics: vec![],
        }
    }

    pub const fn is_in_docblock(&self) -> bool {
        self.docblock
    }

    #[cfg(feature = "docblocks")]
    pub fn enter_docblock(&mut self) {
        self.docblock = true;
    }

    #[cfg(feature = "docblocks")]
    pub fn exit_docblock(&mut self) {
        self.docblock = false;
    }

    pub fn attribute(&mut self, attr: AttributeGroup) {
        self.attributes.push(attr);
    }

    pub fn get_attributes(&mut self) -> Vec<AttributeGroup> {
        let mut attributes = vec![];

        std::mem::swap(&mut self.attributes, &mut attributes);

        attributes
    }

    /// Return the namespace type used in the current state
    ///
    /// The namespace type is retrieve from the last entered
    /// namespace scope.
    ///
    /// Note: even when a namespace scope is exited, the namespace type
    /// is retained, until the next namespace scope is entered.
    pub fn namespace_type(&self) -> Option<&NamespaceType> {
        self.namespace_type.as_ref()
    }

    pub fn namespace(&self) -> Option<&Scope> {
        self.stack.iter().next()
    }

    pub fn add_prefixed_import(
        &mut self,
        kind: &UseKind,
        prefix: ByteString,
        name: ByteString,
        alias: Option<ByteString>,
    ) {
        let coagulated = prefix.coagulate(&[name], Some(b"\\"));

        self.add_import(kind, coagulated, alias);
    }

    pub fn add_import(&mut self, kind: &UseKind, name: ByteString, alias: Option<ByteString>) {
        // We first need to check if the alias has been provided, and if not, create a new
        // symbol using the last part of the name.
        let alias = match alias {
            Some(alias) => alias,
            None => {
                let bytestring = name.clone();
                let parts = bytestring.split(|c| *c == b'\\').collect::<Vec<_>>();
                let last = parts.last().unwrap();

                ByteString::new(last.to_vec())
            }
        };

        // Then we can insert the import into the hashmap.
        self.imports.get_mut(kind).unwrap().insert(alias, name);
    }

    pub fn strip_leading_namespace_qualifier(&mut self, symbol: &ByteString) -> ByteString {
        if symbol.starts_with(b"\\") {
            ByteString::from(&symbol[1..])
        } else {
            symbol.clone()
        }
    }

    pub fn join_with_namespace(&self, name: &ByteString) -> ByteString {
        match self.namespace() {
            Some(Scope::Namespace(namespace)) => namespace.coagulate(&[name.clone()], Some(b"\\")),
            Some(Scope::BracedNamespace(Some(namespace))) => {
                namespace.coagulate(&[name.clone()], Some(b"\\"))
            }
            _ => name.clone(),
        }
    }

    pub fn previous_scope(&self) -> Option<&Scope> {
        self.stack.get(self.stack.len() - 2)
    }

    pub fn enter(&mut self, scope: Scope) {
        match &scope {
            Scope::Namespace(_) => {
                self.namespace_type = Some(NamespaceType::Unbraced);
            }
            Scope::BracedNamespace(_) => {
                self.namespace_type = Some(NamespaceType::Braced);
            }
        }

        self.stack.push_back(scope);
    }

    pub fn exit(&mut self) {
        self.stack.pop_back();
    }
}
