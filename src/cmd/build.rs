use std::{
    fs::{read, read_to_string},
    path::PathBuf,
};

use pxp_parser::{
    parse,
    parser::ast::{
        arguments::{Argument, ArgumentList},
        classes::{AnonymousClass, AnonymousClassBody, AnonymousClassMember, ClassStatement, ClassMember},
        constant::{ClassishConstant, ConstantStatement, ConstantEntry},
        control_flow::{IfStatement, IfStatementBody, IfStatementElseIf, IfStatementElseIfBlock},
        functions::{
            AbstractConstructor, AbstractMethod, ArrowFunction, ArrowFunctionBody, Closure,
            ConcreteConstructor, ConcreteMethod, FunctionStatement, FunctionBody,
        },
        interfaces::InterfaceStatement,
        loops::{
            DoWhileStatement, ForStatement, ForStatementBody, ForStatementIterator,
            ForeachStatement, ForeachStatementIterator, WhileStatement, WhileStatementBody,
        },
        namespaces::{BracedNamespace, NamespaceStatement, UnbracedNamespace},
        operators::{
            ArithmeticOperation, AssignmentOperation, BitwiseOperation, ComparisonOperation,
            LogicalOperation, RangeOperation,
        },
        properties::{Property, PropertyEntry, VariableProperty},
        traits::{TraitStatement, TraitMember, TraitUsage},
        try_block::{CatchBlock, FinallyBlock, TryStatement},
        ArrayItem, Case, Expression, MatchArm, MatchArmBody, Statement, StaticVar, StringPart, StaticStatement, SwitchStatement, EchoStatement, ExpressionStatement, ReturnStatement, BlockStatement,
    },
};

use crate::{transpile::{short_match::ShortMatchTranspiler, Transpiler, type_alias::TypeAliasTranspiler}, printer::print};

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
        }
    };

    let mut program = match parse(&contents) {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "{}",
                match error.report(&contents, path.to_str(), true, false) {
                    Ok(report) => report,
                    Err(error) => format!("Error: {}", error),
                }
            );
            return;
        }
    };

    transpile_program(&mut program);

    let output = print(&program);

    if options.stdout {
        print!("{}", output);
    }
}

