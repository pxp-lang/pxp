use crate::walk::*;
use pxp_ast::{
    arguments::{Argument, ArgumentList},
    classes::{
        AnonymousClassBody, AnonymousClassExpression, ClassBody, ClassExtends, ClassImplements,
        ClassStatement, ClassishMember,
    },
    constant::{ClassishConstant, ConstantEntry, ConstantStatement},
    control_flow::{
        IfStatement, IfStatementBody, IfStatementElse, IfStatementElseBlock, IfStatementElseIf,
        IfStatementElseIfBlock,
    },
    declares::{DeclareBody, DeclareEntry, DeclareStatement},
    enums::{
        BackedEnumCase, BackedEnumMember, BackedEnumStatement, UnitEnumCase, UnitEnumMember,
        UnitEnumStatement,
    },
    functions::{
        AbstractConstructor, AbstractMethod, ArrowFunctionExpression, ClosureExpression,
        ConcreteConstructor, ConcreteMethod, ConstructorParameter, ConstructorParameterList,
        FunctionBody, FunctionParameter, FunctionParameterList, FunctionStatement, MethodBody,
        ReturnType,
    },
    goto::{GotoStatement, LabelStatement},
    identifiers::{DynamicIdentifier, Identifier, SimpleIdentifier},
    interfaces::{InterfaceBody, InterfaceExtends, InterfaceStatement},
    literals::Literal,
    loops::{
        BreakStatement, ContinueStatement, DoWhileStatement, ForStatement, ForStatementBody,
        ForStatementIterator, ForeachStatement, ForeachStatementBody, ForeachStatementIterator,
        Level, WhileStatement, WhileStatementBody,
    },
    namespaces::{BracedNamespace, NamespaceStatement, UnbracedNamespace},
    operators::{
        ArithmeticOperationExpression, AssignmentOperationExpression, BitwiseOperationExpression,
        ComparisonOperationExpression, LogicalOperationExpression,
    },
    properties::{Property, PropertyEntry, VariableProperty},
    traits::{TraitBody, TraitStatement, TraitUsage, TraitUsageAdaptation},
    try_block::{CatchBlock, FinallyBlock, TryStatement},
    variables::{BracedVariableVariable, SimpleVariable, Variable, VariableVariable},
    ArrayExpression, ArrayIndexExpression, ArrayItem, BlockStatement, BoolExpression, Case,
    CastExpression, CloneExpression, ClosingTagStatement, CoalesceExpression, ConcatExpression,
    ConstantFetchExpression, DefaultMatchArm, DieExpression, EchoOpeningTagStatement,
    EchoStatement, EmptyExpression, ErrorSuppressExpression, EvalExpression, ExitExpression,
    Expression, ExpressionStatement, ExpressionStringPart, FullOpeningTagStatement,
    FunctionCallExpression, FunctionClosureCreationExpression, GlobalStatement, GroupUseStatement,
    HaltCompilerStatement, HeredocExpression, IncludeExpression, IncludeOnceExpression,
    InlineHtmlStatement, InstanceofExpression, InterpolatedStringExpression, IssetExpression,
    ListEntry, ListExpression, LiteralStringPart, MagicConstantExpression, MatchArm,
    MatchExpression, MethodCallExpression, MethodClosureCreationExpression, NewExpression,
    NowdocExpression, NullsafeMethodCallExpression, NullsafePropertyFetchExpression,
    ParenthesizedExpression, PrintExpression, PropertyFetchExpression, ReferenceExpression,
    RequireExpression, RequireOnceExpression, ReturnStatement, ShellExecExpression,
    ShortArrayExpression, ShortOpeningTagStatement, ShortTernaryExpression, Statement,
    StaticMethodCallExpression, StaticMethodClosureCreationExpression,
    StaticPropertyFetchExpression, StaticStatement, StaticVar, StaticVariableMethodCallExpression,
    StaticVariableMethodClosureCreationExpression, StringPart, SwitchStatement, TernaryExpression,
    ThrowExpression, UnsetExpression, Use, UseStatement, YieldExpression, YieldFromExpression, data_type::DataType,
};
use pxp_span::Span;
use pxp_syntax::comments::Comment;
use pxp_type::Type;

