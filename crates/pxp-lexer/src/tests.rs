use pxp_source::{SourceFile, Language};
use pxp_token::{Token, TokenKind};
use super::Lexer;

static LEXER: Lexer = Lexer::new();

#[test]
fn it_can_tokenize_keywords() {
    let tokens = tokenize("<?php die from print readonly global abstract as break case switch catch class clone const continue declare default do echo else elseif empty enddeclare endfor endforeach endif endswitch endwhile enum extends false final finally fn for foreach function goto if implements include include_once instanceof insteadof eval exit unset isset list interface match namespace new null private protected public require require_once return static throw trait try true use var yield while array", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Die,
        TokenKind::From,
        TokenKind::Print,
        TokenKind::Readonly,
        TokenKind::Global,
        TokenKind::Abstract,
        TokenKind::As,
        TokenKind::Break,
        TokenKind::Case,
        TokenKind::Switch,
        TokenKind::Catch,
        TokenKind::Class,
        TokenKind::Clone,
        TokenKind::Const,
        TokenKind::Continue,
        TokenKind::Declare,
        TokenKind::Default,
        TokenKind::Do,
        TokenKind::Echo,
        TokenKind::Else,
        TokenKind::ElseIf,
        TokenKind::Empty,
        TokenKind::EndDeclare,
        TokenKind::EndFor,
        TokenKind::EndForeach,
        TokenKind::EndIf,
        TokenKind::EndSwitch,
        TokenKind::EndWhile,
        TokenKind::Enum,
        TokenKind::Extends,
        TokenKind::False,
        TokenKind::Final,
        TokenKind::Finally,
        TokenKind::Fn,
        TokenKind::For,
        TokenKind::Foreach,
        TokenKind::Function,
        TokenKind::Goto,
        TokenKind::If,
        TokenKind::Implements,
        TokenKind::Include,
        TokenKind::IncludeOnce,
        TokenKind::Instanceof,
        TokenKind::Insteadof,
        TokenKind::Eval,
        TokenKind::Exit,
        TokenKind::Unset,
        TokenKind::Isset,
        TokenKind::List,
        TokenKind::Interface,
        TokenKind::Match,
        TokenKind::Namespace,
        TokenKind::New,
        TokenKind::Null,
        TokenKind::Private,
        TokenKind::Protected,
        TokenKind::Public,
        TokenKind::Require,
        TokenKind::RequireOnce,
        TokenKind::Return,
        TokenKind::Static,
        TokenKind::Throw,
        TokenKind::Trait,
        TokenKind::Try,
        TokenKind::True,
        TokenKind::Use,
        TokenKind::Var,
        TokenKind::Yield,
        TokenKind::While,
        TokenKind::Array,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_operators() {
    let tokens = tokenize("<?php $ & &= && -> ?-> @ * #[ ! != !== <=> || ^ ^= -= ?? ??= *= : , -- /= . .= => :: == ... = > >= ++ { [ ( << <<= >> >>= < <= - \\ % %= | |= + += ** **= ? ?: } ] ) ; / === ~ and or xor", Language::Php);
    
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Dollar,
        TokenKind::BitwiseAnd,
        TokenKind::BitwiseAndAssign,
        TokenKind::And,
        TokenKind::Arrow,
        TokenKind::NullsafeArrow,
        TokenKind::ErrorControl,
        TokenKind::Multiply,
        TokenKind::Attribute,
        TokenKind::Not,
        TokenKind::NotEqual,
        TokenKind::NotIdentical,
        TokenKind::Spaceship,
        TokenKind::Or,
        TokenKind::BitwiseXor,
        TokenKind::BitwiseXorAssign,
        TokenKind::SubtractAssign,
        TokenKind::NullCoalesce,
        TokenKind::NullCoalesceAssign,
        TokenKind::MultiplyAssign,
        TokenKind::Colon,
        TokenKind::Comma,
        TokenKind::Decrement,
        TokenKind::DivideAssign,
        TokenKind::Concat,
        TokenKind::ConcatAssign,
        TokenKind::DoubleArrow,
        TokenKind::DoubleColon,
        TokenKind::Equals,
        TokenKind::Ellipsis,
        TokenKind::Assign,
        TokenKind::GreaterThan,
        TokenKind::GreaterThanOrEqual,
        TokenKind::Increment,
        TokenKind::LeftBrace,
        TokenKind::LeftBracket,
        TokenKind::LeftParen,
        TokenKind::LeftShift,
        TokenKind::LeftShiftAssign,
        TokenKind::RightShift,
        TokenKind::RightShiftAssign,
        TokenKind::LessThan,
        TokenKind::LessThanOrEqual,
        TokenKind::Subtract,
        TokenKind::NamespaceSeparator,
        TokenKind::Modulo,
        TokenKind::ModuloAssign,
        TokenKind::BitwiseOr,
        TokenKind::BitwiseOrAssign,
        TokenKind::Add,
        TokenKind::AddAssign,
        TokenKind::Pow,
        TokenKind::PowAssign,
        TokenKind::Question,
        TokenKind::QuestionColon,
        TokenKind::RightBrace,
        TokenKind::RightBracket,
        TokenKind::RightParen,
        TokenKind::SemiColon,
        TokenKind::Divide,
        TokenKind::Identical,
        TokenKind::BitwiseNot,
        TokenKind::LogicalAnd,
        TokenKind::LogicalOr,
        TokenKind::LogicalXor,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_single_quoted_strings() {
    let tokens = tokenize("<?php 'Hello, world!'", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::SingleQuotedString,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_double_quoted_strings() {
    let tokens = tokenize("<?php \"Hello, world!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::DoubleQuotedString,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_strings_with_naked_interpolation() {
    let tokens = tokenize("<?php \"Hello, $name!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::InterpolatedStringPart,
        TokenKind::Variable,
        TokenKind::InterpolatedStringPart,
        TokenKind::DoubleQuote,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_strings_with_braced_interpolation() {
    let tokens = tokenize("<?php \"Hello, {$name}!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::InterpolatedStringPart,
        TokenKind::LeftBrace,
        TokenKind::Variable,
        TokenKind::RightBrace,
        TokenKind::InterpolatedStringPart,
        TokenKind::DoubleQuote,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_strings_with_varname_interpolation() {
    let tokens = tokenize("<?php \"Hello, ${name}!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::InterpolatedStringPart,
        TokenKind::DollarLeftBrace,
        TokenKind::Identifier,
        TokenKind::RightBrace,
        TokenKind::InterpolatedStringPart,
        TokenKind::DoubleQuote,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_strings_with_property_access_interpolation() {
    let tokens = tokenize("<?php \"Hello, {$user->name}!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::InterpolatedStringPart,
        TokenKind::LeftBrace,
        TokenKind::Variable,
        TokenKind::Arrow,
        TokenKind::Identifier,
        TokenKind::RightBrace,
        TokenKind::InterpolatedStringPart,
        TokenKind::DoubleQuote,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_strings_with_nullsafe_property_access_interpolation() {
    let tokens = tokenize("<?php \"Hello, {$user?->name}!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::InterpolatedStringPart,
        TokenKind::LeftBrace,
        TokenKind::Variable,
        TokenKind::NullsafeArrow,
        TokenKind::Identifier,
        TokenKind::RightBrace,
        TokenKind::InterpolatedStringPart,
        TokenKind::DoubleQuote,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_strings_with_var_offset_interpolation() {
    let tokens = tokenize("<?php \"Hello, {$names[0]}!\"", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::InterpolatedStringPart,
        TokenKind::LeftBrace,
        TokenKind::Variable,
        TokenKind::LeftBracket,
        TokenKind::Integer,
        TokenKind::RightBracket,
        TokenKind::RightBrace,
        TokenKind::InterpolatedStringPart,
        TokenKind::DoubleQuote,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);
    }
}

#[test]
fn it_can_tokenize_slash_comments() {
    let tokens = tokenize("<?php // Hello, world!", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::SlashComment,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_hash_comments() {
    let tokens = tokenize("<?php # Hello, world!", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::HashComment,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_block_comments() {
    let tokens = tokenize("<?php /* Hello, world! */", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::BlockComment,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_multiline_block_comments() {
    let tokens = tokenize("<?php /* Hello, \n world! */", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::BlockComment,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_doc_block_comments() {
    let tokens = tokenize("<?php /** Hello, world! */", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::DocBlockComment,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_integers() {
    let tokens = tokenize("<?php 123 0123 0o123 0x1A 0b11111111 1_234_567", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Integer,
        TokenKind::Integer,
        TokenKind::Integer,
        TokenKind::Integer,
        TokenKind::Integer,
        TokenKind::Integer,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_floats() {
    let tokens = tokenize("<?php 1.234 1.2e3 7E-10 1_234.567", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Float,
        TokenKind::Float,
        TokenKind::Float,
        TokenKind::Float,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_null() {
    let tokens = tokenize("<?php null", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Null,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_true_and_false() {
    let tokens = tokenize("<?php true false", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::True,
        TokenKind::False,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenise_variables() {
    let tokens = tokenize("<?php $name $name1 $name_1 $name_1_2 $name_1_2_3 $_name", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Variable,
        TokenKind::Variable,
        TokenKind::Variable,
        TokenKind::Variable,
        TokenKind::Variable,
        TokenKind::Variable,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i], "i: {}, Literal: {}", i, token.literal);        
    }
}

#[test]
fn it_can_tokenize_identifiers() {
    let tokens = tokenize("<?php name name1 name_1 name_1_2 name_1_2_3 _name", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Identifier,
        TokenKind::Identifier,
        TokenKind::Identifier,
        TokenKind::Identifier,
        TokenKind::Identifier,
        TokenKind::Identifier,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

#[test]
fn it_can_tokenize_qualified_identifiers() {
    let tokens = tokenize("<?php Foo\\Bar Foo\\Bar\\Baz", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::QualifiedIdentifier,
        TokenKind::QualifiedIdentifier,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

#[test]
fn it_can_tokenize_fully_qualified_identifiers() {
    let tokens = tokenize("<?php \\Foo\\Bar \\Foo\\Bar\\Baz", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::FullyQualifiedIdentifier,
        TokenKind::FullyQualifiedIdentifier,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

#[test]
fn it_can_tokenize_full_open_tag() {
    let tokens = tokenize("<?php", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

#[test]
fn it_can_tokenize_short_open_tag() {
    let tokens = tokenize("<?", Language::Php);
    let expected = vec![
        TokenKind::ShortOpenTag,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

#[test]
fn it_can_tokenize_echo_open_tag() {
    let tokens = tokenize("<?=", Language::Php);
    let expected = vec![
        TokenKind::EchoOpenTag,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

#[test]
fn it_can_tokenize_close_tag() {
    let tokens = tokenize("<?php ?>", Language::Php);
    let expected = vec![
        TokenKind::FullOpenTag,
        TokenKind::CloseTag,
        TokenKind::Eof,
    ];

    assert_eq!(tokens.len(), expected.len());
    
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.kind, expected[i]);
    }
}

// TODO: Add tests for heredocs.
// TODO: Add tests for nowdocs.

fn tokenize<'b, B: ?Sized + AsRef<[u8]>>(input: &B, language: Language) -> Vec<Token> {
    let bytes = input.as_ref();
    let source_file = SourceFile::new(None, language, bytes.to_vec());

    LEXER.tokenize(&source_file).unwrap()
}