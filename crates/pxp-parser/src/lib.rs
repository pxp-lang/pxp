use crate::internal::attributes;
use crate::internal::blocks;
use crate::internal::classes;
use crate::internal::constants;
use crate::internal::control_flow;
use crate::internal::enums;
use crate::internal::functions;
use crate::internal::goto;
use crate::internal::identifiers;
use crate::internal::interfaces;
use crate::internal::loops;
use crate::internal::namespaces;
use crate::internal::traits;
use crate::internal::try_block;
use crate::internal::uses;
use crate::internal::utils;
use crate::internal::variables;
use crate::state::State;
use internal::literals::expect_literal;
use pxp_ast::declares::DeclareBody;
use pxp_ast::declares::DeclareEntry;
use pxp_ast::declares::DeclareEntryGroup;
use pxp_ast::declares::DeclareStatement;
use pxp_ast::variables::Variable;
use pxp_ast::Statement;
use pxp_ast::{StatementKind, StaticVar};
use pxp_diagnostics::Diagnostic;
use pxp_lexer::stream::TokenStream;
use pxp_lexer::Lexer;
use pxp_span::Span;
use pxp_symbol::SymbolTable;
use pxp_token::OpenTagKind;
use pxp_token::Token;
use pxp_token::TokenKind;

use pxp_ast::ClosingTagStatement;
use pxp_ast::EchoOpeningTagStatement;
use pxp_ast::EchoStatement;
use pxp_ast::ExpressionStatement;
use pxp_ast::FullOpeningTagStatement;
use pxp_ast::GlobalStatement;
use pxp_ast::HaltCompilerStatement;
use pxp_ast::InlineHtmlStatement;
use pxp_ast::ReturnStatement;
use pxp_ast::ShortOpeningTagStatement;
use pxp_ast::StaticStatement;

mod expressions;
mod internal;
mod macros;
mod state;
mod diagnostics;

pub use diagnostics::ParserDiagnostic;

#[derive(Debug)]
pub struct ParseResult {
    pub ast: Vec<Statement>,
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
}

pub fn parse<B: Sized + AsRef<[u8]>>(
    input: &B,
    symbol_table: &mut SymbolTable,
) -> ParseResult {
    let mut lexer = Lexer::new(input, symbol_table);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_) => {
            todo!("tolerant mode")
        }
    };

    construct(&tokens, symbol_table)
}

pub fn construct(tokens: &[Token], symbol_table: &mut SymbolTable) -> ParseResult {
    let mut stream = TokenStream::new(tokens);
    let mut state = State::new(&mut stream, symbol_table);
    let mut ast = Vec::new();

    while !state.stream.is_eof() {
        ast.push(top_level_statement(&mut state));
    }

    let diagnostics = state.diagnostics;

    ParseResult { ast, diagnostics }
}

fn top_level_statement(state: &mut State) -> Statement {
    let start_span = state.stream.current().span;
    let current = state.stream.current();

    match &current.kind {
        TokenKind::Namespace | TokenKind::Use | TokenKind::Const | TokenKind::HaltCompiler => {
            let comments = state.stream.comments();
            let kind = match &state.stream.current().kind {
                TokenKind::Namespace => namespaces::namespace(state),
                TokenKind::Use => uses::use_statement(state),
                TokenKind::Const => StatementKind::Constant(constants::parse(state)),
                TokenKind::HaltCompiler => {
                    state.stream.next();

                    let content = if let TokenKind::InlineHtml = state.stream.current().kind {
                        let content = *state.stream.current();
                        state.stream.next();
                        Some(content)
                    } else {
                        None
                    };

                    StatementKind::HaltCompiler(HaltCompilerStatement { content })
                }
                _ => unreachable!(),
            };

            Statement::new(
                kind,
                Span::new(start_span.start, state.stream.previous().span.end),
                comments,
            )
        }
        _ => statement(state),
    }
}

