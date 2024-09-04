use pxp_lsp::types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Uri};
use pxp_ast::{visitor::Ancestors, FunctionParameterList, Node, ResolvedName, ReturnType};
use pxp_node_finder::NodeFinder;
use pxp_parser::parse;
use pxp_type::Type;

use crate::backend::Backend;

impl Backend {
    pub fn generate_hover(&self, uri: &Uri, position: &Position) -> Option<Hover> {
        if let Some(document) = self.documents.get_document(uri) {
            let offset = document.offset_at(*position) as usize;
            let parse_result = parse(&document.get_content(None).as_bytes());
            let maybe_node = NodeFinder::find_at_byte_offset(&parse_result.ast, offset);

            #[allow(clippy::question_mark)]
            if maybe_node.is_none() {
                return None;
            }

            let (node, ancestors) = maybe_node.unwrap();

            return generate_hover(node, ancestors);
        }
        
        None
    }
}

fn generate_hover(node: Node, _: Ancestors) -> Option<Hover> {
    let content = if let Some(func) = node.as_function_statement() {
        let name = func.name.as_resolved().unwrap();

        function_hover(name, &func.parameters, &func.return_type)
    } else {
        return None;
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    })
}

fn function_hover(name: &ResolvedName, parameters: &FunctionParameterList, return_type: &Option<ReturnType>) -> String {
    format!(
        "{}\n\n```php\n<?php\nfunction {}({}): {} {{}}\n```\n",
        name.resolved,
        name.original,
        stringify_parameter_list(parameters),
        stringify_return_type(return_type),
    )
}

fn stringify_return_type(return_type: &Option<ReturnType>) -> String {
    return_type.as_ref().map(|d| &d.data_type.kind).unwrap_or_else(|| &Type::Void).to_string()
}

fn stringify_parameter_list(parameters: &FunctionParameterList) -> String {
    parameters
        .iter()
        .map(|p| format!(
            "{} {}",
            p.data_type.as_ref().map(|d| &d.kind).unwrap_or_else(|| &Type::Mixed),
            if p.name.symbol.is_empty() { "$_".to_string() } else { p.name.symbol.to_string() },
        ))
        .collect::<Vec<_>>()
        .join(", ")
}