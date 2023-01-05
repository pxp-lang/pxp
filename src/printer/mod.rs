use pxp_parser::parser::ast::{Statement, Expression, Ending, namespaces::{Namespace, UnbracedNamespace, BracedNamespace, BracedNamespaceBody}, identifiers::{SimpleIdentifier, Identifier}, MatchArm, DefaultMatchArm, MatchArmBody, literals::{Literal, LiteralString, LiteralInteger}, functions::{Function, FunctionParameterList, ReturnType, FunctionBody}, data_type::Type, variables::{SimpleVariable, Variable}, comments::{Comment, CommentFormat}, operators::ArithmeticOperation, arguments::{ArgumentList, Argument}, goto::{GotoLabel, GotoStatement}, StaticVar, loops::{DoWhileStatement, WhileStatement, WhileStatementBody, ForStatement, ForStatementBody, ForeachStatement, ForeachStatementIterator, ForeachStatementBody, BreakStatement, Level, ContinueStatement}, constant::{Constant, ConstantEntry, ClassishConstant}, classes::{Class, ClassExtends, ClassImplements, ClassMember}, traits::{TraitUsage, TraitUsageAdaptation}, modifiers::{VisibilityModifier, PropertyModifierGroup, PropertyModifier, ClassModifierGroup, ClassModifier}, properties::{Property, PropertyEntry, VariableProperty}};

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
        },
        Statement::ShortOpeningTag(_) => {
            state.write("<?");
            state.new_line();
        },
        Statement::EchoOpeningTag(_) => {
            state.write("<?= ");
        },
        Statement::ClosingTag(_) => {
            state.write("?>");
            state.new_line();
        },
        Statement::InlineHtml(html) => {
            state.write(html.to_string());
        },
        Statement::GotoLabel(GotoLabel { comments, label, colon }) => {
            state.write(label.to_string());
            state.write(":");
        },
        Statement::Goto(GotoStatement { comments, keyword, label, semicolon }) => {
            state.write("goto ");
            state.write(label.to_string());
            state.write(";");
        },
        Statement::HaltCompiler { content } => {
            state.write("__halt_compiler();");
            if let Some(content) = content {
                state.write(content.to_string());
            }
        },
        Statement::Static { vars } => {
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
        },
        Statement::DoWhile(DoWhileStatement { body, condition, .. }) => {
            state.write("do ");
            print_statement(state, body);
            state.write(" while (");
            print_expression(state, condition);
            state.write(");");
        },
        Statement::While(WhileStatement { r#while, left_parenthesis, condition, right_parenthesis, body }) => {
            state.write("while (");
            print_expression(state, condition);
            state.write(") ");
            match body {
                WhileStatementBody::Statement(statement) => {
                    print_statement(state, statement);
                },
                WhileStatementBody::Block { colon, statements, endwhile, ending } => {
                    state.write(":");
                    state.indent();
                    state.new_line();
                    print_statements(state, statements);
                    state.dedent();
                    state.new_line();
                    state.write("endwhile");
                    print_ending(state, ending);
                },
            }
        },
        Statement::For(ForStatement { r#for, left_parenthesis, iterator, right_parenthesis, body }) => {
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
                ForStatementBody::Statement(statement) => {
                    print_statement(state, statement);
                },
                ForStatementBody::Block { colon, statements, endfor, ending } => {
                    state.write(":");
                    state.indent();
                    state.new_line();
                    print_statements(state, statements);
                    state.dedent();
                    state.new_line();
                    state.write("endfor");
                    print_ending(state, ending);
                },
            }
        },
        Statement::Foreach(ForeachStatement { foreach, left_parenthesis, iterator, right_parenthesis, body }) => {
            state.write("foreach (");
            match iterator {
                ForeachStatementIterator::Value { expression, r#as, ampersand, value } => {
                    print_expression(state, expression);
                    state.write(" as ");
                    if let Some(ampersand) = ampersand {
                        state.write("&");
                    }
                    print_expression(state, value);
                },
                ForeachStatementIterator::KeyAndValue { expression, r#as, ampersand, key, double_arrow, value } => {
                    print_expression(state, expression);
                    state.write(" as ");
                    if let Some(ampersand) = ampersand {
                        state.write("&");
                    }
                    print_expression(state, key);
                    state.write(" => ");
                    print_expression(state, value);
                },
            }
            state.write(") ");
            match body {
                ForeachStatementBody::Statement(statement) => {
                    print_statement(state, statement);
                },
                ForeachStatementBody::Block { colon, statements, endforeach, ending } => {
                    state.write(":");
                    state.indent();
                    state.new_line();
                    print_statements(state, statements);
                    state.dedent();
                    state.new_line();
                    state.write("endforeach");
                    print_ending(state, ending);
                },
            }
        },
        Statement::Break(BreakStatement { r#break, level, ending }) => {
            state.write("break");
            if let Some(level) = level {
                state.write(" ");
                print_level(state, level);
            }
            print_ending(state, ending);
        },
        Statement::Continue(ContinueStatement { r#continue, level, ending }) => {
            state.write("continue");
            if let Some(level) = level {
                state.write(" ");
                print_level(state, level);
            }
            print_ending(state, ending);
        },
        Statement::Constant(constant) => print_constant(state, constant),
        Statement::Function(function) => print_function(state, function),
        Statement::Class(class) => print_class(state, class),
        Statement::Trait(_) => todo!(),
        Statement::Interface(_) => todo!(),
        Statement::If(_) => todo!(),
        Statement::Switch { switch, left_parenthesis, condition, right_parenthesis, cases } => todo!(),
        Statement::Echo { echo, values, ending } => {
            state.write("echo ");
            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    state.write(", ");
                }

                print_expression(state, value);
            }
            print_ending(state, ending);
        },
        Statement::Expression { expression, ending } => {
            print_expression(state, expression);
            print_ending(state, ending);
        },
        Statement::Return { r#return, value, ending } => {
            state.write("return ");
            if let Some(value) = value {
                print_expression(state, value);
            }            
            print_ending(state, ending);
        },
        Statement::Namespace(namespace) => {
            match namespace {
                Namespace::Unbraced(UnbracedNamespace { start, name, end, statements }) => {
                    state.write("namespace ");
                    print_simple_identifier(state, name);
                    state.write(";");
                    state.new_line();
                    state.new_line();
                    print_statements(state, statements);
                },
                Namespace::Braced(BracedNamespace { namespace, name, body: BracedNamespaceBody { start, end, statements } }) => {
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
                },
            }
        },
        Statement::Use { uses, kind } => todo!(),
        Statement::GroupUse { prefix, kind, uses } => todo!(),
        Statement::Comment(Comment { format, content, .. }) => {
            match format {
                CommentFormat::SingleLine => {
                    state.write("// ");
                    state.write(content.to_string());
                },
                CommentFormat::MultiLine => todo!(),
                CommentFormat::HashMark => todo!(),
                CommentFormat::Document => todo!(),
            }
        },
        Statement::Try(_) => todo!(),
        Statement::UnitEnum(_) => todo!(),
        Statement::BackedEnum(_) => todo!(),
        Statement::Block { left_brace, statements, right_brace } => todo!(),
        Statement::Global { global, variables } => todo!(),
        Statement::Declare(_) => todo!(),
        Statement::Noop(_) => {
            state.write(";");
        },
        Statement::TypeAlias { type_keyword, name, equals, r#type, semicolon } => todo!(),
    }

    state.new_line();
}

