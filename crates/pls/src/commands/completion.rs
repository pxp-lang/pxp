use lsp_types::{CompletionItemKind, CompletionItemLabelDetails, InsertTextFormat, TextEdit};
use lsp_types::{CompletionItem, Position, Uri};
use pxp_ast::visitor::Ancestors;
use pxp_ast::Node;
use pxp_ast::HasId;
use pxp_bytestring::ByteString;
use pxp_index::Index;
use pxp_index::ReflectionClass;
use pxp_inference::{InferenceEngine, TypeMap};
use pxp_node_finder::NodeFinder;
use pxp_parser::parse;
use pxp_type::Type;

use crate::backend::Backend;
use crate::server::Result;

impl Backend {
    pub(crate) fn get_completion_items(&mut self, uri: &Uri, position: Position) -> Result<Vec<CompletionItem>> {
        let mut items = Vec::new();

        let Some(document) = self.documents.get_document(uri) else {
            return Ok(items);
        };

        // Grab the byte offset from the document.
        let offset = document.offset_at(position);
        let parse_result = parse(&document.get_content(None).as_bytes());

        // Get the node and list of ancestors at the byte offset.
        let Some((node, ancestors)) = NodeFinder::find_at_byte_offset(&parse_result.ast, offset as usize) else {
            return Ok(items);
        };

        let map = InferenceEngine::map(&self.index, &parse_result.ast);
        let completion_kind = completion_kind(&node, &ancestors);

        match completion_kind {
            CompletionKind::PropertyOrMethod => complete_property_or_method(&node, &ancestors, &self.index, &map, &mut items),
            CompletionKind::Extends => complete_extends(&node, &ancestors, &self.index, &map, &mut items),
            CompletionKind::ContextualMethodName => complete_contextual_method_names(&node, &ancestors, &map, &self.index, &mut items),
            CompletionKind::ContextualKeywords => complete_keywords(&node, &ancestors, &self.index, &map, &mut items),
        }

        // If we reach this point and haven't found any completions, we can defer to
        // the contextual keywords logic to generate a list of sensible completions.
        if items.is_empty() && completion_kind != CompletionKind::ContextualKeywords {
            complete_keywords(&node, &ancestors, &self.index, &map, &mut items);
        }
        
        Ok(items)
    }
}

fn complete_contextual_method_names(node: &Node, ancestors: &Ancestors, map: &TypeMap, index: &Index, items: &mut Vec<CompletionItem>) {

}

struct CompletionContext;

impl CompletionContext {
    fn class_clause(node: &Node, ancestors: &Ancestors) -> bool {
        node.is_class_statement() || ancestors.find(|n| n.is_class_statement()).is_some()
    }

    fn class_body(node: &Node, ancestors: &Ancestors) -> bool {
        node.is_class_body() || ancestors.find(|n| n.is_class_body()).is_some()
    }

    fn interface_body(node: &Node, ancestors: &Ancestors) -> bool {
        node.is_interface_body() || ancestors.find(|n| n.is_interface_body()).is_some()
    }

    fn enum_body(node: &Node, ancestors: &Ancestors) -> bool {
        node.is_unit_enum_body() || ancestors.find(|n| n.is_unit_enum_body()).is_some() || node.is_backed_enum_body() || ancestors.find(|n| n.is_backed_enum_body()).is_some()
    }

    fn classish_member(node: &Node, ancestors: &Ancestors) -> bool {
        node.is_classish_member() || ancestors.find(|n| n.is_classish_member()).is_some()
    }

    fn not_missing_classish_member(node: &Node, ancestors: &Ancestors) -> bool {
        !node.is_missing_classish_member() && ancestors.find(|n| n.is_missing_classish_member()).is_none()
    }

    fn method_name(node: &Node, ancestors: &Ancestors) -> bool {
        node.is_abstract_method() || node.is_concrete_method() || (node.is_simple_identifier() && ancestors.find(|n| n.is_abstract_method() || n.is_concrete_method()).is_some())
    }
}

fn magic_methods(items: &mut Vec<CompletionItem>) {
    let methods = [
        ("__construct", "($1)\n{$0\n}"),
        ("__call", "(string $${1:name}, array $${2:arguments}): ${3:mixed}\n{$0\n}"),
        ("__callStatic", "(string $${1:name}, array $${2:arguments}): ${3:mixed}\n{$0\n}"),
        ("__clone", "(): void\n{$0\n}"),
        ("__debugInfo", "(): array\n{$0\n}"),
        ("__destruct", "(): void\n{$0\n}"),
        ("__get", "(string $${1:name}): ${3:mixed}\n{$0\n}"),
        ("__invoke", "($1): ${2:mixed}\n{$0\n}"),
        ("__isset", "(string $${1:name}): bool\n{$0\n}"),
        ("__serialize", "(): array\n{$0\n}"),
        ("__set", "(string $${1:name}, mixed $${2:value}): void\n{$0\n}"),
        ("__set_state", "(array $${1:properties}): object\n{$0\n}"),
        ("__sleep", "(): array\n{$0\n}"),
        ("__toString", "(): string\n{$0\n}"),
        ("__unserialize", "(array $${1:data}): void\n{$0\n}"),
        ("__unset", "(string $${1:name}): void\n{$0\n}"),
        ("__wakeup", "(): void\n{$0\n})")
    ];

    for (name, _) in methods {
        items.push(CompletionItem {
            label: format!("{name}()"),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        });
    }
}

