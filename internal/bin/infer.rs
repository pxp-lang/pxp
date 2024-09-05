use pxp_index::{Index, Indexer};
use pxp_inference::InferenceEngine;
use pxp_node_finder::NodeFinder;
use pxp_parser::parse;


fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let no_output = args.iter().any(|arg| arg == "--no-output");
    let path = args.first().expect("missing path to file");
    let input = std::fs::read_to_string(path).expect("failed to read file");
    let offset_marker = input.find('§').expect("missing offset marker");

    let input = input.replace('§', "");
    let result = parse(&input);
    let (node, _) = NodeFinder::find_at_byte_offset(&result.ast, offset_marker).unwrap();

    let mut index = Index::new();
    let mut indexer = Indexer::new(&mut index);
    indexer.index(&result.ast);
    
    let map = InferenceEngine::map(&index, &result.ast[..]);

    if no_output {
        return;
    }

    println!("Node: {:#?}", &node);
    println!("TypeMap: {:#?}", &map);
    println!("Resolved type: {:#?}", &map.resolve(node.id));
}