use std::collections::VecDeque;

use pxp_bytestring::ByteString;
use pxp_source::Source;
use pxp_token::{DocStringIndentationKind, DocStringIndentationAmount, DocStringKind};

use crate::{LexerResult, LexerError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Hash)]
pub(crate) enum State {
    Initial,
    Scripting,
    Halted,
    DoubleQuotedString,
    ShellExec,
    LookingForVarname,
    LookingForProperty,
    VarOffset,
    // FIXME: Find a better way of tokenising heredoc / nowdoc strings.
    DocString(DocStringKind, ByteString, DocStringIndentationKind, DocStringIndentationAmount),
}

#[derive(Debug)]
pub(crate) struct StateMachine<'a> {
    source: Source<'a>,
    state: VecDeque<State>,
}

impl<'a> StateMachine<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self {
            source,
            state: VecDeque::from([State::Initial]),
        }
    }

    pub fn state(&self) -> LexerResult<&State> {
        self.state
            .back()
            .ok_or_else(|| LexerError::UnpredictableState(self.source.position()))
    }

    pub fn replace(&mut self, state: State) {
        let i = self.state.len() - 1;
        self.state[i] = state;
    }

    pub fn enter(&mut self, state: State) {
        self.state.push_back(state);
    }

    pub fn exit(&mut self) {
        self.state.pop_back();
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut Source<'a> {
        &mut self.source
    }
}