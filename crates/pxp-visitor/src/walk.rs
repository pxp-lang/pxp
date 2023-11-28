use pxp_ast::{Statement, StatementKind, Expression, ExpressionKind, goto::{LabelStatement, GotoStatement}, StaticStatement, StaticVar, GlobalStatement, loops::{DoWhileStatement, WhileStatement, WhileStatementBody, ForStatement, ForStatementIterator, ForStatementBody, ForeachStatement, ForeachStatementIterator, ForeachStatementBody, BreakStatement, Level, ContinueStatement}, control_flow::{IfStatement, IfStatementBody, IfStatementElseIf, IfStatementElseIfBlock, IfStatementElse, IfStatementElseBlock}, SwitchStatement, Case, constant::{ConstantStatement, ConstantEntry, ClassishConstant}, functions::{FunctionStatement, FunctionParameterList, FunctionParameter, FunctionBody, AbstractMethod, AbstractConstructor, ConstructorParameterList, ConstructorParameter, ConcreteMethod, MethodBody, ConcreteConstructor}, classes::{ClassStatement, ClassExtends, ClassImplements, ClassBody, ClassishMember}, traits::{TraitUsage, TraitUsageAdaptation, TraitStatement, TraitBody}, properties::{Property, PropertyEntry, VariableProperty}, interfaces::{InterfaceStatement, InterfaceExtends, InterfaceBody}, EchoStatement, ExpressionStatement, ReturnStatement, namespaces::{NamespaceStatement, UnbracedNamespace, BracedNamespace}, UseStatement, Use, GroupUseStatement, try_block::{TryStatement, CatchBlock, FinallyBlock}, enums::{UnitEnumStatement, UnitEnumMember, UnitEnumCase, BackedEnumStatement, BackedEnumMember, BackedEnumCase}, declares::{DeclareStatement, DeclareEntry, DeclareBody}, EvalExpression, arguments::{Argument, ArgumentList}, EmptyExpression, DieExpression, ExitExpression, IssetExpression, UnsetExpression, PrintExpression, literals::Literal, operators::{ArithmeticOperationExpression, AssignmentOperationExpression, BitwiseOperationExpression, ComparisonOperationExpression, LogicalOperationExpression}, ConcatExpression, InstanceofExpression, ReferenceExpression, ParenthesizedExpression, ErrorSuppressExpression, identifiers::{Identifier, DynamicIdentifier}};

use crate::Visitor;

pub fn walk<V: Visitor + ?Sized>(visitor: &mut V, program: &mut [Statement]) {
    for statement in program.iter_mut() {
        visitor.visit_statement(statement);
    }
}

pub fn walk_statement<V: Visitor + ?Sized>(visitor: &mut V, statement: &mut Statement) {
    match &mut statement.kind {
        StatementKind::FullOpeningTag(stmt) => visitor.visit_full_opening_tag(stmt),
        StatementKind::ShortOpeningTag(stmt) => visitor.visit_short_opening_tag(stmt),
        StatementKind::EchoOpeningTag(stmt) => visitor.visit_echo_opening_tag(stmt),
        StatementKind::ClosingTag(stmt) => visitor.visit_closing_tag(stmt),
        StatementKind::InlineHtml(stmt) => visitor.visit_inline_html(stmt),
        StatementKind::Label(stmt) => visitor.visit_label(stmt),
        StatementKind::Goto(stmt) => visitor.visit_goto(stmt),
        StatementKind::HaltCompiler(stmt) => visitor.visit_halt_compiler(stmt),
        StatementKind::Static(stmt) => visitor.visit_static(stmt),
        StatementKind::DoWhile(stmt) => visitor.visit_do_while(stmt),
        StatementKind::While(stmt) => visitor.visit_while(stmt),
        StatementKind::For(stmt) => visitor.visit_for(stmt),
        StatementKind::Foreach(stmt) => visitor.visit_foreach(stmt),
        StatementKind::Break(stmt) => visitor.visit_break(stmt),
        StatementKind::Continue(stmt) => visitor.visit_continue(stmt),
        StatementKind::Constant(stmt) => visitor.visit_constant(stmt),
        StatementKind::Function(stmt) => visitor.visit_function(stmt),
        StatementKind::Class(stmt) => visitor.visit_class(stmt),
        StatementKind::Trait(stmt) => visitor.visit_trait(stmt),
        StatementKind::Interface(stmt) => visitor.visit_interface(stmt),
        StatementKind::If(stmt) => visitor.visit_if(stmt),
        StatementKind::Switch(stmt) => visitor.visit_switch(stmt),
        StatementKind::Echo(stmt) => visitor.visit_echo(stmt),
        StatementKind::Expression(stmt) => visitor.visit_expression_stmt(stmt),
        StatementKind::Return(stmt) => visitor.visit_return(stmt),
        StatementKind::Namespace(stmt) => visitor.visit_namespace(stmt),
        StatementKind::Use(stmt) => visitor.visit_use(stmt),
        StatementKind::GroupUse(stmt) => visitor.visit_group_use(stmt),
        StatementKind::Comment(stmt) => visitor.visit_comment_stmt(stmt),
        StatementKind::Try(stmt) => visitor.visit_try(stmt),
        StatementKind::UnitEnum(stmt) => visitor.visit_unit_enum(stmt),
        StatementKind::BackedEnum(stmt) => visitor.visit_backed_enum(stmt),
        StatementKind::Block(stmt) => visitor.visit_block(stmt),
        StatementKind::Global(stmt) => visitor.visit_global(stmt),
        StatementKind::Declare(stmt) => visitor.visit_declare(stmt),
        StatementKind::Noop(span) => visitor.visit_noop(*span),
    };
}

