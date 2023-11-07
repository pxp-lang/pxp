use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::node::Node;
use crate::parser::ast::arguments::ArgumentPlaceholder;
use crate::parser::ast::arguments::{ArgumentList, SingleArgument};
use crate::parser::ast::classes::AnonymousClassExpression;
use crate::parser::ast::classes::ClassStatement;
use crate::parser::ast::comments::Comment;
use crate::parser::ast::constant::ConstantStatement;
use crate::parser::ast::control_flow::IfStatement;
use crate::parser::ast::declares::DeclareStatement;
use crate::parser::ast::enums::BackedEnumStatement;
use crate::parser::ast::enums::UnitEnumStatement;
use crate::parser::ast::functions::ArrowFunctionExpression;
use crate::parser::ast::functions::ClosureExpression;
use crate::parser::ast::functions::FunctionStatement;
use crate::parser::ast::goto::GotoStatement;
use crate::parser::ast::goto::LabelStatement;
use crate::parser::ast::identifiers::Identifier;
use crate::parser::ast::identifiers::SimpleIdentifier;
use crate::parser::ast::interfaces::InterfaceStatement;
use crate::parser::ast::literals::Literal;
use crate::parser::ast::loops::BreakStatement;
use crate::parser::ast::loops::ContinueStatement;
use crate::parser::ast::loops::DoWhileStatement;
use crate::parser::ast::loops::ForStatement;
use crate::parser::ast::loops::ForeachStatement;
use crate::parser::ast::loops::WhileStatement;
use crate::parser::ast::namespaces::NamespaceStatement;
use crate::parser::ast::operators::ArithmeticOperationExpression;
use crate::parser::ast::operators::AssignmentOperationExpression;
use crate::parser::ast::operators::BitwiseOperationExpression;
use crate::parser::ast::operators::ComparisonOperationExpression;
use crate::parser::ast::operators::LogicalOperationExpression;
use crate::parser::ast::traits::TraitStatement;
use crate::parser::ast::try_block::TryStatement;
use crate::parser::ast::utils::CommaSeparated;
use crate::parser::ast::variables::Variable;

pub mod arguments;
pub mod attributes;
pub mod classes;
pub mod comments;
pub mod constant;
pub mod control_flow;
pub mod data_type;
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

impl Node for Block {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.iter_mut().map(|s| s as &mut dyn Node).collect()
    }
}

