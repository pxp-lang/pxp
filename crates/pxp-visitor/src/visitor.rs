use pxp_ast::{Statement, FullOpeningTagStatement, ShortOpeningTagStatement, EchoOpeningTagStatement, ClosingTagStatement, InlineHtmlStatement, goto::{LabelStatement, GotoStatement}, StaticStatement, GlobalStatement, HaltCompilerStatement, loops::{DoWhileStatement, WhileStatement, ForStatement, ForeachStatement, BreakStatement, ContinueStatement, WhileStatementBody, ForStatementIterator, ForStatementBody, ForeachStatementIterator, ForeachStatementBody, Level}, constant::{ConstantStatement, ConstantEntry, ClassishConstant}, functions::{FunctionStatement, ClosureExpression, ArrowFunctionExpression, FunctionParameterList, FunctionBody, FunctionParameter, AbstractMethod, AbstractConstructor, ConstructorParameterList, ConstructorParameter, ConcreteMethod, MethodBody, ConcreteConstructor}, classes::{ClassStatement, AnonymousClassExpression, ClassExtends, ClassImplements, ClassBody, ClassishMember}, traits::{TraitStatement, TraitUsage, TraitUsageAdaptation, TraitBody}, interfaces::{InterfaceStatement, InterfaceExtends, InterfaceBody}, control_flow::{IfStatement, IfStatementBody, IfStatementElseIf, IfStatementElse, IfStatementElseIfBlock, IfStatementElseBlock}, SwitchStatement, EchoStatement, ReturnStatement, namespaces::{NamespaceStatement, UnbracedNamespace, BracedNamespace}, UseStatement, GroupUseStatement, try_block::{TryStatement, CatchBlock, FinallyBlock}, enums::{UnitEnumStatement, BackedEnumStatement, UnitEnumMember, UnitEnumCase, BackedEnumMember, BackedEnumCase}, BlockStatement, declares::{DeclareStatement, DeclareEntry, DeclareBody}, EvalExpression, EmptyExpression, DieExpression, ExitExpression, IssetExpression, UnsetExpression, PrintExpression, literals::Literal, operators::{ArithmeticOperationExpression, AssignmentOperationExpression, BitwiseOperationExpression, ComparisonOperationExpression, LogicalOperationExpression}, ConcatExpression, InstanceofExpression, ReferenceExpression, ParenthesizedExpression, ErrorSuppressExpression, identifiers::{Identifier, SimpleIdentifier, DynamicIdentifier}, variables::{Variable, SimpleVariable}, IncludeExpression, IncludeOnceExpression, RequireExpression, RequireOnceExpression, FunctionCallExpression, FunctionClosureCreationExpression, MethodCallExpression, MethodClosureCreationExpression, NullsafeMethodCallExpression, StaticMethodCallExpression, StaticVariableMethodCallExpression, StaticMethodClosureCreationExpression, StaticVariableMethodClosureCreationExpression, PropertyFetchExpression, NullsafePropertyFetchExpression, StaticPropertyFetchExpression, ConstantFetchExpression, ShortArrayExpression, ArrayExpression, ListExpression, NewExpression, InterpolatedStringExpression, HeredocExpression, NowdocExpression, ShellExecExpression, BoolExpression, ArrayIndexExpression, MagicConstantExpression, ShortTernaryExpression, TernaryExpression, CoalesceExpression, CloneExpression, MatchExpression, ThrowExpression, YieldExpression, YieldFromExpression, CastExpression, StaticVar, Case, properties::{Property, PropertyEntry, VariableProperty}, Use, arguments::{Argument, ArgumentList}, ExpressionStatement, Expression};
use pxp_span::Span;
use pxp_syntax::comments::Comment;
use crate::walk::*;

pub trait Visitor {
    fn visit(&mut self, node: &mut [Statement]) {
        walk(self, node);
    }

    fn visit_statement(&mut self, node: &mut Statement) {
        walk_statement(self, node)
    }

    fn visit_expression(&mut self, node: &mut Expression) {
        walk_expression(self, node)
    }

    fn visit_full_opening_tag(&mut self, _: &mut FullOpeningTagStatement) {}
    fn visit_short_opening_tag(&mut self, _: &mut ShortOpeningTagStatement) {}
    fn visit_echo_opening_tag(&mut self, _: &mut EchoOpeningTagStatement) {}
    fn visit_closing_tag(&mut self, _: &mut ClosingTagStatement) {}

