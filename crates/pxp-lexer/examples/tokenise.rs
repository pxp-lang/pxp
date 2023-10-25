use pxp_lexer::Lexer;
use pxp_source::{SourceFile, Language};

const CODE: &str = r#"
<?php $ & &= && -> ?-> @ * #[ ! != !== <=> || ^ ^= -= ?? ??= *= : , -- /= . .= => :: == ... = > >= ++ { [ ( << <<= >> >>= < <= - \\ % %= | |= + += ** **= ? ?: } ] ) ; / === ~ and or xor
"#;

fn main() {
    let source_file = SourceFile::new(None, Language::Php, CODE.as_bytes().to_vec());
    let lexer = Lexer::new();

    dbg!(lexer.tokenize(&source_file).unwrap());
}