use pxp_parser::parser::ast::{
    arguments::{Argument, ArgumentList, NamedArgument, PositionalArgument},
    classes::{ClassExtends, ClassImplements, ClassMember, ClassStatement},
    comments::{Comment, CommentFormat},
    constant::{ClassishConstant, ConstantEntry, ConstantStatement},
    control_flow::{IfStatement, IfStatementBody, IfStatementElse, IfStatementElseIf},
    data_type::Type,
    enums::{UnitEnumBody, UnitEnumCase, UnitEnumMember, UnitEnumStatement, BackedEnumStatement, BackedEnumType, BackedEnumBody, BackedEnumMember, BackedEnumCase},
    functions::{
        AbstractConstructor, AbstractMethod, ClosureExpression, ConcreteConstructor,
        ConcreteMethod, ConstructorParameter, ConstructorParameterList, FunctionParameterList,
        FunctionStatement, ReturnType,
    },
    goto::{GotoStatement, LabelStatement},
    identifiers::{Identifier, SimpleIdentifier},
    interfaces::{InterfaceBody, InterfaceExtends, InterfaceMember, InterfaceStatement},
    literals::{Literal, LiteralInteger, LiteralString, LiteralFloat},
    loops::{
        BreakStatement, ContinueStatement, DoWhileStatement, ForStatement, ForStatementBody,
        ForeachStatement, ForeachStatementBody, ForeachStatementIterator, Level, WhileStatement,
        WhileStatementBody,
    },
    modifiers::{
        ClassModifier, ClassModifierGroup, MethodModifier, MethodModifierGroup,
        PromotedPropertyModifier, PromotedPropertyModifierGroup, PropertyModifier,
        PropertyModifierGroup, VisibilityModifier,
    },
    namespaces::{BracedNamespace, BracedNamespaceBody, NamespaceStatement, UnbracedNamespace},
    operators::{
        ArithmeticOperationExpression, AssignmentOperationExpression, BitwiseOperationExpression,
        ComparisonOperationExpression, LogicalOperationExpression, RangeOperationExpression,
    },
    properties::{Property, PropertyEntry, VariableProperty},
    traits::{TraitBody, TraitMember, TraitStatement, TraitUsage, TraitUsageAdaptation},
    utils::CommaSeparated,
    variables::{SimpleVariable, Variable, VariableVariable, BracedVariableVariable},
    ArrayExpression, ArrayIndexExpression, ArrayItem, BlockStatement, BoolExpression,
    CastExpression, CloneExpression, CoalesceExpression, ConcatExpression, ConstantFetchExpression,
    DefaultMatchArm, DieExpression, EchoStatement, EmptyExpression, Ending,
    ErrorSuppressExpression, EvalExpression, ExitExpression, Expression, ExpressionStatement,
    FunctionCallExpression, FunctionClosureCreationExpression, GlobalStatement, GroupUseStatement,
    HaltCompilerStatement, HeredocExpression, IncludeExpression, IncludeOnceExpression,
    InlineHtmlStatement, InstanceofExpression, InterpolatedStringExpression, IssetExpression,
    ListEntry, ListExpression, MagicConstantExpression, MatchArm, MatchArmBody, MatchExpression,
    MethodCallExpression, MethodClosureCreationExpression, NewExpression, NowdocExpression,
    NullsafeMethodCallExpression, NullsafePropertyFetchExpression, ParenthesizedExpression,
    PrintExpression, PropertyFetchExpression, ReferenceExpression, RequireExpression,
    RequireOnceExpression, ReturnStatement, ShellExecExpression, ShortArrayExpression,
    ShortMatchExpression, ShortTernaryExpression, Statement, StaticMethodCallExpression,
    StaticMethodClosureCreationExpression, StaticPropertyFetchExpression, StaticStatement,
    StaticVar, StaticVariableMethodCallExpression, StaticVariableMethodClosureCreationExpression,
    SwitchStatement, TernaryExpression, ThrowExpression, TypeAliasStatement, UnsetExpression,
    UseStatement, YieldExpression, YieldFromExpression, CastKind,
};

struct PrinterState {
    output: String,
    indent: usize,
}

impl PrinterState {
    fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        self.indent -= 1;
    }

    fn indent_string(&self) -> String {
        "    ".repeat(self.indent)
    }

    fn new_line(&mut self) {
        self.output.push('\n');
        self.output.push_str(&self.indent_string());
    }

    fn write(&mut self, string: impl AsRef<str>) {
        self.output.push_str(string.as_ref());
    }

    fn get_output(&self) -> String {
        self.output.clone()
    }
}

pub fn print(program: &[Statement]) -> String {
    let mut state = PrinterState::new();

    for statement in program.iter() {
        print_statement(&mut state, statement);
    }

    state.get_output()
}

