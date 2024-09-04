use pxp_lsp::types::{DocumentSymbol, Position, Range, SymbolKind, Uri};
use pxp_ast::visitor::{Ancestors, NodeVisitor, NodeVisitorEscapeHatch};
use pxp_ast::{AbstractMethod, BracedNamespace, ConcreteMethod, Name, NameKind, Node, NodeKind, ResolvedName, UnbracedNamespace, UnresolvedName};
use pxp_parser::parse;
use pxp_span::{Span, Spanned};

use crate::backend::Backend;
use pxp_lsp::Result;

impl Backend {
    pub fn get_document_symbols(&self, uri: &Uri) -> Result<Vec<DocumentSymbol>> {
        let content = self.documents.get_document_content(uri, None);
        
        if let Some(content) = content {
            let bytes = content.as_bytes();
            let result = parse(&bytes);
            let mut visitor = DocumentSymbolGatherer { content: bytes, symbols: Vec::new() };

            visitor.traverse(&result.ast);

            Ok(visitor.symbols)
        } else {
            Ok(Vec::new())
        }
    }
}

#[derive(Default)]
struct DocumentSymbolGatherer<'a> {
    content: &'a [u8],
    symbols: Vec<DocumentSymbol>,
}

impl<'a> DocumentSymbolGatherer<'a> {
    fn span_to_range(&self, span: Span) -> Range {
        Range {
            start: Position {
                line: span.start_line(self.content) as u32 - 1,
                character: span.start_column(self.content) as u32,
            },
            end: Position {
                line: span.end_line(self.content) as u32 - 1,
                character: span.end_column(self.content) as u32,
            },
        }
    }

    fn original_name(&self, name: &Name) -> String {
        match &name.kind {
            NameKind::Resolved(ResolvedName { original, .. }) => original.to_string(),
            NameKind::Unresolved(UnresolvedName { symbol, .. }) => symbol.to_string(),
            _ => unreachable!(),
        }
    }
}

impl<'a> NodeVisitor<'a> for DocumentSymbolGatherer<'a> {
    fn enter(&mut self, node: Node<'a>, _: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        let span = node.span;

        let (name, kind) = match &node.kind {
            NodeKind::PropertyEntry(entry) => (entry.kind.variable().symbol.to_string(), SymbolKind::PROPERTY),
            NodeKind::ConcreteMethod(ConcreteMethod { name, .. }) | NodeKind::AbstractMethod(AbstractMethod { name, .. }) => (name.symbol.to_string(), SymbolKind::METHOD),
            NodeKind::ConcreteConstructor(_) | NodeKind::AbstractConstructor(_) => ("__construct".to_string(), SymbolKind::CONSTRUCTOR),
            NodeKind::UnitEnumCase(member) => (member.name.symbol.to_string(), SymbolKind::ENUM_MEMBER),
            NodeKind::BackedEnumCase(member) => (member.name.symbol.to_string(), SymbolKind::ENUM_MEMBER),
            NodeKind::FunctionStatement(function) => (self.original_name(&function.name), SymbolKind::FUNCTION),
            _ => return NodeVisitorEscapeHatch::Continue,
        };

        let range = self.span_to_range(span);

        self.symbols.push(DocumentSymbol {
            name: name.to_string(),
            kind,
            detail: None,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        });

        NodeVisitorEscapeHatch::Continue
    }

    fn leave(&mut self, node: Node<'a>, _: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        let span = node.span;

        let (name, kind) = match &node.kind {
            NodeKind::ClassStatement(class) => (self.original_name(&class.name), SymbolKind::CLASS),
            NodeKind::BracedNamespace(BracedNamespace { name: Some(name), .. }) => (name.to_string(), SymbolKind::NAMESPACE),
            NodeKind::UnbracedNamespace(UnbracedNamespace { name, .. }) => (name.to_string(), SymbolKind::NAMESPACE),
            NodeKind::InterfaceStatement(interface) => (self.original_name(&interface.name), SymbolKind::INTERFACE),
            NodeKind::TraitStatement(trait_) => (self.original_name(&trait_.name), SymbolKind::CLASS),
            NodeKind::UnitEnumStatement(enum_) => (self.original_name(&enum_.name), SymbolKind::ENUM),
            NodeKind::BackedEnumStatement(enum_) => (self.original_name(&enum_.name), SymbolKind::ENUM),
            _ => return NodeVisitorEscapeHatch::Continue,
        };

        let range = self.span_to_range(span);
        // Child symbols are inserted inside of `enter()`, so we can
        // safely take the children here.
        let children = std::mem::take(&mut self.symbols);

        self.symbols.push(DocumentSymbol {
            name: name.to_string(),
            kind,
            detail: None,
            tags: None,
            deprecated: None, #[allow(deprecated)]
            range,
            selection_range: range,
            children: Some(children),
        });

        NodeVisitorEscapeHatch::Continue
    }
}