use pxp_parser::parser::ast::{Statement, Expression, Ending, namespaces::{Namespace, UnbracedNamespace, BracedNamespace, BracedNamespaceBody}, identifiers::{SimpleIdentifier, Identifier}, MatchArm, DefaultMatchArm, MatchArmBody, literals::{Literal, LiteralString, LiteralInteger}, functions::{Function, FunctionParameterList, ReturnType, FunctionBody}, data_type::Type, variables::{SimpleVariable, Variable}, comments::{Comment, CommentFormat}, operators::ArithmeticOperation, arguments::{ArgumentList, Argument}};

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
        Statement::ShortOpeningTag(_) => todo!(),
        Statement::EchoOpeningTag(_) => todo!(),
        Statement::ClosingTag(_) => todo!(),
        Statement::InlineHtml(_) => todo!(),
        Statement::GotoLabel(_) => todo!(),
        Statement::Goto(_) => todo!(),
        Statement::HaltCompiler { content } => todo!(),
        Statement::Static { vars } => todo!(),
        Statement::DoWhile(_) => todo!(),
        Statement::While(_) => todo!(),
        Statement::For(_) => todo!(),
        Statement::Foreach(_) => todo!(),
        Statement::Break(_) => todo!(),
        Statement::Continue(_) => todo!(),
        Statement::Constant(_) => todo!(),
        Statement::Function(function) => print_function(state, function),
        Statement::Class(_) => todo!(),
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