    fn visit_inline_html(&mut self, _: &mut InlineHtmlStatement) {}
    fn visit_halt_compiler(&mut self, _: &mut HaltCompilerStatement) {}

    fn visit_label(&mut self, node: &mut LabelStatement) {
        walk_label(self, node)
    }

    fn visit_goto(&mut self, node: &mut GotoStatement) {
        walk_goto(self, node)
    }

    fn visit_static(&mut self, node: &mut StaticStatement) {
        walk_static(self, node)
    }

    fn visit_static_var(&mut self, var: &mut StaticVar) {
        walk_static_var(self, var)
    }

    fn visit_global(&mut self, node: &mut GlobalStatement) {
        walk_global(self, node);
    }

    fn visit_do_while(&mut self, node: &mut DoWhileStatement) {
        walk_do_while(self, node)
    }

    fn visit_while(&mut self, node: &mut WhileStatement) {
        walk_while(self, node)
    }

    fn visit_while_statement_body(&mut self, node: &mut WhileStatementBody) {
        walk_while_statement_body(self, node)
    }

    fn visit_for(&mut self, node: &mut ForStatement) {
        walk_for(self, node)
    }

    fn visit_for_statement_iterator(&mut self, node: &mut ForStatementIterator) {
        walk_for_statement_iterator(self, node);
    }

    fn visit_for_statement_body(&mut self, node: &mut ForStatementBody) {
        walk_for_statement_body(self, node);
    }

    fn visit_foreach(&mut self, node: &mut ForeachStatement) {
        walk_foreach(self, node)
    }

    fn visit_foreach_statement_iterator(&mut self, node: &mut ForeachStatementIterator) {
        walk_foreach_statement_iterator(self, node)
    }

    fn visit_foreach_statement_body(&mut self, node: &mut ForeachStatementBody) {
        walk_foreach_statement_body(self, node)
    }

    fn visit_if(&mut self, node: &mut IfStatement) {
        walk_if(self, node)
    }

    fn visit_if_statement_body(&mut self, node: &mut IfStatementBody) {
        walk_if_statement_body(self, node)
    }

    fn visit_if_statement_elseif(&mut self, node: &mut IfStatementElseIf) {
        walk_if_statement_elseif(self, node)
    }

    fn visit_if_statement_else(&mut self, node: &mut IfStatementElse) {
        walk_if_statement_else(self, node)
    }

    fn visit_if_statement_elseif_block(&mut self, node: &mut IfStatementElseIfBlock) {
        walk_if_statement_elseif_block(self, node)
    }

    fn visit_if_statement_else_block(&mut self, node: &mut IfStatementElseBlock) {
        walk_if_statement_else_block(self, node)
    }

    fn visit_switch(&mut self, node: &mut SwitchStatement) {
        walk_switch(self, node)
    }

    fn visit_switch_case(&mut self, node: &mut Case) {
        walk_switch_case(self, node)
    }

    fn visit_level(&mut self, node: &mut Level) {
        walk_level(self, node)
    }

    fn visit_break(&mut self, node: &mut BreakStatement) {
        walk_break(self, node)
    }

    fn visit_continue(&mut self, node: &mut ContinueStatement) {
        walk_continue(self, node)
    }

    fn visit_constant(&mut self, node: &mut ConstantStatement) {
        walk_constant(self, node)
    }

    fn visit_constant_entry(&mut self, node: &mut ConstantEntry) {
        walk_constant_entry(self, node)
    }

    fn visit_function(&mut self, node: &mut FunctionStatement) {
        walk_function(self, node)
    }

    fn visit_function_parameter_list(&mut self, node: &mut FunctionParameterList) {
        walk_function_parameter_list(self, node)
    }

    fn visit_function_parameter(&mut self, node: &mut FunctionParameter) {
        walk_function_parameter(self, node)
    }

    fn visit_function_body(&mut self, node: &mut FunctionBody) {
        walk_function_body(self, node)
    }

    fn visit_class(&mut self, node: &mut ClassStatement) {
        walk_class(self, node)
    }

    fn visit_class_extends(&mut self, node: &mut ClassExtends) {
        walk_class_extends(self, node)
    }

    fn visit_class_implements(&mut self, node: &mut ClassImplements) {
        walk_class_implements(self, node)
    }

    fn visit_class_body(&mut self, node: &mut ClassBody) {
        walk_class_body(self, node)
    }

    fn visit_classish_member(&mut self, node: &mut ClassishMember) {
        walk_classish_member(self, node)
    }