pub fn walk_expression<V: Visitor + ?Sized>(visitor: &mut V, expression: &mut Expression) {
    match &mut expression.kind {
        ExpressionKind::Missing => visitor.visit_missing_expr(),
        ExpressionKind::Eval(expr) => visitor.visit_eval(expr),
        ExpressionKind::Empty(expr) => visitor.visit_empty(expr),
        ExpressionKind::Die(expr) => visitor.visit_die(expr),
        ExpressionKind::Exit(expr) => visitor.visit_exit(expr),
        ExpressionKind::Isset(expr) => visitor.visit_isset(expr),
        ExpressionKind::Unset(expr) => visitor.visit_unset(expr),
        ExpressionKind::Print(expr) => visitor.visit_print(expr),
        ExpressionKind::Literal(expr) => visitor.visit_literal(expr),
        ExpressionKind::ArithmeticOperation(expr) => visitor.visit_arithmetic_operation(expr),
        ExpressionKind::AssignmentOperation(expr) => visitor.visit_assignment_operation(expr),
        ExpressionKind::BitwiseOperation(expr) => visitor.visit_bitwise_operation(expr),
        ExpressionKind::ComparisonOperation(expr) => visitor.visit_comparison_operation(expr),
        ExpressionKind::LogicalOperation(expr) => visitor.visit_logical_operation(expr),
        ExpressionKind::Concat(expr) => visitor.visit_concat(expr),
        ExpressionKind::Instanceof(expr) => visitor.visit_instanceof(expr),
        ExpressionKind::Reference(expr) => visitor.visit_reference(expr),
        ExpressionKind::Parenthesized(expr) => visitor.visit_parenthesized(expr),
        ExpressionKind::ErrorSuppress(expr) => visitor.visit_error_suppress(expr),
        ExpressionKind::Identifier(expr) => visitor.visit_identifier(expr),
        ExpressionKind::Variable(expr) => visitor.visit_variable(expr),
        ExpressionKind::Include(expr) => visitor.visit_include(expr),
        ExpressionKind::IncludeOnce(expr) => visitor.visit_include_once(expr),
        ExpressionKind::Require(expr) => visitor.visit_require(expr),
        ExpressionKind::RequireOnce(expr) => visitor.visit_require_once(expr),
        ExpressionKind::FunctionCall(expr) => visitor.visit_function_call(expr),
        ExpressionKind::FunctionClosureCreation(expr) => visitor.visit_function_closure_creation(expr),
        ExpressionKind::MethodCall(expr) => visitor.visit_method_call(expr),
        ExpressionKind::MethodClosureCreation(expr) => visitor.visit_method_closure_creation(expr),
        ExpressionKind::NullsafeMethodCall(expr) => visitor.visit_nullsafe_method_call(expr),
        ExpressionKind::StaticMethodCall(expr) => visitor.visit_static_method_call(expr),
        ExpressionKind::StaticVariableMethodCall(expr) => visitor.visit_static_variable_method_call(expr),
        ExpressionKind::StaticMethodClosureCreation(expr) => visitor.visit_static_method_closure_creation(expr),
        ExpressionKind::StaticVariableMethodClosureCreation(expr) => visitor.visit_static_variable_method_closure_creation(expr),
        ExpressionKind::PropertyFetch(expr) => visitor.visit_property_fetch(expr),
        ExpressionKind::NullsafePropertyFetch(expr) => visitor.visit_nullsafe_property_fetch(expr),
        ExpressionKind::StaticPropertyFetch(expr) => visitor.visit_static_property_fetch(expr),
        ExpressionKind::ConstantFetch(expr) => visitor.visit_constant_fetch(expr),
        ExpressionKind::Static => visitor.visit_static_expr(),
        ExpressionKind::Self_ => visitor.visit_self_expr(),
        ExpressionKind::Parent => visitor.visit_parent_expr(),
        ExpressionKind::ShortArray(expr) => visitor.visit_short_array(expr),
        ExpressionKind::Array(expr) => visitor.visit_array(expr),
        ExpressionKind::List(expr) => visitor.visit_list(expr),
        ExpressionKind::Closure(expr) => visitor.visit_closure(expr),
        ExpressionKind::ArrowFunction(expr) => visitor.visit_arrow_function(expr),
        ExpressionKind::New(expr) => visitor.visit_new(expr),
        ExpressionKind::InterpolatedString(expr) => visitor.visit_interpolated_string(expr),
        ExpressionKind::Heredoc(expr) => visitor.visit_heredoc(expr),
        ExpressionKind::Nowdoc(expr) => visitor.visit_nowdoc(expr),
        ExpressionKind::ShellExec(expr) => visitor.visit_shell_exec(expr),
        ExpressionKind::AnonymousClass(expr) => visitor.visit_anonymous_class(expr),
        ExpressionKind::Bool(expr) => visitor.visit_bool(expr),
        ExpressionKind::ArrayIndex(expr) => visitor.visit_array_index(expr),
        ExpressionKind::Null => visitor.visit_null_expr(),
        ExpressionKind::MagicConstant(expr) => visitor.visit_magic_constant(expr),
        ExpressionKind::ShortTernary(expr) => visitor.visit_short_ternary(expr),
        ExpressionKind::Ternary(expr) => visitor.visit_ternary(expr),
        ExpressionKind::Coalesce(expr) => visitor.visit_coalesce(expr),
        ExpressionKind::Clone(expr) => visitor.visit_clone(expr),
        ExpressionKind::Match(expr) => visitor.visit_match(expr),
        ExpressionKind::Throw(expr) => visitor.visit_throw(expr),
        ExpressionKind::Yield(expr) => visitor.visit_yield(expr),
        ExpressionKind::YieldFrom(expr) => visitor.visit_yield_from(expr),
        ExpressionKind::Cast(expr) => visitor.visit_cast(expr),
        ExpressionKind::Noop => visitor.visit_noop_expr(),
    }
}