fn statement(state: &mut State) -> Statement {
    let start_span = state.stream.current().span;
    let comments = state.stream.comments();

    let has_attributes = attributes::gather_attributes(state);
    let current = state.stream.current();
    let peek = state.stream.peek();
    let statement = if has_attributes {
        match &current.kind {
            TokenKind::Abstract => classes::parse(state),
            TokenKind::Readonly if peek.kind != TokenKind::LeftParen => classes::parse(state),
            TokenKind::Final => classes::parse(state),
            TokenKind::Class => classes::parse(state),
            TokenKind::Interface => interfaces::parse(state),
            TokenKind::Trait => traits::parse(state),
            TokenKind::Enum
                if !matches!(
                    peek.kind,
                    TokenKind::LeftParen | TokenKind::DoubleColon | TokenKind::Colon,
                ) =>
            {
                enums::parse(state)
            }
            TokenKind::Function
                if identifiers::is_identifier_maybe_soft_reserved(&peek.kind)
                    || peek.kind == TokenKind::Ampersand =>
            {
                if peek.kind == TokenKind::Ampersand {
                    if !identifiers::is_identifier_maybe_soft_reserved(
                        &state.stream.lookahead(1).kind,
                    ) {
                        let expression = expressions::attributes(state);
                        let ending = utils::skip_ending(state);
                        let ending_span = ending.span();

                        let kind =
                            StatementKind::Expression(ExpressionStatement { expression, ending });

                        return Statement::new(
                            kind,
                            Span::new(start_span.start, ending_span.end),
                            comments,
                        );
                    }

                    functions::function(state)
                } else {
                    functions::function(state)
                }
            }
            _ => StatementKind::Expression(ExpressionStatement {
                expression: expressions::attributes(state),
                ending: utils::skip_ending(state),
            }),
        }
    } else {
        match &current.kind {
            TokenKind::OpenTag(OpenTagKind::Echo) => {
                let span = current.span;
                state.stream.next();

                StatementKind::EchoOpeningTag(EchoOpeningTagStatement { span })
            }
            TokenKind::OpenTag(OpenTagKind::Full) => {
                let span = current.span;
                state.stream.next();

                StatementKind::FullOpeningTag(FullOpeningTagStatement { span })
            }
            TokenKind::OpenTag(OpenTagKind::Short) => {
                let span = current.span;
                state.stream.next();

                StatementKind::ShortOpeningTag(ShortOpeningTagStatement { span })
            }
            TokenKind::CloseTag => {
                let span = current.span;
                state.stream.next();

                StatementKind::ClosingTag(ClosingTagStatement { span })
            }
            TokenKind::Abstract => classes::parse(state),
            TokenKind::Readonly if peek.kind != TokenKind::LeftParen => classes::parse(state),
            TokenKind::Final => classes::parse(state),
            TokenKind::Class => classes::parse(state),
            TokenKind::Interface => interfaces::parse(state),
            TokenKind::Trait => traits::parse(state),
            TokenKind::Enum
                if !matches!(
                    peek.kind,
                    TokenKind::LeftParen | TokenKind::DoubleColon | TokenKind::Colon,
                ) =>
            {
                enums::parse(state)
            }
            TokenKind::Function
                if identifiers::is_identifier_maybe_soft_reserved(&peek.kind)
                    || peek.kind == TokenKind::Ampersand =>
            {
                if peek.kind == TokenKind::Ampersand {
                    if !identifiers::is_identifier_maybe_soft_reserved(
                        &state.stream.lookahead(1).kind,
                    ) {
                        let expression = expressions::attributes(state);
                        let ending = utils::skip_ending(state);
                        let ending_span = ending.span();
                        let kind =
                            StatementKind::Expression(ExpressionStatement { expression, ending });

                        return Statement::new(
                            kind,
                            Span::new(start_span.start, ending_span.end),
                            comments,
                        );
                    }

                    functions::function(state)
                } else {
                    functions::function(state)
                }
            }
            TokenKind::Goto => goto::goto_statement(state),
            token
                if identifiers::is_identifier_maybe_reserved(token)
                    && peek.kind == TokenKind::Colon =>
            {
                goto::label_statement(state)
            }
            TokenKind::Declare => {
                let span = utils::skip(state, TokenKind::Declare);

                let entries = {
                    let start = utils::skip_left_parenthesis(state);
                    let mut entries = Vec::new();
                    loop {
                        let key = identifiers::identifier(state);
                        let span = utils::skip(state, TokenKind::Equals);
                        let value = expect_literal(state);

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
                    let end = utils::skip_right_parenthesis(state);

                    DeclareEntryGroup {
                        left_parenthesis: start,
                        entries,
                        right_parenthesis: end,
                    }
                };

                let body = match state.stream.current().kind {
                    TokenKind::SemiColon => {
                        let span = utils::skip_semicolon(state);

                        DeclareBody::Noop { semicolon: span }
                    }
                    TokenKind::LeftBrace => {
                        let start = utils::skip_left_brace(state);
                        let statements =
                            blocks::multiple_statements_until(state, &TokenKind::RightBrace);
                        let end = utils::skip_right_brace(state);

                        DeclareBody::Braced {
                            left_brace: start,
                            statements,
                            right_brace: end,
                        }
                    }
                    TokenKind::Colon => {
                        let start = utils::skip_colon(state);
                        let statements =
                            blocks::multiple_statements_until(state, &TokenKind::EndDeclare);
                        let end = (
                            utils::skip(state, TokenKind::EndDeclare),
                            utils::skip_semicolon(state),
                        );

                        DeclareBody::Block {
                            colon: start,
                            statements,
                            end,
                        }
                    }
                    _ => {
                        let expression = expressions::create(state);
                        let end = utils::skip_semicolon(state);

                        DeclareBody::Expression {
                            expression,
                            semicolon: end,
                        }
                    }
                };

                StatementKind::Declare(DeclareStatement {
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
                    variables.push(variables::dynamic_variable(state));

                    if state.stream.current().kind == TokenKind::Comma {
                        state.stream.next();
                    } else {
                        break;
                    }
                }

                utils::skip_semicolon(state);
                StatementKind::Global(GlobalStatement {
                    global: span,
                    variables,
                })
            }
            TokenKind::Static if matches!(peek.kind, TokenKind::Variable) => {
                state.stream.next();

                let mut vars = vec![];

                // `loop` instead of `while` as we don't allow for extra commas.
                loop {
                    let var = variables::simple_variable(state);
                    let mut default = None;

                    if state.stream.current().kind == TokenKind::Equals {
                        state.stream.next();

                        default = Some(expressions::create(state));
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

                utils::skip_semicolon(state);

                StatementKind::Static(StaticStatement { vars })
            }
            TokenKind::InlineHtml => {
                let html = *state.stream.current();
                state.stream.next();

                StatementKind::InlineHtml(InlineHtmlStatement { html })
            }
            TokenKind::Do => loops::do_while_statement(state),
            TokenKind::While => loops::while_statement(state),
            TokenKind::For => loops::for_statement(state),
            TokenKind::Foreach => loops::foreach_statement(state),
            TokenKind::Continue => loops::continue_statement(state),
            TokenKind::Break => loops::break_statement(state),
            TokenKind::Switch => control_flow::switch_statement(state),
            TokenKind::If => control_flow::if_statement(state),
            TokenKind::Try => try_block::try_block(state),
            TokenKind::LeftBrace => blocks::block_statement(state),
            TokenKind::SemiColon => {
                let start = current.span;

                state.stream.next();

                StatementKind::Noop(start)
            }
            TokenKind::Echo => {
                state.stream.next();

                let mut values = Vec::new();
                // FIXME: We should check for a semi-colon here and produce a better error,
                //        currently the error says that the semi-colon is unexpected (which it is)
                //        but a more appropriate error would be that the expression is missing and
                //        that the semi-colon is fine where it is (or at least not the real problem).
                loop {
                    values.push(expressions::create(state));

                    if state.stream.current().kind == TokenKind::Comma {
                        state.stream.next();
                    } else {
                        break;
                    }
                }

                StatementKind::Echo(EchoStatement {
                    echo: current.span,
                    values,
                    ending: utils::skip_ending(state),
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
                    Some(expressions::create(state))
                };

                StatementKind::Return(ReturnStatement {
                    r#return: current.span,
                    value,
                    ending: utils::skip_ending(state),
                })
            }
            _ => StatementKind::Expression(ExpressionStatement {
                expression: expressions::create(state),
                ending: utils::skip_ending(state),
            }),
        }
    };

    Statement::new(
        statement,
        Span::new(start_span.start, state.stream.previous().span.end),
        comments,
    )
}
