use pxp_span::{Span, HasSpan};

use crate::{LiteralInteger, LiteralFloat, LiteralString};

mod eval;
mod empty;
mod die;
mod exit;
mod isset;
mod unset;
mod print;
mod integer;
mod float;
mod string;
mod infix;
mod postfix;
mod prefix;
mod reference;
mod parenthesized;
mod error_suppress;
mod identifier;
mod include;
mod include_once;
mod require;
mod require_once;
mod function_call;
mod function_closure_creation;
mod method_call;
mod method_closure_creation;
mod nullsafe_method_call;
mod static_method_call;
mod static_variable_method_call;
mod static_method_closure_creation;
mod static_variable_method_closure_creation;
mod property_fetch;
mod nullsafe_property_fetch;
mod static_property_fetch;
mod constant_fetch;
mod short_array;
mod array;
mod list;
mod closure;
mod arrow_function;
mod new;
mod interpolated_string;
mod heredoc;
mod nowdoc;
mod shell_exec;
mod anonymous_class;
mod array_index;
mod magic_constant;
mod short_ternary;
mod ternary;
mod coalesce;
mod clone;
mod r#match;
mod throw;
mod r#yield;
mod yield_from;
mod cast;
mod r#true;
mod r#false;
mod null;

pub use eval::*;
pub use empty::*;
pub use die::*;
pub use exit::*;
pub use isset::*;
pub use unset::*;
pub use print::*;
pub use integer::*;
pub use float::*;
pub use string::*;
pub use infix::*;
pub use postfix::*;
pub use prefix::*;
pub use reference::*;
pub use parenthesized::*;
pub use error_suppress::*;
pub use identifier::*;
pub use include::*;
pub use include_once::*;
pub use require::*;
pub use require_once::*;
pub use function_call::*;
pub use function_closure_creation::*;
pub use method_call::*;
pub use method_closure_creation::*;
pub use nullsafe_method_call::*;
pub use static_method_call::*;
pub use static_variable_method_call::*;
pub use static_method_closure_creation::*;
pub use static_variable_method_closure_creation::*;
pub use property_fetch::*;
pub use nullsafe_property_fetch::*;
pub use static_property_fetch::*;
pub use constant_fetch::*;
pub use short_array::*;
pub use array::*;
pub use list::*;
pub use closure::*;
pub use arrow_function::*;
pub use new::*;
pub use interpolated_string::*;
pub use heredoc::*;
pub use nowdoc::*;
pub use shell_exec::*;
pub use anonymous_class::*;
pub use array_index::*;
pub use magic_constant::*;
pub use short_ternary::*;
pub use ternary::*;
pub use coalesce::*;
pub use clone::*;
pub use r#match::*;
pub use throw::*;
pub use r#yield::*;
pub use yield_from::*;
pub use cast::*;
pub use r#true::*;
pub use r#false::*;
pub use null::*;

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl HasSpan for Expression {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Eval(EvalExpression),
    Empty(EmptyExpression),
    Die(DieExpression),
    Exit(ExitExpression),
    Isset(IssetExpression),
    Unset(UnsetExpression),
    Print(PrintExpression),
    Integer(LiteralInteger),
    Float(LiteralFloat),
    String(LiteralString),
    Infix(InfixExpression),
    Postfix(PostfixExpression),
    Prefix(PrefixExpression),
    // ArithmeticOperation(ArithmeticOperationExpression),
    // AssignmentOperation(AssignmentOperationExpression),
    // BitwiseOperation(BitwiseOperationExpression),
    // ComparisonOperation(ComparisonOperationExpression),
    // LogicalOperation(LogicalOperationExpression),
    // Concat(ConcatExpression),
    // Instanceof(InstanceofExpression),
    Reference(ReferenceExpression),
    Parenthesized(ParenthesizedExpression),
    ErrorSuppress(ErrorSuppressExpression),
    Identifier(Identifier),
    Variable(Variable),
    Include(IncludeExpression),
    IncludeOnce(IncludeOnceExpression),
    Require(RequireExpression),
    RequireOnce(RequireOnceExpression),
    FunctionCall(FunctionCallExpression),
    FunctionClosureCreation(FunctionClosureCreationExpression),
    MethodCall(MethodCallExpression),
    MethodClosureCreation(MethodClosureCreationExpression),
    NullsafeMethodCall(NullsafeMethodCallExpression),
    StaticMethodCall(StaticMethodCallExpression),
    StaticVariableMethodCall(StaticVariableMethodCallExpression),
    StaticMethodClosureCreation(StaticMethodClosureCreationExpression),
    StaticVariableMethodClosureCreation(StaticVariableMethodClosureCreationExpression),
    PropertyFetch(PropertyFetchExpression),
    NullsafePropertyFetch(NullsafePropertyFetchExpression),
    StaticPropertyFetch(StaticPropertyFetchExpression),
    ConstantFetch(ConstantFetchExpression),
    ShortArray(ShortArrayExpression),
    Array(ArrayExpression),
    List(ListExpression),
    Closure(ClosureExpression),
    ArrowFunction(ArrowFunctionExpression),
    New(NewExpression),
    InterpolatedString(InterpolatedStringExpression),
    Heredoc(HeredocExpression),
    Nowdoc(NowdocExpression),
    ShellExec(ShellExecExpression),
    AnonymousClass(AnonymousClassExpression),
    ArrayIndex(ArrayIndexExpression),
    MagicConstant(MagicConstantExpression),
    ShortTernary(ShortTernaryExpression),
    Ternary(TernaryExpression),
    Coalesce(CoalesceExpression),
    Clone(CloneExpression),
    Match(MatchExpression),
    Throw(ThrowExpression),
    Yield(YieldExpression),
    YieldFrom(YieldFromExpression),
    Cast(CastExpression),
    True(TrueExpression),
    False(FalseExpression),
    Null(NullExpression),
    Noop,
}