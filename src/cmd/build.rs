use std::{path::PathBuf, fs::{read, read_to_string}};

use pxp_parser::{parse, parser::ast::{Statement, Expression, StaticVar, loops::{DoWhileStatement, WhileStatement, WhileStatementBody}, MatchArm, MatchArmBody}};

use crate::transpile::{Transpiler, short_match::ShortMatchTranspiler};

#[derive(Debug)]
pub struct BuildOptions {
    pub stdout: bool,
}

pub fn build(options: BuildOptions) {
    todo!()
}

pub fn build_single_file(path: PathBuf, options: BuildOptions) {
    let contents = match read_to_string(&path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        },
    };

    let mut program = match parse(&contents) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{}", match error.report(&contents, path.to_str(), true, false) {
                Ok(report) => report,
                Err(error) => format!("Error: {}", error),
            });
            return;
        },
    };

    transpile_program(&mut program);

    dbg!(program);
}

fn transpile_program(program: &mut Vec<Statement>) {
    let mut transpilers: Vec<Box<dyn Transpiler>> = vec![
        Box::new(ShortMatchTranspiler),
    ];

    for transpiler in transpilers.iter_mut() {
        for statement in program.iter_mut() {
            transpile_statement(transpiler, statement);
        }
    }
}

fn transpile_statement(transpiler: &mut Box<dyn Transpiler>, statement: &mut Statement) {
    transpiler.transpile_statement(statement);

    match statement {
        Statement::Static { vars } => {
            for StaticVar { default, .. } in vars {
                if let Some(default) = default {
                    transpile_expression(transpiler, default);
                }
            }
        },
        Statement::DoWhile(DoWhileStatement { condition, body, .. }) => {
            transpile_expression(transpiler, condition);
            transpile_statement(transpiler, body);
        },
        Statement::While(WhileStatement { condition, body, .. }) => {
            transpile_expression(transpiler, condition);

            match body {
                WhileStatementBody::Block { statements, .. } => {
                    for statement in statements {
                        transpile_statement(transpiler, statement);
                    }
                },
                WhileStatementBody::Statement(statement) => {
                    transpile_statement(transpiler, statement);
                },
            }
        },
        Statement::For(_) => todo!(),
        Statement::Foreach(_) => todo!(),
        Statement::Break(_) => todo!(),
        Statement::Continue(_) => todo!(),
        Statement::Constant(_) => todo!(),
        Statement::Function(_) => todo!(),
        Statement::Class(_) => todo!(),
        Statement::Trait(_) => todo!(),
        Statement::Interface(_) => todo!(),
        Statement::If(_) => todo!(),
        Statement::Switch { switch, left_parenthesis, condition, right_parenthesis, cases } => todo!(),
        Statement::Echo { values, .. } => {
            for value in values {
                transpile_expression(transpiler, value);
            }
        },
        Statement::Expression { expression, ending } => todo!(),
        Statement::Return { r#return, value, ending } => todo!(),
        Statement::Namespace(_) => todo!(),
        Statement::Use { uses, kind } => todo!(),
        Statement::GroupUse { prefix, kind, uses } => todo!(),
        Statement::Try(_) => todo!(),
        Statement::UnitEnum(_) => todo!(),
        Statement::BackedEnum(_) => todo!(),
        Statement::Block { left_brace, statements, right_brace } => todo!(),
        Statement::Global { global, variables } => todo!(),
        Statement::Declare(_) => todo!(),
        Statement::TypeAlias { type_keyword, name, equals, r#type, semicolon } => todo!(),
        _ => {},
    }
}

fn transpile_expression(transpiler: &mut Box<dyn Transpiler>, expression: &mut Expression) {
    transpiler.transpile_expression(expression);
    
    match expression {
        Expression::Eval { eval, argument } => todo!(),
        Expression::Empty { empty, argument } => todo!(),
        Expression::Die { die, argument } => todo!(),
        Expression::Exit { exit, argument } => todo!(),
        Expression::Isset { isset, arguments } => todo!(),
        Expression::Unset { unset, arguments } => todo!(),
        Expression::Print { print, value, argument } => todo!(),
        Expression::ArithmeticOperation(_) => todo!(),
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
        Expression::Identifier(_) => todo!(),
        Expression::Variable(_) => todo!(),
        Expression::Include { include, path } => todo!(),
        Expression::IncludeOnce { include_once, path } => todo!(),
        Expression::Require { require, path } => todo!(),
        Expression::RequireOnce { require_once, path } => todo!(),
        Expression::FunctionCall { target, arguments } => todo!(),
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
        Expression::ArrayIndex { array, left_bracket, index, right_bracket } => todo!(),
        Expression::Null => todo!(),
        Expression::MagicConstant(_) => todo!(),
        Expression::ShortTernary { condition, question_colon, r#else } => todo!(),
        Expression::Ternary { condition, question, then, colon, r#else } => todo!(),
        Expression::Coalesce { lhs, double_question, rhs } => todo!(),
        Expression::Clone { target } => todo!(),
        Expression::Match { keyword, left_parenthesis, condition, right_parenthesis, default, arms } => {
            transpile_expression(transpiler, condition);

            for MatchArm { conditions, body, .. } in arms {
                for condition in conditions {
                    transpile_expression(transpiler, condition);
                }

                match body {
                    MatchArmBody::Block { statements, .. } => {
                        for statement in statements {
                            transpile_statement(transpiler, statement);
                        }
                    },
                    MatchArmBody::Expression(expression) => {
                        transpile_expression(transpiler, expression);
                    },
                }
            }
        },
        Expression::ShortMatch { keyword, default, arms } => todo!(),
        Expression::Throw { value } => todo!(),
        Expression::Yield { key, value } => todo!(),
        Expression::YieldFrom { value } => todo!(),
        Expression::Cast { cast, kind, value } => todo!(),
        _ => {},
    }
}