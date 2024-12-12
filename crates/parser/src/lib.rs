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
use internal::literals::parse_literal;
use pxp_ast::Statement;
use pxp_ast::*;
use pxp_ast::{StatementKind, StaticVar};
use pxp_bytestring::ByteStr;
use pxp_bytestring::ByteString;
use pxp_diagnostics::Diagnostic;
use pxp_diagnostics::Severity;
use pxp_lexer::Lexer;
use pxp_span::Span;
use pxp_span::Spanned;

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

mod diagnostics;
mod expressions;
mod internal;
mod macros;
mod state;

pub use diagnostics::ParserDiagnostic;

#[derive(Debug)]
pub struct ParseResult {
    pub ast: Vec<Statement>,
    pub diagnostics: Vec<Diagnostic<ParserDiagnostic>>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    state: State,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            state: State::new(),
        }
    }

    fn next(&mut self) -> Span {
        let span = self.current_span();

        self.lexer.next();

        span
    }

    fn is_eof(&self) -> bool {
        self.current_kind() == TokenKind::Eof
    }

    fn current(&self) -> Token {
        self.lexer.current()
    }

    fn current_kind(&self) -> TokenKind {
        self.current().kind
    }

    fn current_span(&self) -> Span {
        self.current().span
    }

    fn current_symbol(&self) -> &ByteStr {
        self.current().symbol
    }

    fn current_symbol_as_bytestring(&self) -> ByteString {
        self.current_symbol().to_bytestring()
    }

    fn peek(&mut self) -> Token {
        self.lexer.peek()
    }

    fn peek_kind(&mut self) -> TokenKind {
        self.lexer.peek().kind
    }

    fn next_but_first<T>(&mut self, mut cb: impl FnMut(&mut Self) -> T) -> T {
        let result = cb(self);

        self.next();

        result
    }

    fn collect_until<T>(&mut self, kind: TokenKind, mut cb: impl FnMut(&mut Self) -> T) -> Vec<T> {
        let mut items = Vec::new();

        while self.current_kind() != kind {
            items.push(cb(self));
        }

        items
    }

    fn expect_any(&mut self, kinds: &[TokenKind]) -> Span {
        for kind in kinds {
            if self.current_kind() == *kind {
                let span = self.current_span();

                self.next();

                return span;
            }
        }

        self.expected_any_of_tokens(kinds);

        Span::missing()
    }

    fn expect(&mut self, kind: TokenKind) -> Span {
        let span = self.current_span();

        if self.is_eof() && kind != TokenKind::Eof {
            self.unexpected_end_of_file();

            Span::missing()
        } else if self.current_kind() != kind {
            self.expected_token(kind);

            Span::missing()
        } else {
            self.next();

            span
        }
    }

    fn try_consume(&mut self, kind: TokenKind) -> Option<Span> {
        match self.current_kind() {
            k if k == kind => {
                let span = self.current_span();

                self.next();

                Some(span)
            }
            _ => None,
        }
    }

    fn diagnostic(&mut self, diagnostic: ParserDiagnostic, severity: Severity, span: Span) {
        self.state.diagnostics.push(Diagnostic::new(diagnostic, severity, span));
    }
}

fn top_level_statement(state: &mut State) -> Statement {
    let start_span = state.current().span;
    let current = state.current();

    match &current.kind {
        TokenKind::Namespace | TokenKind::Use | TokenKind::Const | TokenKind::HaltCompiler => {
            let comments = state.comments();
            let kind = match &state.current().kind {
                TokenKind::Namespace => namespaces::parse_namespace(state),
                TokenKind::Use => uses::parse_use_statement(state),
                TokenKind::Const => StatementKind::Constant(constants::parse_constant(state)),
                TokenKind::HaltCompiler => {
                    let start = current.span;

                    state.next();

                    let (span, content) = if let TokenKind::InlineHtml = state.current().kind {
                        let content = state.current().clone();
                        state.next();
                        (Span::combine(start, content.span), Some(content))
                    } else {
                        (start, None)
                    };

                    StatementKind::HaltCompiler(HaltCompilerStatement {
                        id: state.id(),
                        span,
                        content,
                    })
                }
                _ => unreachable!(),
            };

            Statement::new(
                state.id(),
                kind,
                Span::new(start_span.start, state.previous().span.end),
                comments,
            )
        }
        _ => statement(state),
    }
}

