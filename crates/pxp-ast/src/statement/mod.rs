use pxp_span::{Span, HasSpan};

mod inline_html;
mod label;
mod goto;
mod halt_compiler;
mod r#static;
mod do_while;
mod r#while;
mod r#for;
mod foreach;
mod r#break;
mod r#continue;
mod constant;
mod function;
mod class;
mod r#trait;
mod interface;
mod r#if;
mod switch;
mod echo;
mod expression;
mod r#return;
mod namespace;
mod r#use;
mod group_use;
mod r#try;
mod unit_enum;
mod backed_enum;
mod block;
mod global;
mod declare;
mod noop;

pub use inline_html::*;
pub use label::*;
pub use goto::*;
pub use halt_compiler::*;
pub use r#static::*;
pub use do_while::*;
pub use r#while::*;
pub use r#for::*;
pub use foreach::*;
pub use r#break::*;
pub use r#continue::*;
pub use constant::*;
pub use function::*;
pub use class::*;
pub use r#trait::*;
pub use interface::*;
pub use r#if::*;
pub use switch::*;
pub use echo::*;
pub use expression::*;
pub use r#return::*;
pub use namespace::*;
pub use r#use::*;
pub use group_use::*;
pub use r#try::*;
pub use unit_enum::*;
pub use backed_enum::*;
pub use block::*;
pub use global::*;
pub use declare::*;
pub use noop::*;

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl HasSpan for Statement {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum StatementKind {
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
    Try(TryStatement),
    UnitEnum(UnitEnumStatement),
    BackedEnum(BackedEnumStatement),
    Block(BlockStatement),
    Global(GlobalStatement),
    Declare(DeclareStatement),
    Noop(NoopStatement),
}