    fn visit_classish_constant(&mut self, node: &mut ClassishConstant) {
        walk_classish_constant(self, node)
    }

    fn visit_trait_usage(&mut self, node: &mut TraitUsage) {
        walk_trait_usage(self, node)
    }

    fn visit_trait_usage_adaptation(&mut self, node: &mut TraitUsageAdaptation) {
        walk_trait_usage_adaptation(self, node)
    }

    fn visit_property(&mut self, node: &mut Property) {
        walk_property(self, node)
    }

    fn visit_property_entry(&mut self, node: &mut PropertyEntry) {
        walk_property_entry(self, node)
    }

    fn visit_variable_property(&mut self, node: &mut VariableProperty) {
        walk_variable_property(self, node)
    }

    fn visit_abstract_method(&mut self, node: &mut AbstractMethod) {
        walk_abstract_method(self, node)
    }

    fn visit_abstract_constructor(&mut self, node: &mut AbstractConstructor) {
        walk_abstract_constructor(self, node)
    }

    fn visit_constructor_parameter_list(&mut self, node: &mut ConstructorParameterList) {
        walk_constructor_parameter_list(self, node)
    }

    fn visit_concrete_method(&mut self, node: &mut ConcreteMethod) {
        walk_concrete_method(self, node)
    }

    fn visit_method_body(&mut self, node: &mut MethodBody) {
        walk_method_body(self, node)
    }

    fn visit_constructor_parameter(&mut self, node: &mut ConstructorParameter) {
        walk_constructor_parameter(self, node)
    }

    fn visit_concrete_constructor(&mut self, node: &mut ConcreteConstructor) {
        walk_concrete_constructor(self, node)
    }

    fn visit_interface(&mut self, node: &mut InterfaceStatement) {
        walk_interface(self, node)
    }

    fn visit_interface_extends(&mut self, node: &mut InterfaceExtends) {
        walk_interface_extends(self, node)
    }

    fn visit_interface_body(&mut self, node: &mut InterfaceBody) {
        walk_interface_body(self, node)
    }

    fn visit_trait(&mut self, node: &mut TraitStatement) {
        walk_trait(self, node)
    }

    fn visit_trait_body(&mut self, node: &mut TraitBody) {
        walk_trait_body(self, node)
    }

    fn visit_echo(&mut self, node: &mut EchoStatement) {
        walk_echo(self, node)
    }

    fn visit_expression_stmt(&mut self, node: &mut ExpressionStatement) {
        walk_expression_stmt(self, node)
    }

    fn visit_return(&mut self, node: &mut ReturnStatement) {
        walk_return(self, node)
    }

    fn visit_namespace(&mut self, node: &mut NamespaceStatement) {
        walk_namespace(self, node)
    }

    fn visit_unbraced_namespace(&mut self, node: &mut UnbracedNamespace) {
        walk_unbraced_namespace(self, node)
    }

    fn visit_braced_namespace(&mut self, node: &mut BracedNamespace) {
        walk_braced_namespace(self, node)
    }

    fn visit_use(&mut self, node: &mut UseStatement) {
        walk_use(self, node)
    }

    fn visit_use_use(&mut self, node: &mut Use) {
        walk_use_use(self, node)
    }

    fn visit_group_use(&mut self, node: &mut GroupUseStatement) {
        walk_group_use(self, node)
    }

    fn visit_comment_stmt(&mut self, _: &mut Comment) {}

    fn visit_try(&mut self, node: &mut TryStatement) {
        walk_try(self, node)
    }

    fn visit_catch_block(&mut self, node: &mut CatchBlock) {
        walk_catch_block(self, node)
    }

    fn visit_finally_block(&mut self, node: &mut FinallyBlock) {
        walk_finally_block(self, node)
    }

    fn visit_unit_enum(&mut self, node: &mut UnitEnumStatement) {
        walk_unit_enum(self, node)
    }

    fn visit_unit_enum_member(&mut self, node: &mut UnitEnumMember) {
        walk_unit_enum_member(self, node)
    }

    fn visit_unit_enum_case(&mut self, node: &mut UnitEnumCase) {
        walk_unit_enum_case(self, node)
    }

    fn visit_backed_enum(&mut self, node: &mut BackedEnumStatement) {
        walk_backed_enum(self, node)
    }

    fn visit_backed_enum_member(&mut self, node: &mut BackedEnumMember) {
        walk_backed_enum_member(self, node)
    }

