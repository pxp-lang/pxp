// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.

use crate::visitor_mut::VisitorMut;
use pxp_ast::*;

pub fn walk_mut<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut [Statement]) {
    for statement in node {
        visitor.visit_statement(statement);
    }
}

pub fn walk_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Statement) {
    visitor.visit_statement_kind(&mut node.kind);
}

pub fn walk_statement_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut StatementKind) {}

pub fn walk_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Expression) {
    visitor.visit_expression_kind(&mut node.kind);
}

pub fn walk_expression_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ExpressionKind) {}

pub fn walk_expression_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ExpressionStatement,
) {
    visitor.visit_expression(&mut node.expression);
    visitor.visit_ending(&mut node.ending);
}

pub fn walk_global_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut GlobalStatement) {
    for item in &mut node.variables {
        visitor.visit_variable(item);
    }
}

pub fn walk_block_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut BlockStatement) {
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_cast_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut CastKind) {}

pub fn walk_case<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Case) {
    if let Some(item) = &mut node.condition {
        visitor.visit_expression(item);
    }
    visitor.visit(&mut node.body);
}

pub fn walk_use<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Use) {
    visitor.visit_name(&mut node.name);
    if let Some(item) = &mut node.alias {
        visitor.visit_simple_identifier(item);
    }
    if let Some(item) = &mut node.kind {
        visitor.visit_use_kind(item);
    }
}

pub fn walk_use_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UseKind) {}

pub fn walk_eval_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut EvalExpression) {
    visitor.visit_single_argument(&mut node.argument);
}

pub fn walk_empty_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut EmptyExpression) {
    visitor.visit_single_argument(&mut node.argument);
}

pub fn walk_die_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut DieExpression) {
    if let Some(item) = &mut node.argument {
        visitor.visit_single_argument(item);
    }
}

pub fn walk_exit_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ExitExpression) {
    if let Some(item) = &mut node.argument {
        visitor.visit_single_argument(item);
    }
}

pub fn walk_isset_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut IssetExpression) {
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_unset_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UnsetExpression) {
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_print_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut PrintExpression) {
    if let Some(item) = &mut node.value {
        visitor.visit_expression(item);
    }
    if let Some(item) = &mut node.argument {
        visitor.visit_single_argument(item);
    }
}

pub fn walk_concat_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConcatExpression,
) {
    visitor.visit_expression(&mut node.left);
    visitor.visit_expression(&mut node.right);
}

pub fn walk_instanceof_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut InstanceofExpression,
) {
    visitor.visit_expression(&mut node.left);
    visitor.visit_expression(&mut node.right);
}

pub fn walk_reference_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ReferenceExpression,
) {
    visitor.visit_expression(&mut node.right);
}

pub fn walk_parenthesized_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ParenthesizedExpression,
) {
    visitor.visit_expression(&mut node.expr);
}

pub fn walk_error_suppress_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ErrorSuppressExpression,
) {
    visitor.visit_expression(&mut node.expr);
}

pub fn walk_include_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut IncludeExpression,
) {
    visitor.visit_expression(&mut node.path);
}

pub fn walk_include_once_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut IncludeOnceExpression,
) {
    visitor.visit_expression(&mut node.path);
}

pub fn walk_require_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut RequireExpression,
) {
    visitor.visit_expression(&mut node.path);
}

pub fn walk_require_once_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut RequireOnceExpression,
) {
    visitor.visit_expression(&mut node.path);
}

pub fn walk_function_call_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut FunctionCallExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_function_closure_creation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut FunctionClosureCreationExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_argument_placeholder(&mut node.placeholder);
}

pub fn walk_method_call_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut MethodCallExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_expression(&mut node.method);
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_method_closure_creation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut MethodClosureCreationExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_expression(&mut node.method);
    visitor.visit_argument_placeholder(&mut node.placeholder);
}

pub fn walk_nullsafe_method_call_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut NullsafeMethodCallExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_expression(&mut node.method);
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_static_method_call_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut StaticMethodCallExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_identifier(&mut node.method);
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_static_variable_method_call_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut StaticVariableMethodCallExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_variable(&mut node.method);
    visitor.visit_argument_list(&mut node.arguments);
}

pub fn walk_static_method_closure_creation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut StaticMethodClosureCreationExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_identifier(&mut node.method);
    visitor.visit_argument_placeholder(&mut node.placeholder);
}

