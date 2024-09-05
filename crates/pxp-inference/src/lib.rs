mod generator;
mod scope;

use pxp_ast::{NodeId, Statement};
use pxp_bytestring::ByteString;
use pxp_index::Index;
use pxp_type::Type;

pub use scope::Scope;
use scope::ScopeStack;

/// The main type inference engine.
///
/// This uses an `Index` to perform type inference based on the various structures
/// that are discovered inside of a project or workspace.
///
/// The main usage of the type inference engine is inside of the static analyser and
/// language server. That has directly impacted the design of this crate.
///
/// Generating a `TypeMap` is relatively low-cost, so when you need to provide a completion (for example),
/// you can take the latest version of the AST, get the `TypeMap` and then for the same AST, get the
/// type of `Node` that you are interested in. Using the same AST will ensure that the `NodeId` values
/// are the same, making lookups cheap.
#[derive(Debug, Clone, Copy)]
pub struct InferenceEngine;

impl InferenceEngine {
    /// Generate a `TypeMap` from the given AST.
    pub fn map(index: &Index, ast: &[Statement]) -> TypeMap {
        let mut generator = generator::TypeMapGenerator::new(index);
        generator.generate(ast)
    }
}

/// A map of `NodeId` values to their associated types.
#[derive(Debug, Clone)]
pub struct TypeMap {
    scopes: ScopeStack,
}

impl TypeMap {
    pub(crate) fn new() -> Self {
        Self {
            scopes: ScopeStack::new(),
        }
    }

    /// Use the given `NodeId` to resolve the type of the node.
    /// Returns `None` if the type cannot be resolved.
    pub fn resolve(&self, id: NodeId) -> Option<TypeMapResult> {
        for scope in self.scopes.iter() {
            if let Some(ty) = scope.get_type(id) {
                return Some(TypeMapResult { scope, ty });
            }
        }

        None
    }

    pub(crate) fn resolve_type(&self, id: NodeId) -> Type<ByteString> {
        self.resolve(id).map(|r| r.ty.clone()).unwrap_or_else(|| Type::Mixed)
    }
}

#[derive(Debug, Clone)]
pub struct TypeMapResult<'a> {
    pub scope: &'a Scope,
    pub ty: &'a Type<ByteString>,
}

#[cfg(test)]
mod tests {
    
    use pxp_bytestring::ByteString;
    use pxp_index::Indexer;
    use pxp_node_finder::NodeFinder;
    use pxp_parser::parse;

    use super::*;

    #[test]
    fn string_literals() {
        assert_eq!(infer("<?php 'Hello, world§';", None), Type::String);
    }

    #[test]
    fn integer_literals() {
        assert_eq!(infer("<?php 42§;", None), Type::Integer);
    }

    #[test]
    fn float_literals() {
        assert_eq!(infer("<?php 42.0§;", None), Type::Float);
    }

    #[test]
    fn boolean_literals() {
        assert_eq!(infer("<?php true§;", None), Type::Boolean);
        assert_eq!(infer("<?php false§;", None), Type::Boolean);
    }

    #[test]
    fn simple_variables_post_assignment() {
        assert_eq!(
            infer(
                "<?php
        $name = 'Ryan';
        $name§;
        ",
                None
            ),
            Type::String
        );
    }

    #[test]
    fn simple_variables_multi_assignments() {
        assert_eq!(
            infer(
                "<?php
        $name = 'Ryan';
        $name = 42;
        $name§;
        ",
                None
            ),
            Type::Integer
        );
    }

    #[test]
    fn simple_variables_with_scope_change() {
        assert_eq!(
            infer(
                "<?php
        $name = 'Ryan';

        function name() {
            $name = 123;
        }

        $name§;",
                None
            ),
            Type::String
        );
    }

    #[test]
    fn function_parameters() {
        assert_eq!(
            infer(
                "<?php
        function greet(string $name) {
            $name§;
        }
        ",
                None
            ),
            Type::String
        );
    }

    #[test]
    fn function_return_type() {
        assert_eq!(
            infer(
                "<?php
        function greet(): string {
            
        }

        $name = greet();
        $name§;
        ",
                None
            ),
            Type::String
        );
    }

    #[test]
    fn new_class() {
        assert_eq!(infer(
            "<?php

            use App\\Foo;

            $foo = new Foo();
            $foo§;
            ",
                index()
            ),
            Type::Named(ByteString::from("App\\Foo"))
        );
    }

    #[test]
    fn class_property_untyped() {
        assert_eq!(infer("
        <?php

        use App\\Foo;

        $foo = new Foo();
        ($foo->foo)§;
        ", index()), Type::Mixed);
    }

    #[test]
    fn class_property_typed() {
        assert_eq!(infer("
        <?php

        use App\\Foo;

        $foo = new Foo();
        ($foo->foop)§;
        ", index()), Type::String);
    }

    #[test]
    fn class_property_chained() {
        assert_eq!(infer("
        <?php

        use App\\Foo;

        $foo = new Foo();
        ($foo->bar->baz)§;
        ", index()), Type::String);
    }

    #[test]
    fn class_method_call() {
        assert_eq!(infer("
        <?php

        use App\\Foo;

        $foo = new Foo();
        ($foo->getBar())§;
        ", index()), Type::Named(ByteString::from(b"App\\Bar")));
    }

    #[test]
    fn class_static_method_call() {
        assert_eq!(infer("
        <?php

        use App\\Bar;

        (Bar::the())§;
        ", index()), Type::Named(ByteString::from(b"App\\Bar")));
    }

    /// Infer the type using the given input.
    /// The cursor position (denoted by the § character) is used to determine the target node.
    fn infer(input: &str, index: Option<Index>) -> Type<ByteString> {
        let offset = input.find('§').expect("failed to locate cursor marker");
        let input = input.replace('§', "");
        let result = parse(&input);
        let index = index.unwrap_or_else(|| {
            let mut index = Index::new();
            let mut indexer = Indexer::new(&mut index);
            let ast = result.ast.to_vec();

            indexer.index(&ast);
            index
        });

        let map = InferenceEngine::map(&index, &result.ast[..]);
        let (node, _) = NodeFinder::find_at_byte_offset(&result.ast[..], offset)
            .expect("failed to locate node");

        let result = map.resolve(node.id).expect("Failed to resolve type.");

        result.ty.clone()
    }

    fn index() -> Option<Index> {
        let mut index = Index::new();
        let mut indexer = Indexer::new(&mut index);

        let paths = ["tests/fixtures/Foo.php", "tests/fixtures/Bar.php"];

        for path in paths {
            let result = parse(&std::fs::read(path).expect("failed to read fixture path"));

            indexer.index(&result.ast);
        }

        Some(index)
    }
}