fn print_statement(state: &mut PrinterState, statement: &Statement) {
    match statement {
        Statement::FullOpeningTag(_) => {
            state.write("<?php");
            state.new_line();
        }
        Statement::ShortOpeningTag(_) => {
            state.write("<?");
            state.new_line();
        }
        Statement::EchoOpeningTag(_) => {
            state.write("<?= ");
        }
        Statement::ClosingTag(_) => {
            state.write("?>");
            state.new_line();
        }
        Statement::InlineHtml(InlineHtmlStatement { html }) => {
            state.write(html.to_string());
        }
        Statement::Label(LabelStatement {
            comments,
            label,
            colon,
        }) => {
            state.write(label.to_string());
            state.write(":");
        }
        Statement::Goto(GotoStatement {
            comments,
            keyword,
            label,
            semicolon,
        }) => {
            state.write("goto ");
            state.write(label.to_string());
            state.write(";");
        }
        Statement::HaltCompiler(HaltCompilerStatement { content }) => {
            state.write("__halt_compiler();");
            if let Some(content) = content {
                state.write(content.to_string());
            }
        }
        Statement::Static(StaticStatement { vars }) => {
            state.write("static ");
            for (i, StaticVar { var, default }) in vars.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }
                print_variable(state, var);
                if let Some(default) = default {
                    state.write(" = ");
                    print_expression(state, default);
                }
            }
            state.write(";");
        }
        Statement::DoWhile(DoWhileStatement {
            body, condition, ..
        }) => {
            state.write("do ");
            print_statement(state, body);
            state.write(" while (");
            print_expression(state, condition);
            state.write(");");
        }
        Statement::While(WhileStatement {
            r#while,
            left_parenthesis,
            condition,
            right_parenthesis,
            body,
        }) => {
            state.write("while (");
            print_expression(state, condition);
            state.write(") ");
            match body {
                WhileStatementBody::Statement { statement } => {
                    print_statement(state, statement);
                }
                WhileStatementBody::Block {
                    colon,
                    statements,
                    endwhile,
                    ending,
                } => {
                    state.write(":");
                    state.indent();
                    state.new_line();
                    print_statements(state, statements);
                    state.dedent();
                    state.new_line();
                    state.write("endwhile");
                    print_ending(state, ending);
                }
            }
        }
        Statement::For(ForStatement {
            r#for,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        }) => {
            state.write("for (");
            for (i, initialization) in iterator.initializations.inner.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }

                print_expression(state, initialization);
            }
            state.write("; ");
            for (i, condition) in iterator.conditions.inner.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }

                print_expression(state, condition);
            }
            state.write("; ");
            for (i, r#loop) in iterator.r#loop.inner.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }

                print_expression(state, r#loop);
            }
            state.write(") ");
            match body {
                ForStatementBody::Statement { statement } => {
                    print_statement(state, statement);
                }
                ForStatementBody::Block {
                    colon,
                    statements,
                    endfor,
                    ending,
                } => {
                    state.write(":");
                    state.indent();
                    state.new_line();
                    print_statements(state, statements);
                    state.dedent();
                    state.new_line();
                    state.write("endfor");
                    print_ending(state, ending);
                }
            }
        }
        Statement::Foreach(ForeachStatement {
            foreach,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        }) => {
            state.write("foreach (");
            match iterator {
                ForeachStatementIterator::Value {
                    expression,
                    r#as,
                    ampersand,
                    value,
                } => {
                    print_expression(state, expression);
                    state.write(" as ");
                    if let Some(ampersand) = ampersand {
                        state.write("&");
                    }
                    print_expression(state, value);
                }
                ForeachStatementIterator::KeyAndValue {
                    expression,
                    r#as,
                    ampersand,
                    key,
                    double_arrow,
                    value,
                } => {
                    print_expression(state, expression);
                    state.write(" as ");
                    if let Some(ampersand) = ampersand {
                        state.write("&");
                    }
                    print_expression(state, key);
                    state.write(" => ");
                    print_expression(state, value);
                }
            }
            state.write(") ");
            match body {
                ForeachStatementBody::Statement { statement } => {
                    print_statement(state, statement);
                }
                ForeachStatementBody::Block {
                    colon,
                    statements,
                    endforeach,
                    ending,
                } => {
                    state.write(":");
                    state.indent();
                    state.new_line();
                    print_statements(state, statements);
                    state.dedent();
                    state.new_line();
                    state.write("endforeach");
                    print_ending(state, ending);
                }
            }
        }
        Statement::Break(BreakStatement {
            r#break,
            level,
            ending,
        }) => {
            state.write("break");
            if let Some(level) = level {
                state.write(" ");
                print_level(state, level);
            }
            print_ending(state, ending);
        }
        Statement::Continue(ContinueStatement {
            r#continue,
            level,
            ending,
        }) => {
            state.write("continue");
            if let Some(level) = level {
                state.write(" ");
                print_level(state, level);
            }
            print_ending(state, ending);
        }
        Statement::Constant(constant) => print_constant(state, constant),
        Statement::Function(function) => print_function(state, function),
        Statement::Class(class) => print_class(state, class),
        Statement::Trait(trait_) => print_trait(state, trait_),
        Statement::Interface(interface) => print_interface(state, interface),
        Statement::If(statement) => print_if(state, statement),
        Statement::Switch(SwitchStatement {
            switch,
            left_parenthesis,
            condition,
            right_parenthesis,
            cases,
        }) => todo!(),
        Statement::Echo(EchoStatement {
            echo,
            values,
            ending,
        }) => {
            state.write("echo ");
            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }

                print_expression(state, value);
            }
            print_ending(state, ending);
        }
        Statement::Expression(ExpressionStatement { expression, ending }) => {
            print_expression(state, expression);
            print_ending(state, ending);
        }
        Statement::Return(ReturnStatement {
            r#return,
            value,
            ending,
        }) => {
            state.write("return ");
            if let Some(value) = value {
                print_expression(state, value);
            }
            print_ending(state, ending);
        }
        Statement::Namespace(namespace) => match namespace {
            NamespaceStatement::Unbraced(UnbracedNamespace {
                start,
                name,
                end,
                statements,
            }) => {
                state.write("namespace ");
                print_simple_identifier(state, name);
                state.write(";");
                state.new_line();
                state.new_line();
                print_statements(state, statements);
            }
            NamespaceStatement::Braced(BracedNamespace {
                namespace,
                name,
                body:
                    BracedNamespaceBody {
                        start,
                        end,
                        statements,
                    },
            }) => {
                state.write("namespace ");
                if let Some(name) = name {
                    print_simple_identifier(state, name);
                }
                state.write(" {");
                state.indent();
                state.new_line();
                print_statements(state, statements);
                state.dedent();
                state.write("}");
                state.new_line();
                state.new_line();
            }
        },
        Statement::Use(UseStatement { uses, kind }) => todo!(),
        Statement::GroupUse(GroupUseStatement { prefix, kind, uses }) => todo!(),
        Statement::Comment(Comment {
            content, ..
        }) => {
            state.write(content.to_string());
        },
        Statement::Try(_) => todo!(),
        Statement::UnitEnum(unit) => print_unit_enum(state, unit),
        Statement::BackedEnum(backed) => print_backed_enum(state, backed),
        Statement::Block(BlockStatement {
            left_brace,
            statements,
            right_brace,
        }) => {
            state.write("{");
            state.indent();
            state.new_line();
            print_statements(state, statements);
            state.dedent();
            state.new_line();
            state.write("}");
        }
        Statement::Global(GlobalStatement { global, variables }) => todo!(),
        Statement::Declare(_) => todo!(),
        Statement::Noop(_) => {
            state.write(";");
        }
        Statement::TypeAlias(TypeAliasStatement {
            type_keyword,
            name,
            equals,
            r#type,
            semicolon,
        }) => todo!(),
    }

    state.new_line();
}

fn print_if(state: &mut PrinterState, statement: &IfStatement) {
    state.write("if (");
    print_expression(state, &statement.condition);
    state.write(")");
    print_if_statement_body(state, &statement.body);
}

