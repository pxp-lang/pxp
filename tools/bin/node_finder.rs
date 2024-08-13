use pxp_node_finder::NodeFinder;
use pxp_parser::parse;


fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let path = args.get(0).expect("missing path to file");
    let input = std::fs::read_to_string(path).expect("failed to read file");
    // We need to +1 because the offset character is in place of the actual target node.
    let offset_marker = input.find("§").expect("missing offset marker") + 1;

    println!("Locating node at offset: {}", offset_marker);

    let input = input.replace("§", "");
    let result = parse(&input);

    // dbg!(&result.ast);

    let node = NodeFinder::find_at_byte_offset(&result.ast, offset_marker);

    dbg!(node);
}