use crate::internal::identifiers;
use crate::internal::utils;
use crate::scoped;
use crate::state::NamespaceType;
use crate::state::Scope;
use crate::state::State;
use crate::ParserDiagnostic;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::namespaces::BracedNamespace;
use pxp_ast::namespaces::BracedNamespaceBody;
use pxp_ast::namespaces::NamespaceStatement;
use pxp_ast::namespaces::UnbracedNamespace;
use pxp_ast::Block;
use pxp_ast::StatementKind;

use pxp_diagnostics::Severity;
use pxp_span::Span;
use pxp_token::TokenKind;

pub fn namespace(state: &mut State) -> StatementKind {
    let start = utils::skip(state, TokenKind::Namespace);
    let name = identifiers::optional_name(state);

    let current = state.stream.current();

    if let Some(name) = &name {
        if current.kind != TokenKind::LeftBrace {
            if let Some(NamespaceType::Braced) = state.namespace_type() {
                state.diagnostic(
                    ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                    Severity::Error,
                    current.span,
                );
            }

            return unbraced_namespace(state, start, name.clone());
        }
    }

    match state.namespace_type() {
        Some(NamespaceType::Unbraced) => {
            state.diagnostic(
                ParserDiagnostic::CannotMixBracketedAndUnbracketedNamespaceDeclarations,
                Severity::Error,
                current.span,
            );

            braced_namespace(state, start, name)
        }
        Some(NamespaceType::Braced) if state.namespace().is_some() => {
            state.diagnostic(
                ParserDiagnostic::NestedNamespace,
                Severity::Error,
                current.span,
            );

            braced_namespace(state, start, name)
        }
        _ => braced_namespace(state, start, name),
    }
}

fn unbraced_namespace(state: &mut State, start: Span, name: SimpleIdentifier) -> StatementKind {
    let end = utils::skip_semicolon(state);

    let statements = scoped!(state, Scope::Namespace(name.clone()), {
        let mut statements = Block::new();

        while state.stream.current().kind != TokenKind::Namespace && !state.stream.is_eof() {
            // NOTE: If we encounter a right-brace here, it's possible that we're in a nested namespace.
            // We should check to see if the previous scope is a BracedNamespace and break out of this scope.
            if state.stream.current().kind == TokenKind::RightBrace {
                if let Some(Scope::BracedNamespace(_)) = state.previous_scope() {
                    break;
                }
            }

            statements.push(crate::top_level_statement(state));
        }

        statements
    });

    StatementKind::Namespace(NamespaceStatement::Unbraced(UnbracedNamespace {
        start,
        end,
        name,
        statements,
    }))
}

fn braced_namespace(
    state: &mut State,
    span: Span,
    name: Option<SimpleIdentifier>,
) -> StatementKind {
    let body = scoped!(state, Scope::BracedNamespace(name.clone()), {
        let start = utils::skip_left_brace(state);

        let mut statements = Block::new();
        while state.stream.current().kind != TokenKind::RightBrace && !state.stream.is_eof() {
            statements.push(crate::top_level_statement(state));
        }

        let end = utils::skip_right_brace(state);

        BracedNamespaceBody {
            start,
            end,
            statements,
        }
    });

    StatementKind::Namespace(NamespaceStatement::Braced(BracedNamespace {
        namespace: span,
        name,
        body,
    }))
}