fn print_if_statement_body(state: &mut PrinterState, body: &IfStatementBody) {
    match body {
        IfStatementBody::Statement {
            statement,
            elseifs,
            r#else,
        } => {
            state.write(" ");
            print_statement(state, statement);
            for elseif in elseifs {
                state.write(" ");
                print_elseif(state, elseif);
            }
            if let Some(r#else) = r#else {
                state.write(" ");
                print_else(state, r#else);
            }
        }
        IfStatementBody::Block {
            colon,
            statements,
            elseifs,
            r#else,
            endif,
            ending,
        } => {
            todo!()
        }
    }
}

fn print_else(state: &mut PrinterState, r#else: &IfStatementElse) {
    todo!()
}

fn print_elseif(state: &mut PrinterState, elseif: &IfStatementElseIf) {
    todo!()
}

fn print_interface(state: &mut PrinterState, interface: &InterfaceStatement) {
    state.write("interface ");
    print_simple_identifier(state, &interface.name);
    if let Some(InterfaceExtends { extends, parents }) = &interface.extends {
        state.write(" extends ");
        for (i, parent) in parents.inner.iter().enumerate() {
            if i > 0 {
                state.write(", ");
            }
            print_simple_identifier(state, parent);
        }
    }
    state.write(" {");
    state.indent();
    state.new_line();
    print_interface_body(state, &interface.body);
    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_interface_body(state: &mut PrinterState, body: &InterfaceBody) {
    for (i, member) in body.members.iter().enumerate() {
        if i > 0 {
            state.new_line();
        }
        print_interface_member(state, member);
    }
}

fn print_interface_member(state: &mut PrinterState, member: &InterfaceMember) {
    match member {
        InterfaceMember::Constant(constant) => print_classish_constant(state, constant),
        InterfaceMember::Constructor(method) => print_abstract_constructor(state, method),
        InterfaceMember::Method(method) => print_abstract_method(state, method),
    }
}

fn print_trait(state: &mut PrinterState, r#trait: &TraitStatement) {
    state.write("trait ");
    print_simple_identifier(state, &r#trait.name);
    state.write(" {");
    state.indent();
    state.new_line();
    print_trait_body(state, &r#trait.body);
    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_trait_body(state: &mut PrinterState, body: &TraitBody) {
    for (i, member) in body.members.iter().enumerate() {
        if i > 0 {
            state.new_line();
        }
        print_trait_member(state, member);
    }
}

fn print_trait_member(state: &mut PrinterState, member: &TraitMember) {
    match member {
        TraitMember::Constant(constant) => print_classish_constant(state, constant),
        TraitMember::TraitUsage(usage) => print_trait_usage(state, usage),
        TraitMember::Property(property) => print_property(state, property),
        TraitMember::VariableProperty(property) => print_variable_property(state, property),
        TraitMember::AbstractMethod(method) => print_abstract_method(state, method),
        TraitMember::AbstractConstructor(method) => print_abstract_constructor(state, method),
        TraitMember::ConcreteMethod(method) => print_concrete_method(state, method),
        TraitMember::ConcreteConstructor(method) => print_concrete_constructor(state, method),
    }
}

fn print_unit_enum(state: &mut PrinterState, unit: &UnitEnumStatement) {
    state.write("enum ");
    print_simple_identifier(state, &unit.name);

    for (i, identifier) in unit.implements.iter().enumerate() {
        if i == 0 {
            state.write(" implements ");
        } else {
            state.write(", ");
        }
        print_simple_identifier(state, identifier);
    }

    state.write(" {");
    state.indent();
    state.new_line();
    print_enum_body(state, &unit.body);
    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_backed_enum(state: &mut PrinterState, backed: &BackedEnumStatement) {
    state.write("enum ");
    print_simple_identifier(state, &backed.name);
    state.write(": ");
    match &backed.backed_type {
        BackedEnumType::Int(_, _) => state.write("int"),
        BackedEnumType::String(_, _) => state.write("string"),
    }

    for (i, identifier) in backed.implements.iter().enumerate() {
        if i == 0 {
            state.write(" implements ");
        } else {
            state.write(", ");
        }
        print_simple_identifier(state, identifier);
    }

    state.write(" {");
    state.indent();
    state.new_line();
    print_backed_enum_body(state, &backed.body);
    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_enum_body(state: &mut PrinterState, body: &UnitEnumBody) {
    for (i, member) in body.members.iter().enumerate() {
        print_enum_member(state, member);

        if i < body.members.len() - 1 {
            state.new_line();
            state.new_line();
        }
    }
}

fn print_backed_enum_body(state: &mut PrinterState, body: &BackedEnumBody) {
    for (i, member) in body.members.iter().enumerate() {
        print_backed_enum_member(state, member);

        if i < body.members.len() - 1 {
            state.new_line();
            state.new_line();
        }
    }
}

fn print_enum_member(state: &mut PrinterState, member: &UnitEnumMember) {
    match member {
        UnitEnumMember::Case(case) => print_unit_enum_case(state, case),
        UnitEnumMember::Method(method) => print_concrete_method(state, method),
        UnitEnumMember::Constant(constant) => print_classish_constant(state, constant),
        UnitEnumMember::TraitUsage(usage) => print_trait_usage(state, usage),
    }
}

fn print_backed_enum_member(state: &mut PrinterState, member: &BackedEnumMember) {
    match member {
        BackedEnumMember::Case(case) => print_backed_enum_case(state, case),
        BackedEnumMember::Method(method) => print_concrete_method(state, method),
        BackedEnumMember::Constant(constant) => print_classish_constant(state, constant),
        BackedEnumMember::TraitUsage(usage) => print_trait_usage(state, usage),
    }
}

fn print_unit_enum_case(state: &mut PrinterState, member: &UnitEnumCase) {
    state.write("case ");
    print_simple_identifier(state, &member.name);
    state.write(";");
}

fn print_backed_enum_case(state: &mut PrinterState, member: &BackedEnumCase) {
    state.write("case ");
    print_simple_identifier(state, &member.name);
    state.write(" = ");
    print_expression(state, &member.value);
    state.write(";");
}

fn print_class(state: &mut PrinterState, class: &ClassStatement) {
    print_class_modifier_group(state, &class.modifiers);

    state.write("class ");
    print_simple_identifier(state, &class.name);

    if let Some(ClassExtends { extends, parent }) = &class.extends {
        state.write(" extends ");
        print_simple_identifier(state, parent);
    }

    if let Some(ClassImplements {
        implements,
        interfaces,
    }) = &class.implements
    {
        state.write(" implements ");
        for (i, interface) in interfaces.inner.iter().enumerate() {
            if i > 0 {
                state.write(", ");
            }
            print_simple_identifier(state, interface);
        }
    }

    state.new_line();
    state.write("{");
    state.indent();
    state.new_line();

    for (i, member) in class.body.members.iter().enumerate() {
        print_class_member(state, member);

        if i < class.body.members.len() - 1 {
            state.new_line();
            state.new_line();
        }
    }

    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_class_modifier_group(state: &mut PrinterState, modifiers: &ClassModifierGroup) {
    for (i, modifier) in modifiers.modifiers.iter().enumerate() {
        if i > 0 {
            state.write(" ");
        }
        print_class_modifier(state, modifier);
    }
}

fn print_class_modifier(state: &mut PrinterState, modifier: &ClassModifier) {
    match modifier {
        ClassModifier::Final(_) => state.write("final"),
        ClassModifier::Abstract(_) => state.write("abstract"),
        ClassModifier::Readonly(_) => state.write("readonly"),
    }
}

fn print_class_member(state: &mut PrinterState, member: &ClassMember) {
    match member {
        ClassMember::Constant(constant) => {
            print_classish_constant(state, constant);
        }
        ClassMember::TraitUsage(trait_usage) => print_trait_usage(state, trait_usage),
        ClassMember::Property(property) => print_property(state, property),
        ClassMember::VariableProperty(property) => print_variable_property(state, property),
        ClassMember::AbstractMethod(method) => print_abstract_method(state, method),
        ClassMember::AbstractConstructor(method) => print_abstract_constructor(state, method),
        ClassMember::ConcreteMethod(method) => print_concrete_method(state, method),
        ClassMember::ConcreteConstructor(method) => print_concrete_constructor(state, method),
    }
}

fn print_concrete_constructor(state: &mut PrinterState, method: &ConcreteConstructor) {
    if !method.modifiers.is_empty() {
        print_method_modifier_group(state, &method.modifiers);
        state.write(" ");
    }

    state.write("function ");
    if method.ampersand.is_some() {
        state.write("&");
    }
    state.write("__construct(");
    print_constructor_parameter_list(state, &method.parameters);
    state.write(") {");
    state.indent();
    state.new_line();
    print_statements(state, &method.body.statements);
    state.dedent();
    state.write("}");
}

fn print_constructor_parameter_list(
    state: &mut PrinterState,
    parameters: &ConstructorParameterList,
) {
    for (i, parameter) in parameters.parameters.inner.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }

        print_constructor_parameter(state, parameter);
    }
}

fn print_constructor_parameter(state: &mut PrinterState, parameter: &ConstructorParameter) {
    print_promoted_property_modifier_group(state, &parameter.modifiers);
    state.write(" ");

    if let Some(data_type) = &parameter.data_type {
        print_type(state, data_type);
        state.write(" ");
    }

    if parameter.ellipsis.is_some() {
        state.write("...");
    }

    if parameter.ampersand.is_some() {
        state.write("&");
    }

    print_simple_variable(state, &parameter.name);

    if let Some(default) = &parameter.default {
        state.write(" = ");
        print_expression(state, default);
    }
}

fn print_promoted_property_modifier_group(
    state: &mut PrinterState,
    modifiers: &PromotedPropertyModifierGroup,
) {
    for (i, modifier) in modifiers.modifiers.iter().enumerate() {
        if i > 0 {
            state.write(" ");
        }
        print_promoted_property_modifier(state, modifier);
    }
}

fn print_promoted_property_modifier(state: &mut PrinterState, modifier: &PromotedPropertyModifier) {
    match modifier {
        PromotedPropertyModifier::Public(_) => state.write("public"),
        PromotedPropertyModifier::Protected(_) => state.write("protected"),
        PromotedPropertyModifier::Private(_) => state.write("private"),
        PromotedPropertyModifier::Readonly(_) => state.write("readonly"),
    }
}

fn print_concrete_method(state: &mut PrinterState, method: &ConcreteMethod) {
    if !method.modifiers.is_empty() {
        print_method_modifier_group(state, &method.modifiers);
        state.write(" ");
    }

    state.write("function ");
    if method.ampersand.is_some() {
        state.write("&");
    }
    print_simple_identifier(state, &method.name);
    state.write("(");
    print_function_parameter_list(state, &method.parameters);
    state.write(") ");
    if let Some(ReturnType { data_type, .. }) = &method.return_type {
        state.write(": ");
        print_type(state, data_type);
    }
    state.write(" {");
    state.indent();
    state.new_line();
    print_statements(state, &method.body.statements);
    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_abstract_constructor(state: &mut PrinterState, method: &AbstractConstructor) {
    if !method.modifiers.is_empty() {
        print_method_modifier_group(state, &method.modifiers);
        state.write(" ");
    }

    state.write("function ");
    if method.ampersand.is_some() {
        state.write("&");
    }
    state.write("__construct");
    state.write("(");
    print_function_parameter_list(state, &method.parameters);
    state.write(");");
    state.new_line();
}

fn print_abstract_method(state: &mut PrinterState, method: &AbstractMethod) {
    if !method.modifiers.is_empty() {
        print_method_modifier_group(state, &method.modifiers);
        state.write(" ");
    }

    state.write("function ");
    if method.ampersand.is_some() {
        state.write("&");
    }
    print_simple_identifier(state, &method.name);
    state.write("(");
    print_function_parameter_list(state, &method.parameters);
    state.write(")");
    if let Some(ReturnType { data_type, .. }) = &method.return_type {
        state.write(": ");
        print_type(state, data_type);
    }
    state.write(";");
}

fn print_method_modifier_group(state: &mut PrinterState, modifiers: &MethodModifierGroup) {
    for (i, modifier) in modifiers.modifiers.iter().enumerate() {
        if i > 0 {
            state.write(" ");
        }

        print_method_modifier(state, modifier);
    }
}

fn print_method_modifier(state: &mut PrinterState, modifier: &MethodModifier) {
    match modifier {
        MethodModifier::Final(_) => state.write("final"),
        MethodModifier::Static(_) => state.write("static"),
        MethodModifier::Abstract(_) => state.write("abstract"),
        MethodModifier::Public(_) => state.write("public"),
        MethodModifier::Protected(_) => state.write("protected"),
        MethodModifier::Private(_) => state.write("private"),
    }
}

fn print_variable_property(state: &mut PrinterState, property: &VariableProperty) {
    state.write("var ");
    if let Some(data_type) = &property.r#type {
        print_type(state, data_type);
        state.write(" ");
    }
    for (i, property) in property.entries.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }
        print_property_entry(state, property);
    }
    state.write(";");
}

fn print_property(state: &mut PrinterState, property: &Property) {
    print_modifier_group(state, &property.modifiers);
    state.write(" ");
    if let Some(data_type) = &property.r#type {
        print_type(state, data_type);
        state.write(" ");
    }
    for (i, property) in property.entries.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }
        print_property_entry(state, property);
    }
    state.write(";");
}

fn print_property_entry(state: &mut PrinterState, property: &PropertyEntry) {
    match property {
        PropertyEntry::Uninitialized { variable } => {
            print_simple_variable(state, variable);
        }
        PropertyEntry::Initialized {
            variable,
            equals,
            value,
        } => {
            print_simple_variable(state, variable);
            state.write(" = ");
            print_expression(state, value);
        }
    }
}

fn print_modifier_group(state: &mut PrinterState, modifiers: &PropertyModifierGroup) {
    for (i, modifier) in modifiers.modifiers.iter().enumerate() {
        if i > 0 {
            state.write(" ");
        }

        print_property_modifier(state, modifier);
    }
}

fn print_property_modifier(state: &mut PrinterState, modifier: &PropertyModifier) {
    match modifier {
        PropertyModifier::Public(_) => state.write("public"),
        PropertyModifier::Protected(_) => state.write("protected"),
        PropertyModifier::Private(_) => state.write("private"),
        PropertyModifier::Static(_) => state.write("static"),
        PropertyModifier::Readonly(_) => state.write("readonly"),
    }
}

fn print_trait_usage(state: &mut PrinterState, trait_usage: &TraitUsage) {
    state.write("use ");
    for (i, name) in trait_usage.traits.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }
        print_simple_identifier(state, name);
    }

    if !trait_usage.adaptations.is_empty() {
        state.write(" {");
        state.indent();
        state.new_line();

        for (i, adaptation) in trait_usage.adaptations.iter().enumerate() {
            if i > 0 {
                state.new_line();
            }

            print_trait_adaptation(state, adaptation);
        }

        state.dedent();
        state.new_line();
        state.write("}");
    } else {
        state.write(";");
    }
}

