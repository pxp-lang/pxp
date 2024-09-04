use pxp_node_finder::NodeFinder;
use pxp_parser::parse;


fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let path = args.first().expect("missing path to file");
    let input = std::fs::read_to_string(path).expect("failed to read file");
    let offset_marker = input.find('§').expect("missing offset marker");

    println!("Locating node at offset: {}", offset_marker);

    let input = input.replace('§', "");
    let result = parse(&input);

    // dbg!(&result.ast);

    let Some((node, ancestors)) = NodeFinder::find_at_byte_offset(&result.ast, offset_marker) else {
        println!("No node found.");
        return;
    };

    dbg!(node);

    if args.contains(&"--ancestors".to_string()) {
        dbg!(ancestors);
    }
}