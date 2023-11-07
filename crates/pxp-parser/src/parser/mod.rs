use crate::expect_literal;
use crate::lexer::token::OpenTagKind;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::lexer::Lexer;
use crate::parser::ast::declares::DeclareBody;
use crate::parser::ast::declares::DeclareEntry;
use crate::parser::ast::declares::DeclareEntryGroup;
use crate::parser::ast::declares::DeclareStatement;
use crate::parser::ast::variables::Variable;
use crate::parser::ast::{Program, Statement, StaticVar};
use crate::parser::error::ParseErrorStack;
use crate::parser::error::ParseResult;
use crate::parser::internal::attributes;
use crate::parser::internal::blocks;
use crate::parser::internal::classes;
use crate::parser::internal::constants;
use crate::parser::internal::control_flow;
use crate::parser::internal::enums;
use crate::parser::internal::functions;
use crate::parser::internal::goto;
use crate::parser::internal::identifiers;
use crate::parser::internal::interfaces;
use crate::parser::internal::loops;
use crate::parser::internal::namespaces;
use crate::parser::internal::traits;
use crate::parser::internal::try_block;
use crate::parser::internal::uses;
use crate::parser::internal::utils;
use crate::parser::internal::variables;
use crate::parser::state::State;

pub use crate::lexer::stream::TokenStream;

use self::ast::ClosingTagStatement;
use self::ast::EchoOpeningTagStatement;
use self::ast::EchoStatement;
use self::ast::ExpressionStatement;
use self::ast::FullOpeningTagStatement;
use self::ast::GlobalStatement;
use self::ast::HaltCompilerStatement;
use self::ast::InlineHtmlStatement;
use self::ast::ReturnStatement;
use self::ast::ShortOpeningTagStatement;
use self::ast::StaticStatement;
use self::internal::precedences::Precedence;

pub mod ast;
pub mod error;

mod expressions;
mod internal;
mod macros;
mod state;

pub fn parse<B: ?Sized + AsRef<[u8]>>(input: &B) -> Result<Program, ParseErrorStack> {
    let lexer = Lexer::new();
    let tokens = match lexer.tokenize(input) {
        Ok(tokens) => tokens,
        Err(error) => {
            return Err(ParseErrorStack {
                errors: vec![error.into()],
                partial: Vec::new(),
            })
        }
    };

    construct(&tokens)
}

pub fn construct(tokens: &[Token]) -> Result<Program, ParseErrorStack> {
    let mut stream = TokenStream::new(tokens);
    let mut state = State::new(&mut stream);

    let mut program = Program::new();

    while !state.stream.is_eof() {
        let statement = match top_level_statement(&mut state) {
            Ok(statement) => statement,
            Err(error) => {
                let mut previous = state.errors;
                previous.push(error);

                return Err(ParseErrorStack {
                    errors: previous,
                    partial: program,
                });
            }
        };

        program.push(statement);
    }

    let errors = state.errors;
    if !errors.is_empty() {
        return Err(ParseErrorStack {
            errors,
            partial: program,
        });
    }

    Ok(program.to_vec())
}

fn top_level_statement(state: &mut State) -> ParseResult<Statement> {
    let statement = match &state.stream.current().kind {
        TokenKind::Namespace => namespaces::namespace(state)?,
        TokenKind::Use => uses::use_statement(state)?,
        TokenKind::Const => Statement::Constant(constants::parse(state)?),
        TokenKind::HaltCompiler => {
            state.stream.next();

            let content = if let TokenKind::InlineHtml = state.stream.current().kind.clone() {
                let content = state.stream.current().value.clone();
                state.stream.next();
                Some(content)
            } else {
                None
            };

            Statement::HaltCompiler(HaltCompilerStatement { content })
        }
        _ => statement(state)?,
    };

    Ok(statement)
}

