use lsp_types::CompletionItemKind;
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

        let Some(completion_kind) = completion_kind(&node, &ancestors) else {
            return Ok(items);
        };

        let map = InferenceEngine::map(&self.index, &parse_result.ast);

        match completion_kind {
            CompletionKind::PropertyOrMethod => complete_property_or_method(&node, &ancestors, &self.index, &map, &mut items),
        }

        Ok(items)
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

    for candidate in candidates {
        // FIXME: Filter out properties that can't be accessed from the current scope.
        for property in candidate.get_properties() {
            items.push(CompletionItem {
                label: property.get_name().to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                ..Default::default()
            })
        }

        // FIXME: Filter out methods that can't be accessed from the current scope.
        for method in candidate.get_methods() {
            items.push(CompletionItem {
                label: format!("{}()", method.get_name()),
                kind: Some(CompletionItemKind::METHOD),
                ..Default::default()
            })
        }
    }
}

enum CompletionKind {
    PropertyOrMethod,
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
        _ => unreachable!(),
    }
}

fn completion_kind(node: &Node, ancestors: &Ancestors) -> Option<CompletionKind> {
    if node.is_property_fetch_expression() || ancestors.find(|n| n.is_property_fetch_expression()).is_some() {
        return Some(CompletionKind::PropertyOrMethod);
    }

    None
}