use std::cmp;
use std::collections::HashMap;

use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

/// Prints the tokens as a string
///
/// # Example
///
/// ```
/// use pretty_assertions::assert_str_eq;
/// use php_parser_rs::lexer::Lexer;
/// use php_parser_rs::printer::print;
///
/// let code = r#"
/// <?php
///
/// $a = 1;
/// $b = ['a', 'b', 'c'];
/// $c = "'Hello, World'? 'Hello, World'!";
///
/// __halt_compiler();
/// "#;
///
/// let tokens = Lexer::new().tokenize(code.as_bytes()).unwrap();
///
/// assert_str_eq!(print(&tokens), code);
/// ```
pub fn print(tokens: &[Token]) -> String {
    let mut lines: HashMap<usize, Vec<&Token>> = HashMap::new();
    let mut max_line = 0;

    for token in tokens {
        lines.entry(token.span.line).or_default().push(token);
        max_line = cmp::max(max_line, token.span.line);
    }

    let mut output = vec![];
    let mut last = 0;

    for line in 1..=max_line {
        if line < last {
            continue;
        }

        last = line;
        let representation = match lines.get(&line) {
            Some(tokens) => {
                let mut representation = "".to_owned();

                for token in tokens {
                    if token.kind == TokenKind::Eof {
                        break;
                    }

                    let repeat = token.span.column - representation.len() - 1;

                    representation.push_str(&" ".repeat(repeat));
                    representation.push_str(&token.value.to_string());
                }

                let mut result = vec![];
                let lines = representation.lines();
                last += lines.clone().count();
                for line in lines {
                    result.push(line);
                }

                result.join("\n")
            }
            None => "".to_owned(),
        };

        output.push(representation);
    }

    output.join("\n")
}