pub trait VisitorMut {
    fn visit(&mut self, node: &mut [Statement]) {
        walk_mut(self, node);
    }

    fn visit_statement(&mut self, node: &mut Statement) {
        walk_statement_mut(self, node)
    }

    fn visit_expression(&mut self, node: &mut Expression) {
        walk_expression_mut(self, node)
    }

    fn visit_full_opening_tag(&mut self, _: &mut FullOpeningTagStatement) {}
    fn visit_short_opening_tag(&mut self, _: &mut ShortOpeningTagStatement) {}
    fn visit_echo_opening_tag(&mut self, _: &mut EchoOpeningTagStatement) {}
    fn visit_closing_tag(&mut self, _: &mut ClosingTagStatement) {}

    fn visit_inline_html(&mut self, _: &mut InlineHtmlStatement) {}
    fn visit_halt_compiler(&mut self, _: &mut HaltCompilerStatement) {}

    fn visit_label(&mut self, node: &mut LabelStatement) {
        walk_label_mut(self, node)
    }

    fn visit_goto(&mut self, node: &mut GotoStatement) {
        walk_goto_mut(self, node)
    }

    fn visit_static(&mut self, node: &mut StaticStatement) {
        walk_static_mut(self, node)
    }

    fn visit_static_var(&mut self, var: &mut StaticVar) {
        walk_static_var_mut(self, var)
    }

    fn visit_global(&mut self, node: &mut GlobalStatement) {
        walk_global_mut(self, node);
    }

    fn visit_do_while(&mut self, node: &mut DoWhileStatement) {
        walk_do_while_mut(self, node)
    }

    fn visit_while(&mut self, node: &mut WhileStatement) {
        walk_while_mut(self, node)
    }

    fn visit_while_statement_body(&mut self, node: &mut WhileStatementBody) {
        walk_while_statement_body_mut(self, node)
    }

    fn visit_for(&mut self, node: &mut ForStatement) {
        walk_for_mut(self, node)
    }

    fn visit_for_statement_iterator(&mut self, node: &mut ForStatementIterator) {
        walk_for_statement_iterator_mut(self, node);
    }

    fn visit_for_statement_body(&mut self, node: &mut ForStatementBody) {
        walk_for_statement_body_mut(self, node);
    }

    fn visit_foreach(&mut self, node: &mut ForeachStatement) {
        walk_foreach_mut(self, node)
    }

    fn visit_foreach_statement_iterator(&mut self, node: &mut ForeachStatementIterator) {
        walk_foreach_statement_iterator_mut(self, node)
    }

    fn visit_foreach_statement_body(&mut self, node: &mut ForeachStatementBody) {
        walk_foreach_statement_body_mut(self, node)
    }

    fn visit_if(&mut self, node: &mut IfStatement) {
        walk_if_mut(self, node)
    }

    fn visit_if_statement_body(&mut self, node: &mut IfStatementBody) {
        walk_if_statement_body_mut(self, node)
    }

    fn visit_if_statement_elseif(&mut self, node: &mut IfStatementElseIf) {
        walk_if_statement_elseif_mut(self, node)
    }

    fn visit_if_statement_else(&mut self, node: &mut IfStatementElse) {
        walk_if_statement_else_mut(self, node)
    }

    fn visit_if_statement_elseif_block(&mut self, node: &mut IfStatementElseIfBlock) {
        walk_if_statement_elseif_block_mut(self, node)
    }

    fn visit_if_statement_else_block(&mut self, node: &mut IfStatementElseBlock) {
        walk_if_statement_else_block_mut(self, node)
    }

    fn visit_switch(&mut self, node: &mut SwitchStatement) {
        walk_switch_mut(self, node)
    }

    fn visit_switch_case(&mut self, node: &mut Case) {
        walk_switch_case_mut(self, node)
    }

    fn visit_level(&mut self, node: &mut Level) {
        walk_level_mut(self, node)
    }

    fn visit_break(&mut self, node: &mut BreakStatement) {
        walk_break_mut(self, node)
    }

    fn visit_continue(&mut self, node: &mut ContinueStatement) {
        walk_continue_mut(self, node)
    }

    fn visit_constant(&mut self, node: &mut ConstantStatement) {
        walk_constant_mut(self, node)
    }