fn transpile_program(program: &mut Vec<Statement>) {
    let mut transpilers: Vec<Box<dyn Transpiler>> = vec![
        Box::new(ShortMatchTranspiler),
        Box::new(TypeAliasTranspiler::new()),
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
        Statement::Static(StaticStatement { vars, .. }) => {
            for StaticVar { default, .. } in vars {
                if let Some(default) = default {
                    transpile_expression(transpiler, default);
                }
            }
        }
        Statement::DoWhile(DoWhileStatement {
            condition, body, ..
        }) => {
            transpile_expression(transpiler, condition);
            transpile_statement(transpiler, body);
        }
        Statement::While(WhileStatement {
            condition, body, ..
        }) => {
            transpile_expression(transpiler, condition);

            match body {
                WhileStatementBody::Block { statements, .. } => {
                    for statement in statements {
                        transpile_statement(transpiler, statement);
                    }
                }
                WhileStatementBody::Statement(statement) => {
                    transpile_statement(transpiler, statement);
                }
            }
        }
        Statement::For(ForStatement {
            r#for,
            left_parenthesis,
            iterator:
                ForStatementIterator {
                    initializations,
                    initializations_semicolon,
                    conditions,
                    conditions_semicolon,
                    r#loop,
                },
            right_parenthesis,
            body,
        }) => {
            for initialization in initializations.inner.iter_mut() {
                transpile_expression(transpiler, initialization);
            }

            for condition in conditions.inner.iter_mut() {
                transpile_expression(transpiler, condition);
            }

            for r#loop in r#loop.inner.iter_mut() {
                transpile_expression(transpiler, r#loop);
            }

            match body {
                ForStatementBody::Statement(statement) => {
                    transpile_statement(transpiler, statement);
                }
                ForStatementBody::Block {
                    colon,
                    statements,
                    endfor,
                    ending,
                } => {
                    for statement in statements {
                        transpile_statement(transpiler, statement);
                    }
                }
            }
        }
        Statement::Foreach(ForeachStatement {
            foreach,
            left_parenthesis,
            iterator,
            right_parenthesis,
            body,
        }) => match iterator {
            ForeachStatementIterator::Value {
                expression,
                r#as,
                ampersand,
                value,
            } => {
                transpile_expression(transpiler, expression);
                transpile_expression(transpiler, value);
            }
            ForeachStatementIterator::KeyAndValue {
                expression,
                r#as,
                ampersand,
                key,
                double_arrow,
                value,
            } => {
                transpile_expression(transpiler, expression);
                transpile_expression(transpiler, key);
                transpile_expression(transpiler, value);
            }
        },
        Statement::Constant(ConstantStatement {
            comments,
            r#const,
            entries,
            semicolon,
        }) => {
            for entry in entries.iter_mut() {
                transpile_expression(transpiler, &mut entry.value);
            }
        }
        Statement::Function(FunctionStatement {
            comments,
            attributes,
            function,
            ampersand,
            name,
            parameters,
            return_type,
            body:
                FunctionBody {
                    left_brace,
                    statements,
                    right_brace,
                    ..
                },
        }) => {
            for parameter in parameters.parameters.inner.iter_mut() {
                if let Some(default) = &mut parameter.default {
                    transpile_expression(transpiler, default);
                }
            }

            for statement in statements {
                transpile_statement(transpiler, statement);
            }
        }
        Statement::Class(ClassStatement {
            attributes,
            modifiers,
            class,
            name,
            extends,
            implements,
            body,
        }) => {
            for member in body.members.iter_mut() {
                transpile_class_member(transpiler, member);
            }
        }
        Statement::Trait(TraitStatement {
            r#trait,
            name,
            attributes,
            body,
        }) => {
            for member in body.members.iter_mut() {
                transpile_trait_member(transpiler, member);
            }
        }
        Statement::Interface(InterfaceStatement {
            attributes,
            interface,
            name,
            extends,
            body,
        }) => {}
        Statement::If(IfStatement {
            r#if,
            left_parenthesis,
            condition,
            right_parenthesis,
            body,
        }) => {
            transpile_expression(transpiler, condition);

            match body {
                IfStatementBody::Statement {
                    statement,
                    elseifs,
                    r#else,
                } => {
                    transpile_statement(transpiler, statement);

                    for IfStatementElseIf {
                        elseif,
                        left_parenthesis,
                        condition,
                        right_parenthesis,
                        statement,
                    } in elseifs.iter_mut()
                    {
                        transpile_expression(transpiler, condition);
                        transpile_statement(transpiler, statement);
                    }

                    if let Some(r#else) = r#else {
                        transpile_statement(transpiler, &mut r#else.statement);
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
                    for statement in statements {
                        transpile_statement(transpiler, statement);
                    }

                    for IfStatementElseIfBlock {
                        elseif,
                        left_parenthesis,
                        condition,
                        right_parenthesis,
                        colon,
                        statements,
                    } in elseifs.iter_mut()
                    {
                        transpile_expression(transpiler, condition);

                        for statement in statements {
                            transpile_statement(transpiler, statement);
                        }
                    }

                    if let Some(r#else) = r#else {
                        for statement in r#else.statements.iter_mut() {
                            transpile_statement(transpiler, statement);
                        }
                    }
                }
            }
        }
        Statement::Switch(SwitchStatement {
            switch,
            left_parenthesis,
            condition,
            right_parenthesis,
            cases,
        }) => {
            transpile_expression(transpiler, condition);

            for Case { condition, body } in cases {
                if let Some(condition) = condition {
                    transpile_expression(transpiler, condition);
                }

                for statement in body {
                    transpile_statement(transpiler, statement);
                }
            }
        }
        Statement::Echo(EchoStatement { values, .. }) => {
            for value in values {
                transpile_expression(transpiler, value);
            }
        }
        Statement::Expression(ExpressionStatement { expression, ending }) => {
            transpile_expression(transpiler, expression);
        }
        Statement::Return(ReturnStatement {
            r#return,
            value,
            ending,
        }) => {
            if let Some(value) = value {
                transpile_expression(transpiler, value);
            }
        }
        Statement::Namespace(namespace) => {
            match namespace {
                NamespaceStatement::Unbraced(UnbracedNamespace {
                    start,
                    name,
                    end,
                    statements,
                }) => {
                    for statement in statements {
                        transpile_statement(transpiler, statement);
                    }
                }
                NamespaceStatement::Braced(BracedNamespace {
                    namespace,
                    name,
                    body,
                }) => {
                    for statement in body.statements.iter_mut() {
                        transpile_statement(transpiler, statement);
                    }
                }
            };
        }
        Statement::Try(TryStatement {
            start,
            end,
            body,
            catches,
            finally,
        }) => {
            for statement in body.iter_mut() {
                transpile_statement(transpiler, statement);
            }

            for CatchBlock {
                start,
                end,
                types,
                var,
                body,
            } in catches.iter_mut()
            {
                for statement in body.iter_mut() {
                    transpile_statement(transpiler, statement);
                }
            }

            if let Some(FinallyBlock { start, end, body }) = finally {
                for statement in body.iter_mut() {
                    transpile_statement(transpiler, statement);
                }
            }
        }
        // FIXME: Transpile unit enum members.
        Statement::UnitEnum(_) => {}
        // FIXME: Transpile backed enum members.
        Statement::BackedEnum(_) => {}
        Statement::Block(BlockStatement {
            left_brace,
            statements,
            right_brace,
        }) => {
            for statement in statements.iter_mut() {
                transpile_statement(transpiler, statement);
            }
        }
        _ => {}
    }
}

fn transpile_expression(transpiler: &mut Box<dyn Transpiler>, expression: &mut Expression) {
    transpiler.transpile_expression(expression);

    match expression {
        Expression::Eval { eval, argument } => {
            transpile_argument(transpiler, &mut argument.argument)
        }
        Expression::Empty { empty, argument } => {
            transpile_argument(transpiler, &mut argument.argument)
        }
        Expression::Die { die, argument } => {
            if let Some(argument) = argument {
                transpile_argument(transpiler, &mut argument.argument)
            }
        }
        Expression::Exit { exit, argument } => {
            if let Some(argument) = argument {
                transpile_argument(transpiler, &mut argument.argument)
            }
        }
        Expression::Isset { isset, arguments } => {
            for argument in arguments.arguments.iter_mut() {
                transpile_argument(transpiler, argument);
            }
        }
        Expression::Unset { unset, arguments } => {
            for argument in arguments.arguments.iter_mut() {
                transpile_argument(transpiler, argument);
            }
        }
        Expression::Print {
            print,
            value,
            argument,
        } => {
            if let Some(argument) = argument {
                transpile_argument(transpiler, &mut argument.argument)
            }
        }
        Expression::ArithmeticOperation(operation) => match operation {
            ArithmeticOperation::Addition { left, plus, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Subtraction { left, minus, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Multiplication {
                left,
                asterisk,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Division { left, slash, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Modulo {
                left,
                percent,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Exponentiation { left, pow, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Negative { minus, right } => {
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::Positive { plus, right } => {
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::PreIncrement { increment, right } => {
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::PostIncrement { left, increment } => {
                transpile_expression(transpiler, left);
            }
            ArithmeticOperation::PreDecrement { decrement, right } => {
                transpile_expression(transpiler, right);
            }
            ArithmeticOperation::PostDecrement { left, decrement } => {
                transpile_expression(transpiler, left);
            }
        },
        Expression::AssignmentOperation(operation) => match operation {
            AssignmentOperation::Assign {
                left,
                equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Addition {
                left,
                plus_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Subtraction {
                left,
                minus_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Multiplication {
                left,
                asterisk_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Division {
                left,
                slash_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Modulo {
                left,
                percent_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Exponentiation {
                left,
                pow_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Concat {
                left,
                dot_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::BitwiseAnd {
                left,
                ampersand_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::BitwiseOr {
                left,
                pipe_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::BitwiseXor {
                left,
                caret_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::LeftShift {
                left,
                left_shift_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::RightShift {
                left,
                right_shift_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            AssignmentOperation::Coalesce {
                left,
                coalesce_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
        },
        Expression::BitwiseOperation(operation) => match operation {
            BitwiseOperation::And { left, and, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            BitwiseOperation::Or { left, or, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            BitwiseOperation::Xor { left, xor, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            BitwiseOperation::LeftShift {
                left,
                left_shift,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            BitwiseOperation::RightShift {
                left,
                right_shift,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            BitwiseOperation::Not { not, right } => {
                transpile_expression(transpiler, right);
            }
        },
        Expression::ComparisonOperation(operation) => match operation {
            ComparisonOperation::Equal {
                left,
                double_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::Identical {
                left,
                triple_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::NotEqual {
                left,
                bang_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::AngledNotEqual {
                left,
                angled_left_right,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::NotIdentical {
                left,
                bang_double_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::LessThan {
                left,
                less_than,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::GreaterThan {
                left,
                greater_than,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::LessThanOrEqual {
                left,
                less_than_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::GreaterThanOrEqual {
                left,
                greater_than_equals,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            ComparisonOperation::Spaceship {
                left,
                spaceship,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
        },
        Expression::LogicalOperation(operation) => match operation {
            LogicalOperation::And {
                left,
                double_ampersand,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            LogicalOperation::Or {
                left,
                double_pipe,
                right,
            } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            LogicalOperation::Not { bang, right } => {
                transpile_expression(transpiler, right);
            }
            LogicalOperation::LogicalAnd { left, and, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            LogicalOperation::LogicalOr { left, or, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
            LogicalOperation::LogicalXor { left, xor, right } => {
                transpile_expression(transpiler, left);
                transpile_expression(transpiler, right);
            }
        },
        Expression::RangeOperation(operation) => match operation {
            RangeOperation::Exclusive {
                lower_bound,
                double_dot,
                upper_bound,
            } => {
                transpile_expression(transpiler, lower_bound);
                transpile_expression(transpiler, upper_bound);
            }
            RangeOperation::Inclusive {
                lower_bound,
                double_dot_equals,
                upper_bound,
            } => {
                transpile_expression(transpiler, lower_bound);
                transpile_expression(transpiler, upper_bound);
            }
            RangeOperation::Endless {
                lower_bound,
                double_dot,
            } => {
                transpile_expression(transpiler, lower_bound);
            }
        },
        Expression::Concat { left, dot, right } => {
            transpile_expression(transpiler, left);
            transpile_expression(transpiler, right);
        }
        Expression::Instanceof {
            left,
            instanceof,
            right,
        } => {
            transpile_expression(transpiler, left);
            transpile_expression(transpiler, right);
        }
        Expression::Reference { ampersand, right } => {
            transpile_expression(transpiler, right);
        }
        Expression::Parenthesized { start, expr, end } => {
            transpile_expression(transpiler, expr);
        }
        Expression::ErrorSuppress { at, expr } => {
            transpile_expression(transpiler, expr);
        }
        Expression::Include { include, path } => {
            transpile_expression(transpiler, path);
        }
        Expression::IncludeOnce { include_once, path } => {
            transpile_expression(transpiler, path);
        }
        Expression::Require { require, path } => {
            transpile_expression(transpiler, path);
        }
        Expression::RequireOnce { require_once, path } => {
            transpile_expression(transpiler, path);
        }
        Expression::FunctionCall { target, arguments } => {
            transpile_expression(transpiler, target);
            for argument in arguments.arguments.iter_mut() {
                transpile_argument(transpiler, argument);
            }
        }
        Expression::FunctionClosureCreation {
            target,
            placeholder,
        } => {
            transpile_expression(transpiler, target);
        }
        Expression::MethodCall {
            target,
            arrow,
            method,
            arguments,
        } => {
            transpile_expression(transpiler, target);
            transpile_expression(transpiler, method);
            transpile_argument_list(transpiler, arguments);
        }
        Expression::MethodClosureCreation {
            target,
            arrow,
            method,
            placeholder,
        } => {
            transpile_expression(transpiler, target);
            transpile_expression(transpiler, method);
        }
        Expression::NullsafeMethodCall {
            target,
            question_arrow,
            method,
            arguments,
        } => {
            transpile_expression(transpiler, target);
            transpile_expression(transpiler, method);
            transpile_argument_list(transpiler, arguments);
        }
        Expression::StaticMethodCall {
            target,
            double_colon,
            method,
            arguments,
        } => {
            transpile_expression(transpiler, target);
            transpile_argument_list(transpiler, arguments);
        }
        Expression::StaticVariableMethodCall {
            target,
            double_colon,
            method,
            arguments,
        } => {
            transpile_expression(transpiler, target);
            transpile_argument_list(transpiler, arguments);
        }
        Expression::StaticMethodClosureCreation {
            target,
            double_colon,
            method,
            placeholder,
        } => {
            transpile_expression(transpiler, target);
        }
        Expression::StaticVariableMethodClosureCreation {
            target,
            double_colon,
            method,
            placeholder,
        } => {
            transpile_expression(transpiler, target);
        }
        Expression::PropertyFetch {
            target,
            arrow,
            property,
        } => {
            transpile_expression(transpiler, target);
            transpile_expression(transpiler, property);
        }
        Expression::NullsafePropertyFetch {
            target,
            question_arrow,
            property,
        } => {
            transpile_expression(transpiler, target);
            transpile_expression(transpiler, property);
        }
        Expression::StaticPropertyFetch {
            target,
            double_colon,
            property,
        } => {
            transpile_expression(transpiler, target);
        }
        Expression::ConstantFetch {
            target,
            double_colon,
            constant,
        } => {
            transpile_expression(transpiler, target);
        }
        Expression::ShortArray { start, items, end } => {
            for item in items.inner.iter_mut() {
                match item {
                    ArrayItem::Skipped => {}
                    ArrayItem::Value { value } => {
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::ReferencedValue { ampersand, value } => {
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::SpreadValue { ellipsis, value } => {
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::KeyValue {
                        key,
                        double_arrow,
                        value,
                    } => {
                        transpile_expression(transpiler, key);
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::ReferencedKeyValue {
                        key,
                        double_arrow,
                        ampersand,
                        value,
                    } => {
                        transpile_expression(transpiler, key);
                        transpile_expression(transpiler, value);
                    }
                }
            }
        }
        Expression::Array {
            array,
            start,
            items,
            end,
        } => {
            for item in items.inner.iter_mut() {
                match item {
                    ArrayItem::Skipped => {}
                    ArrayItem::Value { value } => {
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::ReferencedValue { ampersand, value } => {
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::SpreadValue { ellipsis, value } => {
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::KeyValue {
                        key,
                        double_arrow,
                        value,
                    } => {
                        transpile_expression(transpiler, key);
                        transpile_expression(transpiler, value);
                    }
                    ArrayItem::ReferencedKeyValue {
                        key,
                        double_arrow,
                        ampersand,
                        value,
                    } => {
                        transpile_expression(transpiler, key);
                        transpile_expression(transpiler, value);
                    }
                }
            }
        }
        Expression::Closure(Closure {
            comments,
            attributes,
            r#static,
            function,
            ampersand,
            parameters,
            uses,
            return_type,
            body: FunctionBody { statements, .. },
        }) => {
            for statement in statements.iter_mut() {
                transpile_statement(transpiler, statement);
            }
        }
        Expression::ArrowFunction(ArrowFunction {
            comments,
            r#static,
            ampersand,
            r#fn,
            attributes,
            parameters,
            return_type,
            body,
        }) => match body {
            ArrowFunctionBody::Block {
                left_brace,
                statements,
                right_brace,
            } => {
                for statement in statements.iter_mut() {
                    transpile_statement(transpiler, statement);
                }
            }
            ArrowFunctionBody::Expression {
                double_arrow,
                expression,
            } => {
                transpile_expression(transpiler, expression);
            }
        },
        Expression::New {
            new,
            target,
            arguments,
        } => {
            transpile_expression(transpiler, target);
            if let Some(arguments) = arguments {
                transpile_argument_list(transpiler, arguments);
            }
        }
        Expression::InterpolatedString { parts } => {
            for part in parts.iter_mut() {
                match part {
                    StringPart::Literal(_) => {}
                    StringPart::Expression(expression) => {
                        transpile_expression(transpiler, expression);
                    }
                }
            }
        }
        Expression::Heredoc { parts } => {
            for part in parts.iter_mut() {
                match part {
                    StringPart::Literal(_) => {}
                    StringPart::Expression(expression) => {
                        transpile_expression(transpiler, expression);
                    }
                }
            }
        }
        Expression::Nowdoc { value } => {}
        Expression::ShellExec { parts } => {
            for part in parts.iter_mut() {
                match part {
                    StringPart::Literal(_) => {}
                    StringPart::Expression(expression) => {
                        transpile_expression(transpiler, expression);
                    }
                }
            }
        }
        Expression::AnonymousClass(AnonymousClass {
            attributes,
            class,
            extends,
            implements,
            body:
                AnonymousClassBody {
                    left_brace,
                    members,
                    right_brace,
                },
        }) => {
            for member in members.iter_mut() {
                transpile_anonymous_class_member(transpiler, member);
            }
        }
        Expression::ArrayIndex {
            array,
            left_bracket,
            index,
            right_bracket,
        } => {
            if let Some(index) = index {
                transpile_expression(transpiler, index);
            }
        }
        Expression::ShortTernary {
            condition,
            question_colon,
            r#else,
        } => {
            transpile_expression(transpiler, condition);
            transpile_expression(transpiler, r#else);
        }
        Expression::Ternary {
            condition,
            question,
            then,
            colon,
            r#else,
        } => {
            transpile_expression(transpiler, condition);
            transpile_expression(transpiler, then);
            transpile_expression(transpiler, r#else);
        }
        Expression::Coalesce {
            lhs,
            double_question,
            rhs,
        } => {
            transpile_expression(transpiler, lhs);
            transpile_expression(transpiler, rhs);
        }
        Expression::Clone { target } => {
            transpile_expression(transpiler, target);
        }
        Expression::Match {
            keyword,
            left_parenthesis,
            condition,
            right_parenthesis,
            default,
            arms,
            left_brace,
            right_brace,
        } => {
            transpile_expression(transpiler, condition);

            for MatchArm {
                conditions, body, ..
            } in arms
            {
                for condition in conditions {
                    transpile_expression(transpiler, condition);
                }

                match body {
                    MatchArmBody::Block { statements, .. } => {
                        for statement in statements {
                            transpile_statement(transpiler, statement);
                        }
                    }
                    MatchArmBody::Expression(expression) => {
                        transpile_expression(transpiler, expression);
                    }
                }
            }
        }
        Expression::Throw { value } => {
            transpile_expression(transpiler, value);
        }
        Expression::Yield { key, value } => {
            if let Some(key) = key {
                transpile_expression(transpiler, key);
            }

            if let Some(value) = value {
                transpile_expression(transpiler, value);
            }
        }
        Expression::YieldFrom { value } => {
            transpile_expression(transpiler, value);
        }
        Expression::Cast { cast, kind, value } => {
            transpile_expression(transpiler, value);
        }
        _ => {}
    }
}

fn transpile_argument(transpiler: &mut Box<dyn Transpiler>, argument: &mut Argument) {
    match argument {
        Argument::Positional {
            comments,
            ellipsis,
            value,
        } => {
            transpile_expression(transpiler, value);
        }
        Argument::Named {
            comments,
            name,
            colon,
            ellipsis,
            value,
        } => {
            transpile_expression(transpiler, value);
        }
    }
}

fn transpile_argument_list(transpiler: &mut Box<dyn Transpiler>, argument_list: &mut ArgumentList) {
    for argument in argument_list.arguments.iter_mut() {
        transpile_argument(transpiler, argument);
    }
}

fn transpile_class_member(transpiler: &mut Box<dyn Transpiler>, member: &mut ClassMember) {
    transpiler.transpile_class_member(member);

    match member {
        ClassMember::Constant(constant) => transpile_classish_constant(transpiler, constant),
        ClassMember::TraitUsage(TraitUsage {
            r#use,
            traits,
            adaptations,
        }) => {}
        ClassMember::Property(property) => transpile_property(transpiler, property),
        ClassMember::VariableProperty(property) => {
            transpile_variable_property(transpiler, property)
        }
        ClassMember::ConcreteMethod(method) => transpile_concrete_method(transpiler, method),
        ClassMember::ConcreteConstructor(method) => {
            transpile_concrete_constructor(transpiler, method)
        }
        ClassMember::AbstractMethod(method) => transpile_abstract_method(transpiler, method),
        ClassMember::AbstractConstructor(method) => {
            transpile_abstract_constructor(transpiler, method)
        }
    }
}

fn transpile_trait_member(transpiler: &mut Box<dyn Transpiler>, member: &mut TraitMember) {
    transpiler.transpile_trait_member(member);

    match member {
        TraitMember::Constant(constant) => transpile_classish_constant(transpiler, constant),
        TraitMember::TraitUsage(TraitUsage {
            r#use,
            traits,
            adaptations,
        }) => {}
        TraitMember::Property(property) => transpile_property(transpiler, property),
        TraitMember::VariableProperty(property) => {
            transpile_variable_property(transpiler, property)
        }
        TraitMember::ConcreteMethod(method) => transpile_concrete_method(transpiler, method),
        TraitMember::ConcreteConstructor(method) => {
            transpile_concrete_constructor(transpiler, method)
        }
        TraitMember::AbstractMethod(method) => transpile_abstract_method(transpiler, method),
        TraitMember::AbstractConstructor(method) => {
            transpile_abstract_constructor(transpiler, method)
        }
    }
}

fn transpile_anonymous_class_member(
    transpiler: &mut Box<dyn Transpiler>,
    member: &mut AnonymousClassMember,
) {
    transpiler.transpile_anonymous_class_member(member);

    match member {
        AnonymousClassMember::Constant(constant) => {
            transpile_classish_constant(transpiler, constant)
        }
        AnonymousClassMember::TraitUsage(TraitUsage {
            r#use,
            traits,
            adaptations,
        }) => {}
        AnonymousClassMember::Property(property) => transpile_property(transpiler, property),
        AnonymousClassMember::VariableProperty(property) => {
            transpile_variable_property(transpiler, property)
        }
        AnonymousClassMember::ConcreteMethod(method) => {
            transpile_concrete_method(transpiler, method)
        }
        AnonymousClassMember::ConcreteConstructor(method) => {
            transpile_concrete_constructor(transpiler, method)
        }
    }
}

fn transpile_classish_constant(
    transpiler: &mut Box<dyn Transpiler>,
    constant: &mut ClassishConstant,
) {
    for ConstantEntry {
        name,
        equals,
        value,
    } in constant.entries.iter_mut()
    {
        transpile_expression(transpiler, value);
    }
}

fn transpile_property(transpiler: &mut Box<dyn Transpiler>, property: &mut Property) {
    transpiler.transpile_property(property);

    for entry in property.entries.iter_mut() {
        match entry {
            PropertyEntry::Initialized {
                variable,
                equals,
                value,
            } => {
                transpile_expression(transpiler, value);
            }
            _ => continue,
        }
    }
}

fn transpile_variable_property(
    transpiler: &mut Box<dyn Transpiler>,
    property: &mut VariableProperty,
) {
    transpiler.transpile_variable_property(property);
    
    for entry in property.entries.iter_mut() {
        match entry {
            PropertyEntry::Initialized {
                variable,
                equals,
                value,
            } => {
                transpile_expression(transpiler, value);
            }
            _ => continue,
        }
    }
}

fn transpile_abstract_method(transpiler: &mut Box<dyn Transpiler>, method: &mut AbstractMethod) {
    //
}

fn transpile_abstract_constructor(
    transpiler: &mut Box<dyn Transpiler>,
    constructor: &mut AbstractConstructor,
) {
    //
}

fn transpile_concrete_method(transpiler: &mut Box<dyn Transpiler>, method: &mut ConcreteMethod) {
    for statement in method.body.statements.iter_mut() {
        transpile_statement(transpiler, statement);
    }
}

fn transpile_concrete_constructor(
    transpiler: &mut Box<dyn Transpiler>,
    constructor: &mut ConcreteConstructor,
) {
    for statement in constructor.body.statements.iter_mut() {
        transpile_statement(transpiler, statement);
    }
}
