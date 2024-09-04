<?php

use Symfony\Component\Yaml\Yaml;

require_once __DIR__ . '/../vendor/autoload.php';

class VisitorGenerator
{
    public array $yaml;

    public function __construct(
        /** @var VisitorTemplate[] */
        protected array $templates,
    ) {
        $this->yaml = Yaml::parseFile(__DIR__ . '/../../crates/pxp-ast/meta/ast.yaml');
    }

    public function generate(): void
    {
        foreach ($this->templates as $template) {
            $this->generateForTemplate($template);
        }
    }

    public function generateForTemplate(VisitorTemplate $template): void
    {
        $methods = $this->generateVisitorMethods($template);
        $visitor = sprintf($template->getVisitorTemplate(), implode("\n\n", $methods));

        file_put_contents(__DIR__ . '/../../crates/pxp-ast/src/visitor/' . $template->getVisitorFilename(), $visitor);

        echo "Generated " . class_basename($template) . ", saved to " . $template->getVisitorFilename() . "\n";

        $walkers = $this->generateWalkerFunctions($template);
        $walk = sprintf($template->getWalkTemplate(), implode("\n\n", $walkers));

        file_put_contents(__DIR__ . '/../../crates/pxp-ast/src/visitor/' . $template->getWalkFilename(), $walk);

        echo "Generated " . class_basename($template) . ", saved to " . $template->getWalkFilename() . "\n";
    }

    public function generateVisitorMethods(VisitorTemplate $template): array
    {
        $methods = [];

        foreach ($this->yaml as $type => $fields) {
            if ($this->isTypeAlias($fields)) {
                continue;
            }

            $method = sprintf("fn %s(&mut self, node: %s%s) {\n", $this->generateVisitorMethodName($type), $template->getNodeTypePrefix(), $this->stripTypeToRoot($type));
            $fields = $this->getAllVisitableFields($fields);

            if (count($fields) > 0) {
                $method .= sprintf("%s(self, node);\n", $this->generateWalkMethodName($type, $template));
            }

            close_method:
            $method .= "}\n";

            $methods[] = $method;
        }

        return $methods;
    }

    public function generateWalkerFunctions(VisitorTemplate $template): array
    {
        $walkers = [];

        foreach ($this->yaml as $type => $fields) {
            if ($this->isTypeAlias($fields)) {
                continue;
            }

            $isEnum = $this->isEnum($fields);
            $fields = $this->getAllVisitableFields($fields);

            // We don't need to generate `walk` methods for things that
            // don't have any visitable / walkable fields.
            if (count($fields) === 0) {
                continue;
            }

            $function = sprintf(
                "pub fn %s<V: %s + ?Sized>(visitor: &mut V, node: %s%s) {\n",
                $this->generateWalkMethodName($type, $template),
                class_basename($template),
                $template->getNodeTypePrefix(),
                $this->stripTypeToRoot($type),
            );

            $function = match ($isEnum) {
                true => $this->generateWalkEnumFunction($function, $type, $fields, $template),
                false => $this->generateWalkStructFunction($function, $type, $fields, $template),
            };

            $function .= "}\n";

            $walkers[] = $function;
        }

        return $walkers;
    }

