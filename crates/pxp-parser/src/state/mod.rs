use std::collections::VecDeque;
use std::fmt::Display;

use pxp_ast::attributes::AttributeGroup;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_lexer::stream::TokenStream;
use pxp_symbol::SymbolTable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NamespaceType {
    Braced,
    Unbraced,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scope {
    Namespace(SimpleIdentifier),
    BracedNamespace(Option<SimpleIdentifier>),
}

#[derive(Debug)]
pub struct State<'a> {
    pub stack: VecDeque<Scope>,
    pub stream: &'a mut TokenStream<'a>,
    pub symbol_table: &'a SymbolTable<'a>,
    pub attributes: Vec<AttributeGroup>,
    pub namespace_type: Option<NamespaceType>,
}

impl<'a> State<'a> {
    pub fn new(tokens: &'a mut TokenStream<'a>, symbol_table: &'a SymbolTable) -> Self {
        Self {
            stack: VecDeque::with_capacity(32),
            stream: tokens,
            symbol_table,
            namespace_type: None,
            attributes: vec![],
        }
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

    pub fn named<T: Display + ?Sized>(&self, name: &T) -> String {
        match self.namespace() {
            Some(Scope::Namespace(n)) | Some(Scope::BracedNamespace(Some(n))) => {
                unimplemented!()
                // format!("{}\\{}", n, name)
            }
            _ => name.to_string(),
        }
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