macro_rules! walk {
    (
        using($v:ident, $n:ident);

        $($label:ident: $node:ty => $body:block )+
    ) => {
        $(
            pub fn $label<V: Visitor + ?Sized>($v: &mut V, $n: &mut $node) $body
        )+
    }
}

walk! {
    using(visitor, node);

    walk_label: LabelStatement => {
        visitor.visit_simple_identifier(&mut node.label)
    }

    walk_goto: GotoStatement => {
        visitor.visit_simple_identifier(&mut node.label)
    }

    walk_static: StaticStatement => {
        for variable in node.vars.iter_mut() {
            visitor.visit_static_var(variable)
        }
    }

    walk_static_var: StaticVar => {
        visitor.visit_variable(&mut node.var);
        
        if let Some(default) = &mut node.default {
            visitor.visit_expression(default);
        }
    }

    walk_global: GlobalStatement => {
        for variable in node.variables.iter_mut() {
            visitor.visit_variable(variable)
        }
    }

    walk_do_while: DoWhileStatement => {
        visitor.visit_statement(&mut node.body);
        visitor.visit_expression(&mut node.condition);
    }

    walk_while: WhileStatement => {
        visitor.visit_expression(&mut node.condition);
        visitor.visit_while_statement_body(&mut node.body);
    }

    walk_while_statement_body: WhileStatementBody => {
        match node {
            WhileStatementBody::Statement { statement } => {
                visitor.visit_statement(statement);
            },
            WhileStatementBody::Block { statements, .. } => {
                visitor.visit(statements)
            }
        }
    }

    walk_for: ForStatement => {
        visitor.visit_for_statement_iterator(&mut node.iterator);
        visitor.visit_for_statement_body(&mut node.body);
    }

    walk_for_statement_iterator: ForStatementIterator => {
        for init in node.initializations.iter_mut() {
            visitor.visit_expression(init);
        }

        for condition in node.conditions.iter_mut() {
            visitor.visit_expression(condition);
        }

        for r#loop in node.r#loop.iter_mut() {
            visitor.visit_expression(r#loop);
        }
    }

    walk_for_statement_body: ForStatementBody => {
        match node {
            ForStatementBody::Statement { statement } => {
                visitor.visit_statement(statement);
            },
            ForStatementBody::Block { statements, .. } => {
                visitor.visit(statements)
            }
        }
    }

    walk_foreach: ForeachStatement => {
        visitor.visit_foreach_statement_iterator(&mut node.iterator);
        visitor.visit_foreach_statement_body(&mut node.body);
    }

    walk_foreach_statement_iterator: ForeachStatementIterator => {
        match node {
            ForeachStatementIterator::Value { expression, value, .. } => {
                visitor.visit_expression(expression);
                visitor.visit_expression(value);
            },
            ForeachStatementIterator::KeyAndValue { expression, key, value, .. } => {
                visitor.visit_expression(expression);
                visitor.visit_expression(key);
                visitor.visit_expression(value);
            },
        }
    }

    walk_foreach_statement_body: ForeachStatementBody => {
        match node {
            ForeachStatementBody::Statement { statement } => {
                visitor.visit_statement(statement)
            },
            ForeachStatementBody::Block { statements, .. } => {
                visitor.visit(statements)
            }
        }
    }

    walk_if: IfStatement => {
        visitor.visit_expression(&mut node.condition);
        visitor.visit_if_statement_body(&mut node.body);
    }

    walk_if_statement_body: IfStatementBody => {
        match node {
            IfStatementBody::Statement { statement, elseifs, r#else } => {
                visitor.visit_statement(statement);
                
                for r#elseif in elseifs.iter_mut() {
                    visitor.visit_if_statement_elseif(r#elseif);
                }

                if let Some(r#else) = r#else {
                    visitor.visit_if_statement_else(r#else);
                }
            },
            IfStatementBody::Block { statements, elseifs, r#else, .. } => {
                visitor.visit(statements);

                for r#elseif in elseifs.iter_mut() {
                    visitor.visit_if_statement_elseif_block(r#elseif);
                }

                if let Some(r#else) = r#else {
                    visitor.visit_if_statement_else_block(r#else);
                }
            },
        }
    }

    walk_if_statement_elseif: IfStatementElseIf => {
        visitor.visit_expression(&mut node.condition);
        visitor.visit_statement(&mut node.statement);
    }

    walk_if_statement_elseif_block: IfStatementElseIfBlock => {
        visitor.visit_expression(&mut node.condition);
        visitor.visit(&mut node.statements);
    }

    walk_if_statement_else: IfStatementElse => {
        visitor.visit_statement(&mut node.statement);
    }

    walk_if_statement_else_block: IfStatementElseBlock => {
        visitor.visit(&mut node.statements);
    }

    walk_switch: SwitchStatement => {
        visitor.visit_expression(&mut node.condition);

        for case in node.cases.iter_mut() {
            visitor.visit_switch_case(case);
        }
    }

    walk_switch_case: Case => {
        if let Some(condition) = &mut node.condition {
            visitor.visit_expression(condition);
        }

        visitor.visit(&mut node.body);
    }

    walk_level: Level => {
        match node {
            Level::Literal(literal) => visitor.visit_literal(literal),
            Level::Parenthesized { level, .. } => visitor.visit_level(level),
        }
    }

    walk_break: BreakStatement => {
        if let Some(level) = &mut node.level {
            visitor.visit_level(level);
        }
    }
    
    walk_continue: ContinueStatement => {
        if let Some(level) = &mut node.level {
            visitor.visit_level(level);
        }
    }

    walk_constant: ConstantStatement => {
        for entry in node.entries.iter_mut() {
            visitor.visit_constant_entry(entry);
        }
    }

    walk_constant_entry: ConstantEntry => {
        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit_expression(&mut node.value);
    }

    walk_function: FunctionStatement => {
        // FIXME: Walk attributes here.
        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit_function_parameter_list(&mut node.parameters);
        visitor.visit_function_body(&mut node.body);
    }

    walk_function_parameter_list: FunctionParameterList => {
        for parameter in node.parameters.iter_mut() {
            visitor.visit_function_parameter(parameter);
        }
    }

    walk_function_parameter: FunctionParameter => {
        visitor.visit_simple_variable(&mut node.name);
        // FIXME: Walk attributes here.

        if let Some(default) = &mut node.default {
            visitor.visit_expression(default);
        }
    }

    walk_function_body: FunctionBody => {
        visitor.visit(&mut node.statements);
    }

    walk_class: ClassStatement => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.
        visitor.visit_simple_identifier(&mut node.name);

        if let Some(extends) = &mut node.extends {
            visitor.visit_class_extends(extends);
        }

        if let Some(implements) = &mut node.implements {
            visitor.visit_class_implements(implements);
        }

        visitor.visit_class_body(&mut node.body);
    }

    walk_class_extends: ClassExtends => {
        visitor.visit_simple_identifier(&mut node.parent);
    }

    walk_class_implements: ClassImplements => {
        for interface in node.interfaces.iter_mut() {
            visitor.visit_simple_identifier(interface);
        }
    }

    walk_class_body: ClassBody => {
        for member in node.members.iter_mut() {
            visitor.visit_classish_member(member);
        }
    }

    walk_classish_member: ClassishMember => {
        match node {
            ClassishMember::Constant(constant) => {
                visitor.visit_classish_constant(constant);
            },
            ClassishMember::TraitUsage(usage) => {
                visitor.visit_trait_usage(usage);
            },
            ClassishMember::Property(property) => {
                visitor.visit_property(property);
            },
            ClassishMember::VariableProperty(property) => {
                visitor.visit_variable_property(property);
            },
            ClassishMember::AbstractMethod(method) => {
                visitor.visit_abstract_method(method);
            },
            ClassishMember::AbstractConstructor(method) => {
                visitor.visit_abstract_constructor(method);
            },
            ClassishMember::ConcreteMethod(method) => {
                visitor.visit_concrete_method(method);
            },
            ClassishMember::ConcreteConstructor(method) => {
                visitor.visit_concrete_constructor(method);
            },
        }
    }

    walk_classish_constant: ClassishConstant => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.

        for entries in node.entries.iter_mut() {
            visitor.visit_constant_entry(entries);
        }
    }

    walk_trait_usage: TraitUsage => {
        for r#trait in node.traits.iter_mut() {
            visitor.visit_simple_identifier(r#trait);
        }

        for adaptation in node.adaptations.iter_mut() {
            visitor.visit_trait_usage_adaptation(adaptation);
        }
    }

    walk_trait_usage_adaptation: TraitUsageAdaptation => {
        match node {
            TraitUsageAdaptation::Alias { r#trait, method, alias, visibility } => {
                if let Some(r#trait) = r#trait {
                    visitor.visit_simple_identifier(r#trait);
                }

                visitor.visit_simple_identifier(method);
                visitor.visit_simple_identifier(alias);

                if let Some(_visibility) = visibility {
                    // FIXME: Visit visibility here.
                    // visitor.visit_visibility_modifier(visibility);
                }
            },
            TraitUsageAdaptation::Visibility { r#trait, method, visibility: _visibility } => {
                if let Some(r#trait) = r#trait {
                    visitor.visit_simple_identifier(r#trait);
                }

                visitor.visit_simple_identifier(method);
                // FIXME: Visit visibility here.
            },
            TraitUsageAdaptation::Precedence { r#trait, method, insteadof } => {
                if let Some(r#trait) = r#trait {
                    visitor.visit_simple_identifier(r#trait);
                }

                visitor.visit_simple_identifier(method);

                for insteadof in insteadof.iter_mut() {
                    visitor.visit_simple_identifier(insteadof);
                }
            }
        }
    }

    walk_property: Property => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.
        // FIXME: Walk type here.
        
        for entry in node.entries.iter_mut() {
            visitor.visit_property_entry(entry);
        }
    }

    walk_property_entry: PropertyEntry => {
        match node {
            PropertyEntry::Uninitialized { variable } => {
                visitor.visit_simple_variable(variable);
            },
            PropertyEntry::Initialized { variable, value, .. } => {
                visitor.visit_simple_variable(variable);
                visitor.visit_expression(value);
            },
        }
    }

    walk_variable_property: VariableProperty => {
        // FIXME: Walk attributes here.
        // FIXME: Walk type here.

        for entry in node.entries.iter_mut() {
            visitor.visit_property_entry(entry);
        }
    }

    walk_abstract_method: AbstractMethod => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.

        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit_function_parameter_list(&mut node.parameters);

        // FIXME: Walk return type here.
    }

    walk_abstract_constructor: AbstractConstructor => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.

        visitor.visit_constructor_parameter_list(&mut node.parameters);
    }

    walk_constructor_parameter_list: ConstructorParameterList => {
        for parameter in node.parameters.iter_mut() {
            visitor.visit_constructor_parameter(parameter);
        }
    }

    walk_constructor_parameter: ConstructorParameter => {
        // FIXME: Walk attributes here.
        // FIXME: Visit modifiers here.
        // FIXME: Visit type here.
        visitor.visit_simple_variable(&mut node.name);

        if let Some(default) = &mut node.default {
            visitor.visit_expression(default);
        }
    }

    walk_concrete_method: ConcreteMethod => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.

        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit_function_parameter_list(&mut node.parameters);

        // FIXME: Walk return type here.

        visitor.visit_method_body(&mut node.body);
    }

    walk_method_body: MethodBody => {
        visitor.visit(&mut node.statements);
    }

    walk_concrete_constructor: ConcreteConstructor => {
        // FIXME: Walk attributes here.
        // FIXME: Walk modifiers here.

        visitor.visit_constructor_parameter_list(&mut node.parameters);
        visitor.visit_method_body(&mut node.body);
    }

    walk_interface: InterfaceStatement => {
        // FIXME: Walk attributes here.

        visitor.visit_simple_identifier(&mut node.name);

        if let Some(extends) = &mut node.extends {
            visitor.visit_interface_extends(extends);
        }

        visitor.visit_interface_body(&mut node.body);
    }

    walk_interface_extends: InterfaceExtends => {
        for parent in node.parents.iter_mut() {
            visitor.visit_simple_identifier(parent);
        }
    }

    walk_interface_body: InterfaceBody => {
        for member in node.members.iter_mut() {
            visitor.visit_classish_member(member);
        }
    }

    walk_trait: TraitStatement => {
        // FIXME: Walk attributes here.

        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit_trait_body(&mut node.body);
    }

    walk_trait_body: TraitBody => {
        for member in node.members.iter_mut() {
            visitor.visit_classish_member(member);
        }
    }

    walk_echo: EchoStatement => {
        for value in node.values.iter_mut() {
            visitor.visit_expression(value);
        }
    }

    walk_expression_stmt: ExpressionStatement => {
        visitor.visit_expression(&mut node.expression);
    }

    walk_return: ReturnStatement => {
        if let Some(expression) = &mut node.value {
            visitor.visit_expression(expression);
        }
    }

    walk_namespace: NamespaceStatement => {
        match node {
            NamespaceStatement::Unbraced(node) => {
                visitor.visit_unbraced_namespace(node);
            },
            NamespaceStatement::Braced(node) => {
                visitor.visit_braced_namespace(node);
            },
        }
    }

    walk_unbraced_namespace: UnbracedNamespace => {
        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit(&mut node.statements);
    }

    walk_braced_namespace: BracedNamespace => {
        if let Some(name) = &mut node.name {
            visitor.visit_simple_identifier(name);
        }

        visitor.visit(&mut node.body.statements);
    }

    walk_use: UseStatement => {
        for r#use in node.uses.iter_mut() {
            visitor.visit_use_use(r#use);
        }
    }

    walk_use_use: Use => {
        visitor.visit_simple_identifier(&mut node.name);

        if let Some(alias) = &mut node.alias {
            visitor.visit_simple_identifier(alias);
        }
    }

    walk_group_use: GroupUseStatement => {
        visitor.visit_simple_identifier(&mut node.prefix);

        for r#use in node.uses.iter_mut() {
            visitor.visit_use_use(r#use);
        }
    }

    walk_try: TryStatement => {
        visitor.visit(&mut node.body);

        for catch in node.catches.iter_mut() {
            visitor.visit_catch_block(catch);
        }

        if let Some(finally) = &mut node.finally {
            visitor.visit_finally_block(finally);
        }
    }

    walk_catch_block: CatchBlock => {
        if let Some(variable) = &mut node.var {
            visitor.visit_simple_variable(variable);
        }

        visitor.visit(&mut node.body);
    }

    walk_finally_block: FinallyBlock => {
        visitor.visit(&mut node.body);
    }

    walk_unit_enum: UnitEnumStatement => {
        // FIXME: Walk attributes here.

        visitor.visit_simple_identifier(&mut node.name);

        for implements in node.implements.iter_mut() {
            visitor.visit_simple_identifier(implements);
        }

        for member in node.body.members.iter_mut() {
            visitor.visit_unit_enum_member(member);
        }
    }

    walk_unit_enum_member: UnitEnumMember => {
        match node {
            UnitEnumMember::Case(node) => {
                visitor.visit_unit_enum_case(node);
            }
            UnitEnumMember::Classish(node) => {
                visitor.visit_classish_member(node);
            }
        }
    }

    walk_unit_enum_case: UnitEnumCase => {
        // FIXME: Walk attributes here.

        visitor.visit_simple_identifier(&mut node.name);
    }

    walk_backed_enum: BackedEnumStatement => {
        // FIXME: Walk attributes here.

        visitor.visit_simple_identifier(&mut node.name);

        for implements in node.implements.iter_mut() {
            visitor.visit_simple_identifier(implements);
        }

        for member in node.body.members.iter_mut() {
            visitor.visit_backed_enum_member(member);
        }
    }

    walk_backed_enum_member: BackedEnumMember => {
        match node {
            BackedEnumMember::Case(node) => {
                visitor.visit_backed_enum_case(node);
            }
            BackedEnumMember::Classish(node) => {
                visitor.visit_classish_member(node);
            }
        }
    }

    walk_backed_enum_case: BackedEnumCase => {
        // FIXME: Walk attributes here.

        visitor.visit_simple_identifier(&mut node.name);
        visitor.visit_expression(&mut node.value);
    }

    walk_declare: DeclareStatement => {
        for entry in node.entries.entries.iter_mut() {
            visitor.visit_declare_entry(entry);
        }

        visitor.visit_declare_body(&mut node.body);
    }

    walk_declare_entry: DeclareEntry => {
        visitor.visit_simple_identifier(&mut node.key);
        visitor.visit_literal(&mut node.value);
    }

    walk_declare_body: DeclareBody => {
        match node {
            DeclareBody::Noop { .. } => {},
            DeclareBody::Braced { statements, .. } => {
                visitor.visit(statements);
            },
            DeclareBody::Expression { expression, .. } => {
                visitor.visit_expression(expression);
            },
            DeclareBody::Block { statements, .. } => {
                visitor.visit(statements);
            },
        }
    }

    walk_argument_list: ArgumentList => {
        for argument in node.arguments.iter_mut() {
            visitor.visit_argument(argument);
        }
    }

    walk_argument: Argument => {
        match node {
            Argument::Positional(node) => {
                visitor.visit_expression(&mut node.value);
            },
            Argument::Named(node) => {
                visitor.visit_simple_identifier(&mut node.name);
                visitor.visit_expression(&mut node.value);
            },
        }
    }

    walk_eval: EvalExpression => {
        if let Some(argument) = &mut node.argument.argument {
            visitor.visit_argument(argument);
        }
    }

    walk_empty: EmptyExpression => {
        if let Some(argument) = &mut node.argument.argument {
            visitor.visit_argument(argument);
        }
    }

    walk_die: DieExpression => {
        if let Some(argument) = &mut node.argument {
            if let Some(argument) = &mut argument.argument {
                visitor.visit_argument(argument);
            }
        }
    }

    walk_exit: ExitExpression => {
        if let Some(argument) = &mut node.argument {
            if let Some(argument) = &mut argument.argument {
                visitor.visit_argument(argument);
            }
        }
    }

    walk_isset: IssetExpression => {
        visitor.visit_argument_list(&mut node.arguments);
    }

    walk_unset: UnsetExpression => {
        visitor.visit_argument_list(&mut node.arguments);
    }

    walk_print: PrintExpression => {
        if let Some(value) = &mut node.value {
            visitor.visit_expression(value);
        }

        if let Some(argument) = &mut node.argument {
            if let Some(argument) = &mut argument.argument {
                visitor.visit_argument(argument);
            }
        }
    }

    walk_arithmetic_operation: ArithmeticOperationExpression => {
        match node {
            ArithmeticOperationExpression::Addition {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Subtraction {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Multiplication {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Division {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Modulo {
                left, right, ..
            }  => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Exponentiation {
                left, right, ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Negative {
                right, ..
            } => {
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::Positive {
                right,
                ..
            } => {
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::PreIncrement {
                right, ..
            } => {
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::PostIncrement {
                left, ..
            } => {
                visitor.visit_expression(left);
            },
            ArithmeticOperationExpression::PreDecrement {
                right, ..
            } => {
                visitor.visit_expression(right);
            },
            ArithmeticOperationExpression::PostDecrement {
                left, ..
            } => {
                visitor.visit_expression(left);
            },
        }
    }

    walk_assignment_operation: AssignmentOperationExpression => {
        match node {
            AssignmentOperationExpression::Assign {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Addition {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Subtraction {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Multiplication {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Division {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Modulo {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Exponentiation {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Concat {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::BitwiseAnd {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::BitwiseOr {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::BitwiseXor {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::LeftShift {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::RightShift {
                left,
                right,
                ..
            } |
            AssignmentOperationExpression::Coalesce {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            }
        }
    }

    walk_bitwise_operation: BitwiseOperationExpression => {
        match node {
            BitwiseOperationExpression::And {
                left,
                right,
                ..
            } |
            BitwiseOperationExpression::Or {
                left,
                right,
                ..
            } |
            BitwiseOperationExpression::Xor {
                left,
                right,
                ..
            } |
            BitwiseOperationExpression::LeftShift {
                left,
                right,
                ..
            } |
            BitwiseOperationExpression::RightShift {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            BitwiseOperationExpression::Not {
                right,
                ..
            } => {
                visitor.visit_expression(right);
            },
        }
    }

    walk_comparison_operation: ComparisonOperationExpression => {
        match node {
            ComparisonOperationExpression::Equal {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::Identical {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::NotEqual {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::AngledNotEqual {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::NotIdentical {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::LessThan {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::GreaterThan {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::LessThanOrEqual {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::GreaterThanOrEqual {
                left,
                right,
                ..
            } |
            ComparisonOperationExpression::Spaceship {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            }
        }
    }

    walk_logical_operation: LogicalOperationExpression => {
        match node {
            LogicalOperationExpression::And {
                left,
                right,
                ..
            } |
            LogicalOperationExpression::Or {
                left,
                right,
                ..
            } |
            LogicalOperationExpression::LogicalAnd {
                left,
                right,
                ..
            } |
            LogicalOperationExpression::LogicalOr {
                left,
                right,
                ..
            } |
            LogicalOperationExpression::LogicalXor {
                left,
                right,
                ..
            } => {
                visitor.visit_expression(left);
                visitor.visit_expression(right);
            },
            LogicalOperationExpression::Not {
                right,
                ..
            } => {
                visitor.visit_expression(right);
            },
        }
    }

    walk_concat: ConcatExpression => {
        visitor.visit_expression(&mut node.left);
        visitor.visit_expression(&mut node.right);
    }

    walk_instanceof: InstanceofExpression => {
        visitor.visit_expression(&mut node.left);
        visitor.visit_expression(&mut node.right);
    }

    walk_reference: ReferenceExpression => {
        visitor.visit_expression(&mut node.right);
    }

    walk_parenthesized: ParenthesizedExpression => {
        visitor.visit_expression(&mut node.expr);
    }

    walk_error_suppress: ErrorSuppressExpression => {
        visitor.visit_expression(&mut node.expr);
    }

    walk_identifier: Identifier => {
        match node {
            Identifier::SimpleIdentifier(node) => {
                visitor.visit_simple_identifier(node);
            },
            Identifier::DynamicIdentifier(node) => {
                visitor.visit_dynamic_identifier(node);
            },
        }
    }

    walk_dynamic_identifier: DynamicIdentifier => {
        visitor.visit_expression(&mut node.expr);
    }
}