    private function generateWalkEnumFunction(string $function, string $type, array $fields, VisitorTemplate $template): string
    {
        $function .= "match node {\n";

        foreach ($fields as $variant => $field) {
            // Bit of hardcoding here, but we don't need to walk over comments.
            if ($type === 'StatementKind' && $variant === 'Comment') {
                continue;
            }

            $function .= sprintf("%s::%s", $type, $variant);

            if ($field === '') {
                $function .= " => {},\n";
            } elseif (is_string($field) && $this->isSimpleType($field) && $field !== 'Span') {
                // Enum variants with a single field can be walked directly.
                $function .= sprintf("(inner) => visitor.%s(inner),\n", $this->generateVisitorMethodName($field, $template));
            } elseif (is_array($field)) {
                // Enum struct variants needs to be destructured.
                $function .= " { ";
                
                foreach ($field as $subfield => $subtype) {
                    $function .= sprintf("%s, ", $subfield);
                }
                
                $function = rtrim($function, ', ');
                $function .= ", .. } => {\n";
                
                foreach ($field as $subfield => $subtype) {
                    if (in_array($subtype, ['Comment', 'CommentGroup', 'BackedEnumType', 'Type', 'Type<Name>', 'Span', 'Option<Span>', 'ByteString', 'Token', 'bool', 'NameQualification', '(Span, Span)', 'Level', 'Box<Level>'])) {
                        continue;
                    }

                    $function .= $this->generateWalkLogicForType($subfield, $subtype, $template);
                }

                $function .= "},\n";
            }
        }

        $function .= "_ => {},\n";
        $function .= "}\n";

        return $function;
    }

    private function generateWalkStructFunction(string $function, string $type, array $fields, VisitorTemplate $template): string
    {
        foreach ($fields as $field => $type) {
            if (in_array($type, ['CommentGroup', 'BackedEnumType', 'Type', 'Type<Name>', 'Span', 'Option<Span>', 'ByteString', 'Token', 'bool', 'NameQualification', '(Span, Span)', 'Level', 'Box<Level>'])) {
                continue;
            }

            try {
                $function .= $this->generateWalkLogicForType($field, $type, $template, prefix: true);
            } catch (TypeError $e) {
                dd($function, $field, $type, $e->getMessage());
            }
        }

        return $function;
    }

    private function generateWalkLogicForType(string $field, string $type, VisitorTemplate $template, bool $prefix = false): string
    {
        $logic = '';

        if (is_string($type) && $this->isSimpleType($type)) {
            $logic .= sprintf("visitor.%s(%s%s);\n", $this->generateVisitorMethodName($type), $prefix ? "{$template->getNodeTypePrefix()}node." : '', $field);
        }

        if (is_string($type) && $this->isVecType($type)) {
            $logic .= sprintf("for item in %s%s {\n", $prefix ? "{$template->getNodeTypePrefix()}node." : '', $field);
            $logic .= sprintf("visitor.%s(item);\n", $this->generateVisitorMethodName($type));
            $logic .= '}';
        }

        if (is_string($type) && $this->isOptionType($type)) {
            $logic .= sprintf("if let Some(item) = %s%s {\n", $prefix ? "{$template->getNodeTypePrefix()}node." : '', $field);
            $logic .= sprintf("visitor.%s(item);\n", $this->generateVisitorMethodName($type));
            $logic .= '}';
        }

        if (is_string($type) && $this->isCommaSeparatedType($type)) {
            $logic .= sprintf("for item in %s%s.inner {\n", $prefix ? "{$template->getNodeTypePrefix()}node." : '', $field);
            $logic .= sprintf("visitor.%s(item);\n", $this->generateVisitorMethodName($type));
            $logic .= '}';
        }

        return $logic;
    }

    private function isSimpleType(string $type): bool
    {
        return !$this->isVecType($type) && ! $this->isOptionType($type) && ! $this->isCommaSeparatedType($type);
    }

    private function isVecType(string $type): bool
    {
        return str_starts_with($type, 'Vec<');
    }

    private function isOptionType(string $type): bool
    {
        return str_starts_with($type, 'Option<');
    }

    private function isCommaSeparatedType(string $type): bool
    {
        return str_starts_with($type, 'CommaSeparated<');
    }

    private function getAllVisitableFields(array $fields): array
    {
        return collect($fields)
            ->filter(function (mixed $field, string $key) {
                // These are reserved keys.
                if (in_array($key, ['as', 'derive', 'node', 'children'])) {
                    return false;
                }

                if (is_string($field)) {
                    $stripped = $this->stripTypeToRoot($field);

                    return !in_array($stripped, ['Comment', 'CommentGroup', 'BackedEnumType', 'Type', 'Type<Name>', 'Span', 'Option<Span>', 'ByteString', 'Token', 'bool', 'NameQualification', '(Span, Span)', 'Level', 'Box<Level>'], true);
                }

                return true;
            })
            ->all();
    }

