use pxp_lexer::Lexer;
use pxp_source::{SourceFile, Language};

const CODE: &str = r#"
<?php die from print readonly global abstract as break case switch catch class clone const continue declare default do echo else elseif empty enddeclare endfor endforeach endif endswitch endwhile enum extends false final finally fn for foreach function goto if implements include include_once instanceof insteadof eval exit unset isset list interface match namespace new null private protected public require require_once return static throw trait try true use var yield while array
"#;

fn main() {
    let source_file = SourceFile::new(None, Language::Php, CODE.as_bytes().to_vec());
    let lexer = Lexer::new();

    dbg!(lexer.tokenize(&source_file).unwrap());
}