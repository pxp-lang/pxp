use crate::arguments::ArgumentPlaceholder;
use crate::arguments::{ArgumentList, SingleArgument};
use crate::classes::AnonymousClassExpression;
use crate::classes::ClassStatement;
use crate::comments::Comment;
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
use crate::node::Node;
use crate::operators::ArithmeticOperationExpression;
use crate::operators::AssignmentOperationExpression;
use crate::operators::BitwiseOperationExpression;
use crate::operators::ComparisonOperationExpression;
use crate::operators::LogicalOperationExpression;
use crate::traits::TraitStatement;
use crate::try_block::TryStatement;
use crate::utils::CommaSeparated;
use crate::variables::Variable;
use comments::CommentGroup;
use pxp_bytestring::ByteString;
use pxp_span::Span;
use pxp_token::TokenKind;

pub mod arguments;
pub mod attributes;
pub mod classes;
pub mod comments;
pub mod constant;
pub mod control_flow;
pub mod data_type;
pub mod declares;
pub mod downcast;
pub mod enums;
pub mod functions;
pub mod goto;
pub mod identifiers;
pub mod interfaces;
pub mod literals;
pub mod loops;
pub mod modifiers;
pub mod namespaces;
pub mod node;
pub mod operators;
pub mod properties;
pub mod traits;
pub mod traverser;
pub mod try_block;
pub mod utils;
pub mod variables;

pub type Block = Vec<Statement>;

impl Node for Block {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.iter_mut().map(|s| s as &mut dyn Node).collect()
    }
}