fn complete_keywords(node: &Node, ancestors: &Ancestors, _: &Index, _: &TypeMap, items: &mut Vec<CompletionItem>) {
    if CompletionContext::method_name(node, ancestors) {
        magic_methods(items);
        return;
    }

    if CompletionContext::classish_member(node, ancestors) && CompletionContext::not_missing_classish_member(node, ancestors) {
        keywords(items, &["function", "const"]);
        return;
    }

    if CompletionContext::class_body(node, ancestors) {
        keywords(items, &["public", "protected", "private", "function", "const"]);
        return;
    }

    if CompletionContext::enum_body(node, ancestors) {
        keywords(items, &["case", "const", "public", "protected", "private"]);
        return;
    }

    if CompletionContext::interface_body(node, ancestors) {
        keywords(items, &["public", "function", "const"]);
        return;
    }

    if CompletionContext::class_clause(node, ancestors) {
        keywords(items, &["implements", "extends"]);
        return;
    }
}

fn keywords(items: &mut Vec<CompletionItem>, keywords: &[&str]) {
    for keyword in keywords {
        items.push(CompletionItem {
            label: keyword.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        });
    }
}

fn complete_extends(node: &Node, ancestors: &Ancestors, index: &Index, _: &TypeMap, items: &mut Vec<CompletionItem>) {
    if !node.is_class_extends() && ancestors.find(|n| n.is_class_extends()).is_none() {
        return;
    }

    let Some(class_statement) = ancestors.find(|node| node.is_class_statement()).map(|node| node.as_class_statement()).flatten() else {
        return
    };

    for class in index.get_extendable_classes() {
        if class.get_name() == &class_statement.name.to_resolved().resolved {
            continue;
        }

        // FIXME: These completion items also need to import the chosen class, if required.
        items.push(CompletionItem {
            label: class.get_short_name().to_string(),
            kind: Some(CompletionItemKind::CLASS),
            label_details: Some(CompletionItemLabelDetails {
                description: Some(class.get_name().to_string()),
                detail: None,
            }),
            ..Default::default()
        });
    }
}

fn complete_property_or_method(node: &Node, ancestors: &Ancestors, index: &Index, map: &TypeMap, items: &mut Vec<CompletionItem>) {
    let property_fetch = if node.is_property_fetch_expression() {
        node.clone().as_property_fetch_expression().unwrap()
    } else if let Some(parent) = ancestors.find(|p| p.is_property_fetch_expression()) {
        parent.as_property_fetch_expression().unwrap()
    } else {
        return;
    };

    let Some(map_result) = map.resolve(property_fetch.target.id()) else {
        return
    };
    
    // We can only complete properties on known object-like types.
    if ! map_result.ty.is_object_like() {
        return;
    }

    let candidates = get_reflection_classes(index, map_result.ty);

    if candidates.is_empty() {
        return;
    }

    let scope = map_result.scope.get_class(index);

    for candidate in candidates {
        for property in candidate.get_accessible_properties(scope.as_ref(), index) {
            items.push(CompletionItem {
                label: property.get_name().to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                label_details: Some(CompletionItemLabelDetails {
                    description: Some(property.get_type().to_string()),
                    detail: None,
                }),
                ..Default::default()
            })
        }

        for method in candidate.get_accessible_methods(scope.as_ref(), index) {
            items.push(CompletionItem {
                label: format!("{}()", method.get_name()),
                kind: Some(CompletionItemKind::METHOD),
                label_details: Some(CompletionItemLabelDetails {
                    description: Some(method.get_return_type().to_string()),
                    detail: None,
                }),
                ..Default::default()
            })
        }
    }
}

#[derive(PartialEq, Eq)]
enum CompletionKind {
    PropertyOrMethod,
    Extends,
    ContextualMethodName,
    ContextualKeywords,
}

fn get_reflection_classes(index: &Index, typ: &Type<ByteString>) -> Vec<ReflectionClass> {
    match typ {
        Type::Named(name) => if let Some(class) = index.get_class(name) {
            vec![class]
        } else {
            vec![]
        },
        Type::Union(inner) => inner.iter().flat_map(|t| get_reflection_classes(index, t)).collect(),
        Type::Intersection(inner) => inner.iter().flat_map(|t| get_reflection_classes(index, t)).collect(),
        Type::Nullable(inner) => get_reflection_classes(index, inner),
        _ => Vec::new(),
    }
}

fn completion_kind(node: &Node, ancestors: &Ancestors) -> CompletionKind {
    if node.is_property_fetch_expression() || ancestors.find(|n| n.is_property_fetch_expression()).is_some() {
        return CompletionKind::PropertyOrMethod;
    }

    if node.is_simple_identifier() && ancestors.find(|n| n.is_concrete_method()).is_some() {
        return CompletionKind::ContextualMethodName;
    }

    if node.is_class_extends() || ancestors.find(|n| n.is_class_extends()).is_some() {
        return CompletionKind::Extends;
    }

    CompletionKind::ContextualKeywords
}