fn print_trait_adaptation(state: &mut PrinterState, adaptation: &TraitUsageAdaptation) {
    match adaptation {
        TraitUsageAdaptation::Alias {
            r#trait,
            method,
            alias,
            visibility,
        } => {
            if let Some(r#trait) = r#trait {
                print_simple_identifier(state, r#trait);
                state.write("::");
            }
            print_simple_identifier(state, method);
            state.write(" as ");
            if let Some(visibility) = visibility {
                print_visibility_modifier(state, visibility);
                state.write(" ");
            }
            print_simple_identifier(state, alias);
        }
        TraitUsageAdaptation::Visibility {
            r#trait,
            method,
            visibility,
        } => {
            if let Some(r#trait) = r#trait {
                print_simple_identifier(state, r#trait);
                state.write("::");
            }
            print_simple_identifier(state, method);
            state.write(" as ");
            print_visibility_modifier(state, visibility);
        }
        TraitUsageAdaptation::Precedence {
            r#trait,
            method,
            insteadof,
        } => {
            if let Some(r#trait) = r#trait {
                print_simple_identifier(state, r#trait);
                state.write("::");
            }
            print_simple_identifier(state, method);
            state.write(" insteadof ");
            for (i, name) in insteadof.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }
                print_simple_identifier(state, name);
            }
        }
    }
    state.write(";");
}

fn print_visibility_modifier(state: &mut PrinterState, modifier: &VisibilityModifier) {
    match modifier {
        VisibilityModifier::Public(_) => state.write("public"),
        VisibilityModifier::Protected(_) => state.write("protected"),
        VisibilityModifier::Private(_) => state.write("private"),
    }
}

fn print_classish_constant(state: &mut PrinterState, constant: &ClassishConstant) {
    state.write("const ");
    print_constant_entries(state, &constant.entries);
    state.write(";");
}

fn print_constant(state: &mut PrinterState, constant: &ConstantStatement) {
    state.write("const ");
    print_constant_entries(state, &constant.entries);
    state.write(";");
}

fn print_constant_entries(state: &mut PrinterState, entries: &[ConstantEntry]) {
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }

        print_constant_entry(state, entry);
    }
}

