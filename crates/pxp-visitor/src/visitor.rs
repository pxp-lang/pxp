use pxp_ast::{Statement, FullOpeningTagStatement, ShortOpeningTagStatement, EchoOpeningTagStatement, ClosingTagStatement, InlineHtmlStatement, goto::{LabelStatement, GotoStatement}, StaticStatement, GlobalStatement, HaltCompilerStatement, loops::{DoWhileStatement, WhileStatement, ForStatement, ForeachStatement, BreakStatement, ContinueStatement}, constant::ConstantStatement, functions::{FunctionStatement, ClosureExpression, ArrowFunctionExpression}, classes::{ClassStatement, AnonymousClassExpression}, traits::TraitStatement, interfaces::InterfaceStatement, control_flow::IfStatement, SwitchStatement, EchoStatement, ExpressionStatement, ReturnStatement, namespaces::NamespaceStatement, UseStatement, GroupUseStatement, try_block::TryStatement, enums::{UnitEnumStatement, BackedEnumStatement}, BlockStatement, declares::DeclareStatement, EvalExpression, EmptyExpression, DieExpression, ExitExpression, IssetExpression, UnsetExpression, PrintExpression, literals::Literal, operators::{ArithmeticOperationExpression, AssignmentOperationExpression, BitwiseOperationExpression, ComparisonOperationExpression, LogicalOperationExpression}, ConcatExpression, InstanceofExpression, ReferenceExpression, ParenthesizedExpression, ErrorSuppressExpression, identifiers::Identifier, variables::Variable, IncludeExpression, IncludeOnceExpression, RequireExpression, RequireOnceExpression, FunctionCallExpression, FunctionClosureCreationExpression, MethodCallExpression, MethodClosureCreationExpression, NullsafeMethodCallExpression, StaticMethodCallExpression, StaticVariableMethodCallExpression, StaticMethodClosureCreationExpression, StaticVariableMethodClosureCreationExpression, PropertyFetchExpression, NullsafePropertyFetchExpression, StaticPropertyFetchExpression, ConstantFetchExpression, ShortArrayExpression, ArrayExpression, ListExpression, NewExpression, InterpolatedStringExpression, HeredocExpression, NowdocExpression, ShellExecExpression, BoolExpression, ArrayIndexExpression, MagicConstantExpression, ShortTernaryExpression, TernaryExpression, CoalesceExpression, CloneExpression, MatchExpression, ThrowExpression, YieldExpression, YieldFromExpression, CastExpression};
use pxp_span::Span;
use pxp_syntax::comments::Comment;
use crate::walk::*;

pub trait Visitor {
    fn visit(&mut self, program: &mut [Statement]) {
        walk(self, program);
    }

    fn visit_statement(&mut self, statement: &mut Statement) {
        walk_statement(self, statement)
    }

    fn visit_full_opening_tag(&mut self, statement: &mut FullOpeningTagStatement) {}
    fn visit_short_opening_tag(&mut self, statement: &mut ShortOpeningTagStatement) {}
    fn visit_echo_opening_tag(&mut self, statement: &mut EchoOpeningTagStatement) {}
    fn visit_closing_tag(&mut self, statement: &mut ClosingTagStatement) {}

    fn visit_inline_html(&mut self, statement: &mut InlineHtmlStatement) {}
    fn visit_halt_compiler(&mut self, statement: &mut HaltCompilerStatement) {}

    fn visit_label(&mut self, statement: &mut LabelStatement) {}
    fn visit_goto(&mut self, statement: &mut GotoStatement) {}

    fn visit_static(&mut self, statement: &mut StaticStatement) {}
    fn visit_global(&mut self, statement: &mut GlobalStatement) {}

    fn visit_do_while(&mut self, statement: &mut DoWhileStatement) {
        todo!("walk do while")
    }

    fn visit_while(&mut self, statement: &mut WhileStatement) {
        todo!("walk while")
    }

    fn visit_for(&mut self, statement: &mut ForStatement) {
        todo!("walk for")
    }

    fn visit_foreach(&mut self, statement: &mut ForeachStatement) {
        todo!("walk foreach")
    }

    fn visit_if(&mut self, statement: &mut IfStatement) {
        todo!("walk if")
    }

    fn visit_switch(&mut self, statement: &mut SwitchStatement) {
        todo!("walk switch")
    }

    fn visit_break(&mut self, statement: &mut BreakStatement) {
        todo!("walk break")
    }

    fn visit_continue(&mut self, statement: &mut ContinueStatement) {
        todo!("walk continue")
    }

    fn visit_constant(&mut self, statement: &mut ConstantStatement) {
        todo!("walk constant")
    }

    fn visit_function(&mut self, statement: &mut FunctionStatement) {
        todo!("walk function")
    }