pub fn walk_static_variable_method_closure_creation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut StaticVariableMethodClosureCreationExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_variable(&mut node.method);
    visitor.visit_argument_placeholder(&mut node.placeholder);
}

pub fn walk_property_fetch_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut PropertyFetchExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_expression(&mut node.property);
}

pub fn walk_nullsafe_property_fetch_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut NullsafePropertyFetchExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_expression(&mut node.property);
}

pub fn walk_static_property_fetch_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut StaticPropertyFetchExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_variable(&mut node.property);
}

pub fn walk_constant_fetch_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConstantFetchExpression,
) {
    visitor.visit_expression(&mut node.target);
    visitor.visit_identifier(&mut node.constant);
}

pub fn walk_short_array_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ShortArrayExpression,
) {
    for item in &mut node.items.inner {
        visitor.visit_array_item(item);
    }
}

pub fn walk_array_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ArrayExpression) {
    for item in &mut node.items.inner {
        visitor.visit_array_item(item);
    }
}

pub fn walk_list_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ListExpression) {
    for item in &mut node.items {
        visitor.visit_list_entry(item);
    }
}

pub fn walk_new_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut NewExpression) {
    visitor.visit_expression(&mut node.target);
    if let Some(item) = &mut node.arguments {
        visitor.visit_argument_list(item);
    }
}

pub fn walk_interpolated_string_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut InterpolatedStringExpression,
) {
    for item in &mut node.parts {
        visitor.visit_string_part(item);
    }
}

pub fn walk_heredoc_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut HeredocExpression,
) {
    for item in &mut node.parts {
        visitor.visit_string_part(item);
    }
}

pub fn walk_shell_exec_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ShellExecExpression,
) {
    for item in &mut node.parts {
        visitor.visit_string_part(item);
    }
}

pub fn walk_array_index_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ArrayIndexExpression,
) {
    visitor.visit_expression(&mut node.array);
    if let Some(item) = &mut node.index {
        visitor.visit_expression(item);
    }
}

pub fn walk_short_ternary_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ShortTernaryExpression,
) {
    visitor.visit_expression(&mut node.condition);
    visitor.visit_expression(&mut node.r#else);
}

pub fn walk_ternary_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut TernaryExpression,
) {
    visitor.visit_expression(&mut node.condition);
    visitor.visit_expression(&mut node.then);
    visitor.visit_expression(&mut node.r#else);
}

pub fn walk_coalesce_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut CoalesceExpression,
) {
    visitor.visit_expression(&mut node.lhs);
    visitor.visit_expression(&mut node.rhs);
}

pub fn walk_clone_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut CloneExpression) {
    visitor.visit_expression(&mut node.target);
}

pub fn walk_match_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut MatchExpression) {
    visitor.visit_expression(&mut node.condition);
    if let Some(item) = &mut node.default {
        visitor.visit_default_match_arm(item);
    }
    for item in &mut node.arms {
        visitor.visit_match_arm(item);
    }
}

pub fn walk_throw_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ThrowExpression) {
    visitor.visit_expression(&mut node.value);
}

pub fn walk_yield_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut YieldExpression) {
    if let Some(item) = &mut node.key {
        visitor.visit_expression(item);
    }
    if let Some(item) = &mut node.value {
        visitor.visit_expression(item);
    }
}

pub fn walk_yield_from_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut YieldFromExpression,
) {
    visitor.visit_expression(&mut node.value);
}

pub fn walk_cast_expression<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut CastExpression) {
    visitor.visit_cast_kind(&mut node.kind);
    visitor.visit_expression(&mut node.value);
}

pub fn walk_default_match_arm<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut DefaultMatchArm) {
    visitor.visit_expression(&mut node.body);
}

pub fn walk_match_arm<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut MatchArm) {
    for item in &mut node.conditions {
        visitor.visit_expression(item);
    }
    visitor.visit_expression(&mut node.body);
}

pub fn walk_string_part<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut StringPart) {}

pub fn walk_literal_string_part<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut LiteralStringPart,
) {
}

pub fn walk_expression_string_part<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ExpressionStringPart,
) {
    visitor.visit_expression(&mut node.expression);
}

pub fn walk_array_item<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ArrayItem) {}

pub fn walk_list_entry<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ListEntry) {}

pub fn walk_positional_argument<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut PositionalArgument,
) {
    visitor.visit_expression(&mut node.value);
}

pub fn walk_named_argument<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut NamedArgument) {
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_expression(&mut node.value);
}

