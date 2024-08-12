use pxp_index::Index;
use pxp_inference::InferenceEngine;
use pxp_node_finder::NodeFinder;
use pxp_parser::parse;
use pxp_symbol::SymbolTable;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let path = args.get(0).expect("missing path to file");
    let input = std::fs::read_to_string(path).expect("failed to read file");
    let offset_marker = input.find("§").expect("missing offset marker");

    let input = input.replace("§", "");
    let result = parse(&input, SymbolTable::the());
    let node = NodeFinder::find_at_byte_offset(&result.ast, offset_marker);

    let index = Index::new();
    let inference_engine = InferenceEngine::new(&index);
    let map = inference_engine.map(&result.ast[..]);

    println!("Node: {:#?}", &node);
    println!("TypeMap: {:#?}", &map);
    println!("Resolved type: {:#?}", &map.resolve(node.unwrap().id));
}