fn print_constant_entry(state: &mut PrinterState, entry: &ConstantEntry) {
    print_simple_identifier(state, &entry.name);
    state.write(" = ");
    print_expression(state, &entry.value);
}

fn print_level(state: &mut PrinterState, level: &Level) {
    match level {
        Level::Literal(LiteralInteger { value, span }) => state.write(value.to_string()),
        Level::Parenthesized {
            left_parenthesis,
            level,
            right_parenthesis,
        } => {
            state.write("(");
            print_level(state, level);
            state.write(")");
        }
    }
}

fn print_function(state: &mut PrinterState, function: &FunctionStatement) {
    state.write("function ");
    if function.ampersand.is_some() {
        state.write("&");
    }
    print_simple_identifier(state, &function.name);
    state.write("(");
    print_function_parameter_list(state, &function.parameters);
    state.write(")");

    if let Some(ReturnType { data_type, .. }) = &function.return_type {
        state.write(": ");
        print_type(state, data_type);
    }

    state.write(" {");
    state.indent();
    state.new_line();

    print_statements(state, &function.body.statements);

    state.dedent();
    state.new_line();
    state.write("}");
    state.new_line();
}

fn print_function_parameter_list(state: &mut PrinterState, parameters: &FunctionParameterList) {
    for (i, parameter) in parameters.parameters.inner.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }

        if let Some(data_type) = &parameter.data_type {
            print_type(state, data_type);
            state.write(" ");
        }

        print_simple_variable(state, &parameter.name);

        if let Some(default) = &parameter.default {
            state.write(" = ");
            print_expression(state, default);
        }
    }
}

fn print_simple_variable(state: &mut PrinterState, name: &SimpleVariable) {
    state.write(name.name.to_string());
}

fn print_variable_variable(state: &mut PrinterState, variable: &VariableVariable) {
    state.write("$");
    print_variable(state, &variable.variable);
}

fn print_braced_variable_variable(state: &mut PrinterState, variable: &BracedVariableVariable) {
    state.write("${");
    print_expression(state, &variable.variable);
    state.write("}");
}