pub type Program = Block;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum UseKind {
    Normal,
    Function,
    Const,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct StaticVar {
    pub var: Variable,
    pub default: Option<Expression>,
}

impl Node for StaticVar {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.var];
        if let Some(default) = &mut self.default {
            children.push(default);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Ending {
    Semicolon(Span),
    CloseTag(Span),
}

impl Ending {
    pub fn span(&self) -> Span {
        match self {
            Ending::Semicolon(span) => *span,
            Ending::CloseTag(span) => *span,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct HaltCompilerStatement {
    pub content: Option<ByteString>,
}

impl Node for HaltCompilerStatement {}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct StaticStatement {
    pub vars: Vec<StaticVar>,
}

impl Node for StaticStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.vars.iter_mut().map(|v| v as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct SwitchStatement {
    pub switch: Span,
    pub left_parenthesis: Span,
    pub condition: Expression,
    pub right_parenthesis: Span,
    pub cases: Vec<Case>,
}

impl Node for SwitchStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.condition];
        children.extend(self.cases.iter_mut().map(|c| c as &mut dyn Node));
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct EchoStatement {
    pub echo: Span,
    pub values: Vec<Expression>,
    pub ending: Ending,
}

impl Node for EchoStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.values.iter_mut().map(|v| v as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ReturnStatement {
    pub r#return: Span,
    pub value: Option<Expression>,
    pub ending: Ending,
}

impl Node for ReturnStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        if let Some(value) = &mut self.value {
            vec![value]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct UseStatement {
    pub kind: UseKind,
    pub uses: Vec<Use>,
}

impl Node for UseStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.uses.iter_mut().map(|u| u as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct GroupUseStatement {
    pub prefix: SimpleIdentifier,
    pub kind: UseKind,
    pub uses: Vec<Use>,
}

impl Node for GroupUseStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.prefix];
        children.extend(self.uses.iter_mut().map(|u| u as &mut dyn Node));
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
    pub comments: CommentGroup,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            kind,
            span,
            comments,
        }
    }
}

impl Node for Statement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.kind]
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
    pub html: ByteString,
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

impl Node for StatementKind {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            StatementKind::Label(statement) => vec![statement],
            StatementKind::Goto(statement) => vec![statement],
            StatementKind::HaltCompiler(statement) => vec![statement],
            StatementKind::Static(statement) => vec![statement],
            StatementKind::DoWhile(statement) => vec![statement],
            StatementKind::While(statement) => vec![statement],
            StatementKind::For(statement) => vec![statement],
            StatementKind::Foreach(statement) => vec![statement],
            StatementKind::Break(statement) => vec![statement],
            StatementKind::Continue(statement) => vec![statement],
            StatementKind::Constant(statement) => vec![statement],
            StatementKind::Function(statement) => vec![statement],
            StatementKind::Class(statement) => vec![statement],
            StatementKind::Trait(statement) => vec![statement],
            StatementKind::Interface(statement) => vec![statement],
            StatementKind::If(statement) => vec![statement],
            StatementKind::Switch(statement) => vec![statement],
            StatementKind::Echo(statement) => vec![statement],
            StatementKind::Expression(statement) => vec![statement],
            StatementKind::Return(statement) => vec![statement],
            StatementKind::Namespace(statement) => vec![statement],
            StatementKind::Use(statement) => vec![statement],
            StatementKind::GroupUse(statement) => vec![statement],
            StatementKind::Comment(statement) => vec![statement],
            StatementKind::Try(statement) => vec![statement],
            StatementKind::UnitEnum(statement) => vec![statement],
            StatementKind::BackedEnum(statement) => vec![statement],
            StatementKind::Block(statement) => vec![statement],
            StatementKind::Global(statement) => vec![statement],
            StatementKind::Declare(statement) => vec![statement],
            _ => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ExpressionStatement {
    pub expression: Expression,
    pub ending: Ending,
}

impl Node for ExpressionStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.expression]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct GlobalStatement {
    pub global: Span,
    pub variables: Vec<Variable>,
}

impl Node for GlobalStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.variables
            .iter_mut()
            .map(|v| v as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct BlockStatement {
    pub left_brace: Span,
    pub statements: Vec<Statement>,
    pub right_brace: Span,
}

impl Node for BlockStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.statements
            .iter_mut()
            .map(|s| s as &mut dyn Node)
            .collect()
    }
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
        kind.clone().into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Case {
    pub condition: Option<Expression>,
    pub body: Block,
}

impl Node for Case {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(condition) = &mut self.condition {
            children.push(condition);
        }
        children.extend(
            self.body
                .iter_mut()
                .map(|statement| statement as &mut dyn Node)
                .collect::<Vec<&mut dyn Node>>(),
        );
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Use {
    pub name: SimpleIdentifier,
    pub alias: Option<SimpleIdentifier>,
    pub kind: Option<UseKind>,
}

impl Node for Use {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![&mut self.name];
        if let Some(alias) = &mut self.alias {
            children.push(alias);
        }
        children
    }
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

impl Node for FunctionCallExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.arguments]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionClosureCreationExpression {
    pub target: Box<Expression>,
    // `foo`
    pub placeholder: ArgumentPlaceholder, // `(...)`
}

impl Node for FunctionClosureCreationExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut()]
    }
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

impl Node for MethodCallExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![
            self.target.as_mut(),
            self.method.as_mut(),
            &mut self.arguments,
        ]
    }
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

impl Node for MethodClosureCreationExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), self.method.as_mut()]
    }
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

impl Node for NullsafeMethodCallExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![
            self.target.as_mut(),
            self.method.as_mut(),
            &mut self.arguments,
        ]
    }
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

impl Node for StaticMethodCallExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.method, &mut self.arguments]
    }
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

impl Node for StaticVariableMethodCallExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.method, &mut self.arguments]
    }
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

impl Node for StaticMethodClosureCreationExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.method]
    }
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

impl Node for StaticVariableMethodClosureCreationExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.method]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PropertyFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub arrow: Span,
    // `->`
    pub property: Box<Expression>, // `bar`
}

impl Node for PropertyFetchExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), self.property.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NullsafePropertyFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub question_arrow: Span,
    // `?->`
    pub property: Box<Expression>, // `bar`
}

impl Node for NullsafePropertyFetchExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), self.property.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StaticPropertyFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub double_colon: Span,
    // `::`
    pub property: Variable, // `$bar`
}