pub fn walk_argument<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Argument) {}

pub fn walk_argument_list<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ArgumentList) {
    for item in &mut node.arguments {
        visitor.visit_argument(item);
    }
}

pub fn walk_single_argument<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut SingleArgument) {
    if let Some(item) = &mut node.argument {
        visitor.visit_argument(item);
    }
}

pub fn walk_argument_placeholder<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ArgumentPlaceholder,
) {
}

pub fn walk_attribute<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Attribute) {
    visitor.visit_simple_identifier(&mut node.name);
    if let Some(item) = &mut node.arguments {
        visitor.visit_argument_list(item);
    }
}

pub fn walk_attribute_group<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut AttributeGroup) {
    for item in &mut node.members {
        visitor.visit_attribute(item);
    }
}

pub fn walk_class_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ClassBody) {
    for item in &mut node.members {
        visitor.visit_classish_member(item);
    }
}

pub fn walk_class_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ClassStatement) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_class_modifier_group(&mut node.modifiers);
    visitor.visit_name(&mut node.name);
    if let Some(item) = &mut node.extends {
        visitor.visit_class_extends(item);
    }
    if let Some(item) = &mut node.implements {
        visitor.visit_class_implements(item);
    }
    visitor.visit_class_body(&mut node.body);
}

pub fn walk_anonymous_class_body<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut AnonymousClassBody,
) {
    for item in &mut node.members {
        visitor.visit_classish_member(item);
    }
}

pub fn walk_anonymous_class_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut AnonymousClassExpression,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    if let Some(item) = &mut node.extends {
        visitor.visit_class_extends(item);
    }
    if let Some(item) = &mut node.implements {
        visitor.visit_class_implements(item);
    }
    visitor.visit_anonymous_class_body(&mut node.body);
}

pub fn walk_class_extends<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ClassExtends) {
    visitor.visit_name(&mut node.parent);
}

pub fn walk_class_implements<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ClassImplements) {
    for item in &mut node.interfaces.inner {
        visitor.visit_simple_identifier(item);
    }
}

pub fn walk_classish_member<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ClassishMember) {}

pub fn walk_constant_entry<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ConstantEntry) {
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_expression(&mut node.value);
}

pub fn walk_constant_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConstantStatement,
) {
    for item in &mut node.entries {
        visitor.visit_constant_entry(item);
    }
}

pub fn walk_classish_constant<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ClassishConstant,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_constant_modifier_group(&mut node.modifiers);
    if let Some(item) = &mut node.data_type {
        visitor.visit_data_type(item);
    }
    for item in &mut node.entries {
        visitor.visit_constant_entry(item);
    }
}

pub fn walk_if_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut IfStatement) {
    visitor.visit_expression(&mut node.condition);
    visitor.visit_if_statement_body(&mut node.body);
}

pub fn walk_if_statement_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut IfStatementBody) {
}

pub fn walk_if_statement_else_if<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut IfStatementElseIf,
) {
    visitor.visit_expression(&mut node.condition);
    visitor.visit_statement(&mut node.statement);
}

pub fn walk_if_statement_else<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut IfStatementElse) {
    visitor.visit_statement(&mut node.statement);
}

pub fn walk_if_statement_else_if_block<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut IfStatementElseIfBlock,
) {
    visitor.visit_expression(&mut node.condition);
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_if_statement_else_block<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut IfStatementElseBlock,
) {
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_data_type<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut DataType) {}

pub fn walk_declare_entry<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut DeclareEntry) {
    visitor.visit_simple_identifier(&mut node.key);
    visitor.visit_literal(&mut node.value);
}

pub fn walk_declare_entry_group<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut DeclareEntryGroup,
) {
    for item in &mut node.entries {
        visitor.visit_declare_entry(item);
    }
}

pub fn walk_declare_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut DeclareBody) {}

pub fn walk_declare_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut DeclareStatement,
) {
    visitor.visit_declare_entry_group(&mut node.entries);
    visitor.visit_declare_body(&mut node.body);
}

pub fn walk_unit_enum_case<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UnitEnumCase) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_simple_identifier(&mut node.name);
}

pub fn walk_unit_enum_member<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UnitEnumMember) {}

pub fn walk_unit_enum_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UnitEnumBody) {
    for item in &mut node.members {
        visitor.visit_unit_enum_member(item);
    }
}