pub type Program = Block;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum UseKind {
    Normal,
    Function,
    Const,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Ending {
    Semicolon(Span),
    CloseTag(Span),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub struct HaltCompilerStatement {
    pub content: Option<ByteString>,
}

impl Node for HaltCompilerStatement {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub struct StaticStatement {
    pub vars: Vec<StaticVar>,
}

impl Node for StaticStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.vars.iter_mut().map(|v| v as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub struct UseStatement {
    pub kind: UseKind,
    pub uses: Vec<Use>,
}

impl Node for UseStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.uses.iter_mut().map(|u| u as &mut dyn Node).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Statement {
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct InlineHtmlStatement {
    pub html: ByteString,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct FullOpeningTagStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ShortOpeningTagStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct EchoOpeningTagStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct ClosingTagStatement {
    pub span: Span,
}

impl Node for Statement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Statement::Label(statement) => vec![statement],
            Statement::Goto(statement) => vec![statement],
            Statement::HaltCompiler(statement) => vec![statement],
            Statement::Static(statement) => vec![statement],
            Statement::DoWhile(statement) => vec![statement],
            Statement::While(statement) => vec![statement],
            Statement::For(statement) => vec![statement],
            Statement::Foreach(statement) => vec![statement],
            Statement::Break(statement) => vec![statement],
            Statement::Continue(statement) => vec![statement],
            Statement::Constant(statement) => vec![statement],
            Statement::Function(statement) => vec![statement],
            Statement::Class(statement) => vec![statement],
            Statement::Trait(statement) => vec![statement],
            Statement::Interface(statement) => vec![statement],
            Statement::If(statement) => vec![statement],
            Statement::Switch(statement) => vec![statement],
            Statement::Echo(statement) => vec![statement],
            Statement::Expression(statement) => vec![statement],
            Statement::Return(statement) => vec![statement],
            Statement::Namespace(statement) => vec![statement],
            Statement::Use(statement) => vec![statement],
            Statement::GroupUse(statement) => vec![statement],
            Statement::Comment(statement) => vec![statement],
            Statement::Try(statement) => vec![statement],
            Statement::UnitEnum(statement) => vec![statement],
            Statement::BackedEnum(statement) => vec![statement],
            Statement::Block(statement) => vec![statement],
            Statement::Global(statement) => vec![statement],
            Statement::Declare(statement) => vec![statement],
            _ => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub ending: Ending,
}

impl Node for ExpressionStatement {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![&mut self.expression]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct EvalExpression {
    pub eval: Span,
    // eval
    pub argument: Box<SingleArgument>, // ("$a = 1")
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct EmptyExpression {
    pub empty: Span,
    // empty
    pub argument: Box<SingleArgument>, // ($a)
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DieExpression {
    pub die: Span,
    // die
    pub argument: Option<Box<SingleArgument>>, // (1)
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ExitExpression {
    pub exit: Span,
    // exit
    pub argument: Option<Box<SingleArgument>>, // (1)
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct IssetExpression {
    pub isset: Span,
    // isset
    pub arguments: ArgumentList, // `($a, ...)`
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct UnsetExpression {
    pub unset: Span,
    // unset
    pub arguments: ArgumentList, // `($a, ...)`
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct PrintExpression {
    pub print: Span,
    // print
    pub value: Option<Box<Expression>>,
    // 1
    pub argument: Option<Box<SingleArgument>>, // (1)
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ConcatExpression {
    pub left: Box<Expression>,
    pub dot: Span,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InstanceofExpression {
    pub left: Box<Expression>,
    pub instanceof: Span,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ReferenceExpression {
    pub ampersand: Span,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ParenthesizedExpression {
    pub start: Span,
    pub expr: Box<Expression>,
    pub end: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ErrorSuppressExpression {
    pub at: Span,
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct IncludeExpression {
    pub include: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct IncludeOnceExpression {
    pub include_once: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct RequireExpression {
    pub require: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct RequireOnceExpression {
    pub require_once: Span,
    pub path: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct NowdocExpression {
    pub label: ByteString,
    pub value: ByteString,
}

impl Node for NowdocExpression {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct BoolExpression {
    pub value: bool,
}

impl Node for BoolExpression {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CloneExpression {
    pub target: Box<Expression>,
}

impl Node for CloneExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.target.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ThrowExpression {
    pub value: Box<Expression>,
}

impl Node for ThrowExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.value.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct YieldFromExpression {
    pub value: Box<Expression>,
}

impl Node for YieldFromExpression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        vec![self.value.as_mut()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Expression {
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

impl Node for Expression {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        match self {
            Expression::Eval(expression) => vec![expression],
            Expression::Empty(expression) => vec![expression],
            Expression::Die(expression) => vec![expression],
            Expression::Exit(expression) => vec![expression],
            Expression::Isset(expression) => vec![expression],
            Expression::Unset(expression) => vec![expression],
            Expression::Print(expression) => vec![expression],
            Expression::Literal(literal) => vec![literal],
            Expression::ArithmeticOperation(operation) => vec![operation],
            Expression::AssignmentOperation(operation) => vec![operation],
            Expression::BitwiseOperation(operation) => vec![operation],
            Expression::ComparisonOperation(operation) => vec![operation],
            Expression::LogicalOperation(operation) => vec![operation],
            Expression::Concat(expression) => vec![expression],
            Expression::Instanceof(expression) => vec![expression],
            Expression::Reference(expression) => vec![expression],
            Expression::Parenthesized(expression) => vec![expression],
            Expression::ErrorSuppress(expression) => vec![expression],
            Expression::Identifier(identifier) => vec![identifier],
            Expression::Variable(variable) => vec![variable],
            Expression::Include(expression) => vec![expression],
            Expression::IncludeOnce(expression) => vec![expression],
            Expression::Require(expression) => vec![expression],
            Expression::RequireOnce(expression) => vec![expression],
            Expression::FunctionCall(expression) => vec![expression],
            Expression::FunctionClosureCreation(expression) => vec![expression],
            Expression::MethodCall(expression) => vec![expression],
            Expression::MethodClosureCreation(expression) => vec![expression],
            Expression::NullsafeMethodCall(expression) => vec![expression],
            Expression::StaticMethodCall(expression) => vec![expression],
            Expression::StaticVariableMethodCall(expression) => vec![expression],
            Expression::StaticMethodClosureCreation(expression) => vec![expression],
            Expression::StaticVariableMethodClosureCreation(expression) => vec![expression],
            Expression::PropertyFetch(expression) => vec![expression],
            Expression::NullsafePropertyFetch(expression) => vec![expression],
            Expression::StaticPropertyFetch(expression) => vec![expression],
            Expression::ConstantFetch(expression) => vec![expression],
            Expression::Static => vec![],
            Expression::Self_ => vec![],
            Expression::Parent => vec![],
            Expression::ShortArray(expression) => vec![expression],
            Expression::Array(expression) => vec![expression],
            Expression::List(expression) => vec![expression],
            Expression::Closure(expression) => vec![expression],
            Expression::ArrowFunction(expression) => vec![expression],
            Expression::New(expression) => vec![expression],
            Expression::InterpolatedString(expression) => vec![expression],
            Expression::Heredoc(expression) => vec![expression],
            Expression::Nowdoc(expression) => vec![expression],
            Expression::ShellExec(expression) => vec![expression],
            Expression::AnonymousClass(expression) => vec![expression],
            Expression::Bool(_) => vec![],
            Expression::ArrayIndex(expression) => vec![expression],
            Expression::Null => vec![],
            Expression::MagicConstant(constant) => vec![constant],
            Expression::ShortTernary(expression) => vec![expression],
            Expression::Ternary(expression) => vec![expression],
            Expression::Coalesce(expression) => vec![expression],
            Expression::Clone(expression) => vec![expression],
            Expression::Match(expression) => vec![expression],
            Expression::Throw(expression) => vec![expression],
            Expression::Yield(expression) => vec![expression],
            Expression::YieldFrom(expression) => vec![expression],
            Expression::Cast(expression) => vec![expression],
            Expression::Noop => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum StringPart {
    Literal(LiteralStringPart),
    Expression(ExpressionStringPart),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

pub struct LiteralStringPart {
    pub value: ByteString,
}

impl Node for LiteralStringPart {
    //
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]

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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
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
