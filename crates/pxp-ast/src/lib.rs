use std::fmt::{Display, Formatter};

use crate::arguments::ArgumentPlaceholder;
use crate::arguments::{ArgumentList, SingleArgument};
use crate::classes::AnonymousClassExpression;
use crate::classes::ClassStatement;
use crate::constant::ConstantStatement;
use crate::control_flow::IfStatement;
use crate::declares::DeclareStatement;
use crate::enums::BackedEnumStatement;
use crate::enums::UnitEnumStatement;
use crate::functions::ArrowFunctionExpression;
use crate::functions::ClosureExpression;
use crate::functions::FunctionStatement;
use crate::goto::GotoStatement;
use crate::goto::LabelStatement;
use crate::identifiers::Identifier;
use crate::identifiers::SimpleIdentifier;
use crate::interfaces::InterfaceStatement;
use crate::literals::Literal;
use crate::loops::BreakStatement;
use crate::loops::ContinueStatement;
use crate::loops::DoWhileStatement;
use crate::loops::ForStatement;
use crate::loops::ForeachStatement;
use crate::loops::WhileStatement;
use crate::namespaces::NamespaceStatement;
use crate::operators::ArithmeticOperationExpression;
use crate::operators::AssignmentOperationExpression;
use crate::operators::BitwiseOperationExpression;
use crate::operators::ComparisonOperationExpression;
use crate::operators::LogicalOperationExpression;
use crate::traits::TraitStatement;
use crate::try_block::TryStatement;
use crate::utils::CommaSeparated;
use crate::variables::Variable;
use pxp_span::Span;
use pxp_symbol::Symbol;
use pxp_syntax::comments::{Comment, CommentGroup};
use pxp_token::{Token, TokenKind};

pub mod arguments;
pub mod attributes;
pub mod classes;
pub mod constant;
pub mod control_flow;
pub mod declares;
pub mod enums;
pub mod functions;
pub mod goto;
pub mod identifiers;
pub mod interfaces;
pub mod literals;
pub mod loops;
pub mod modifiers;
pub mod namespaces;
pub mod operators;
pub mod properties;
pub mod traits;
pub mod try_block;
pub mod utils;
pub mod variables;

pub type Block = Vec<Statement>;
pub type NodeId = usize;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum UseKind {
    Normal,
    Function,
    Const,
}

