use pxp_ast::{visitor::{NodeVisitor, NodeVisitorEscapeHatch}, Node};
use pxp_parser::parse;
use pxp_symbol::SymbolTable;

struct TestVisitor {
    output: Vec<String>,
}

impl NodeVisitor for TestVisitor {
    fn enter(&mut self, node: &Node) -> NodeVisitorEscapeHatch {
        self.output.push(format!("Enter {}", node.name()));

        NodeVisitorEscapeHatch::Continue
    }

    fn leave(&mut self, node: &Node) -> NodeVisitorEscapeHatch {
        self.output.push(format!("Leave {}", node.name()));

        NodeVisitorEscapeHatch::Continue
    }
}

const CODE: &'static str = r#"<?php

namespace A;

class B {
    public function c(string $a): string {
        // Comment...
    }
}

echo (new B)->c('Hello, World!');
"#;

#[test]
fn it_traverses_a_node_tree_correctly() {
    let result = parse(&CODE, SymbolTable::the());

    let mut visitor = TestVisitor { output: vec![] };
    visitor.traverse(&result.ast[..]);

    assert_eq!(
        &visitor.output[..],
        &[
            "Enter Statement",
            "Enter StatementKind",
            "Enter FullOpeningTagStatement",
            "Leave FullOpeningTagStatement",
            "Leave StatementKind",
            "Leave Statement",
            "Enter Statement",
            "Enter StatementKind",
            "Enter NamespaceStatement",
            "Enter UnbracedNamespace",
            "Enter Statement",
            "Enter StatementKind",
            "Enter ClassStatement",
            "Enter Name",
            "Leave Name",
            "Enter ClassBody",
            "Enter ClassishMember",
            "Enter ConcreteMethod",
            "Enter SimpleIdentifier",
            "Leave SimpleIdentifier",
            "Enter FunctionParameterList",
            "Enter FunctionParameter",
            "Enter SimpleVariable",
            "Leave SimpleVariable",
            "Enter DataType",
            "Leave DataType",
            "Leave FunctionParameter",
            "Leave FunctionParameterList",
            "Enter ReturnType",
            "Enter DataType",
            "Leave DataType",
            "Leave ReturnType",
            "Enter MethodBody",
            "Leave MethodBody",
            "Leave ConcreteMethod",
            "Leave ClassishMember",
            "Leave ClassBody",
            "Leave ClassStatement",
            "Leave StatementKind",
            "Leave Statement",
            "Enter Statement",
            "Enter StatementKind",
            "Enter EchoStatement",
            "Enter Expression",
            "Enter ExpressionKind",
            "Enter MethodCallExpression",
            "Enter Expression",
            "Enter ExpressionKind",
            "Enter ParenthesizedExpression",
            "Enter Expression",
            "Enter ExpressionKind",
            "Enter NewExpression",
            "Enter Expression",
            "Enter ExpressionKind",
            "Enter Name",
            "Leave Name",
            "Leave ExpressionKind",
            "Leave Expression",
            "Leave NewExpression",
            "Leave ExpressionKind",
            "Leave Expression",
            "Leave ParenthesizedExpression",
            "Leave ExpressionKind",
            "Leave Expression",
            "Enter Expression",
            "Enter ExpressionKind",
            "Enter Identifier",
            "Enter SimpleIdentifier",
            "Leave SimpleIdentifier",
            "Leave Identifier",
            "Leave ExpressionKind",
            "Leave Expression",
            "Enter ArgumentList",
            "Enter Argument",
            "Enter PositionalArgument",
            "Enter Expression",
            "Enter ExpressionKind",
            "Enter Literal",
            "Leave Literal",
            "Leave ExpressionKind",
            "Leave Expression",
            "Leave PositionalArgument",
            "Leave Argument",
            "Leave ArgumentList",
            "Leave MethodCallExpression",
            "Leave ExpressionKind",
            "Leave Expression",
            "Leave EchoStatement",
            "Leave StatementKind",
            "Leave Statement",
            "Leave UnbracedNamespace",
            "Leave NamespaceStatement",
            "Leave StatementKind",
            "Leave Statement"
        ]
    );
}