    fn visit_constant_entry(&mut self, node: &mut ConstantEntry) {
        walk_constant_entry_mut(self, node)
    }

    fn visit_function(&mut self, node: &mut FunctionStatement) {
        walk_function_mut(self, node)
    }

    fn visit_function_parameter_list(&mut self, node: &mut FunctionParameterList) {
        walk_function_parameter_list_mut(self, node)
    }

    fn visit_function_parameter(&mut self, node: &mut FunctionParameter) {
        walk_function_parameter_mut(self, node)
    }

    fn visit_function_body(&mut self, node: &mut FunctionBody) {
        walk_function_body_mut(self, node)
    }

    fn visit_class(&mut self, node: &mut ClassStatement) {
        walk_class_mut(self, node)
    }

    fn visit_class_extends(&mut self, node: &mut ClassExtends) {
        walk_class_extends_mut(self, node)
    }

    fn visit_class_implements(&mut self, node: &mut ClassImplements) {
        walk_class_implements_mut(self, node)
    }

    fn visit_class_body(&mut self, node: &mut ClassBody) {
        walk_class_body_mut(self, node)
    }

    fn visit_classish_member(&mut self, node: &mut ClassishMember) {
        walk_classish_member_mut(self, node)
    }

    fn visit_classish_constant(&mut self, node: &mut ClassishConstant) {
        walk_classish_constant_mut(self, node)
    }

    fn visit_trait_usage(&mut self, node: &mut TraitUsage) {
        walk_trait_usage_mut(self, node)
    }

    fn visit_trait_usage_adaptation(&mut self, node: &mut TraitUsageAdaptation) {
        walk_trait_usage_adaptation_mut(self, node)
    }

    fn visit_property(&mut self, node: &mut Property) {
        walk_property_mut(self, node)
    }

    fn visit_property_entry(&mut self, node: &mut PropertyEntry) {
        walk_property_entry_mut(self, node)
    }

    fn visit_variable_property(&mut self, node: &mut VariableProperty) {
        walk_variable_property_mut(self, node)
    }

    fn visit_abstract_method(&mut self, node: &mut AbstractMethod) {
        walk_abstract_method_mut(self, node)
    }

    fn visit_abstract_constructor(&mut self, node: &mut AbstractConstructor) {
        walk_abstract_constructor_mut(self, node)
    }

    fn visit_constructor_parameter_list(&mut self, node: &mut ConstructorParameterList) {
        walk_constructor_parameter_list_mut(self, node)
    }

    fn visit_concrete_method(&mut self, node: &mut ConcreteMethod) {
        walk_concrete_method_mut(self, node)
    }

    fn visit_method_body(&mut self, node: &mut MethodBody) {
        walk_method_body_mut(self, node)
    }

    fn visit_constructor_parameter(&mut self, node: &mut ConstructorParameter) {
        walk_constructor_parameter_mut(self, node)
    }

    fn visit_concrete_constructor(&mut self, node: &mut ConcreteConstructor) {
        walk_concrete_constructor_mut(self, node)
    }

    fn visit_interface(&mut self, node: &mut InterfaceStatement) {
        walk_interface_mut(self, node)
    }

    fn visit_interface_extends(&mut self, node: &mut InterfaceExtends) {
        walk_interface_extends_mut(self, node)
    }

    fn visit_interface_body(&mut self, node: &mut InterfaceBody) {
        walk_interface_body_mut(self, node)
    }

    fn visit_trait(&mut self, node: &mut TraitStatement) {
        walk_trait_mut(self, node)
    }

    fn visit_trait_body(&mut self, node: &mut TraitBody) {
        walk_trait_body_mut(self, node)
    }

    fn visit_echo(&mut self, node: &mut EchoStatement) {
        walk_echo_mut(self, node)
    }

    fn visit_expression_stmt(&mut self, node: &mut ExpressionStatement) {
        walk_expression_stmt_mut(self, node)
    }

    fn visit_return(&mut self, node: &mut ReturnStatement) {
        walk_return_mut(self, node)
    }

    fn visit_namespace(&mut self, node: &mut NamespaceStatement) {
        walk_namespace_mut(self, node)
    }

    fn visit_unbraced_namespace(&mut self, node: &mut UnbracedNamespace) {
        walk_unbraced_namespace_mut(self, node)
    }