fn statement(state: &mut State) -> Statement {
    let start_span = state.current().span;
    let comments = state.comments();

    let has_attributes = attributes::gather_attributes(state);
    let current = state.current();
    let peek = state.peek();
    let statement = if has_attributes {
        match &current.kind {
            TokenKind::Abstract => classes::parse_class(state),
            TokenKind::Readonly if peek.kind != TokenKind::LeftParen => classes::parse_class(state),
            TokenKind::Final => classes::parse_class(state),
            TokenKind::Class => classes::parse_class(state),
            TokenKind::Interface => interfaces::parse_interface(state),
            TokenKind::Trait => traits::parse_trait(state),
            TokenKind::Enum
                if !matches!(
                    peek.kind,
                    TokenKind::LeftParen | TokenKind::DoubleColon | TokenKind::Colon,
                ) =>
            {
                enums::parse_enum(state)
            }
            TokenKind::Function
                if identifiers::is_identifier_maybe_soft_reserved(&peek.kind)
                    || peek.kind == TokenKind::Ampersand =>
            {
                if peek.kind == TokenKind::Ampersand {
                    if !identifiers::is_identifier_maybe_soft_reserved(&state.lookahead(1).kind) {
                        let expression = expressions::attributes(state);
                        let ending = utils::skip_ending(state);
                        let ending_span = ending.span();

                        let span = Span::combine(start_span, ending_span);
                        let kind = StatementKind::Expression(ExpressionStatement {
                            id: state.id(),
                            span,
                            expression,
                            ending,
                        });

                        return Statement::new(state.id(), kind, span, comments);
                    }

                    functions::parse_function(state)
                } else {
                    functions::parse_function(state)
                }
            }
            _ => {
                let start = current.span;
                let expression = expressions::attributes(state);
                let ending = utils::skip_ending(state);
                let ending_span = ending.span();

                StatementKind::Expression(ExpressionStatement {
                    id: state.id(),
                    span: Span::combine(start, ending_span),
                    expression,
                    ending,
                })
            }
        }
    } else {
        match &current.kind {
            TokenKind::OpenTag(OpenTagKind::Echo) => {
                let span = current.span;
                state.next();

                StatementKind::EchoOpeningTag(EchoOpeningTagStatement {
                    id: state.id(),
                    span,
                })
            }
            TokenKind::OpenTag(OpenTagKind::Full) => {
                let span = current.span;
                state.next();

                StatementKind::FullOpeningTag(FullOpeningTagStatement {
                    id: state.id(),
                    span,
                })
            }
            TokenKind::OpenTag(OpenTagKind::Short) => {
                let span = current.span;
                state.next();

                StatementKind::ShortOpeningTag(ShortOpeningTagStatement {
                    id: state.id(),
                    span,
                })
            }
            TokenKind::CloseTag => {
                let span = current.span;
                state.next();

                StatementKind::ClosingTag(ClosingTagStatement {
                    id: state.id(),
                    span,
                })
            }
            TokenKind::Abstract => classes::parse_class(state),
            TokenKind::Readonly if peek.kind != TokenKind::LeftParen => classes::parse_class(state),
            TokenKind::Final => classes::parse_class(state),
            TokenKind::Class => classes::parse_class(state),
            TokenKind::Interface => interfaces::parse_interface(state),
            TokenKind::Trait => traits::parse_trait(state),
            TokenKind::Enum
                if !matches!(
                    peek.kind,
                    TokenKind::LeftParen | TokenKind::DoubleColon | TokenKind::Colon,
                ) =>
            {
                enums::parse_enum(state)
            }
            TokenKind::Function
                if identifiers::is_identifier_maybe_soft_reserved(&peek.kind)
                    || peek.kind == TokenKind::Ampersand =>
            {
                if peek.kind == TokenKind::Ampersand {
                    if !identifiers::is_identifier_maybe_soft_reserved(&state.lookahead(1).kind) {
                        let expression = expressions::attributes(state);
                        let ending = utils::skip_ending(state);
                        let ending_span = ending.span();

                        let span = Span::combine(start_span, ending_span);

                        let kind = StatementKind::Expression(ExpressionStatement {
                            id: state.id(),
                            span,
                            expression,
                            ending,
                        });

                        return Statement::new(state.id(), kind, span, comments);
                    }

                    functions::parse_function(state)
                } else {
                    functions::parse_function(state)
                }
            }
            TokenKind::Goto => goto::parse_goto_statement(state),
            token
                if identifiers::is_identifier_maybe_reserved(token)
                    && peek.kind == TokenKind::Colon =>
            {
                goto::parse_label_statement(state)
            }
            TokenKind::Declare => {
                let declare = utils::skip(state, TokenKind::Declare);

                let entries = {
                    let start = utils::skip_left_parenthesis(state);
                    let mut entries = Vec::new();
                    loop {
                        let key = identifiers::parse_identifier(state);
                        let start = key.span;
                        let equals = utils::skip(state, TokenKind::Equals);
                        let value = parse_literal(state);
                        let end = value.span;

                        entries.push(DeclareEntry {
                            id: state.id(),
                            span: Span::combine(start, end),
                            key,
                            equals,
                            value,
                        });

                        if state.current().kind == TokenKind::Comma {
                            state.next();
                        } else {
                            break;
                        }
                    }

                    let end = utils::skip_right_parenthesis(state);
                    let span = Span::combine(start, end);

                    DeclareEntryGroup {
                        id: state.id(),
                        span,
                        left_parenthesis: start,
                        entries,
                        right_parenthesis: end,
                    }
                };

                let body = match state.current().kind {
                    TokenKind::SemiColon => {
                        let span = utils::skip_semicolon(state);

                        DeclareBody::Noop(DeclareBodyNoop {
                            id: state.id(),
                            span,
                            semicolon: span,
                        })
                    }
                    TokenKind::LeftBrace => {
                        let start = utils::skip_left_brace(state);
                        let statements =
                            blocks::parse_multiple_statements_until(state, &TokenKind::RightBrace);
                        let end = utils::skip_right_brace(state);

                        DeclareBody::Braced(DeclareBodyBraced {
                            id: state.id(),
                            span: Span::combine(start, end),
                            left_brace: start,
                            statements,
                            right_brace: end,
                        })
                    }
                    TokenKind::Colon => {
                        let start = utils::skip_colon(state);
                        let statements =
                            blocks::parse_multiple_statements_until(state, &TokenKind::EndDeclare);
                        let enddeclare = utils::skip(state, TokenKind::EndDeclare);
                        let semicolon = utils::skip_semicolon(state);

                        DeclareBody::Block(DeclareBodyBlock {
                            id: state.id(),
                            span: Span::combine(start, semicolon),
                            colon: start,
                            statements,
                            enddeclare,
                            semicolon,
                        })
                    }
                    _ => {
                        let expression = expressions::create(state);
                        let end = utils::skip_semicolon(state);
                        let span = Span::combine(expression.span(), end.span());

                        DeclareBody::Expression(DeclareBodyExpression {
                            id: state.id(),
                            span,
                            expression,
                            semicolon: end,
                        })
                    }
                };

                let span = Span::combine(declare, body.span());

                StatementKind::Declare(DeclareStatement {
                    id: state.id(),
                    span,
                    declare,
                    entries,
                    body,
                })
            }
            TokenKind::Global => {
                let global = current.span;
                state.next();

                let mut variables = vec![];
                // `loop` instead of `while` as we don't allow for extra commas.
                loop {
                    variables.push(variables::parse_dynamic_variable(state));

                    if state.current().kind == TokenKind::Comma {
                        state.next();
                    } else {
                        break;
                    }
                }

                let semicolon = utils::skip_semicolon(state);
                let span = Span::combine(global, semicolon);

                StatementKind::Global(GlobalStatement {
                    id: state.id(),
                    span,
                    global,
                    variables,
                    semicolon,
                })
            }
            TokenKind::Static if matches!(peek.kind, TokenKind::Variable) => {
                state.next();

                let mut vars = vec![];

                // `loop` instead of `while` as we don't allow for extra commas.
                loop {
                    let var = variables::parse_simple_variable(state);
                    let mut default = None;

                    if state.current().kind == TokenKind::Equals {
                        state.next();

                        default = Some(expressions::create(state));
                    }

                    let span = if let Some(default) = &default {
                        Span::combine(var.span, default.span)
                    } else {
                        var.span
                    };

                    vars.push(StaticVar {
                        id: state.id(),
                        span,
                        var: Variable::SimpleVariable(var),
                        default,
                    });

                    if state.current().kind == TokenKind::Comma {
                        state.next();
                    } else {
                        break;
                    }
                }

                let semicolon = utils::skip_semicolon(state);
                let span = Span::combine(current.span, semicolon);

                StatementKind::Static(StaticStatement {
                    id: state.id(),
                    span,
                    vars,
                    semicolon,
                })
            }
            TokenKind::InlineHtml => {
                let html = state.current().clone();
                state.next();

                StatementKind::InlineHtml(InlineHtmlStatement {
                    id: state.id(),
                    span: html.span,
                    html,
                })
            }
            TokenKind::Do => loops::parse_do_while_statement(state),
            TokenKind::While => loops::parse_while_statement(state),
            TokenKind::For => loops::parse_for_statement(state),
            TokenKind::Foreach => loops::parse_foreach_statement(state),
            TokenKind::Continue => loops::parse_continue_statement(state),
            TokenKind::Break => loops::parse_break_statement(state),
            TokenKind::Switch => control_flow::parse_switch_statement(state),
            TokenKind::If => control_flow::parse_if_statement(state),
            TokenKind::Try => try_block::parse_try_block(state),
            TokenKind::LeftBrace => blocks::parse_block_statement(state),
            TokenKind::SemiColon => {
                let start = current.span;

                state.next();

                StatementKind::Noop(start)
            }
            TokenKind::Echo => {
                let echo = current.span;
                state.next();

                let mut values = Vec::new();
                // FIXME: We should check for a semi-colon here and produce a better error,
                //        currently the error says that the semi-colon is unexpected (which it is)
                //        but a more appropriate error would be that the expression is missing and
                //        that the semi-colon is fine where it is (or at least not the real problem).
                loop {
                    values.push(expressions::create(state));

                    if state.current().kind == TokenKind::Comma {
                        state.next();
                    } else {
                        break;
                    }
                }

                let ending = utils::skip_ending(state);
                let end = ending.span();

                StatementKind::Echo(EchoStatement {
                    id: state.id(),
                    span: Span::combine(echo, end),
                    echo,
                    values,
                    ending,
                })
            }
            TokenKind::Return => {
                let r#return = current.span;

                state.next();

                let value = if matches!(
                    state.current().kind,
                    TokenKind::SemiColon | TokenKind::CloseTag
                ) {
                    None
                } else {
                    Some(expressions::create(state))
                };

                let ending = utils::skip_ending(state);
                let end = ending.span();

                StatementKind::Return(ReturnStatement {
                    id: state.id(),
                    span: Span::combine(r#return, end),
                    r#return,
                    value,
                    ending,
                })
            }
            _ => {
                let expression = expressions::create(state);
                let ending = utils::skip_ending(state);

                StatementKind::Expression(ExpressionStatement {
                    id: state.id(),
                    span: Span::combine(expression.span, ending.span()),
                    expression,
                    ending,
                })
            }
        }
    };

    let span = statement.span();

    Statement::new(state.id(), statement, span, comments)
}
