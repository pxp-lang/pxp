use std::collections::VecDeque;

use crate::lexer::byte_string::ByteString;
use crate::lexer::error::SyntaxError;
use crate::lexer::error::SyntaxResult;
use crate::lexer::state::source::Source;
use crate::lexer::token::DocStringIndentationAmount;
use crate::lexer::token::DocStringIndentationKind;
use crate::lexer::token::DocStringKind;

pub mod source;

#[derive(Debug)]
pub enum StackFrame {
    Initial,
    Scripting,
    Halted,
    DoubleQuote,
    ShellExec,
    DocString(
        DocStringKind,
        ByteString,
        DocStringIndentationKind,
        DocStringIndentationAmount,
    ),
    LookingForVarname,
    LookingForProperty,
    VarOffset,
}

#[derive(Debug)]
pub struct State<'a> {
    pub source: Source<'a>,
    pub stack: VecDeque<StackFrame>,
}

impl<'a> State<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self {
            source,
            stack: VecDeque::from([StackFrame::Initial]),
        }
    }

    pub fn frame(&self) -> SyntaxResult<&StackFrame> {
        self.stack
            .back()
            .ok_or_else(|| SyntaxError::UnpredictableState(self.source.span()))
    }

    pub fn replace(&mut self, state: StackFrame) {
        let i = self.stack.len() - 1;

        self.stack[i] = state;
    }

    pub fn enter(&mut self, state: StackFrame) {
        self.stack.push_back(state);
    }

    pub fn exit(&mut self) {
        self.stack.pop_back();
    }
}