fn print_type(state: &mut PrinterState, r#type: &Type) {
    state.write(r#type.to_string());
}

fn print_expression(state: &mut PrinterState, expression: &Expression) {
    match expression {
        Expression::Eval(EvalExpression { eval, argument }) => {
            state.write("eval(");
            print_argument(state, &argument.argument);
            state.write(")");
        }
        Expression::Empty(EmptyExpression { empty, argument }) => {
            state.write("empty(");
            print_argument(state, &argument.argument);
            state.write(")");
        }
        Expression::Die(DieExpression { die, argument }) => {
            state.write("die");
            if let Some(argument) = argument.as_ref() {
                state.write("(");
                print_argument(state, &argument.argument);
                state.write(")");
            }
        }
        Expression::Exit(ExitExpression { exit, argument }) => {
            state.write("exit");
            if let Some(argument) = argument.as_ref() {
                state.write("(");
                print_argument(state, &argument.argument);
                state.write(")");
            }
        }
        Expression::Isset(IssetExpression { isset, arguments }) => {
            state.write("isset(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::Unset(UnsetExpression { unset, arguments }) => {
            state.write("unset(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::Print(PrintExpression {
            print,
            value,
            argument,
        }) => {
            state.write("print");
            if let Some(value) = value {
                state.write(" ");
                print_expression(state, value);
            } else if let Some(argument) = argument.as_ref() {
                state.write("(");
                print_argument(state, &argument.argument);
                state.write(")");
            }
        }
        Expression::Literal(literal) => print_literal(state, literal),
        Expression::ArithmeticOperation(operation) => print_arithmetic_operation(state, operation),
        Expression::AssignmentOperation(operation) => print_assignment_operation(state, operation),
        Expression::BitwiseOperation(operation) => print_bitwise_operation(state, operation),
        Expression::ComparisonOperation(operation) => print_comparison_operation(state, operation),
        Expression::LogicalOperation(operation) => print_logical_operation(state, operation),
        Expression::RangeOperation(operation) => print_range_operation(state, operation),
        Expression::Concat(ConcatExpression { left, dot, right }) => {
            print_expression(state, left);
            state.write(" . ");
            print_expression(state, right);
        }
        Expression::Instanceof(InstanceofExpression {
            left,
            instanceof,
            right,
        }) => {
            print_expression(state, left);
            state.write(" instanceof ");
            print_expression(state, right);
        }
        Expression::Reference(ReferenceExpression { ampersand, right }) => {
            state.write("&");
            print_expression(state, right);
        }
        Expression::Parenthesized(ParenthesizedExpression { start, expr, end }) => {
            state.write("(");
            print_expression(state, expr);
            state.write(")");
        }
        Expression::ErrorSuppress(ErrorSuppressExpression { at, expr }) => {
            state.write("@");
            print_expression(state, expr);
        }
        Expression::Identifier(identifier) => print_identifier(state, identifier),
        Expression::Variable(variable) => print_variable(state, variable),
        Expression::Include(IncludeExpression { include, path }) => {
            state.write("include ");
            print_expression(state, path);
        }
        Expression::IncludeOnce(IncludeOnceExpression { include_once, path }) => {
            state.write("include_once ");
            print_expression(state, path);
        }
        Expression::Require(RequireExpression { require, path }) => {
            state.write("require ");
            print_expression(state, path);
        }
        Expression::RequireOnce(RequireOnceExpression { require_once, path }) => {
            state.write("require_once ");
            print_expression(state, path);
        }
        Expression::FunctionCall(FunctionCallExpression { target, arguments }) => {
            print_expression(state, target);
            state.write("(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::FunctionClosureCreation(FunctionClosureCreationExpression {
            target,
            placeholder,
        }) => {
            print_expression(state, target);
            state.write("(...)");
        }
        Expression::MethodCall(MethodCallExpression {
            target,
            arrow,
            method,
            arguments,
        }) => {
            print_expression(state, target);
            state.write("->");
            print_expression(state, method);
            state.write("(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::MethodClosureCreation(MethodClosureCreationExpression {
            target,
            arrow,
            method,
            placeholder,
        }) => {
            print_expression(state, target);
            state.write("->");
            print_expression(state, method);
            state.write("(...)");
        }
        Expression::NullsafeMethodCall(NullsafeMethodCallExpression {
            target,
            question_arrow,
            method,
            arguments,
        }) => {
            print_expression(state, target);
            state.write("?->");
            print_expression(state, method);
            state.write("(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::StaticMethodCall(StaticMethodCallExpression {
            target,
            double_colon,
            method,
            arguments,
        }) => {
            print_expression(state, target);
            state.write("::");
            print_identifier(state, method);
            state.write("(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::StaticVariableMethodCall(StaticVariableMethodCallExpression {
            target,
            double_colon,
            method,
            arguments,
        }) => {
            print_expression(state, target);
            state.write("::");
            print_variable(state, method);
            state.write("(");
            print_argument_list(state, arguments);
            state.write(")");
        }
        Expression::StaticMethodClosureCreation(StaticMethodClosureCreationExpression {
            target,
            double_colon,
            method,
            placeholder,
        }) => {
            print_expression(state, target);
            state.write("::");
            print_identifier(state, method);
            state.write("(...)");
        }
        Expression::StaticVariableMethodClosureCreation(
            StaticVariableMethodClosureCreationExpression {
                target,
                double_colon,
                method,
                placeholder,
            },
        ) => {
            print_expression(state, target);
            state.write("::");
            print_variable(state, method);
            state.write("(...)");
        }
        Expression::PropertyFetch(PropertyFetchExpression {
            target,
            arrow,
            property,
        }) => {
            print_expression(state, target);
            state.write("->");
            print_expression(state, property);
        }
        Expression::NullsafePropertyFetch(NullsafePropertyFetchExpression {
            target,
            question_arrow,
            property,
        }) => {
            print_expression(state, target);
            state.write("?->");
            print_expression(state, property);
        }
        Expression::StaticPropertyFetch(StaticPropertyFetchExpression {
            target,
            double_colon,
            property,
        }) => {
            print_expression(state, target);
            state.write("::");
            print_variable(state, property);
        }
        Expression::ConstantFetch(ConstantFetchExpression {
            target,
            double_colon,
            constant,
        }) => {
            print_expression(state, target);
            state.write("::");
            print_identifier(state, constant);
        }
        Expression::Static => {
            state.write("static");
        }
        Expression::Self_ => {
            state.write("self");
        }
        Expression::Parent => {
            state.write("parent");
        }
        Expression::ShortArray(ShortArrayExpression { start, items, end }) => {
            state.write("[");
            state.indent();
            state.new_line();
            print_array_items(state, items);
            state.dedent();
            state.new_line();
            state.write("]");
        }
        Expression::Array(ArrayExpression {
            array,
            start,
            items,
            end,
        }) => {
            state.write("array(");
            state.indent();
            state.new_line();
            print_array_items(state, items);
            state.dedent();
            state.new_line();
            state.write(")");
        }
        Expression::List(ListExpression {
            list,
            start,
            items,
            end,
        }) => {
            state.write("list(");
            state.indent();
            state.new_line();
            print_list_items(state, items);
            state.dedent();
            state.new_line();
            state.write(")");
        }
        Expression::Closure(closure) => print_closure(state, closure),
        Expression::ArrowFunction(_) => todo!(),
        Expression::New(NewExpression {
            new,
            target,
            arguments,
        }) => print_new(state, target, arguments.as_ref()),
        Expression::InterpolatedString(InterpolatedStringExpression { parts }) => todo!(),
        Expression::Heredoc(HeredocExpression { parts }) => todo!(),
        Expression::Nowdoc(NowdocExpression { value }) => todo!(),
        Expression::ShellExec(ShellExecExpression { parts }) => todo!(),
        Expression::AnonymousClass(_) => todo!(),
        Expression::Bool(BoolExpression { value }) => {
            state.write(value.to_string());
        }
        Expression::ArrayIndex(ArrayIndexExpression {
            array,
            left_bracket,
            index,
            right_bracket,
        }) => todo!(),
        Expression::Null => todo!(),
        Expression::MagicConstant(constant) => print_magic_constant(state, constant),
        Expression::ShortTernary(ShortTernaryExpression {
            condition,
            question_colon,
            r#else,
        }) => todo!(),
        Expression::Ternary(TernaryExpression {
            condition,
            question,
            then,
            colon,
            r#else,
        }) => todo!(),
        Expression::Coalesce(CoalesceExpression {
            lhs,
            double_question,
            rhs,
        }) => todo!(),
        Expression::Clone(CloneExpression { target }) => todo!(),
        Expression::Match(MatchExpression {
            keyword,
            condition,
            default,
            arms,
            ..
        }) => {
            state.write("match (");
            print_expression(state, condition);
            state.write(") {");
            state.indent();
            state.new_line();
            for arm in arms {
                print_match_arm(state, arm);
            }
            if let Some(default) = default {
                print_default_match_arm(state, default);
            }
            state.dedent();
            state.new_line();
            state.write("}");
        }
        Expression::ShortMatch(ShortMatchExpression {
            keyword,
            default,
            arms,
        }) => todo!(),
        Expression::Throw(ThrowExpression { value }) => {
            state.write("throw ");
            print_expression(state, value);
        },
        Expression::Yield(YieldExpression { key, value }) => {
            state.write("yield ");
            if let Some(key) = key {
                print_expression(state, key);
                state.write(" => ");
            }
            if let Some(value) = value {
                print_expression(state, value);
            }
        },
        Expression::YieldFrom(YieldFromExpression { value }) => {
            state.write("yield from ");
            print_expression(state, value);
        },
        Expression::Cast(CastExpression { cast, kind, value }) => {
            state.write(format!("({})", match kind {
                CastKind::Array => "array",
                CastKind::Bool => "bool",
                CastKind::Float => "float",
                CastKind::Int => "int",
                CastKind::Object => "object",
                CastKind::String => "string",
                CastKind::Unset => "unset",
            }));

            print_expression(state, value);
        },
        Expression::Noop => {},
    }
}

fn print_new(state: &mut PrinterState, target: &Expression, arguments: Option<&ArgumentList>) {
    state.write("new ");
    print_expression(state, target);
    state.write("(");
    if let Some(arguments) = arguments {
        print_argument_list(state, arguments);
    }
    state.write(")");
}

fn print_magic_constant(state: &mut PrinterState, constant: &MagicConstantExpression) {
    match constant {
        MagicConstantExpression::Directory(_) => state.write("__DIR__"),
        MagicConstantExpression::File(_) => state.write("__FILE__"),
        MagicConstantExpression::Line(_) => state.write("__LINE__"),
        MagicConstantExpression::Class(_) => state.write("__CLASS__"),
        MagicConstantExpression::Function(_) => state.write("__FUNCTION__"),
        MagicConstantExpression::Method(_) => state.write("__METHOD__"),
        MagicConstantExpression::Namespace(_) => state.write("__NAMESPACE__"),
        MagicConstantExpression::Trait(_) => state.write("__TRAIT__"),
        MagicConstantExpression::CompilerHaltOffset(_) => state.write("__COMPILER_HALT_OFFSET__"),
    }
}

fn print_closure(state: &mut PrinterState, closure: &ClosureExpression) {
    state.write("function ");
    if closure.ampersand.is_some() {
        state.write("&");
    }
    state.write("(");
    print_function_parameter_list(state, &closure.parameters);
    state.write(")");

    if let Some(uses) = &closure.uses {
        state.write(" use (");
        for r#use in uses.variables.inner.iter() {
            if r#use.ampersand.is_some() {
                state.write("&");
            }

            print_simple_variable(state, &r#use.variable);
        }
        state.write(")");
    }

    if let Some(ReturnType { data_type, .. }) = &closure.return_type {
        state.write(": ");
        print_type(state, &data_type);
    }

    state.write(" {");
    state.indent();
    state.new_line();

    print_statements(state, &closure.body.statements);

    state.dedent();
    state.new_line();
    state.write("}");
}

fn print_list_items(state: &mut PrinterState, items: &[ListEntry]) {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }

        match item {
            ListEntry::Skipped => {}
            ListEntry::Value { value } => {
                print_expression(state, value);
            }
            ListEntry::KeyValue {
                key,
                double_arrow,
                value,
            } => {
                print_expression(state, key);
                state.write(" => ");
                print_expression(state, value);
            }
        }
    }
}