    fn visit_braced_namespace(&mut self, node: &mut BracedNamespace) {
        walk_braced_namespace_mut(self, node)
    }

    fn visit_use(&mut self, node: &mut UseStatement) {
        walk_use_mut(self, node)
    }

    fn visit_use_use(&mut self, node: &mut Use) {
        walk_use_use_mut(self, node)
    }

    fn visit_group_use(&mut self, node: &mut GroupUseStatement) {
        walk_group_use_mut(self, node)
    }

    fn visit_comment_stmt(&mut self, _: &mut Comment) {}

    fn visit_try(&mut self, node: &mut TryStatement) {
        walk_try_mut(self, node)
    }

    fn visit_catch_block(&mut self, node: &mut CatchBlock) {
        walk_catch_block_mut(self, node)
    }

    fn visit_finally_block(&mut self, node: &mut FinallyBlock) {
        walk_finally_block_mut(self, node)
    }

    fn visit_unit_enum(&mut self, node: &mut UnitEnumStatement) {
        walk_unit_enum_mut(self, node)
    }

    fn visit_unit_enum_member(&mut self, node: &mut UnitEnumMember) {
        walk_unit_enum_member_mut(self, node)
    }

    fn visit_unit_enum_case(&mut self, node: &mut UnitEnumCase) {
        walk_unit_enum_case_mut(self, node)
    }

    fn visit_backed_enum(&mut self, node: &mut BackedEnumStatement) {
        walk_backed_enum_mut(self, node)
    }

    fn visit_backed_enum_member(&mut self, node: &mut BackedEnumMember) {
        walk_backed_enum_member_mut(self, node)
    }

    fn visit_backed_enum_case(&mut self, node: &mut BackedEnumCase) {
        walk_backed_enum_case_mut(self, node)
    }

    fn visit_block(&mut self, node: &mut BlockStatement) {
        walk_mut(self, &mut node.statements);
    }

    fn visit_declare(&mut self, node: &mut DeclareStatement) {
        walk_declare_mut(self, node)
    }

    fn visit_declare_entry(&mut self, node: &mut DeclareEntry) {
        walk_declare_entry_mut(self, node)
    }

    fn visit_declare_body(&mut self, node: &mut DeclareBody) {
        walk_declare_body_mut(self, node)
    }

    fn visit_noop(&mut self, _: Span) {}

    fn visit_missing_expr(&mut self) {}

    fn visit_argument_list(&mut self, node: &mut ArgumentList) {
        walk_argument_list_mut(self, node)
    }

    fn visit_argument(&mut self, node: &mut Argument) {
        walk_argument_mut(self, node)
    }

    fn visit_eval(&mut self, node: &mut EvalExpression) {
        walk_eval_mut(self, node)
    }

    fn visit_empty(&mut self, node: &mut EmptyExpression) {
        walk_empty_mut(self, node)
    }

    fn visit_die(&mut self, node: &mut DieExpression) {
        walk_die_mut(self, node)
    }

    fn visit_exit(&mut self, node: &mut ExitExpression) {
        walk_exit_mut(self, node)
    }

    fn visit_isset(&mut self, node: &mut IssetExpression) {
        walk_isset_mut(self, node)
    }

    fn visit_unset(&mut self, node: &mut UnsetExpression) {
        walk_unset_mut(self, node)
    }

    fn visit_print(&mut self, node: &mut PrintExpression) {
        walk_print_mut(self, node)
    }

    fn visit_literal(&mut self, _: &mut Literal) {}

    fn visit_arithmetic_operation(&mut self, node: &mut ArithmeticOperationExpression) {
        walk_arithmetic_operation_mut(self, node)
    }

    fn visit_assignment_operation(&mut self, node: &mut AssignmentOperationExpression) {
        walk_assignment_operation_mut(self, node)
    }

    fn visit_bitwise_operation(&mut self, node: &mut BitwiseOperationExpression) {
        walk_bitwise_operation_mut(self, node)
    }

    fn visit_comparison_operation(&mut self, node: &mut ComparisonOperationExpression) {
        walk_comparison_operation_mut(self, node)
    }

    fn visit_logical_operation(&mut self, node: &mut LogicalOperationExpression) {
        walk_logical_operation_mut(self, node)
    }