    fn visit_backed_enum_case(&mut self, node: &mut BackedEnumCase) {
        walk_backed_enum_case(self, node)
    }

    fn visit_block(&mut self, node: &mut BlockStatement) {
        walk(self, &mut node.statements);
    }

    fn visit_declare(&mut self, node: &mut DeclareStatement) {
        walk_declare(self, node)
    }

    fn visit_declare_entry(&mut self, node: &mut DeclareEntry) {
        walk_declare_entry(self, node)
    }

    fn visit_declare_body(&mut self, node: &mut DeclareBody) {
        walk_declare_body(self, node)
    }

    fn visit_noop(&mut self, _: Span) {
        
    }

    fn visit_missing_expr(&mut self) {}

    fn visit_argument_list(&mut self, node: &mut ArgumentList) {
        walk_argument_list(self, node)
    }

    fn visit_argument(&mut self, node: &mut Argument) {
        walk_argument(self, node)
    }

    fn visit_eval(&mut self, node: &mut EvalExpression) {
        walk_eval(self, node)
    }

    fn visit_empty(&mut self, node: &mut EmptyExpression) {
        walk_empty(self, node)
    }

    fn visit_die(&mut self, node: &mut DieExpression) {
        walk_die(self, node)
    }

    fn visit_exit(&mut self, node: &mut ExitExpression) {
        walk_exit(self, node)
    }

    fn visit_isset(&mut self, node: &mut IssetExpression) {
        walk_isset(self, node)
    }

    fn visit_unset(&mut self, node: &mut UnsetExpression) {
        walk_unset(self, node)
    }

    fn visit_print(&mut self, node: &mut PrintExpression) {
        walk_print(self, node)
    }

    fn visit_literal(&mut self, _: &mut Literal) {}

    fn visit_arithmetic_operation(&mut self, node: &mut ArithmeticOperationExpression) {
        walk_arithmetic_operation(self, node)
    }

    fn visit_assignment_operation(&mut self, node: &mut AssignmentOperationExpression) {
        walk_assignment_operation(self, node)
    }

    fn visit_bitwise_operation(&mut self, node: &mut BitwiseOperationExpression) {
        walk_bitwise_operation(self, node)
    }    

    fn visit_comparison_operation(&mut self, node: &mut ComparisonOperationExpression) {
        walk_comparison_operation(self, node)
    }

    fn visit_logical_operation(&mut self, node: &mut LogicalOperationExpression) {
        walk_logical_operation(self, node)
    }

    fn visit_concat(&mut self, node: &mut ConcatExpression) {
        walk_concat(self, node)
    }

    fn visit_instanceof(&mut self, node: &mut InstanceofExpression) {
        walk_instanceof(self, node)
    }

    fn visit_reference(&mut self, node: &mut ReferenceExpression) {
        walk_reference(self, node)
    }

    fn visit_parenthesized(&mut self, node: &mut ParenthesizedExpression) {
        walk_parenthesized(self, node)
    }

    fn visit_error_suppress(&mut self, node: &mut ErrorSuppressExpression) {
        walk_error_suppress(self, node)
    }

    fn visit_identifier(&mut self, node: &mut Identifier) {
        walk_identifier(self, node)
    }

    fn visit_simple_identifier(&mut self, _: &mut SimpleIdentifier) {
        
    }

    fn visit_dynamic_identifier(&mut self, node: &mut DynamicIdentifier) {
        walk_dynamic_identifier(self, node)
    }

    fn visit_variable(&mut self, node: &mut Variable) {
        todo!("walk variable - maybe walk variable kind?")
    }

    fn visit_simple_variable(&mut self, variable: &mut SimpleVariable) {}

    fn visit_include(&mut self, node: &mut IncludeExpression) {
        todo!("walk include")
    }

    fn visit_include_once(&mut self, node: &mut IncludeOnceExpression) {
        todo!("walk include once")
    }

    fn visit_require(&mut self, node: &mut RequireExpression) {
        todo!("walk require")
    }

    fn visit_require_once(&mut self, node: &mut RequireOnceExpression) {
        todo!("walk require once")
    }

    fn visit_function_call(&mut self, node: &mut FunctionCallExpression) {
        todo!("walk function call")
    }

    fn visit_function_closure_creation(&mut self, node: &mut FunctionClosureCreationExpression) {
        todo!("walk function closure creation")
    }