    fn visit_class(&mut self, statement: &mut ClassStatement) {
        todo!("walk class")
    }

    fn visit_trait(&mut self, statement: &mut TraitStatement) {
        todo!("walk trait")
    }

    fn visit_interface(&mut self, statement: &mut InterfaceStatement) {
        todo!("walk interface")
    }

    fn visit_echo(&mut self, statement: &mut EchoStatement) {
        todo!("walk echo")
    }

    fn visit_expression_stmt(&mut self, statement: &mut ExpressionStatement) {
        todo!("walk expression")
    }

    fn visit_return(&mut self, statement: &mut ReturnStatement) {
        todo!("walk return")
    }

    fn visit_namespace(&mut self, statement: &mut NamespaceStatement) {
        todo!("walk namespace")
    }

    fn visit_use(&mut self, statement: &mut UseStatement) {
        todo!("walk use")
    }

    fn visit_group_use(&mut self, statement: &mut GroupUseStatement) {
        todo!("walk group use")
    }

    fn visit_comment_stmt(&mut self, statement: &mut Comment) {
        todo!("walk comment")
    }

    fn visit_try(&mut self, statement: &mut TryStatement) {
        todo!("walk try")
    }

    fn visit_unit_enum(&mut self, statement: &mut UnitEnumStatement) {
        todo!("walk unit enum")
    }

    fn visit_backed_enum(&mut self, statement: &mut BackedEnumStatement) {
        todo!("walk backed enum")
    }

    fn visit_block(&mut self, statement: &mut BlockStatement) {
        todo!("walk block")
    }

    fn visit_declare(&mut self, statement: &mut DeclareStatement) {
        todo!("walk declare")
    }

    fn visit_noop(&mut self, statement: Span) {
        todo!("walk noop")
    }

    fn visit_missing_expr(&mut self) {}

    fn visit_eval(&mut self, expression: &mut EvalExpression) {
        todo!("walk eval")
    }

    fn visit_empty(&mut self, expression: &mut EmptyExpression) {
        todo!("walk empty")
    }

    fn visit_die(&mut self, expression: &mut DieExpression) {
        todo!("walk die")
    }

    fn visit_exit(&mut self, expression: &mut ExitExpression) {
        todo!("walk exit")
    }

    fn visit_isset(&mut self, expression: &mut IssetExpression) {
        todo!("walk isset")
    }

    fn visit_unset(&mut self, expression: &mut UnsetExpression) {
        todo!("walk unset")
    }

    fn visit_print(&mut self, expression: &mut PrintExpression) {
        todo!("walk print")
    }

    fn visit_literal(&mut self, expression: &mut Literal) {
        todo!("walk literal - maybe walk literal kind?")
    }

    fn visit_arithmetic_operation(&mut self, expression: &mut ArithmeticOperationExpression) {
        todo!("walk arithmetic operation")
    }

    fn visit_assignment_operation(&mut self, expression: &mut AssignmentOperationExpression) {
        todo!("walk assignment operation")
    }

    fn visit_bitwise_operation(&mut self, expression: &mut BitwiseOperationExpression) {
        todo!("walk bitwise operation")
    }    

    fn visit_comparison_operation(&mut self, expression: &mut ComparisonOperationExpression) {
        todo!("walk comparsion operation")
    }

    fn visit_logical_operation(&mut self, expression: &mut LogicalOperationExpression) {
        todo!("walk logical operation")
    }

    fn visit_concat(&mut self, expression: &mut ConcatExpression) {
        todo!("walk concat")
    }

    fn visit_instanceof(&mut self, expression: &mut InstanceofExpression) {
        todo!("walk instanceof")
    }

    fn visit_reference(&mut self, expression: &mut ReferenceExpression) {
        todo!("walk reference")
    }

    fn visit_parenthesized(&mut self, expression: &mut ParenthesizedExpression) {
        todo!("walk parenthesized")
    }

    fn visit_error_suppress(&mut self, expression: &mut ErrorSuppressExpression) {
        todo!("walk error suppress")
    }

    fn visit_identifier(&mut self, expression: &mut Identifier) {
        todo!("walk identifier - maybe walk identifier kind?")
    }

    fn visit_variable(&mut self, expression: &mut Variable) {
        todo!("walk variable - maybe walk variable kind?")
    }

    fn visit_include(&mut self, expression: &mut IncludeExpression) {
        todo!("walk include")
    }

    fn visit_include_once(&mut self, expression: &mut IncludeOnceExpression) {
        todo!("walk include once")
    }

    fn visit_require(&mut self, expression: &mut RequireExpression) {
        todo!("walk require")
    }

    fn visit_require_once(&mut self, expression: &mut RequireOnceExpression) {
        todo!("walk require once")
    }