fn statement(state: &mut State) -> ParseResult<Statement> {
    let has_attributes = attributes::gather_attributes(state)?;

    let current = state.stream.current();
    let peek = state.stream.peek();
    let statement = if has_attributes {
        match &current.kind {
            TokenKind::Abstract => classes::parse(state)?,
            TokenKind::Readonly if peek.kind != TokenKind::LeftParen => classes::parse(state)?,
            TokenKind::Final => classes::parse(state)?,
            TokenKind::Class => classes::parse(state)?,
            TokenKind::Interface => interfaces::parse(state)?,
            TokenKind::Trait => traits::parse(state)?,
            TokenKind::Enum
                if !matches!(
                    peek.kind,
                    TokenKind::LeftParen | TokenKind::DoubleColon | TokenKind::Colon,
                ) =>
            {
                enums::parse(state)?
            }
            TokenKind::Function
                if identifiers::is_identifier_maybe_soft_reserved(&peek.kind)
                    || peek.kind == TokenKind::Ampersand =>
            {
                if peek.kind == TokenKind::Ampersand {
                    if !identifiers::is_identifier_maybe_soft_reserved(
                        &state.stream.lookahead(1).kind,
                    ) {
                        return Ok(Statement::Expression(ExpressionStatement {
                            expression: expressions::attributes(state, &Precedence::Lowest)?,
                            ending: utils::skip_ending(state)?,
                        }));
                    }

                    functions::function(state)?
                } else {
                    functions::function(state)?
                }
            }
            _ => Statement::Expression(ExpressionStatement {
                expression: expressions::attributes(state, &Precedence::Lowest)?,
                ending: utils::skip_ending(state)?,
            }),
        }
    } else {
        match &current.kind {
            TokenKind::OpenTag(OpenTagKind::Echo) => {
                let span = current.span;
                state.stream.next();

                Statement::EchoOpeningTag(EchoOpeningTagStatement { span })
            }
            TokenKind::OpenTag(OpenTagKind::Full) => {
                let span = current.span;
                state.stream.next();

                Statement::FullOpeningTag(FullOpeningTagStatement { span })
            }
            TokenKind::OpenTag(OpenTagKind::Short) => {
                let span = current.span;
                state.stream.next();

                Statement::ShortOpeningTag(ShortOpeningTagStatement { span })
            }
            TokenKind::CloseTag => {
                let span = current.span;
                state.stream.next();

                Statement::ClosingTag(ClosingTagStatement { span })
            }
            TokenKind::Abstract => classes::parse(state)?,
            TokenKind::Readonly if peek.kind != TokenKind::LeftParen => classes::parse(state)?,
            TokenKind::Final => classes::parse(state)?,
            TokenKind::Class => classes::parse(state)?,
            TokenKind::Interface => interfaces::parse(state)?,
            TokenKind::Trait => traits::parse(state)?,
            TokenKind::Enum
                if !matches!(
                    peek.kind,
                    TokenKind::LeftParen | TokenKind::DoubleColon | TokenKind::Colon,
                ) =>
            {
                enums::parse(state)?
            }
            TokenKind::Function
                if identifiers::is_identifier_maybe_soft_reserved(&peek.kind)
                    || peek.kind == TokenKind::Ampersand =>
            {
                if peek.kind == TokenKind::Ampersand {
                    if !identifiers::is_identifier_maybe_soft_reserved(
                        &state.stream.lookahead(1).kind,
                    ) {
                        return Ok(Statement::Expression(ExpressionStatement {
                            expression: expressions::attributes(state, &Precedence::Lowest)?,
                            ending: utils::skip_ending(state)?,
                        }));
                    }

                    functions::function(state)?
                } else {
                    functions::function(state)?
                }
            }
            TokenKind::Goto => goto::goto_statement(state)?,
            token
                if identifiers::is_identifier_maybe_reserved(token)
                    && peek.kind == TokenKind::Colon =>
            {
                goto::label_statement(state)?
            }
            TokenKind::Declare => {
                let span = utils::skip(state, TokenKind::Declare)?;

                let entries = {
                    let start = utils::skip_left_parenthesis(state)?;
                    let mut entries = Vec::new();
                    loop {
                        let key = identifiers::identifier(state)?;
                        let span = utils::skip(state, TokenKind::Equals)?;
                        let value = expect_literal!(state);

                        entries.push(DeclareEntry {
                            key,
                            equals: span,
                            value,
                        });

                        if state.stream.current().kind == TokenKind::Comma {
                            state.stream.next();
                        } else {
                            break;
                        }
                    }
                    let end = utils::skip_right_parenthesis(state)?;

                    DeclareEntryGroup {
                        left_parenthesis: start,
                        entries,
                        right_parenthesis: end,
                    }
                };

                let body = match state.stream.current().kind.clone() {
                    TokenKind::SemiColon => {
                        let span = utils::skip_semicolon(state)?;

                        DeclareBody::Noop { semicolon: span }
                    }
                    TokenKind::LeftBrace => {
                        let start = utils::skip_left_brace(state)?;
                        let statements =
                            blocks::multiple_statements_until(state, &TokenKind::RightBrace)?;
                        let end = utils::skip_right_brace(state)?;

                        DeclareBody::Braced {
                            left_brace: start,
                            statements,
                            right_brace: end,
                        }
                    }
                    TokenKind::Colon => {
                        let start = utils::skip_colon(state)?;
                        let statements =
                            blocks::multiple_statements_until(state, &TokenKind::EndDeclare)?;
                        let end = (
                            utils::skip(state, TokenKind::EndDeclare)?,
                            utils::skip_semicolon(state)?,
                        );

                        DeclareBody::Block {
                            colon: start,
                            statements,
                            end,
                        }
                    }
                    _ => {
                        let expression = expressions::create(state)?;
                        let end = utils::skip_semicolon(state)?;

                        DeclareBody::Expression {
                            expression,
                            semicolon: end,
                        }
                    }
                };

                Statement::Declare(DeclareStatement {
                    declare: span,
                    entries,
                    body,
                })
            }
            TokenKind::Global => {
                let span = current.span;
                state.stream.next();

                let mut variables = vec![];
                // `loop` instead of `while` as we don't allow for extra commas.
                loop {
                    variables.push(variables::dynamic_variable(state)?);

                    if state.stream.current().kind == TokenKind::Comma {
                        state.stream.next();
                    } else {
                        break;
                    }
                }

                utils::skip_semicolon(state)?;
                Statement::Global(GlobalStatement {
                    global: span,
                    variables,
                })
            }
            TokenKind::Static if matches!(peek.kind, TokenKind::Variable) => {
                state.stream.next();

                let mut vars = vec![];

                // `loop` instead of `while` as we don't allow for extra commas.
                loop {
                    let var = variables::simple_variable(state)?;
                    let mut default = None;

                    if state.stream.current().kind == TokenKind::Equals {
                        state.stream.next();

                        default = Some(expressions::create(state)?);
                    }

                    vars.push(StaticVar {
                        var: Variable::SimpleVariable(var),
                        default,
                    });

                    if state.stream.current().kind == TokenKind::Comma {
                        state.stream.next();
                    } else {
                        break;
                    }
                }

                utils::skip_semicolon(state)?;

                Statement::Static(StaticStatement { vars })
            }
            TokenKind::InlineHtml => {
                let html = state.stream.current().value.clone();
                state.stream.next();

                Statement::InlineHtml(InlineHtmlStatement { html })
            }
            TokenKind::Do => loops::do_while_statement(state)?,
            TokenKind::While => loops::while_statement(state)?,
            TokenKind::For => loops::for_statement(state)?,
            TokenKind::Foreach => loops::foreach_statement(state)?,
            TokenKind::Continue => loops::continue_statement(state)?,
            TokenKind::Break => loops::break_statement(state)?,
            TokenKind::Switch => control_flow::switch_statement(state)?,
            TokenKind::If => control_flow::if_statement(state)?,
            TokenKind::Try => try_block::try_block(state)?,
            TokenKind::LeftBrace => blocks::block_statement(state)?,
            TokenKind::SemiColon => {
                let start = current.span;

                state.stream.next();

                Statement::Noop(start)
            }
            TokenKind::Echo => {
                state.stream.next();

                let mut values = Vec::new();
                loop {
                    values.push(expressions::create(state)?);

                    if state.stream.current().kind == TokenKind::Comma {
                        state.stream.next();
                    } else {
                        break;
                    }
                }

                Statement::Echo(EchoStatement {
                    echo: current.span,
                    values,
                    ending: utils::skip_ending(state)?,
                })
            }
            TokenKind::Return => {
                state.stream.next();

                let value = if matches!(
                    state.stream.current().kind,
                    TokenKind::SemiColon | TokenKind::CloseTag
                ) {
                    None
                } else {
                    expressions::create(state).map(Some)?
                };

                Statement::Return(ReturnStatement {
                    r#return: current.span,
                    value,
                    ending: utils::skip_ending(state)?,
                })
            }
            _ => Statement::Expression(ExpressionStatement {
                expression: expressions::create(state)?,
                ending: utils::skip_ending(state)?,
            }),
        }
    };

    Ok(statement)
}