impl Display for UseKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UseKind::Normal => write!(f, "use"),
            UseKind::Function => write!(f, "use function"),
            UseKind::Const => write!(f, "use const"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct StaticVar {
    pub var: Variable,
    pub default: Option<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Ending {
    Missing(Span),
    Semicolon(Span),
    CloseTag(Span),
}

impl Ending {
    pub fn span(&self) -> Span {
        match self {
            Ending::Semicolon(span) => *span,
            Ending::CloseTag(span) => *span,
            Ending::Missing(span) => *span,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct HaltCompilerStatement {
    pub content: Option<Token>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct StaticStatement {
    pub vars: Vec<StaticVar>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SwitchStatement {
    pub switch: Span,
    pub left_parenthesis: Span,
    pub condition: Expression,
    pub right_parenthesis: Span,
    pub cases: Vec<Case>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct EchoStatement {
    pub echo: Span,
    pub values: Vec<Expression>,
    pub ending: Ending,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ReturnStatement {
    pub r#return: Span,
    pub value: Option<Expression>,
    pub ending: Ending,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UseStatement {
    pub kind: UseKind,
    pub uses: Vec<Use>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct GroupUseStatement {
    pub prefix: SimpleIdentifier,
    pub kind: UseKind,
    pub uses: Vec<Use>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub id: NodeId,
    pub kind: StatementKind,
    pub span: Span,
    pub comments: CommentGroup,
}

impl Statement {
    pub fn new(id: NodeId, kind: StatementKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            id,
            kind,
            span,
            comments,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum StatementKind {
    FullOpeningTag(FullOpeningTagStatement),
    ShortOpeningTag(ShortOpeningTagStatement),
    EchoOpeningTag(EchoOpeningTagStatement),
    ClosingTag(ClosingTagStatement),
    InlineHtml(InlineHtmlStatement),
    Label(LabelStatement),
    Goto(GotoStatement),
    HaltCompiler(HaltCompilerStatement),
    Static(StaticStatement),
    DoWhile(DoWhileStatement),
    While(WhileStatement),
    For(ForStatement),
    Foreach(ForeachStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Constant(ConstantStatement),
    Function(FunctionStatement),
    Class(ClassStatement),
    Trait(TraitStatement),
    Interface(InterfaceStatement),
    If(IfStatement),
    Switch(SwitchStatement),
    Echo(EchoStatement),
    Expression(ExpressionStatement),
    Return(ReturnStatement),
    Namespace(NamespaceStatement),
    Use(UseStatement),
    GroupUse(GroupUseStatement),
    Comment(Comment),
    Try(TryStatement),
    UnitEnum(UnitEnumStatement),
    BackedEnum(BackedEnumStatement),
    Block(BlockStatement),
    Global(GlobalStatement),
    Declare(DeclareStatement),
    Noop(Span),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct InlineHtmlStatement {
    pub html: Token,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct FullOpeningTagStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ShortOpeningTagStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct EchoOpeningTagStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ClosingTagStatement {
    pub span: Span,
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ExpressionStatement {
    pub expression: Expression,
    pub ending: Ending,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct GlobalStatement {
    pub global: Span,
    pub variables: Vec<Variable>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BlockStatement {
    pub left_brace: Span,
    pub statements: Vec<Statement>,
    pub right_brace: Span,
}

// See https://www.php.net/manual/en/language.types.type-juggling.php#language.types.typecasting for more info.
#[derive(Debug, PartialEq, Eq, Clone)]

pub enum CastKind {
    Int,
    Bool,
    Float,
    String,
    Array,
    Object,
    Unset,
}

impl From<TokenKind> for CastKind {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::StringCast | TokenKind::BinaryCast => Self::String,
            TokenKind::ObjectCast => Self::Object,
            TokenKind::BoolCast | TokenKind::BooleanCast => Self::Bool,
            TokenKind::IntCast | TokenKind::IntegerCast => Self::Int,
            TokenKind::FloatCast | TokenKind::DoubleCast | TokenKind::RealCast => Self::Float,
            TokenKind::UnsetCast => Self::Unset,
            TokenKind::ArrayCast => Self::Array,
            _ => unreachable!(),
        }
    }
}

impl From<&TokenKind> for CastKind {
    fn from(kind: &TokenKind) -> Self {
        kind.into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Case {
    pub condition: Option<Expression>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Use {
    pub name: SimpleIdentifier,
    pub alias: Option<SimpleIdentifier>,
    pub kind: Option<UseKind>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EvalExpression {
    pub eval: Span,
    // eval
    pub argument: Box<SingleArgument>, // ("$a = 1")
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EmptyExpression {
    pub empty: Span,
    // empty
    pub argument: Box<SingleArgument>, // ($a)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DieExpression {
    pub die: Span,
    // die
    pub argument: Option<Box<SingleArgument>>, // (1)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExitExpression {
    pub exit: Span,
    // exit
    pub argument: Option<Box<SingleArgument>>, // (1)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IssetExpression {
    pub isset: Span,
    // isset
    pub arguments: ArgumentList, // `($a, ...)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnsetExpression {
    pub unset: Span,
    // unset
    pub arguments: ArgumentList, // `($a, ...)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrintExpression {
    pub print: Span,
    // print
    pub value: Option<Box<Expression>>,
    // 1
    pub argument: Option<Box<SingleArgument>>, // (1)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConcatExpression {
    pub left: Box<Expression>,
    pub dot: Span,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InstanceofExpression {
    pub left: Box<Expression>,
    pub instanceof: Span,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReferenceExpression {
    pub ampersand: Span,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParenthesizedExpression {
    pub start: Span,
    pub expr: Box<Expression>,
    pub end: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ErrorSuppressExpression {
    pub at: Span,
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IncludeExpression {
    pub include: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IncludeOnceExpression {
    pub include_once: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RequireExpression {
    pub require: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RequireOnceExpression {
    pub require_once: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionCallExpression {
    pub target: Box<Expression>,
    // `foo`
    pub arguments: ArgumentList, // `(1, 2, 3)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionClosureCreationExpression {
    pub target: Box<Expression>,
    // `foo`
    pub placeholder: ArgumentPlaceholder, // `(...)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodCallExpression {
    pub target: Box<Expression>,
    // `$foo`
    pub arrow: Span,
    // `->`
    pub method: Box<Expression>,
    // `bar`
    pub arguments: ArgumentList, // `(1, 2, 3)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MethodClosureCreationExpression {
    pub target: Box<Expression>,
    // `$foo`
    pub arrow: Span,
    // `->`
    pub method: Box<Expression>,
    // `bar`
    pub placeholder: ArgumentPlaceholder, // `(...)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NullsafeMethodCallExpression {
    pub target: Box<Expression>,
    // `$foo`
    pub question_arrow: Span,
    // `?->`
    pub method: Box<Expression>,
    // `bar`
    pub arguments: ArgumentList, // `(1, 2, 3)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticMethodCallExpression {
    pub target: Box<Expression>,
    // `Foo`
    pub double_colon: Span,
    // `::`
    pub method: Identifier,
    // `bar`
    pub arguments: ArgumentList, // `(1, 2, 3)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticVariableMethodCallExpression {
    pub target: Box<Expression>,
    // `Foo`
    pub double_colon: Span,
    // `::`
    pub method: Variable,
    // `$bar`
    pub arguments: ArgumentList, // `(1, 2, 3)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticMethodClosureCreationExpression {
    pub target: Box<Expression>,
    // `Foo`
    pub double_colon: Span,
    // `::`
    pub method: Identifier,
    // `bar`
    pub placeholder: ArgumentPlaceholder, // `(...)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticVariableMethodClosureCreationExpression {
    pub target: Box<Expression>,
    // `Foo`
    pub double_colon: Span,
    // `::`
    pub method: Variable,
    // `$bar`
    pub placeholder: ArgumentPlaceholder, // `(...)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PropertyFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub arrow: Span,
    // `->`
    pub property: Box<Expression>, // `bar`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NullsafePropertyFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub question_arrow: Span,
    // `?->`
    pub property: Box<Expression>, // `bar`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticPropertyFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub double_colon: Span,
    // `::`
    pub property: Variable, // `$bar`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstantFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub double_colon: Span,
    // `::`
    pub constant: Identifier, // `bar`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShortArrayExpression {
    pub start: Span,
    // `[`
    pub items: CommaSeparated<ArrayItem>,
    // `1, 2, 3`
    pub end: Span, // `]`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayExpression {
    pub array: Span,
    // `array`
    pub start: Span,
    // `(`
    pub items: CommaSeparated<ArrayItem>,
    // `1, 2, 3`
    pub end: Span, // `)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListExpression {
    pub list: Span,
    // `list`
    pub start: Span,
    // `(`
    pub items: Vec<ListEntry>,
    // `$a, $b`
    pub end: Span, // `)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewExpression {
    pub new: Span,
    // `new`
    pub target: Box<Expression>,
    // `Foo`
    pub arguments: Option<ArgumentList>, // `(1, 2, 3)`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InterpolatedStringExpression {
    pub parts: Vec<StringPart>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HeredocExpression {
    pub label: Symbol,
    pub parts: Vec<StringPart>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NowdocExpression {
    pub label: Token,
    pub value: Token,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShellExecExpression {
    pub parts: Vec<StringPart>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoolExpression {
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayIndexExpression {
    pub array: Box<Expression>,
    pub left_bracket: Span,
    pub index: Option<Box<Expression>>,
    pub right_bracket: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShortTernaryExpression {
    pub condition: Box<Expression>,
    // `foo()`
    pub question_colon: Span,
    // `?:`
    pub r#else: Box<Expression>, // `bar()`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    // `foo()`
    pub question: Span,
    // `?`
    pub then: Box<Expression>,
    // `bar()`
    pub colon: Span,
    // `:`
    pub r#else: Box<Expression>, // `baz()`
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CoalesceExpression {
    pub lhs: Box<Expression>,
    pub double_question: Span,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CloneExpression {
    pub target: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MatchExpression {
    pub keyword: Span,
    pub left_parenthesis: Span,
    pub condition: Box<Expression>,
    pub right_parenthesis: Span,
    pub left_brace: Span,
    pub default: Option<Box<DefaultMatchArm>>,
    pub arms: Vec<MatchArm>,
    pub right_brace: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThrowExpression {
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct YieldExpression {
    pub key: Option<Box<Expression>>,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct YieldFromExpression {
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CastExpression {
    pub cast: Span,
    pub kind: CastKind,
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression {
    pub id: NodeId,
    pub kind: ExpressionKind,
    pub span: Span,
    pub comments: CommentGroup,
}

impl Expression {
    pub fn new(id: NodeId, kind: ExpressionKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            id,
            kind,
            span,
            comments,
        }
    }

    pub fn missing(id: NodeId, span: Span) -> Self {
        Self::new(id, ExpressionKind::Missing, span, CommentGroup::default())
    }

    pub fn noop(id: NodeId, span: Span) -> Self {
        Self::new(id, ExpressionKind::Noop, span, CommentGroup::default())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ExpressionKind {
    Missing,
    // eval("$a = 1")
    Eval(EvalExpression),
    // empty($a)
    Empty(EmptyExpression),
    // die, die(1)
    Die(DieExpression),
    // exit, exit(1)
    Exit(ExitExpression),
    // isset($a), isset($a, ...)
    Isset(IssetExpression),
    // unset($a), isset($a, ...)
    Unset(UnsetExpression),
    // print(1), print 1;
    Print(PrintExpression),
    Literal(Literal),
    ArithmeticOperation(ArithmeticOperationExpression),
    AssignmentOperation(AssignmentOperationExpression),
    BitwiseOperation(BitwiseOperationExpression),
    ComparisonOperation(ComparisonOperationExpression),
    LogicalOperation(LogicalOperationExpression),
    // $a . $b
    Concat(ConcatExpression),
    // $foo instanceof Bar
    Instanceof(InstanceofExpression),
    // &$foo
    Reference(ReferenceExpression),
    // ($a && $b)
    Parenthesized(ParenthesizedExpression),
    // @foo()
    ErrorSuppress(ErrorSuppressExpression),
    // `foo`, `foo_bar`, etc
    Identifier(Identifier),
    // `$foo`, `$foo_bar`, etc
    Variable(Variable),
    // include "foo.php"
    Include(IncludeExpression),
    // include_once "foo.php"
    IncludeOnce(IncludeOnceExpression),
    // require "foo.php"
    Require(RequireExpression),
    // require_once "foo.php"
    RequireOnce(RequireOnceExpression),
    // `foo(1, 2, 3)`
    FunctionCall(FunctionCallExpression),
    // `foo(...)`
    FunctionClosureCreation(FunctionClosureCreationExpression),
    // `$foo->bar(1, 2, 3)`
    MethodCall(MethodCallExpression),
    // `$foo->bar(...)`
    MethodClosureCreation(MethodClosureCreationExpression),
    // `$foo?->bar(1, 2, 3)`
    NullsafeMethodCall(NullsafeMethodCallExpression),
    // `Foo::bar(1, 2, 3)`
    StaticMethodCall(StaticMethodCallExpression),
    // `Foo::$bar(1, 2, 3)`
    StaticVariableMethodCall(StaticVariableMethodCallExpression),
    // `Foo::bar(...)`
    StaticMethodClosureCreation(StaticMethodClosureCreationExpression),
    // `Foo::$bar(...)`
    StaticVariableMethodClosureCreation(StaticVariableMethodClosureCreationExpression),
    // `foo()->bar`
    PropertyFetch(PropertyFetchExpression),
    // `foo()?->bar`
    NullsafePropertyFetch(NullsafePropertyFetchExpression),
    // `foo()::$bar`
    StaticPropertyFetch(StaticPropertyFetchExpression),
    // `foo()::bar` or `foo()::{$name}`
    ConstantFetch(ConstantFetchExpression),
    // `static`
    Static,
    // `self`
    Self_,
    // `parent`
    Parent,
    // `[1, 2, 3]`
    ShortArray(ShortArrayExpression),
    // `array(1, 2, 3)`
    Array(ArrayExpression),
    // list($a, $b)
    List(ListExpression),
    // `function() {}`
    Closure(ClosureExpression),
    // `fn() => $foo`
    ArrowFunction(ArrowFunctionExpression),
    // `new Foo(1, 2, 3)`
    New(NewExpression),
    // `"foo $bar foo"`
    InterpolatedString(InterpolatedStringExpression),
    // `<<<"EOT"` / `<<<EOT`
    Heredoc(HeredocExpression),
    // `<<<'EOT'`
    Nowdoc(NowdocExpression),
    // ``foo``
    ShellExec(ShellExecExpression),
    // `new class { ... }`
    AnonymousClass(AnonymousClassExpression),
    // `true`, `false`
    Bool(BoolExpression),
    // `$foo[0]`
    ArrayIndex(ArrayIndexExpression),
    // `null`
    Null,
    // `__DIR__`, etc
    MagicConstant(MagicConstantExpression),
    // `foo() ?: bar()`
    ShortTernary(ShortTernaryExpression),
    // `foo() ? bar() : baz()`
    Ternary(TernaryExpression),
    // `foo() ?? bar()`
    Coalesce(CoalesceExpression),
    // `clone $foo`
    Clone(CloneExpression),
    // `match ($foo) { ... }`
    Match(MatchExpression),
    // `throw new Exception`
    Throw(ThrowExpression),
    // `yield $foo`
    Yield(YieldExpression),
    // `yield from foo()`
    YieldFrom(YieldFromExpression),
    // `(int) "1"`, etc
    Cast(CastExpression),
    // ;
    Noop,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DefaultMatchArm {
    pub keyword: Span,      // `default`
    pub double_arrow: Span, // `=>`
    pub body: Expression,   // `foo()`
}
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct MatchArm {
    pub conditions: Vec<Expression>,
    pub arrow: Span,
    pub body: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum MagicConstantExpression {
    Directory(Span),
    File(Span),
    Line(Span),
    Class(Span),
    Function(Span),
    Method(Span),
    Namespace(Span),
    Trait(Span),
    CompilerHaltOffset(Span),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum StringPart {
    Literal(LiteralStringPart),
    Expression(ExpressionStringPart),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LiteralStringPart {
    pub value: Symbol,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ExpressionStringPart {
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ArrayItem {
    Skipped,
    Value {
        value: Expression, // `$foo`
    },
    ReferencedValue {
        ampersand: Span,   // `&`
        value: Expression, // `$foo`
    },
    SpreadValue {
        ellipsis: Span,    // `...`
        value: Expression, // `$foo`
    },
    KeyValue {
        key: Expression,    // `$foo`
        double_arrow: Span, // `=>`
        value: Expression,  // `$bar`
    },
    ReferencedKeyValue {
        key: Expression,    // `$foo`
        double_arrow: Span, // `=>`
        ampersand: Span,    // `&`
        value: Expression,  // `$bar`
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ListEntry {
    Skipped,
    Value {
        value: Expression, // `$foo`
    },
    KeyValue {
        key: Expression,    // `$foo`
        double_arrow: Span, // `=>`
        value: Expression,  // `$bar`
    },
}