    fn visit_function_call(&mut self, expression: &mut FunctionCallExpression) {
        todo!("walk function call")
    }

    fn visit_function_closure_creation(&mut self, expression: &mut FunctionClosureCreationExpression) {
        todo!("walk function closure creation")
    }

    fn visit_method_call(&mut self, expression: &mut MethodCallExpression) {
        todo!("walk method call")
    }

    fn visit_method_closure_creation(&mut self, expression: &mut MethodClosureCreationExpression) {
        todo!("walk method closure creation")
    }

    fn visit_nullsafe_method_call(&mut self, expression: &mut NullsafeMethodCallExpression) {
        todo!("walk nullsafe method call")
    }

    fn visit_static_method_call(&mut self, expression: &mut StaticMethodCallExpression) {
        todo!("walk static method call")
    }

    fn visit_static_variable_method_call(&mut self, expression: &mut StaticVariableMethodCallExpression) {
        todo!("walk static variable method call")
    }

    fn visit_static_method_closure_creation(&mut self, expression: &mut StaticMethodClosureCreationExpression) {
        todo!("walk static method closure creation")
    }

    fn visit_static_variable_method_closure_creation(&mut self, expression: &mut StaticVariableMethodClosureCreationExpression) {
        todo!("walk static variable method closure creation")
    }

    fn visit_property_fetch(&mut self, expression: &mut PropertyFetchExpression) {
        todo!("walk property fetch")
    }

    fn visit_nullsafe_property_fetch(&mut self, expression: &mut NullsafePropertyFetchExpression) {
        todo!("walk nullsafe property fetch")
    }

    fn visit_static_property_fetch(&mut self, expression: &mut StaticPropertyFetchExpression) {
        todo!("walk static property fetch")
    }

    fn visit_constant_fetch(&mut self, expression: &mut ConstantFetchExpression) {
        todo!("walk constant fetch")
    }

    fn visit_static_expr(&mut self) {}
    fn visit_self_expr(&mut self) {}
    fn visit_parent_expr(&mut self) {}

    fn visit_short_array(&mut self, expression: &mut ShortArrayExpression) {
        todo!("walk short array")
    }

    fn visit_array(&mut self, expression: &mut ArrayExpression) {
        todo!("walk array")
    }

    fn visit_list(&mut self, expression: &mut ListExpression) {
        todo!("walk list")
    }

    fn visit_closure(&mut self, expression: &mut ClosureExpression) {
        todo!("walk closure")
    }

    fn visit_arrow_function(&mut self, expression: &mut ArrowFunctionExpression) {
        todo!("walk arrow function")
    }

    fn visit_new(&mut self, expression: &mut NewExpression) {
        todo!("walk new")
    }

    fn visit_interpolated_string(&mut self, expression: &mut InterpolatedStringExpression) {
        todo!("walk interpolated string")
    }

    fn visit_heredoc(&mut self, expression: &mut HeredocExpression) {
        todo!("walk heredoc")
    }

    fn visit_nowdoc(&mut self, expression: &mut NowdocExpression) {
        todo!("walk nowdoc")
    }

    fn visit_shell_exec(&mut self, expression: &mut ShellExecExpression) {
        todo!("walk shell exec")
    }

    fn visit_anonymous_class(&mut self, expression: &mut AnonymousClassExpression) {
        todo!("walk anonymous class")
    }

    fn visit_bool(&mut self, expression: &mut BoolExpression) {
        todo!("walk bool")
    }

    fn visit_array_index(&mut self, expression: &mut ArrayIndexExpression) {
        todo!("walk array index")
    }

    fn visit_null_expr(&mut self) {}

    fn visit_magic_constant(&mut self, expression: &mut MagicConstantExpression) {
        todo!("walk magic constant")
    }

    fn visit_short_ternary(&mut self, expression: &mut ShortTernaryExpression) {
        todo!("walk short ternary")
    }

    fn visit_ternary(&mut self, expression: &mut TernaryExpression) {
        todo!("walk ternary")
    }

    fn visit_coalesce(&mut self, expression: &mut CoalesceExpression) {
        todo!("walk coalesce")
    }

    fn visit_clone(&mut self, expression: &mut CloneExpression) {
        todo!("walk clone")
    }

    fn visit_match(&mut self, expression: &mut MatchExpression) {
        todo!("walk match")
    }

    fn visit_throw(&mut self, expression: &mut ThrowExpression) {
        todo!("walk throw")
    }

    fn visit_yield(&mut self, expression: &mut YieldExpression) {
        todo!("walk yield")
    }

    fn visit_yield_from(&mut self, expression: &mut YieldFromExpression) {
        todo!("walk yield from")
    }

    fn visit_cast(&mut self, expression: &mut CastExpression) {
        todo!("walk cast")
    }

    fn visit_noop_expr(&mut self) {}
}