    private function generateVisitorMethodName(string $type): string
    {
        if ($type === 'Block') {
            return 'visit';
        }
        
        return str($this->stripTypeToRoot($type))->snake()->prepend('visit_');
    }

    private function generateWalkMethodName(string $type, VisitorTemplate $template): string
    {
        return str($this->stripTypeToRoot($type))->snake()->prepend('walk_')->append($template->getWalkMethodSuffix());
    }

    private function stripTypeToRoot(string $type): string
    {
        return str($type)->afterLast('<')->before('>');
    }

    private function isTypeAlias(mixed $definition): bool
    {
        return is_string($definition);
    }

    private function isEnum(array $definition): bool
    {
        return isset($definition['as']) && $definition['as'] === 'Enum';
    }
}

interface VisitorTemplate
{
    /** A string containing the base code for a Visitor trait. */
    public function getVisitorTemplate(): string;

    /** The file that stores the code for the Visitor. */
    public function getVisitorFilename(): string;

    /** The suffix added to generated `walk_*` function names. */
    public function getWalkMethodSuffix(): string;

    /** A string containing the base code for `walk_*` functions. */
    public function getWalkTemplate(): string;

    /** The file that stores the code for the Visitor. */
    public function getWalkFilename(): string;

    /** The prefix added to node types for `visit_*` and `walk_*` functions. */
    public function getNodeTypePrefix(): string;
}

class VisitorMut implements VisitorTemplate
{
    public function getVisitorTemplate(): string
    {
        return <<<RUST
// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.
#![allow(unused)]

use crate::*;
use super::*;
use pxp_span::Span;
use pxp_syntax::comments::Comment;
use pxp_type::Type;

pub trait VisitorMut {
    fn visit(&mut self, node: &mut Vec<Statement>) {
        walk_mut(self, node);
    }

    %s
}
RUST;
    }

    public function getVisitorFilename(): string
    {
        return 'visitor_mut.rs';
    }

    public function getWalkMethodSuffix(): string
    {
        return '_mut';
    }

    public function getWalkTemplate(): string
    {
        return <<<RUST
// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.
#![allow(unused)]

use super::VisitorMut;
use crate::*;

pub fn walk_mut<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut [Statement]) {
    for statement in node {
        visitor.visit_statement(statement);
    }
}

%s
RUST;
    }

    public function getWalkFilename(): string
    {
        return 'walk_mut.rs';
    }

    public function getNodeTypePrefix(): string
    {
        return '&mut ';
    }
}

class Visitor implements VisitorTemplate
{
    public function getVisitorTemplate(): string
    {
        return <<<RUST
// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.
#![allow(unused)]

use crate::*;
use super::*;
use pxp_span::Span;
use pxp_syntax::comments::Comment;
use pxp_type::Type;

pub trait Visitor {
    fn visit(&mut self, node: & Vec<Statement>) {
        walk(self, node);
    }

    %s
}
RUST;
    }

    public function getVisitorFilename(): string {
        return 'visitor.rs';
    }

    public function getWalkMethodSuffix(): string {
        return '';
    }

    public function getWalkTemplate(): string {
        return <<<RUST
// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.
#![allow(unused)]

use super::Visitor;
use crate::*;

pub fn walk<V: Visitor + ?Sized>(visitor: &mut V, node: &[Statement]) {
    for statement in node {
        visitor.visit_statement(statement);
    }
}

%s
RUST;
    }

    public function getWalkFilename(): string {
        return 'walk.rs';
    }

    public function getNodeTypePrefix(): string {
        return '&';
    }
}

$generator = new VisitorGenerator([new VisitorMut, new Visitor]);
$generator->generate();