impl Node for StaticPropertyFetchExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.property]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstantFetchExpression {
    pub target: Box<Expression>,
    // `foo()`
    pub double_colon: Span,
    // `::`
    pub constant: Identifier, // `bar`
}

impl Node for ConstantFetchExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut(), &mut self.constant]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShortArrayExpression {
    pub start: Span,
    // `[`
    pub items: CommaSeparated<ArrayItem>,
    // `1, 2, 3`
    pub end: Span, // `]`
}

impl Node for ShortArrayExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.items]
    }
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

impl Node for ArrayExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.items]
    }
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

impl Node for ListExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.items.iter_mut().map(|i| i as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewExpression {
    pub new: Span,
    // `new`
    pub target: Box<Expression>,
    // `Foo`
    pub arguments: Option<ArgumentList>, // `(1, 2, 3)`
}

impl Node for NewExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![self.target.as_mut()];
        if let Some(arguments) = &mut self.arguments {
            children.push(arguments);
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InterpolatedStringExpression {
    pub parts: Vec<StringPart>,
}

impl Node for InterpolatedStringExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.parts
            .iter_mut()
            .map(|part| part as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HeredocExpression {
    pub label: ByteString,
    pub parts: Vec<StringPart>,
}

impl Node for HeredocExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.parts
            .iter_mut()
            .map(|part| part as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NowdocExpression {
    pub label: ByteString,
    pub value: ByteString,
}

impl Node for NowdocExpression {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShellExecExpression {
    pub parts: Vec<StringPart>,
}

impl Node for ShellExecExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.parts
            .iter_mut()
            .map(|part| part as &mut dyn Node)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoolExpression {
    pub value: bool,
}

impl Node for BoolExpression {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArrayIndexExpression {
    pub array: Box<Expression>,
    pub left_bracket: Span,
    pub index: Option<Box<Expression>>,
    pub right_bracket: Span,
}

impl Node for ArrayIndexExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(index) = &mut self.index {
            children.push(index.as_mut());
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShortTernaryExpression {
    pub condition: Box<Expression>,
    // `foo()`
    pub question_colon: Span,
    // `?:`
    pub r#else: Box<Expression>, // `bar()`
}

impl Node for ShortTernaryExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.condition.as_mut(), self.r#else.as_mut()]
    }
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

impl Node for TernaryExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![
            self.condition.as_mut(),
            self.then.as_mut(),
            self.r#else.as_mut(),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CoalesceExpression {
    pub lhs: Box<Expression>,
    pub double_question: Span,
    pub rhs: Box<Expression>,
}

impl Node for CoalesceExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.lhs.as_mut(), self.rhs.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CloneExpression {
    pub target: Box<Expression>,
}

impl Node for CloneExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut()]
    }
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

impl Node for MatchExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![self.condition.as_mut()];
        if let Some(default) = &mut self.default {
            children.push(default.as_mut());
        }
        children.extend(
            self.arms
                .iter_mut()
                .map(|arm| arm as &mut dyn Node)
                .collect::<Vec<&mut dyn Node>>(),
        );
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThrowExpression {
    pub value: Box<Expression>,
}

impl Node for ThrowExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.value.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct YieldExpression {
    pub key: Option<Box<Expression>>,
    pub value: Option<Box<Expression>>,
}

impl Node for YieldExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = vec![];
        if let Some(key) = &mut self.key {
            children.push(key.as_mut());
        }
        if let Some(value) = &mut self.value {
            children.push(value.as_mut());
        }
        children
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct YieldFromExpression {
    pub value: Box<Expression>,
}

impl Node for YieldFromExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.value.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CastExpression {
    pub cast: Span,
    pub kind: CastKind,
    pub value: Box<Expression>,
}

impl Node for CastExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.value.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
    pub comments: CommentGroup,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span, comments: CommentGroup) -> Self {
        Self {
            kind,
            span,
            comments,
        }
    }

    pub fn noop(span: Span) -> Self {
        Self::new(ExpressionKind::Noop, span, CommentGroup::default())
    }
}

impl Node for Expression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.kind]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum ExpressionKind {
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

impl Node for EvalExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.argument.as_mut()]
    }
}