    fn visit_concat(&mut self, node: &mut ConcatExpression) {
        walk_concat_mut(self, node)
    }

    fn visit_instanceof(&mut self, node: &mut InstanceofExpression) {
        walk_instanceof_mut(self, node)
    }

    fn visit_reference(&mut self, node: &mut ReferenceExpression) {
        walk_reference_mut(self, node)
    }

    fn visit_parenthesized(&mut self, node: &mut ParenthesizedExpression) {
        walk_parenthesized_mut(self, node)
    }

    fn visit_error_suppress(&mut self, node: &mut ErrorSuppressExpression) {
        walk_error_suppress_mut(self, node)
    }

    fn visit_identifier(&mut self, node: &mut Identifier) {
        walk_identifier_mut(self, node)
    }

    fn visit_simple_identifier(&mut self, _: &mut SimpleIdentifier) {}

    fn visit_dynamic_identifier(&mut self, node: &mut DynamicIdentifier) {
        walk_dynamic_identifier_mut(self, node)
    }

    fn visit_variable(&mut self, node: &mut Variable) {
        walk_variable_mut(self, node)
    }

    fn visit_simple_variable(&mut self, _: &mut SimpleVariable) {}

    fn visit_variable_variable(&mut self, node: &mut VariableVariable) {
        walk_variable_variable_mut(self, node);
    }

    fn visit_braced_variable_variable(&mut self, node: &mut BracedVariableVariable) {
        walk_braced_variable_variable_mut(self, node);
    }

    fn visit_include(&mut self, node: &mut IncludeExpression) {
        walk_include_mut(self, node);
    }

    fn visit_include_once(&mut self, node: &mut IncludeOnceExpression) {
        walk_include_once_mut(self, node);
    }

    fn visit_require(&mut self, node: &mut RequireExpression) {
        walk_require_mut(self, node);
    }

    fn visit_require_once(&mut self, node: &mut RequireOnceExpression) {
        walk_require_once_mut(self, node);
    }

    fn visit_function_call(&mut self, node: &mut FunctionCallExpression) {
        walk_function_call_mut(self, node);
    }

    fn visit_function_closure_creation(&mut self, node: &mut FunctionClosureCreationExpression) {
        walk_function_closure_creation_mut(self, node);
    }

    fn visit_method_call(&mut self, node: &mut MethodCallExpression) {
        walk_method_call_mut(self, node);
    }

    fn visit_method_closure_creation(&mut self, node: &mut MethodClosureCreationExpression) {
        walk_method_closure_creation_mut(self, node);
    }

    fn visit_nullsafe_method_call(&mut self, node: &mut NullsafeMethodCallExpression) {
        walk_nullsafe_method_call_mut(self, node);
    }

    fn visit_static_method_call(&mut self, node: &mut StaticMethodCallExpression) {
        walk_static_method_call_mut(self, node);
    }

    fn visit_static_variable_method_call(&mut self, node: &mut StaticVariableMethodCallExpression) {
        walk_static_variable_method_call_mut(self, node);
    }

    fn visit_static_method_closure_creation(
        &mut self,
        node: &mut StaticMethodClosureCreationExpression,
    ) {
        walk_static_method_closure_creation_mut(self, node);
    }

    fn visit_static_variable_method_closure_creation(
        &mut self,
        node: &mut StaticVariableMethodClosureCreationExpression,
    ) {
        walk_static_variable_method_closure_creation_mut(self, node);
    }

    fn visit_property_fetch(&mut self, node: &mut PropertyFetchExpression) {
        walk_property_fetch_mut(self, node);
    }

    fn visit_nullsafe_property_fetch(&mut self, node: &mut NullsafePropertyFetchExpression) {
        walk_nullsafe_property_fetch_mut(self, node);
    }

    fn visit_static_property_fetch(&mut self, node: &mut StaticPropertyFetchExpression) {
        walk_static_property_fetch_mut(self, node);
    }

    fn visit_constant_fetch(&mut self, node: &mut ConstantFetchExpression) {
        walk_constant_fetch_mut(self, node);
    }

    fn visit_static_expr(&mut self) {}
    fn visit_self_expr(&mut self) {}
    fn visit_parent_expr(&mut self) {}

