use pxp_parser::parse;

use super::BuildCommand;

pub fn run(args: BuildCommand) {
    let file = args.file.unwrap();
    let contents = std::fs::read(&file).unwrap();
    let mut ast = parse(&contents).unwrap();
}