impl Node for EmptyExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.argument.as_mut()]
    }
}

impl Node for DieExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        if let Some(argument) = &mut self.argument {
            vec![argument.as_mut()]
        } else {
            vec![]
        }
    }
}

impl Node for ExitExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        if let Some(argument) = &mut self.argument {
            vec![argument.as_mut()]
        } else {
            vec![]
        }
    }
}

impl Node for IssetExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.arguments]
    }
}

impl Node for UnsetExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.arguments]
    }
}

impl Node for PrintExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        if let Some(argument) = &mut self.argument {
            vec![argument.as_mut()]
        } else if let Some(value) = &mut self.value {
            vec![value.as_mut()]
        } else {
            vec![]
        }
    }
}

impl Node for ConcatExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.left.as_mut(), self.right.as_mut()]
    }
}

impl Node for InstanceofExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.left.as_mut(), self.right.as_mut()]
    }
}

impl Node for ReferenceExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.right.as_mut()]
    }
}

impl Node for ParenthesizedExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.expr.as_mut()]
    }
}

impl Node for ErrorSuppressExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.expr.as_mut()]
    }
}

impl Node for IncludeExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.path.as_mut()]
    }
}

impl Node for IncludeOnceExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.path.as_mut()]
    }
}

impl Node for RequireExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.path.as_mut()]
    }
}

impl Node for RequireOnceExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.path.as_mut()]
    }
}

