<?php

use Symfony\Component\Yaml\Yaml;

require_once __DIR__ . '/../vendor/autoload.php';

class VisitorMut
{
    const VISITOR_TEMPLATE = <<<'EOF'
    use pxp_ast::*;
    use crate::walk::*;
    use pxp_span::Span;
    use pxp_syntax::comments::Comment;
    use pxp_type::Type;

    pub trait VisitorMut {
        fn visit(&mut self, node: &mut [Statement]) {
            walk_mut(self, node);
        }

        fn visit_statement(&mut self, node: &mut Statement) {
            walk_statement_mut(self, node);
        }

        fn visit_expression(&mut self, node: &mut Expression) {
            walk_expression_mut(self, node);
        }

        %s
    }
    EOF;

    const FILENAME = "visitor_mut.rs";
    const NODE = "&mut ";
}

class Visitor
{
    const VISITOR_TEMPLATE = <<<'EOF'
    use pxp_ast::*;
    use crate::walk::*;
    use pxp_span::Span;
    use pxp_syntax::comments::Comment;
    use pxp_type::Type;

    pub trait VisitorMut {
        fn visit(&mut self, node: &[Statement]) {
            walk(self, node);
        }

        fn visit_statement(&mut self, node: &Statement) {
            walk_statement(self, node);
        }

        fn visit_expression(&mut self, node: &Expression) {
            walk_expression(self, node);
        }

        %s
    }
    EOF;

    const FILENAME = "visitor.rs";
    const NODE = "&";
}

const SPECIAL_NODES = ['Statement', 'Expression', 'StatementKind', 'ExpressionKind'];

function main() {
    $yaml = Yaml::parseFile(__DIR__ . '/../../crates/pxp-ast/meta/ast.yaml');
    
    foreach ([VisitorMut::class, Visitor::class] as $trait) {
        $methods = generate_methods($yaml, $trait);
        $generated = sprintf($trait::VISITOR_TEMPLATE, implode("\n\n", $methods));    

        file_put_contents(__DIR__ . '/../../crates/pxp-visitor/src/' . $trait::FILENAME, $generated);

        echo "Generated " . class_basename($trait) . ".\n";
    }
}

function generate_methods(array $yaml, string $trait): array
{
    $methods = [];

    foreach ($yaml as $type => $fields) {
        // Type alias.
        if (is_string($fields)) {
            continue;
        }

        // Skip special nodes. The visitor methods are hardcoded.
        if (in_array($type, SPECIAL_NODES, true)) {
            continue;
        }

        $method = sprintf("fn %s(&mut self, node: %s%s) {\n", type_name_to_method($type), $trait::NODE, strip_type_to_root($type));
        $fields = get_visitable_fields($fields);

        if (count($fields) === 0) {
            goto end_method;
        }

        $method .= sprintf("   %s(self, node);\n", type_name_to_walk_method($type));

        end_method:
        $method .= "}\n";
        $methods[] = $method;
    }

    return $methods;
}

function strip_type_to_root(string $type): string
{
    return str($type)->afterLast('<')->before('>');
}

function get_visitable_fields(array $fields): array
{
    $reserved = ['as', 'derive'];

    return collect($fields)
        ->filter(function ($field, string $key) use ($reserved) {
            if (in_array($key, $reserved, true)) {
                return false;
            }

            if (is_string($field) && ($field === 'Span' || strip_type_to_root($field) === 'Span')) {
                return false;
            }

            if (is_string($field) && ($field === 'Token' || strip_type_to_root($field) === 'Token')) {
                return false;
            }

            return true;
        })
        ->all();
}

function type_name_to_method(string $type): string {
    return str(strip_type_to_root($type))->snake()->prepend('visit_');
}

function type_name_to_walk_method(string $type): string {
    return str(strip_type_to_root($type))->snake()->prepend('walk_');
}

function is_vec(string $type): bool {
    return str($type)->startsWith('Vec<');
}

function is_option(string $type): bool {
    return str($type)->startsWith('Option<');
}

function strip_option(string $type): string {
    return str($type)->after('Option<')->beforeLast('>');
}

main();