use pxp_ast::{
    Comment, CommentGroup, CommentKind, HashMarkComment, MultiLineComment, SingleLineComment,
};
use pxp_token::TokenKind;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn comments(&mut self) -> CommentGroup {
        let mut comments = vec![];

        std::mem::swap(&mut self.comments, &mut comments);

        CommentGroup {
            id: self.id(),
            comments,
        }
    }

    pub(crate) fn collect_comments(&mut self) {
        loop {
            if self.is_eof() {
                break;
            }

            if !matches!(
                self.current_kind(),
                TokenKind::SingleLineComment
                    | TokenKind::MultiLineComment
                    | TokenKind::HashMarkComment
                    | TokenKind::DocBlockComment
                    | TokenKind::OpenPhpDoc,
            ) {
                break;
            }

            let id = self.id();
            let comment_id = self.id();

            let (comment, move_forward) = match self.current_kind() {
                TokenKind::SingleLineComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::SingleLine(SingleLineComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                TokenKind::MultiLineComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::MultiLine(MultiLineComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                TokenKind::HashMarkComment => (
                    Comment {
                        id,
                        span: self.current_span(),
                        kind: CommentKind::HashMark(HashMarkComment {
                            id: comment_id,
                            span: self.current_span(),
                            content: self.current_symbol_as_bytestring(),
                        }),
                    },
                    true,
                ),
                TokenKind::OpenPhpDoc => {
                    let docblock = self.parse_docblock();

                    (
                        Comment {
                            id,
                            span: docblock.span,
                            kind: CommentKind::DocBlock(docblock),
                        },
                        false,
                    )
                }
                _ => unreachable!(),
            };

            self.comments.push(comment);

            if move_forward {
                self.next();
            }
        }
    }
}