fn print_array_items(state: &mut PrinterState, items: &CommaSeparated<ArrayItem>) {
    for (i, item) in items.inner.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }

        print_array_item(state, item);
    }
}

fn print_array_item(state: &mut PrinterState, item: &ArrayItem) {
    match item {
        ArrayItem::Skipped => {}
        ArrayItem::Value { value } => {
            print_expression(state, value);
        }
        ArrayItem::ReferencedValue { ampersand, value } => {
            state.write("&");
            print_expression(state, value);
        }
        ArrayItem::SpreadValue { ellipsis, value } => {
            state.write("...");
            print_expression(state, value);
        }
        ArrayItem::KeyValue {
            key,
            double_arrow,
            value,
        } => {
            print_expression(state, key);
            state.write(" => ");
            print_expression(state, value);
        }
        ArrayItem::ReferencedKeyValue {
            key,
            double_arrow,
            ampersand,
            value,
        } => {
            print_expression(state, key);
            state.write(" => &");
            print_expression(state, value);
        }
    }
}

fn print_range_operation(state: &mut PrinterState, operation: &RangeOperationExpression) {
    match operation {
        RangeOperationExpression::Exclusive {
            lower_bound,
            double_dot,
            upper_bound,
        } => {
            print_expression(state, lower_bound);
            state.write("..");
            print_expression(state, upper_bound);
        }
        RangeOperationExpression::Inclusive {
            lower_bound,
            double_dot_equals,
            upper_bound,
        } => {
            print_expression(state, lower_bound);
            state.write("..=");
            print_expression(state, upper_bound);
        }
        RangeOperationExpression::Endless {
            lower_bound,
            double_dot,
        } => {
            print_expression(state, lower_bound);
            state.write("..");
        }
    }
}

fn print_logical_operation(state: &mut PrinterState, operation: &LogicalOperationExpression) {
    match operation {
        LogicalOperationExpression::And {
            left,
            double_ampersand,
            right,
        } => {
            print_expression(state, left);
            state.write(" && ");
            print_expression(state, right);
        }
        LogicalOperationExpression::Or {
            left,
            double_pipe,
            right,
        } => {
            print_expression(state, left);
            state.write(" || ");
            print_expression(state, right);
        }
        LogicalOperationExpression::Not { bang, right } => {
            state.write("!");
            print_expression(state, right);
        }
        LogicalOperationExpression::LogicalAnd { left, and, right } => {
            print_expression(state, left);
            state.write(" and ");
            print_expression(state, right);
        }
        LogicalOperationExpression::LogicalOr { left, or, right } => {
            print_expression(state, left);
            state.write(" or ");
            print_expression(state, right);
        }
        LogicalOperationExpression::LogicalXor { left, xor, right } => {
            print_expression(state, left);
            state.write(" xor ");
            print_expression(state, right);
        }
    }
}