pub fn walk_unit_enum_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut UnitEnumStatement,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_name(&mut node.name);
    for item in &mut node.implements {
        visitor.visit_simple_identifier(item);
    }
    visitor.visit_unit_enum_body(&mut node.body);
}

pub fn walk_backed_enum_case<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut BackedEnumCase) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_expression(&mut node.value);
}

pub fn walk_backed_enum_member<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut BackedEnumMember,
) {
}

pub fn walk_backed_enum_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut BackedEnumBody) {
    for item in &mut node.members {
        visitor.visit_backed_enum_member(item);
    }
}

pub fn walk_backed_enum_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut BackedEnumStatement,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_name(&mut node.name);
    for item in &mut node.implements {
        visitor.visit_simple_identifier(item);
    }
    visitor.visit_backed_enum_body(&mut node.body);
}

pub fn walk_return_type<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ReturnType) {
    visitor.visit_data_type(&mut node.data_type);
}

pub fn walk_function_parameter<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut FunctionParameter,
) {
    visitor.visit_simple_variable(&mut node.name);
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    if let Some(item) = &mut node.data_type {
        visitor.visit_data_type(item);
    }
    if let Some(item) = &mut node.default {
        visitor.visit_expression(item);
    }
}

pub fn walk_function_parameter_list<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut FunctionParameterList,
) {
    for item in &mut node.parameters.inner {
        visitor.visit_function_parameter(item);
    }
}

pub fn walk_function_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut FunctionBody) {
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_function_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut FunctionStatement,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_function_parameter_list(&mut node.parameters);
    if let Some(item) = &mut node.return_type {
        visitor.visit_return_type(item);
    }
    visitor.visit_function_body(&mut node.body);
}

pub fn walk_closure_use_variable<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ClosureUseVariable,
) {
    visitor.visit_simple_variable(&mut node.variable);
}

pub fn walk_closure_use<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ClosureUse) {
    for item in &mut node.variables.inner {
        visitor.visit_closure_use_variable(item);
    }
}

pub fn walk_closure_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ClosureExpression,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_function_parameter_list(&mut node.parameters);
    if let Some(item) = &mut node.uses {
        visitor.visit_closure_use(item);
    }
    if let Some(item) = &mut node.return_type {
        visitor.visit_return_type(item);
    }
    visitor.visit_function_body(&mut node.body);
}

pub fn walk_arrow_function_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ArrowFunctionExpression,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_function_parameter_list(&mut node.parameters);
    if let Some(item) = &mut node.return_type {
        visitor.visit_return_type(item);
    }
    visitor.visit_expression(&mut node.body);
}

pub fn walk_constructor_parameter<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConstructorParameter,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_simple_variable(&mut node.name);
    if let Some(item) = &mut node.data_type {
        visitor.visit_data_type(item);
    }
    if let Some(item) = &mut node.default {
        visitor.visit_expression(item);
    }
    visitor.visit_promoted_property_modifier_group(&mut node.modifiers);
}

pub fn walk_constructor_parameter_list<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConstructorParameterList,
) {
    for item in &mut node.parameters.inner {
        visitor.visit_constructor_parameter(item);
    }
}

pub fn walk_abstract_constructor<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut AbstractConstructor,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_method_modifier_group(&mut node.modifiers);
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_constructor_parameter_list(&mut node.parameters);
}

pub fn walk_concrete_constructor<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConcreteConstructor,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_method_modifier_group(&mut node.modifiers);
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_constructor_parameter_list(&mut node.parameters);
    visitor.visit_method_body(&mut node.body);
}

pub fn walk_abstract_method<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut AbstractMethod) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_method_modifier_group(&mut node.modifiers);
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_function_parameter_list(&mut node.parameters);
    if let Some(item) = &mut node.return_type {
        visitor.visit_return_type(item);
    }
}

pub fn walk_concrete_method<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ConcreteMethod) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_method_modifier_group(&mut node.modifiers);
    visitor.visit_simple_identifier(&mut node.name);
    visitor.visit_function_parameter_list(&mut node.parameters);
    if let Some(item) = &mut node.return_type {
        visitor.visit_return_type(item);
    }
    visitor.visit_method_body(&mut node.body);
}

pub fn walk_method_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut MethodBody) {
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_label_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut LabelStatement) {
    visitor.visit_simple_identifier(&mut node.label);
}

pub fn walk_goto_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut GotoStatement) {
    visitor.visit_simple_identifier(&mut node.label);
}

pub fn walk_identifier<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Identifier) {}

