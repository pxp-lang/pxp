use std::{collections::HashMap, task::Context};

use pxp_ast::{identifiers::SimpleIdentifier, namespaces::{BracedNamespace, NamespaceStatement, UnbracedNamespace}, Statement, UseKind, UseStatement};
use pxp_symbol::{Symbol, SymbolTable};
use pxp_visitor::{walk_mut, walk_namespace_mut, walk_use_mut, VisitorMut};

#[derive(Debug)]
pub struct NameResolvingVisitor<'a> {
    context: NameResolvingContext,
    symbols: &'a mut SymbolTable,
}

impl<'a> NameResolvingVisitor<'a> {
    pub fn new(symbols: &'a mut SymbolTable) -> Self {
        NameResolvingVisitor {
            context: NameResolvingContext::default(),
            symbols,
        }
    }

    pub fn get_context(&self) -> &NameResolvingContext {
        &self.context
    }
}

impl VisitorMut for NameResolvingVisitor<'_> {
    fn visit(&mut self, node: &mut [Statement]) {
        self.context = NameResolvingContext::new();
        walk_mut(self, node);
    }

    fn visit_namespace(&mut self, node: &mut NamespaceStatement) {
        match node {
            NamespaceStatement::Unbraced(UnbracedNamespace { name, .. }) =>{
                self.context.start_namespace(Some(name.clone()));
            },
            NamespaceStatement::Braced(BracedNamespace { name, .. }) => {
                self.context.start_namespace(name.clone());
            },
        };

        walk_namespace_mut(self, node);
    }

    fn visit_use(&mut self, node: &mut UseStatement) {
        for r#use in node.uses.iter() {
            if r#use.name.symbol.is_missing() {
                continue;
            }

            let kind = r#use.kind.unwrap_or(node.kind);
            let alias = r#use.alias.as_ref().map_or(r#use.name.get_last_part(), |alias| alias.symbol);
            let name = r#use.name.symbol;

            self.context.add_alias(kind, alias, name);
        }

        walk_use_mut(self, node);
    }
}

#[derive(Debug, Default)]
pub struct NameResolvingContext {
    namespace: Option<SimpleIdentifier>,
    aliases: HashMap<UseKind, HashMap<Symbol, Symbol>>,
}

impl NameResolvingContext {
    fn new() -> Self {
        let mut aliases = HashMap::new();
        aliases.insert(UseKind::Normal, HashMap::new());
        aliases.insert(UseKind::Function, HashMap::new());
        aliases.insert(UseKind::Const, HashMap::new());

        Self {
            namespace: None,
            aliases,
        }
    }

    fn start_namespace(&mut self, namespace: Option<SimpleIdentifier>) {
        self.namespace = namespace;
    }

    fn add_alias(&mut self, kind: UseKind, alias: Symbol, name: Symbol) {
        self.aliases.get_mut(&kind).unwrap().insert(alias, name);
    }
}