fn print_comparison_operation(state: &mut PrinterState, operation: &ComparisonOperationExpression) {
    match operation {
        ComparisonOperationExpression::Equal {
            left,
            double_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" == ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::Identical {
            left,
            triple_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" === ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::NotEqual {
            left,
            bang_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" != ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::AngledNotEqual {
            left,
            angled_left_right,
            right,
        } => {
            print_expression(state, left);
            state.write(" <> ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::NotIdentical {
            left,
            bang_double_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" !== ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::LessThan {
            left,
            less_than,
            right,
        } => {
            print_expression(state, left);
            state.write(" < ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::GreaterThan {
            left,
            greater_than,
            right,
        } => {
            print_expression(state, left);
            state.write(" > ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::LessThanOrEqual {
            left,
            less_than_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" <= ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::GreaterThanOrEqual {
            left,
            greater_than_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" >= ");
            print_expression(state, right);
        }
        ComparisonOperationExpression::Spaceship {
            left,
            spaceship,
            right,
        } => {
            print_expression(state, left);
            state.write(" <=> ");
            print_expression(state, right);
        }
    }
}

fn print_bitwise_operation(state: &mut PrinterState, operation: &BitwiseOperationExpression) {
    match operation {
        BitwiseOperationExpression::And { left, and, right } => {
            print_expression(state, left);
            state.write(" & ");
            print_expression(state, right);
        }
        BitwiseOperationExpression::Or { left, or, right } => {
            print_expression(state, left);
            state.write(" | ");
            print_expression(state, right);
        }
        BitwiseOperationExpression::Xor { left, xor, right } => {
            print_expression(state, left);
            state.write(" ^ ");
            print_expression(state, right);
        }
        BitwiseOperationExpression::LeftShift {
            left,
            left_shift,
            right,
        } => {
            print_expression(state, left);
            state.write(" << ");
            print_expression(state, right);
        }
        BitwiseOperationExpression::RightShift {
            left,
            right_shift,
            right,
        } => {
            print_expression(state, left);
            state.write(" >> ");
            print_expression(state, right);
        }
        BitwiseOperationExpression::Not { not, right } => {
            state.write("~");
            print_expression(state, right);
        }
    }
}

fn print_assignment_operation(state: &mut PrinterState, operation: &AssignmentOperationExpression) {
    match operation {
        AssignmentOperationExpression::Assign {
            left,
            equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" = ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Addition {
            left,
            plus_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" += ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Subtraction {
            left,
            minus_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" -= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Multiplication {
            left,
            asterisk_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" *= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Division {
            left,
            slash_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" /= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Modulo {
            left,
            percent_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" %= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Exponentiation {
            left,
            pow_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" **= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Concat {
            left,
            dot_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" .= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::BitwiseAnd {
            left,
            ampersand_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" &= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::BitwiseOr {
            left,
            pipe_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" |= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::BitwiseXor {
            left,
            caret_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" ^= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::LeftShift {
            left,
            left_shift_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" <<= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::RightShift {
            left,
            right_shift_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" >>= ");
            print_expression(state, right);
        }
        AssignmentOperationExpression::Coalesce {
            left,
            coalesce_equals,
            right,
        } => {
            print_expression(state, left);
            state.write(" ??= ");
            print_expression(state, right);
        }
    }
}

fn print_identifier(state: &mut PrinterState, identifier: &Identifier) {
    match identifier {
        Identifier::SimpleIdentifier(identifier) => print_simple_identifier(state, identifier),
        Identifier::DynamicIdentifier(_) => todo!(),
    }
}

fn print_argument_list(state: &mut PrinterState, arguments: &ArgumentList) {
    for (i, argument) in arguments.arguments.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }

        print_argument(state, argument);
    }
}

fn print_argument(state: &mut PrinterState, argument: &Argument) {
    match argument {
        Argument::Positional(PositionalArgument {
            comments,
            ellipsis,
            value,
        }) => {
            if ellipsis.is_some() {
                state.write("...");
            }
            print_expression(state, value);
        }
        Argument::Named(NamedArgument {
            comments,
            name,
            colon,
            ellipsis,
            value,
        }) => {
            print_simple_identifier(state, name);
            if ellipsis.is_some() {
                state.write("...");
            }
            state.write(": ");
            print_expression(state, value);
        }
    }
}

fn print_variable(state: &mut PrinterState, variable: &Variable) {
    match variable {
        Variable::SimpleVariable(variable) => print_simple_variable(state, variable),
        Variable::VariableVariable(variable) => print_variable_variable(state, variable),
        Variable::BracedVariableVariable(variable) => print_braced_variable_variable(state, variable),
    }
}

fn print_arithmetic_operation(state: &mut PrinterState, operation: &ArithmeticOperationExpression) {
    match operation {
        ArithmeticOperationExpression::Addition { left, plus, right } => {
            print_expression(state, left);
            state.write(" + ");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Subtraction { left, minus, right } => {
            print_expression(state, left);
            state.write(" - ");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Multiplication {
            left,
            asterisk,
            right,
        } => {
            print_expression(state, left);
            state.write(" * ");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Division { left, slash, right } => {
            print_expression(state, left);
            state.write(" / ");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Modulo {
            left,
            percent,
            right,
        } => {
            print_expression(state, left);
            state.write(" % ");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Exponentiation { left, pow, right } => {
            print_expression(state, left);
            state.write(" ** ");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Negative { minus, right } => {
            state.write("-");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::Positive { plus, right } => {
            state.write("+");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::PreIncrement { increment, right } => {
            state.write("++");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::PostIncrement { left, increment } => {
            print_expression(state, left);
            state.write("++");
        }
        ArithmeticOperationExpression::PreDecrement { decrement, right } => {
            state.write("--");
            print_expression(state, right);
        }
        ArithmeticOperationExpression::PostDecrement { left, decrement } => {
            print_expression(state, left);
            state.write("--");
        }
    }
}

fn print_literal(state: &mut PrinterState, literal: &Literal) {
    match literal {
        Literal::String(LiteralString { value, .. }) => {
            state.write(value.to_string());
        }
        Literal::Integer(LiteralInteger { value, .. }) => {
            state.write(value.to_string());
        }
        Literal::Float(LiteralFloat { value, .. }) => {
            state.write(value.to_string());
        },
    }
}

fn print_default_match_arm(state: &mut PrinterState, default: &DefaultMatchArm) {
    state.write("default => ");
    print_match_arm_body(state, &default.body);
}

fn print_match_arm(state: &mut PrinterState, arm: &MatchArm) {
    for (i, condition) in arm.conditions.iter().enumerate() {
        if i > 0 {
            state.write(", ");
        }
        print_expression(state, condition);
    }

    state.write(" => ");
    print_match_arm_body(state, &arm.body);
    state.write(",");
    state.new_line();
}

fn print_match_arm_body(state: &mut PrinterState, body: &MatchArmBody) {
    match body {
        MatchArmBody::Block { statements, .. } => {
            state.write("{");
            state.indent();
            state.new_line();
            print_statements(state, statements);
            state.dedent();
            state.new_line();
            state.write("}");
        }
        MatchArmBody::Expression { expression } => {
            print_expression(state, expression);
        }
    }
}

fn print_statements(state: &mut PrinterState, statements: &[Statement]) {
    for statement in statements {
        print_statement(state, statement);
    }
}

fn print_ending(state: &mut PrinterState, ending: &Ending) {
    match ending {
        Ending::Semicolon(_) => state.write(";"),
        Ending::CloseTag(_) => state.write("?>"),
    }
}

fn print_simple_identifier(state: &mut PrinterState, identifier: &SimpleIdentifier) {
    state.write(identifier.value.to_string());
}