pub fn walk_simple_identifier<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut SimpleIdentifier,
) {
}

pub fn walk_dynamic_identifier<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut DynamicIdentifier,
) {
    visitor.visit_expression(&mut node.expr);
}

pub fn walk_interface_extends<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut InterfaceExtends,
) {
    for item in &mut node.parents.inner {
        visitor.visit_name(item);
    }
}

pub fn walk_interface_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut InterfaceBody) {
    for item in &mut node.members {
        visitor.visit_classish_member(item);
    }
}

pub fn walk_interface_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut InterfaceStatement,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_name(&mut node.name);
    if let Some(item) = &mut node.extends {
        visitor.visit_interface_extends(item);
    }
    visitor.visit_interface_body(&mut node.body);
}

pub fn walk_literal<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Literal) {
    visitor.visit_literal_kind(&mut node.kind);
}

pub fn walk_literal_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut LiteralKind) {}

pub fn walk_foreach_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ForeachStatement,
) {
    visitor.visit_foreach_statement_iterator(&mut node.iterator);
    visitor.visit_foreach_statement_body(&mut node.body);
}

pub fn walk_foreach_statement_iterator<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ForeachStatementIterator,
) {
}

pub fn walk_foreach_statement_body<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ForeachStatementBody,
) {
}

pub fn walk_for_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ForStatement) {
    visitor.visit_for_statement_iterator(&mut node.iterator);
    visitor.visit_for_statement_body(&mut node.body);
}

pub fn walk_for_statement_iterator<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ForStatementIterator,
) {
    for item in &mut node.initializations.inner {
        visitor.visit_expression(item);
    }
    for item in &mut node.conditions.inner {
        visitor.visit_expression(item);
    }
    for item in &mut node.r#loop.inner {
        visitor.visit_expression(item);
    }
}

pub fn walk_for_statement_body<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ForStatementBody,
) {
}

pub fn walk_do_while_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut DoWhileStatement,
) {
    visitor.visit_statement(&mut node.body);
    visitor.visit_expression(&mut node.condition);
}

pub fn walk_while_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut WhileStatement) {
    visitor.visit_expression(&mut node.condition);
    visitor.visit_while_statement_body(&mut node.body);
}

pub fn walk_while_statement_body<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut WhileStatementBody,
) {
}

pub fn walk_level<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Level) {}

pub fn walk_break_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut BreakStatement) {
    if let Some(item) = &mut node.level {
        visitor.visit_level(item);
    }
    visitor.visit_ending(&mut node.ending);
}

pub fn walk_continue_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ContinueStatement,
) {
    if let Some(item) = &mut node.level {
        visitor.visit_level(item);
    }
    visitor.visit_ending(&mut node.ending);
}

pub fn walk_promoted_property_modifier_group<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut PromotedPropertyModifierGroup,
) {
    for item in &mut node.modifiers {
        visitor.visit_promoted_property_modifier(item);
    }
}

pub fn walk_property_modifier_group<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut PropertyModifierGroup,
) {
    for item in &mut node.modifiers {
        visitor.visit_property_modifier(item);
    }
}

pub fn walk_method_modifier_group<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut MethodModifierGroup,
) {
    for item in &mut node.modifiers {
        visitor.visit_method_modifier(item);
    }
}

pub fn walk_class_modifier_group<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ClassModifierGroup,
) {
    for item in &mut node.modifiers {
        visitor.visit_class_modifier(item);
    }
}

pub fn walk_constant_modifier_group<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ConstantModifierGroup,
) {
    for item in &mut node.modifiers {
        visitor.visit_constant_modifier(item);
    }
}

pub fn walk_unbraced_namespace<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut UnbracedNamespace,
) {
    visitor.visit_name(&mut node.name);
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_braced_namespace<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut BracedNamespace) {
    if let Some(item) = &mut node.name {
        visitor.visit_name(item);
    }
    visitor.visit_braced_namespace_body(&mut node.body);
}

pub fn walk_braced_namespace_body<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut BracedNamespaceBody,
) {
    for item in &mut node.statements {
        visitor.visit_statement(item);
    }
}

pub fn walk_namespace_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut NamespaceStatement,
) {
}

pub fn walk_arithmetic_operation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ArithmeticOperationExpression,
) {
}

pub fn walk_assignment_operation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut AssignmentOperationExpression,
) {
}

pub fn walk_bitwise_operation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut BitwiseOperationExpression,
) {
}

pub fn walk_comparison_operation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut ComparisonOperationExpression,
) {
}