fn print_class(state: &mut PrinterState, class: &Class) {
    print_class_modifier_group(state, &class.modifiers);

    state.write("class ");
    print_simple_identifier(state, &class.name);
    
    if let Some(ClassExtends { extends, parent }) = &class.extends {
        state.write(" extends ");
        print_simple_identifier(state, parent);
    }

    if let Some(ClassImplements { implements, interfaces }) = &class.implements {
        state.write(" implements ");
        for (i, interface) in interfaces.inner.iter().enumerate() {
            if i > 0 {
                state.write(", ");
            }
            print_simple_identifier(state, interface);
        }
    }

    state.write(" {");
    state.indent();
    state.new_line();

    for member in class.body.members.iter() {
        print_class_member(state, member);
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
        },
        ClassMember::TraitUsage(trait_usage) => print_trait_usage(state, trait_usage),
        ClassMember::Property(property) => print_property(state, property),
        ClassMember::VariableProperty(property) => print_variable_property(state, property),
        ClassMember::AbstractMethod(_) => todo!(),
        ClassMember::AbstractConstructor(_) => todo!(),
        ClassMember::ConcreteMethod(_) => todo!(),
        ClassMember::ConcreteConstructor(_) => todo!(),
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
        },
        PropertyEntry::Initialized { variable, equals, value } => {
            print_simple_variable(state, variable);
            state.write(" = ");
            print_expression(state, value);
        },
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
        state.write("{");
        state.indent();
        state.new_line();

        for adaptation in trait_usage.adaptations.iter() {
            print_trait_adaptation(state, adaptation);
        }

        state.dedent();
        state.new_line();
        state.write("}");
    } else {
        state.write(";");
        state.new_line();
    }
}