    fn visit_method_call(&mut self, node: &mut MethodCallExpression) {
        todo!("walk method call")
    }

    fn visit_method_closure_creation(&mut self, node: &mut MethodClosureCreationExpression) {
        todo!("walk method closure creation")
    }

    fn visit_nullsafe_method_call(&mut self, node: &mut NullsafeMethodCallExpression) {
        todo!("walk nullsafe method call")
    }

    fn visit_static_method_call(&mut self, node: &mut StaticMethodCallExpression) {
        todo!("walk static method call")
    }

    fn visit_static_variable_method_call(&mut self, node: &mut StaticVariableMethodCallExpression) {
        todo!("walk static variable method call")
    }

    fn visit_static_method_closure_creation(&mut self, node: &mut StaticMethodClosureCreationExpression) {
        todo!("walk static method closure creation")
    }

    fn visit_static_variable_method_closure_creation(&mut self, node: &mut StaticVariableMethodClosureCreationExpression) {
        todo!("walk static variable method closure creation")
    }

    fn visit_property_fetch(&mut self, node: &mut PropertyFetchExpression) {
        todo!("walk property fetch")
    }

    fn visit_nullsafe_property_fetch(&mut self, node: &mut NullsafePropertyFetchExpression) {
        todo!("walk nullsafe property fetch")
    }

    fn visit_static_property_fetch(&mut self, node: &mut StaticPropertyFetchExpression) {
        todo!("walk static property fetch")
    }

    fn visit_constant_fetch(&mut self, node: &mut ConstantFetchExpression) {
        todo!("walk constant fetch")
    }

    fn visit_static_expr(&mut self) {}
    fn visit_self_expr(&mut self) {}
    fn visit_parent_expr(&mut self) {}

    fn visit_short_array(&mut self, node: &mut ShortArrayExpression) {
        todo!("walk short array")
    }

    fn visit_array(&mut self, node: &mut ArrayExpression) {
        todo!("walk array")
    }

    fn visit_list(&mut self, node: &mut ListExpression) {
        todo!("walk list")
    }

    fn visit_closure(&mut self, node: &mut ClosureExpression) {
        todo!("walk closure")
    }

    fn visit_arrow_function(&mut self, node: &mut ArrowFunctionExpression) {
        todo!("walk arrow function")
    }

    fn visit_new(&mut self, node: &mut NewExpression) {
        todo!("walk new")
    }

    fn visit_interpolated_string(&mut self, node: &mut InterpolatedStringExpression) {
        todo!("walk interpolated string")
    }

    fn visit_heredoc(&mut self, node: &mut HeredocExpression) {
        todo!("walk heredoc")
    }

    fn visit_nowdoc(&mut self, node: &mut NowdocExpression) {
        todo!("walk nowdoc")
    }

    fn visit_shell_exec(&mut self, node: &mut ShellExecExpression) {
        todo!("walk shell exec")
    }

    fn visit_anonymous_class(&mut self, node: &mut AnonymousClassExpression) {
        todo!("walk anonymous class")
    }

    fn visit_bool(&mut self, node: &mut BoolExpression) {
        todo!("walk bool")
    }

    fn visit_array_index(&mut self, node: &mut ArrayIndexExpression) {
        todo!("walk array index")
    }

    fn visit_null_expr(&mut self) {}

    fn visit_magic_constant(&mut self, node: &mut MagicConstantExpression) {
        todo!("walk magic constant")
    }

    fn visit_short_ternary(&mut self, node: &mut ShortTernaryExpression) {
        todo!("walk short ternary")
    }

    fn visit_ternary(&mut self, node: &mut TernaryExpression) {
        todo!("walk ternary")
    }

    fn visit_coalesce(&mut self, node: &mut CoalesceExpression) {
        todo!("walk coalesce")
    }

    fn visit_clone(&mut self, node: &mut CloneExpression) {
        todo!("walk clone")
    }

    fn visit_match(&mut self, node: &mut MatchExpression) {
        todo!("walk match")
    }

    fn visit_throw(&mut self, node: &mut ThrowExpression) {
        todo!("walk throw")
    }

    fn visit_yield(&mut self, node: &mut YieldExpression) {
        todo!("walk yield")
    }

    fn visit_yield_from(&mut self, node: &mut YieldFromExpression) {
        todo!("walk yield from")
    }

    fn visit_cast(&mut self, node: &mut CastExpression) {
        todo!("walk cast")
    }

    fn visit_noop_expr(&mut self) {}
}