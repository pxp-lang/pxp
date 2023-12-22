use pxp_span::Span;
use pxp_symbol::{SymbolTable, Symbol};
use pxp_type::Type;

use crate::{token::{Token, TokenKind}, ast::{Node, NodeKind, Text, Tag, TagKind}};

use self::state::State;

mod state;

pub struct Parser;

impl Parser {
    pub const fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: &[Token], symbol_table: &mut SymbolTable) -> ParseResult<Vec<Node>> {
        let mut state = State::new(tokens, symbol_table);
        let mut nodes = Vec::new();

        if state.current().kind != TokenKind::OpenPhpdoc {
            return Err(ParseError::MissingPhpdocOpen);
        }

        state.next();

        if let TokenKind::PhpdocEol = state.current().kind {
            state.next();
        }

        while state.current().kind != TokenKind::ClosePhpdoc {
            let current = state.current();

            match current.kind {
                TokenKind::PhpdocEol => {
                    state.next();
                },
                TokenKind::HorizontalWhitespace => {
                    state.next();
                }
                TokenKind::PhpdocTag => {
                    let node = self.parse_tag(&mut state)?;
                    nodes.push(node);
                },
                _ => {
                    let node = self.parse_text(&mut state)?;
                    nodes.push(node);
                }
            }
        }

        if state.current().kind != TokenKind::ClosePhpdoc {
            return Err(ParseError::MissingPhpdocClose);
        }

        state.next();

        Ok(nodes)
    }

    fn parse_tag(&self, state: &mut State) -> ParseResult<Node> {
        let start_span = state.current().span;
        let tag_token = state.current();
        state.next();
        let tag = state.symbol_table.resolve(tag_token.symbol).unwrap();

        Ok(match &tag[..] {
            b"@param" => self.parse_param_tag(state)?,
            _ => {
                let description = self.parse_optional_description(state)?;
                let end_span = state.previous().span;
                let span = Span::new(start_span.start, end_span.end);

                Node::new(NodeKind::Tag(Tag::new(TagKind::Generic { tag: tag_token.symbol, description }, span)), span)
            },
        })
    }

    fn parse_param_tag(&self, state: &mut State) -> ParseResult<Node> {
        let start_span = state.current().span;

        // Skip over tag.
        state.next();
        state.skip_horizontal_whitespace();

        let r#type = match state.current().kind {
            TokenKind::Variable | TokenKind::Variadic | TokenKind::Reference => {
                None
            },
            _ => Some(self.parse_type(state)?),
        };

        let is_reference = match state.current().kind {
            TokenKind::Reference => {
                state.next();

                true
            },
            _ => false,
        };

        state.skip_horizontal_whitespace();

        let is_variadic = match state.current().kind {
            TokenKind::Variadic => {
                state.next();

                true
            },
            _ => false,
        };

        state.skip_horizontal_whitespace();

        let name = state.current().symbol;
        state.next();

        let description = self.parse_optional_description(state)?;
        let end_span = state.previous().span;

        let span = Span::new(start_span.start, end_span.end);

        Ok(Node::new(NodeKind::Tag(Tag::new(TagKind::Param { r#type, is_reference, is_variadic, name, description }, span)), span))
    }

    fn parse_optional_description(&self, state: &mut State) -> ParseResult<Option<Symbol>> {
        if state.is_eof() {
            return Ok(None);
        }

        let current = state.current();

        match current.kind {
            TokenKind::PhpdocEol => {
                Ok(None)
            },
            _ => {
                Ok(Some(self.parse_text_symbol(state)?))
            }
        }
    }

    fn parse_type(&self, state: &mut State) -> ParseResult<Type> {
        let current = state.current();

        if current.kind == TokenKind::Nullable {
            return self.parse_nullable_type(state);
        }

        if current.kind == TokenKind::OpenParen {
            return self.parse_dnf_type(state);
        }

        let ty = self.parse_simple_type(state)?;

        if state.current().kind == TokenKind::Union {
            return self.parse_union_type(state, ty);
        }

        if state.current().kind == TokenKind::Intersection {
            return self.parse_intersection_type(state, ty);
        }

        Ok(ty)
    }

    fn parse_dnf_type(&self, state: &mut State) -> ParseResult<Type> {
        todo!()
    }

    fn parse_intersection_type(&self, state: &mut State, inner: Type) -> ParseResult<Type> {
        let mut types = vec![inner];

        state.next();

        loop {
            let ty = self.parse_simple_type(state)?;
            types.push(ty);

            if state.current().kind != TokenKind::Intersection {
                break;
            }
        }

        Ok(Type::Intersection(types))
    }

    fn parse_union_type(&self, state: &mut State, inner: Type) -> ParseResult<Type> {
        let mut types = vec![inner];

        state.next();

        loop {
            let ty = self.parse_simple_type(state)?;
            types.push(ty);

            if state.current().kind != TokenKind::Union {
                break;
            }
        }

        Ok(Type::Union(types))
    }

    fn parse_nullable_type(&self, state: &mut State) -> ParseResult<Type> {
        let start_span = state.current().span;
        state.next();
        state.skip_horizontal_whitespace();

        let ty = self.parse_simple_type(state)?;
        let end_span = state.previous().span;

        let span = Span::new(start_span.start, end_span.end);

        Ok(Type::Nullable(span, Box::new(ty)))
    }

    fn parse_simple_type(&self, state: &mut State) -> ParseResult<Type> {
        let current = state.current();
        state.next();
        let symbol = state.symbol_table.resolve(current.symbol).unwrap();
        let span = current.span;

        Ok(match &symbol[..] {
            b"int" => Type::Integer(span),
            b"float" => Type::Float(span),
            b"string" => Type::String(span),
            b"void" => Type::Void(span),
            b"null" => Type::Null(span),
            b"true" => Type::True(span),
            b"false" => Type::False(span),
            b"never" => Type::Never(span),
            b"bool" => Type::Boolean(span),
            b"array" => Type::Array(span),
            b"object" => Type::Object(span),
            b"mixed" => Type::Mixed(span),
            b"callable" => Type::Callable(span),
            b"iterable" => Type::Iterable(span),
            b"static" => Type::StaticReference(span),
            b"self" => Type::SelfReference(span),
            b"parent" => Type::ParentReference(span),
            _ => Type::Named(span, current.symbol),
        })
    }

    fn parse_text_symbol(&self, state: &mut State) -> ParseResult<Symbol> {
        let mut symbols = Vec::new();

        // We don't care about leading whitespace in the description.
        if state.current().kind == TokenKind::HorizontalWhitespace {
            state.next();
        }

        loop {
            if state.is_eof() {
                break;
            }

            let current = state.current();

            match current.kind {
                TokenKind::PhpdocEol => break,
                TokenKind::ClosePhpdoc => break,
                _ => {
                    state.next();

                    symbols.push(current.symbol);
                }
            }
        }

        Ok(state.symbol_table.coagulate(&symbols, None))
    }

    fn parse_text(&self, state: &mut State) -> ParseResult<Node> {
        let start_span = state.current().span;
        let symbol = self.parse_text_symbol(state)?;
        let end_span = state.previous().span;
        let span = Span::new(start_span.start, end_span.end);

        Ok(Node::new(NodeKind::Text(Text::new(symbol)), span))
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    MissingPhpdocOpen,
    MissingPhpdocClose,
}
