use pxp_ast::{
    visitor::{Ancestors, NodeVisitor, NodeVisitorEscapeHatch},
    Node, Statement,
};
use pxp_span::ByteOffset;

pub struct NodeFinder<'a> {
    offset: ByteOffset,
    found: Option<(Node<'a>, Ancestors<'a>)>,
}

impl<'a> NodeFinder<'a> {
    pub fn find_at_byte_offset(
        ast: &'a [Statement],
        offset: ByteOffset,
    ) -> Option<(Node<'a>, Ancestors<'a>)> {
        let mut finder = NodeFinder {
            offset,
            found: None,
        };

        finder.traverse(ast);
        finder.found
    }
}

impl<'a> NodeVisitor<'a> for NodeFinder<'a> {
    fn enter(&mut self, node: Node<'a>, ancestors: &mut Ancestors<'a>) -> NodeVisitorEscapeHatch {
        let span = node.span;

        // If the current node is before the offset we're interested in,
        // there's no need to iterate through its children.
        if span.is_before_offset(self.offset) {
            return NodeVisitorEscapeHatch::SkipChildren;
        }

        // If we're looking at a node that comes after the offset we're interested in,
        // we can stop traversing the AST since we should have found the node we're looking for.
        if span.is_after_offset(self.offset) {
            return NodeVisitorEscapeHatch::Stop;
        }

        // If the current node contains the offset we're interested in,
        // we should keep track of it and continue traversing the AST.
        if span.contains_offset(self.offset) {
            self.found = Some((node.clone(), ancestors.clone()));
        }

        NodeVisitorEscapeHatch::Continue
    }
}

#[cfg(test)]
mod tests {
    use pxp_ast::ExpressionKind;
    use pxp_lexer::Lexer;
    use pxp_parser::{ParseResult, Parser};

    use super::*;

    #[test]
    fn it_can_find_a_node_at_offset() {
        let (result, offset) = parse_with_offset_indicator(
            r#"
        <?php

        echo (new A)->§
        "#,
        );

        let (node, _) = NodeFinder::find_at_byte_offset(&result.ast[..], offset).unwrap();

        assert!(node.is_property_fetch_expression());

        let property_fetch = node.as_property_fetch_expression().unwrap();

        assert!(matches!(
            property_fetch.target.kind,
            ExpressionKind::Parenthesized(_)
        ));
    }

    fn parse_with_offset_indicator(input: &'static str) -> (ParseResult, ByteOffset) {
        let offset = input.find('§').unwrap();
        let input = input.replace('§', "");
        let result = Parser::parse(Lexer::new(&input));

        (result, offset)
    }
}
