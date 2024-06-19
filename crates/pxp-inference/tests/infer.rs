use pxp_ast::{Name, Node, Statement};
use pxp_index::{Index, Indexer};
use pxp_inference::InferenceEngine;
use pxp_parser::parse;
use pxp_symbol::SymbolTable;
use pxp_type::Type;
use pxp_visitor::{NodeVisitor, NodeVisitorResult};

#[test]
fn is_can_infer_types_of_literal_expressions() {
    assert_eq!(
        infer("<?php 1234^^;"),
        Type::Integer,
    );

    assert_eq!(
        infer("<?php 1234.5678^^;"),
        Type::Float,
    );

    assert_eq!(
        infer("<?php 'hello'^^;"),
        Type::String,
    );

    assert_eq!(
        infer(r#"<?php "hello"^^;"#),
        Type::String,
    );
}

fn infer(input: &str) -> Type<Name> {
    let index = index();
    let engine = InferenceEngine::new();

    let offset = input.find("^^").unwrap() - 1;
    let input = input.replace("^^", "");
    let ast = parse(&input, SymbolTable::the());

    let target = locate_target(&ast.ast, offset);

    engine.infer(&index, &ast.ast, target)
}

fn locate_target(ast: &[Statement], offset: usize) -> &dyn Node {
    let mut locator = NodeLocator { offset, node: None };
    locator.visit_ast(ast);
    locator.node.unwrap()
}

fn index() -> Index {
    let indexer = Indexer::new();
    indexer.get_index().clone()
}

struct NodeLocator<'a> {
    offset: usize,
    node: Option<&'a dyn Node>,
}

impl<'a> NodeVisitor<'a> for NodeLocator<'a> {
    fn visit(&mut self, node: &'a dyn Node) -> NodeVisitorResult {
        let span = node.span();

        if span.is_before(self.offset) {
            // Span is before our offset, so we continue.
            NodeVisitorResult::Continue
        } else if span.is_after(self.offset) {
            // Span is after the offset, so no chance of finding the target and we stop.
            NodeVisitorResult::Stop
        } else if span.contains(self.offset) {
            self.node = Some(node);

            NodeVisitorResult::Continue
        } else {
            unreachable!()
        }
    }
}