fn print_trait_adaptation(state: &mut PrinterState, adaptation: &TraitUsageAdaptation) {
    match adaptation {
        TraitUsageAdaptation::Alias { r#trait, method, alias, visibility } => {
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
        },
        TraitUsageAdaptation::Visibility { r#trait, method, visibility } => {
            if let Some(r#trait) = r#trait {
                print_simple_identifier(state, r#trait);
                state.write("::");
            }
            print_simple_identifier(state, method);
            state.write(" as ");
            print_visibility_modifier(state, visibility);
        },
        TraitUsageAdaptation::Precedence { r#trait, method, insteadof } => {
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
        },
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

fn print_constant(state: &mut PrinterState, constant: &Constant) {
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
        Level::Literal(LiteralInteger { value, span }) => {
            state.write(value.to_string())
        },
        Level::Parenthesized { left_parenthesis, level, right_parenthesis } => {
            state.write("(");
            print_level(state, level);
            state.write(")");
        },
    }
}

fn print_function(state: &mut PrinterState, function: &Function) {
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

fn print_type(state: &mut PrinterState, r#type: &Type) {
    state.write(r#type.to_string());
}

fn print_expression(state: &mut PrinterState, expression: &Expression) {
    match expression {
        Expression::Eval { eval, argument } => todo!(),
        Expression::Empty { empty, argument } => todo!(),
        Expression::Die { die, argument } => todo!(),
        Expression::Exit { exit, argument } => todo!(),
        Expression::Isset { isset, arguments } => todo!(),
        Expression::Unset { unset, arguments } => todo!(),
        Expression::Print { print, value, argument } => todo!(),
        Expression::Literal(literal) => print_literal(state, literal),
        Expression::ArithmeticOperation(operation) => print_arithmetic_operation(state, operation),
        Expression::AssignmentOperation(_) => todo!(),
        Expression::BitwiseOperation(_) => todo!(),
        Expression::ComparisonOperation(_) => todo!(),
        Expression::LogicalOperation(_) => todo!(),
        Expression::RangeOperation(_) => todo!(),
        Expression::Concat { left, dot, right } => todo!(),
        Expression::Instanceof { left, instanceof, right } => todo!(),
        Expression::Reference { ampersand, right } => todo!(),
        Expression::Parenthesized { start, expr, end } => todo!(),
        Expression::ErrorSuppress { at, expr } => todo!(),
        Expression::Identifier(identifier) => print_identifier(state, identifier),
        Expression::Variable(variable) => print_variable(state, variable),
        Expression::Include { include, path } => todo!(),
        Expression::IncludeOnce { include_once, path } => todo!(),
        Expression::Require { require, path } => todo!(),
        Expression::RequireOnce { require_once, path } => todo!(),
        Expression::FunctionCall { target, arguments } => {
            print_expression(state, target);
            state.write("(");
            print_argument_list(state, arguments);
            state.write(")");
        },
        Expression::FunctionClosureCreation { target, placeholder } => todo!(),
        Expression::MethodCall { target, arrow, method, arguments } => todo!(),
        Expression::MethodClosureCreation { target, arrow, method, placeholder } => todo!(),
        Expression::NullsafeMethodCall { target, question_arrow, method, arguments } => todo!(),
        Expression::StaticMethodCall { target, double_colon, method, arguments } => todo!(),
        Expression::StaticVariableMethodCall { target, double_colon, method, arguments } => todo!(),
        Expression::StaticMethodClosureCreation { target, double_colon, method, placeholder } => todo!(),
        Expression::StaticVariableMethodClosureCreation { target, double_colon, method, placeholder } => todo!(),
        Expression::PropertyFetch { target, arrow, property } => todo!(),
        Expression::NullsafePropertyFetch { target, question_arrow, property } => todo!(),
        Expression::StaticPropertyFetch { target, double_colon, property } => todo!(),
        Expression::ConstantFetch { target, double_colon, constant } => todo!(),
        Expression::Static => todo!(),
        Expression::Self_ => todo!(),
        Expression::Parent => todo!(),
        Expression::ShortArray { start, items, end } => todo!(),
        Expression::Array { array, start, items, end } => todo!(),
        Expression::List { list, start, items, end } => todo!(),
        Expression::Closure(_) => todo!(),
        Expression::ArrowFunction(_) => todo!(),
        Expression::New { new, target, arguments } => todo!(),
        Expression::InterpolatedString { parts } => todo!(),
        Expression::Heredoc { parts } => todo!(),
        Expression::Nowdoc { value } => todo!(),
        Expression::ShellExec { parts } => todo!(),
        Expression::AnonymousClass(_) => todo!(),
        Expression::Bool { value } => {
            state.write(value.to_string());
        },
        Expression::ArrayIndex { array, left_bracket, index, right_bracket } => todo!(),
        Expression::Null => todo!(),
        Expression::MagicConstant(_) => todo!(),
        Expression::ShortTernary { condition, question_colon, r#else } => todo!(),
        Expression::Ternary { condition, question, then, colon, r#else } => todo!(),
        Expression::Coalesce { lhs, double_question, rhs } => todo!(),
        Expression::Clone { target } => todo!(),
        Expression::Match { keyword, left_parenthesis, condition, right_parenthesis, default, arms } => {
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
        },
        Expression::ShortMatch { keyword, default, arms } => todo!(),
        Expression::Throw { value } => todo!(),
        Expression::Yield { key, value } => todo!(),
        Expression::YieldFrom { value } => todo!(),
        Expression::Cast { cast, kind, value } => todo!(),
        Expression::Noop => todo!(),
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

        match argument {
            Argument::Positional { comments, ellipsis, value } => {
                if ellipsis.is_some() {
                    state.write("...");
                }
                print_expression(state, value);
            },
            Argument::Named { comments, name, colon, ellipsis, value } => {
                print_simple_identifier(state, name);
                if ellipsis.is_some() {
                    state.write("...");
                }
                state.write(": ");
                print_expression(state, value);
            },
        }
    }
}

fn print_variable(state: &mut PrinterState, variable: &Variable) {
    match variable {
        Variable::SimpleVariable(variable) => print_simple_variable(state, variable),
        Variable::VariableVariable(_) => todo!(),
        Variable::BracedVariableVariable(_) => todo!(),
    }
}

fn print_arithmetic_operation(state: &mut PrinterState, operation: &ArithmeticOperation) {
    match operation {
        ArithmeticOperation::Addition { left, plus, right } => {
            print_expression(state, left);
            state.write(" + ");
            print_expression(state, right);
        },
        ArithmeticOperation::Subtraction { left, minus, right } => {
            print_expression(state, left);
            state.write(" - ");
            print_expression(state, right);
        }
        ArithmeticOperation::Multiplication { left, asterisk, right } => {
            print_expression(state, left);
            state.write(" * ");
            print_expression(state, right);
        }
        ArithmeticOperation::Division { left, slash, right } => {
            print_expression(state, left);
            state.write(" / ");
            print_expression(state, right);
        }
        ArithmeticOperation::Modulo { left, percent, right } => {
            print_expression(state, left);
            state.write(" % ");
            print_expression(state, right);
        }
        ArithmeticOperation::Exponentiation { left, pow, right } => {
            print_expression(state, left);
            state.write(" ** ");
            print_expression(state, right);
        }
        ArithmeticOperation::Negative { minus, right } => {
            state.write("-");
            print_expression(state, right);
        },
        ArithmeticOperation::Positive { plus, right } => {
            state.write("+");
            print_expression(state, right);
        },
        ArithmeticOperation::PreIncrement { increment, right } => {
            state.write("++");
            print_expression(state, right);
        },
        ArithmeticOperation::PostIncrement { left, increment } => {
            print_expression(state, left);
            state.write("++");
        },
        ArithmeticOperation::PreDecrement { decrement, right } => {
            state.write("--");
            print_expression(state, right);
        },
        ArithmeticOperation::PostDecrement { left, decrement } => {
            print_expression(state, left);
            state.write("--");
        },
    }
}

fn print_literal(state: &mut PrinterState, literal: &Literal) {
    match literal {
        Literal::String(LiteralString { value, span }) => {
            state.write(value.to_string());
        },
        Literal::Integer(LiteralInteger { value, .. }) => {
            state.write(value.to_string());
        },
        Literal::Float(_) => todo!(),
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
}

fn print_match_arm_body(state: &mut PrinterState, body: &MatchArmBody) {
    match body {
        MatchArmBody::Block { statements, .. } => {
            state.write("{");
            state.new_line();
            state.indent();
            print_statements(state, statements);
            state.dedent();
            state.write("}");
        },
        MatchArmBody::Expression(expression) => {
            print_expression(state, expression);
        },
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