pub fn walk_logical_operation_expression<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut LogicalOperationExpression,
) {
}

pub fn walk_name<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Name) {
    visitor.visit_name_kind(&mut node.kind);
}

pub fn walk_name_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut NameKind) {}

pub fn walk_special_name<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut SpecialName) {
    visitor.visit_special_name_kind(&mut node.kind);
}

pub fn walk_special_name_kind<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut SpecialNameKind) {
}

pub fn walk_unresolved_name<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UnresolvedName) {}

pub fn walk_resolved_name<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ResolvedName) {}

pub fn walk_property<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Property) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_property_modifier_group(&mut node.modifiers);
    if let Some(item) = &mut node.r#type {
        visitor.visit_data_type(item);
    }
    for item in &mut node.entries {
        visitor.visit_property_entry(item);
    }
}

pub fn walk_variable_property<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut VariableProperty,
) {
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    if let Some(item) = &mut node.r#type {
        visitor.visit_data_type(item);
    }
    for item in &mut node.entries {
        visitor.visit_property_entry(item);
    }
}

pub fn walk_property_entry<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut PropertyEntry) {}

pub fn walk_trait_body<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut TraitBody) {
    for item in &mut node.members {
        visitor.visit_classish_member(item);
    }
}

pub fn walk_trait_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut TraitStatement) {
    visitor.visit_name(&mut node.name);
    for item in &mut node.attributes {
        visitor.visit_attribute_group(item);
    }
    visitor.visit_trait_body(&mut node.body);
}

pub fn walk_trait_usage<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut TraitUsage) {
    for item in &mut node.traits {
        visitor.visit_simple_identifier(item);
    }
    for item in &mut node.adaptations {
        visitor.visit_trait_usage_adaptation(item);
    }
}

pub fn walk_trait_usage_adaptation<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut TraitUsageAdaptation,
) {
}

pub fn walk_catch_type<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut CatchType) {}

pub fn walk_try_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut TryStatement) {
    visitor.visit(&mut node.body);
    for item in &mut node.catches {
        visitor.visit_catch_block(item);
    }
    if let Some(item) = &mut node.finally {
        visitor.visit_finally_block(item);
    }
}

pub fn walk_catch_block<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut CatchBlock) {
    visitor.visit_catch_type(&mut node.types);
    if let Some(item) = &mut node.var {
        visitor.visit_simple_variable(item);
    }
    visitor.visit(&mut node.body);
}

pub fn walk_finally_block<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut FinallyBlock) {
    visitor.visit(&mut node.body);
}

pub fn walk_variable<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut Variable) {}

pub fn walk_simple_variable<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut SimpleVariable) {}

pub fn walk_variable_variable<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut VariableVariable,
) {
    visitor.visit_variable(&mut node.variable);
}

pub fn walk_braced_variable_variable<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut BracedVariableVariable,
) {
    visitor.visit_expression(&mut node.variable);
}

pub fn walk_static_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut StaticStatement) {
    for item in &mut node.vars {
        visitor.visit_static_var(item);
    }
}

pub fn walk_switch_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut SwitchStatement) {
    visitor.visit_expression(&mut node.condition);
    for item in &mut node.cases {
        visitor.visit_case(item);
    }
}

pub fn walk_echo_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut EchoStatement) {
    for item in &mut node.values {
        visitor.visit_expression(item);
    }
    visitor.visit_ending(&mut node.ending);
}

pub fn walk_return_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut ReturnStatement) {
    if let Some(item) = &mut node.value {
        visitor.visit_expression(item);
    }
    visitor.visit_ending(&mut node.ending);
}

pub fn walk_use_statement<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut UseStatement) {
    visitor.visit_use_kind(&mut node.kind);
    for item in &mut node.uses {
        visitor.visit_use(item);
    }
}

pub fn walk_group_use_statement<V: VisitorMut + ?Sized>(
    visitor: &mut V,
    node: &mut GroupUseStatement,
) {
    visitor.visit_simple_identifier(&mut node.prefix);
    visitor.visit_use_kind(&mut node.kind);
    for item in &mut node.uses {
        visitor.visit_use(item);
    }
}

pub fn walk_static_var<V: VisitorMut + ?Sized>(visitor: &mut V, node: &mut StaticVar) {
    visitor.visit_variable(&mut node.var);
    if let Some(item) = &mut node.default {
        visitor.visit_expression(item);
    }
}