impl Node for ExpressionKind {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ExpressionKind::Eval(expression) => vec![expression],
            ExpressionKind::Empty(expression) => vec![expression],
            ExpressionKind::Die(expression) => vec![expression],
            ExpressionKind::Exit(expression) => vec![expression],
            ExpressionKind::Isset(expression) => vec![expression],
            ExpressionKind::Unset(expression) => vec![expression],
            ExpressionKind::Print(expression) => vec![expression],
            ExpressionKind::Literal(literal) => vec![literal],
            ExpressionKind::ArithmeticOperation(operation) => vec![operation],
            ExpressionKind::AssignmentOperation(operation) => vec![operation],
            ExpressionKind::BitwiseOperation(operation) => vec![operation],
            ExpressionKind::ComparisonOperation(operation) => vec![operation],
            ExpressionKind::LogicalOperation(operation) => vec![operation],
            ExpressionKind::Concat(expression) => vec![expression],
            ExpressionKind::Instanceof(expression) => vec![expression],
            ExpressionKind::Reference(expression) => vec![expression],
            ExpressionKind::Parenthesized(expression) => vec![expression],
            ExpressionKind::ErrorSuppress(expression) => vec![expression],
            ExpressionKind::Identifier(identifier) => vec![identifier],
            ExpressionKind::Variable(variable) => vec![variable],
            ExpressionKind::Include(expression) => vec![expression],
            ExpressionKind::IncludeOnce(expression) => vec![expression],
            ExpressionKind::Require(expression) => vec![expression],
            ExpressionKind::RequireOnce(expression) => vec![expression],
            ExpressionKind::FunctionCall(expression) => vec![expression],
            ExpressionKind::FunctionClosureCreation(expression) => vec![expression],
            ExpressionKind::MethodCall(expression) => vec![expression],
            ExpressionKind::MethodClosureCreation(expression) => vec![expression],
            ExpressionKind::NullsafeMethodCall(expression) => vec![expression],
            ExpressionKind::StaticMethodCall(expression) => vec![expression],
            ExpressionKind::StaticVariableMethodCall(expression) => vec![expression],
            ExpressionKind::StaticMethodClosureCreation(expression) => vec![expression],
            ExpressionKind::StaticVariableMethodClosureCreation(expression) => vec![expression],
            ExpressionKind::PropertyFetch(expression) => vec![expression],
            ExpressionKind::NullsafePropertyFetch(expression) => vec![expression],
            ExpressionKind::StaticPropertyFetch(expression) => vec![expression],
            ExpressionKind::ConstantFetch(expression) => vec![expression],
            ExpressionKind::Static => vec![],
            ExpressionKind::Self_ => vec![],
            ExpressionKind::Parent => vec![],
            ExpressionKind::ShortArray(expression) => vec![expression],
            ExpressionKind::Array(expression) => vec![expression],
            ExpressionKind::List(expression) => vec![expression],
            ExpressionKind::Closure(expression) => vec![expression],
            ExpressionKind::ArrowFunction(expression) => vec![expression],
            ExpressionKind::New(expression) => vec![expression],
            ExpressionKind::InterpolatedString(expression) => vec![expression],
            ExpressionKind::Heredoc(expression) => vec![expression],
            ExpressionKind::Nowdoc(expression) => vec![expression],
            ExpressionKind::ShellExec(expression) => vec![expression],
            ExpressionKind::AnonymousClass(expression) => vec![expression],
            ExpressionKind::Bool(_) => vec![],
            ExpressionKind::ArrayIndex(expression) => vec![expression],
            ExpressionKind::Null => vec![],
            ExpressionKind::MagicConstant(constant) => vec![constant],
            ExpressionKind::ShortTernary(expression) => vec![expression],
            ExpressionKind::Ternary(expression) => vec![expression],
            ExpressionKind::Coalesce(expression) => vec![expression],
            ExpressionKind::Clone(expression) => vec![expression],
            ExpressionKind::Match(expression) => vec![expression],
            ExpressionKind::Throw(expression) => vec![expression],
            ExpressionKind::Yield(expression) => vec![expression],
            ExpressionKind::YieldFrom(expression) => vec![expression],
            ExpressionKind::Cast(expression) => vec![expression],
            ExpressionKind::Noop => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct DefaultMatchArm {
    pub keyword: Span,      // `default`
    pub double_arrow: Span, // `=>`
    pub body: Expression,   // `foo()`
}

impl Node for DefaultMatchArm {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.body]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct MatchArm {
    pub conditions: Vec<Expression>,
    pub arrow: Span,
    pub body: Expression,
}

impl Node for MatchArm {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        let mut children: Vec<&mut dyn Node> = self
            .conditions
            .iter_mut()
            .map(|condition| condition as &mut dyn Node)
            .collect();
        children.push(&mut self.body);
        children
    }
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

impl Node for MagicConstantExpression {
    //
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum StringPart {
    Literal(LiteralStringPart),
    Expression(ExpressionStringPart),
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LiteralStringPart {
    pub value: ByteString,
}

impl Node for LiteralStringPart {
    //
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct ExpressionStringPart {
    pub expression: Box<Expression>,
}

impl Node for ExpressionStringPart {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.expression.as_mut()]
    }
}

impl Node for StringPart {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            StringPart::Literal(part) => vec![part],
            StringPart::Expression(part) => vec![part],
        }
    }
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

impl Node for ArrayItem {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ArrayItem::Skipped => vec![],
            ArrayItem::Value { value } => vec![value],
            ArrayItem::ReferencedValue {
                ampersand: _,
                value,
            } => vec![value],
            ArrayItem::SpreadValue { ellipsis: _, value } => vec![value],
            ArrayItem::KeyValue {
                key,
                double_arrow: _,
                value,
            } => vec![key, value],
            ArrayItem::ReferencedKeyValue {
                key,
                double_arrow: _,
                ampersand: _,
                value,
            } => vec![key, value],
        }
    }
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

impl Node for ListEntry {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            ListEntry::Skipped => vec![],
            ListEntry::Value { value } => vec![value],
            ListEntry::KeyValue {
                key,
                double_arrow: _,
                value,
            } => vec![key, value],
        }
    }
}