    fn visit_array_item(&mut self, node: &mut ArrayItem) {
        walk_array_item_mut(self, node);
    }

    fn visit_short_array(&mut self, node: &mut ShortArrayExpression) {
        walk_short_array_mut(self, node);
    }

    fn visit_array(&mut self, node: &mut ArrayExpression) {
        walk_array_mut(self, node);
    }

    fn visit_list(&mut self, node: &mut ListExpression) {
        walk_list_mut(self, node);
    }

    fn visit_list_entry(&mut self, node: &mut ListEntry) {
        walk_list_entry_mut(self, node);
    }

    fn visit_closure(&mut self, node: &mut ClosureExpression) {
        walk_closure_mut(self, node);
    }

    fn visit_arrow_function(&mut self, node: &mut ArrowFunctionExpression) {
        walk_arrow_function_mut(self, node);
    }

    fn visit_new(&mut self, node: &mut NewExpression) {
        walk_new_mut(self, node);
    }

    fn visit_interpolated_string(&mut self, node: &mut InterpolatedStringExpression) {
        walk_interpolated_string_mut(self, node);
    }

    fn visit_string_part(&mut self, node: &mut StringPart) {
        walk_string_part_mut(self, node);
    }

    fn visit_literal_string_part(&mut self, _: &mut LiteralStringPart) {}

    fn visit_expression_string_part(&mut self, node: &mut ExpressionStringPart) {
        walk_expression_string_part_mut(self, node);
    }

    fn visit_heredoc(&mut self, node: &mut HeredocExpression) {
        walk_heredoc_mut(self, node);
    }

    fn visit_nowdoc(&mut self, _: &mut NowdocExpression) {}

    fn visit_shell_exec(&mut self, node: &mut ShellExecExpression) {
        walk_shell_exec_mut(self, node);
    }

    fn visit_anonymous_class(&mut self, node: &mut AnonymousClassExpression) {
        walk_anonymous_class_mut(self, node);
    }

    fn visit_anonymous_class_body(&mut self, node: &mut AnonymousClassBody) {
        walk_anonymous_class_body_mut(self, node);
    }

    fn visit_bool(&mut self, _: &mut BoolExpression) {}

    fn visit_array_index(&mut self, node: &mut ArrayIndexExpression) {
        walk_array_index_mut(self, node);
    }

    fn visit_null_expr(&mut self) {}

    fn visit_magic_constant(&mut self, _: &mut MagicConstantExpression) {}

    fn visit_short_ternary(&mut self, node: &mut ShortTernaryExpression) {
        walk_short_ternary_mut(self, node);
    }

    fn visit_ternary(&mut self, node: &mut TernaryExpression) {
        walk_ternary_mut(self, node);
    }

    fn visit_coalesce(&mut self, node: &mut CoalesceExpression) {
        walk_coalesce_mut(self, node);
    }

    fn visit_clone(&mut self, node: &mut CloneExpression) {
        walk_clone_mut(self, node);
    }

    fn visit_match(&mut self, node: &mut MatchExpression) {
        walk_match_mut(self, node);
    }

    fn visit_match_arm(&mut self, node: &mut MatchArm) {
        walk_match_arm_mut(self, node);
    }

    fn visit_default_match_arm(&mut self, node: &mut DefaultMatchArm) {
        walk_default_match_arm_mut(self, node);
    }

    fn visit_throw(&mut self, node: &mut ThrowExpression) {
        walk_throw_mut(self, node);
    }

    fn visit_yield(&mut self, node: &mut YieldExpression) {
        walk_yield_mut(self, node);
    }

    fn visit_yield_from(&mut self, node: &mut YieldFromExpression) {
        walk_yield_from_mut(self, node);
    }

    fn visit_cast(&mut self, node: &mut CastExpression) {
        walk_cast_mut(self, node);
    }

    fn visit_noop_expr(&mut self) {}

    fn visit_return_type(&mut self, node: &mut ReturnType) {
        walk_return_type_mut(self, node);
    }

    fn visit_data_type(&mut self, node: &mut DataType) {
        walk_data_type_mut(self, node);
    }

    fn visit_type(&mut self, node: &mut Type) {
        walk_type_mut(self, node);
    }
}
