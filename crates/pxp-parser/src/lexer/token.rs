use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use std::fmt::Display;

use crate::lexer::byte_string::ByteString;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, JsonSchema)]

pub struct Span {
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, position: usize) -> Self {
        Self {
            line,
            column,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub enum OpenTagKind {
    Full,  // `<?php`
    Short, // `<?`
    Echo,  // `<?=`
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub enum DocStringKind {
    Heredoc,
    Nowdoc,
}

pub type DocStringIndentationAmount = usize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub enum DocStringIndentationKind {
    Space,
    Tab,
    None,
    Both,
}

impl From<u8> for DocStringIndentationKind {
    fn from(byte: u8) -> Self {
        match byte {
            b' ' => Self::Space,
            b'\t' => Self::Tab,
            _ => unreachable!(),
        }
    }
}

impl From<DocStringIndentationKind> for u8 {
    fn from(kind: DocStringIndentationKind) -> Self {
        match kind {
            DocStringIndentationKind::Space => b' ',
            DocStringIndentationKind::Tab => b'\t',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub enum TokenKind {
    Die,
    // Can't use `Self` as a name here, so suffixing with an underscore.
    Self_,
    Parent,
    Backtick,
    StartDocString(DocStringKind),
    EndDocString(DocStringIndentationKind, usize),
    From,
    Print,
    Dollar,
    HaltCompiler,
    Readonly,
    Global,
    Abstract,
    Ampersand,
    AmpersandEquals,
    And,
    AndEquals,
    Array,
    ArrayCast,
    Arrow,
    QuestionArrow,
    At,
    As,
    Asterisk,
    Attribute,
    Bang,
    BangEquals,
    AngledLeftRight,
    BangDoubleEquals,
    Spaceship,
    BoolCast,
    BooleanCast,
    BooleanAnd,
    BooleanOr,
    Break,
    Callable,
    Caret,
    CaretEquals,
    Case,
    Catch,
    Class,
    ClassConstant,
    TraitConstant,
    FunctionConstant,
    MethodConstant,
    LineConstant,
    FileConstant,
    Clone,
    MinusEquals,
    CloseTag,
    DoubleQuestion,
    DoubleQuestionEquals,
    AsteriskEquals,
    Colon,
    Comma,
    SingleLineComment,
    HashMarkComment,
    MultiLineComment,
    DocumentComment,
    Const,
    LiteralSingleQuotedString,
    LiteralDoubleQuotedString,
    Continue,
    CurlyOpen,
    Declare,
    Decrement,
    Default,
    DirConstant,
    DivEquals,
    Do,
    DollarLeftBrace,
    Dot,
    DotEquals,
    DoubleArrow,
    DoubleCast,
    RealCast,
    FloatCast,
    DoubleColon,
    DoubleEquals,
    DoubleQuote,
    Echo,
    Ellipsis,
    Else,
    ElseIf,
    Empty,
    EndDeclare,
    EndFor,
    EndForeach,
    EndIf,
    EndSwitch,
    EndWhile,
    Enum,
    Eof,
    Equals,
    Extends,
    False,
    Final,
    Finally,
    LiteralFloat,
    Fn,
    For,
    Foreach,
    FullyQualifiedIdentifier,
    Function,
    Goto,
    GreaterThan,
    GreaterThanEquals,
    Identifier,
    If,
    Implements,
    Include,
    IncludeOnce,
    Increment,
    InlineHtml,
    Instanceof,
    Insteadof,
    Eval,
    Exit,
    Unset,
    Isset,
    List,
    LiteralInteger,
    IntCast,
    IntegerCast,
    Interface,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LeftShift,
    LeftShiftEquals,
    RightShift,
    RightShiftEquals,
    LessThan,
    LessThanEquals,
    Match,
    Minus,
    Namespace,
    NamespaceSeparator,
    NamespaceConstant,
    CompilerHaltOffsetConstant,
    New,
    Null,
    ObjectCast,
    UnsetCast,
    OpenTag(OpenTagKind),
    Percent,
    PercentEquals,
    Pipe,
    PipeEquals,
    Plus,
    PlusEquals,
    Pow,
    PowEquals,
    Private,
    Protected,
    Public,
    QualifiedIdentifier,
    Question,
    QuestionColon,
    Require,
    RequireOnce,
    Return,
    RightBrace,
    RightBracket,
    RightParen,
    SemiColon,
    Slash,
    SlashEquals,
    Static,
    StringCast,
    BinaryCast,
    StringPart,
    Switch,
    Throw,
    Trait,
    TripleEquals,
    True,
    Try,
    Use,
    Var,
    Variable,
    Yield,
    While,
    BitwiseNot,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub value: ByteString,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Eof,
            span: Span::new(0, 0, 0),
            value: ByteString::default(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::CompilerHaltOffsetConstant => "__COMPILER_HALT_OFFSET__",
            Self::Die => "die",
            Self::Self_ => "self",
            Self::Parent => "parent",
            Self::Backtick => "`",
            Self::StartDocString(_) => todo!(),
            Self::EndDocString(..) => todo!(),
            Self::BangEquals => "!=",
            Self::From => "from",
            Self::Print => "print",
            Self::BitwiseNot => "~",
            Self::Dollar => "$",
            Self::HaltCompiler => "__halt_compiler",
            Self::Readonly => "readonly",
            Self::AsteriskEquals => "*=",
            Self::ObjectCast => "(object)",
            Self::UnsetCast => "(unset)",
            Self::Abstract => "abstract",
            Self::Ampersand => "&",
            Self::And => "&&",
            Self::AndEquals => "&=",
            Self::Arrow => "->",
            Self::QuestionArrow => "?->",
            Self::Array => "array",
            Self::ArrayCast => "(array)",
            Self::As => "as",
            Self::Asterisk => "*",
            Self::Attribute => "#[",
            Self::Bang => "!",
            Self::BoolCast => "(bool)",
            Self::BooleanCast => "(boolean)",
            Self::BooleanAnd => "&&",
            Self::BooleanOr => "||",
            Self::Break => "break",
            Self::Callable => "callable",
            Self::Caret => "^",
            Self::CaretEquals => "^=",
            Self::Case => "case",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::ClassConstant => "__CLASS__",
            Self::Clone => "clone",
            Self::CloseTag => "?>",
            Self::DoubleQuestion => "??",
            Self::DoubleQuestionEquals => "??=",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::IntCast => "(int)",
            Self::IntegerCast => "(integer)",
            Self::CurlyOpen => "{$",
            Self::Declare => "declare",
            Self::Decrement => "--",
            Self::Default => "default",
            Self::DirConstant => "__DIR__",
            Self::DivEquals => "/=",
            Self::Do => "do",
            Self::Dot => ".",
            Self::DotEquals => ".=",
            Self::DoubleArrow => "=>",
            Self::DoubleCast => "(double)",
            Self::RealCast => "(real)",
            Self::FloatCast => "(float)",
            Self::DoubleColon => "::",
            Self::DoubleEquals => "==",
            Self::Echo => "echo",
            Self::Ellipsis => "...",
            Self::Else => "else",
            Self::ElseIf => "elseif",
            Self::Empty => "empty",
            Self::EndDeclare => "enddeclare",
            Self::EndFor => "endfor",
            Self::EndForeach => "endforeach",
            Self::EndIf => "endif",
            Self::EndSwitch => "endswitch",
            Self::EndWhile => "endwhile",
            Self::Enum => "enum",
            Self::Eof => "[end of file]",
            Self::Equals => "=",
            Self::Extends => "extends",
            Self::False => "false",
            Self::Final => "final",
            Self::Finally => "finally",
            Self::LiteralFloat => return write!(f, "float literal"),
            Self::Fn => "fn",
            Self::For => "for",
            Self::Function => "function",
            Self::Goto => "goto",
            Self::GreaterThan => ">",
            Self::GreaterThanEquals => ">=",
            Self::If => "if",
            Self::Implements => "implements",
            Self::Increment => "++",
            Self::InlineHtml => "InlineHtml",
            Self::LiteralInteger => return write!(f, "integer literal"),
            Self::LeftBrace => "{",
            Self::LeftBracket => "[",
            Self::LeftParen => "(",
            Self::LeftShift => "<<",
            Self::LeftShiftEquals => "<<=",
            Self::RightShift => ">>",
            Self::RightShiftEquals => ">>=",
            Self::LessThan => "<",
            Self::LessThanEquals => "<=",
            Self::Match => "match",
            Self::Minus => "-",
            Self::MinusEquals => "-=",
            Self::Namespace => "namespace",
            Self::NamespaceSeparator => "\\",
            Self::New => "new",
            Self::Null => "null",
            Self::OpenTag(kind) => match kind {
                OpenTagKind::Full => "<?php",
                OpenTagKind::Short => "<?",
                OpenTagKind::Echo => "<?=",
            },
            Self::Percent => "%",
            Self::PercentEquals => "%=",
            Self::Pipe => "|",
            Self::PipeEquals => "|=",
            Self::Plus => "+",
            Self::PlusEquals => "+=",
            Self::Pow => "**",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Public => "public",
            Self::Question => "?",
            Self::QuestionColon => "?:",
            Self::Require => "require",
            Self::RequireOnce => "require_once",
            Self::Return => "return",
            Self::RightBrace => "}",
            Self::RightBracket => "]",
            Self::RightParen => ")",
            Self::SemiColon => ";",
            Self::Slash => "/",
            Self::SlashEquals => "/=",
            Self::Static => "static",
            Self::StringCast => "(string)",
            Self::BinaryCast => "(binary)",
            Self::Switch => "switch",
            Self::Throw => "throw",
            Self::Trait => "trait",
            Self::TripleEquals => "===",
            Self::True => "true",
            Self::Try => "try",
            Self::Use => "use",
            Self::Var => "var",
            Self::Yield => "yield",
            Self::While => "while",
            Self::Global => "global",
            Self::AngledLeftRight => "<>",
            Self::Spaceship => "<=>",
            Self::LogicalAnd => "and",
            Self::LogicalOr => "or",
            Self::LogicalXor => "xor",
            Self::Foreach => "foreach",
            Self::AmpersandEquals => "&=",
            Self::At => "at",
            Self::BangDoubleEquals => "!==",
            Self::TraitConstant => "__TRAIT__",
            Self::FunctionConstant => "__FUNCTION__",
            Self::MethodConstant => "__METHOD__",
            Self::LineConstant => "__LINE__",
            Self::FileConstant => "__FILE__",
            Self::DollarLeftBrace => "${",
            Self::DoubleQuote => "\"",
            Self::Include => "include",
            Self::IncludeOnce => "include_once",
            Self::Instanceof => "instanceof",
            Self::Insteadof => "insteadof",
            Self::Eval => "eval",
            Self::Exit => "exit",
            Self::Unset => "unset",
            Self::Isset => "isset",
            Self::List => "list",
            Self::Interface => "interface",
            Self::NamespaceConstant => "__NAMESPACE__",
            Self::PowEquals => "**=",
            Self::StringPart
            | Self::Variable
            | Self::QualifiedIdentifier
            | Self::Identifier
            | Self::FullyQualifiedIdentifier
            | Self::LiteralSingleQuotedString
            | Self::LiteralDoubleQuotedString
            | Self::SingleLineComment
            | Self::MultiLineComment
            | Self::HashMarkComment
            | Self::DocumentComment => {
                return write!(f, "{:?}", self);
            }
        };

        write!(f, "{}", s)
    }
}
