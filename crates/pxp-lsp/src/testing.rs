use std::path::PathBuf;

use lsp_server::Connection;

use crate::LanguageServer;

pub struct TestServerManager<T: LanguageServer> {
    inner: T,
    workspace: Option<PathBuf>,
}

impl<T: LanguageServer> TestServerManager<T> {
    pub fn new(builder: impl FnOnce() -> T) -> Self {
        Self {
            inner: builder(),
            workspace: None,
        }
    }

    /// Set the workspace directory for the test.
    pub fn workspace(mut self, workspace: PathBuf) -> Self {
        self.workspace = match workspace.canonicalize() {
            Ok(workspace) => Some(workspace),
            Err(err) => panic!("Failed to canonicalize workspace path: {}", err),
        };

        self
    }

    /// Execute the test.
    pub fn go(&mut self, test: impl FnOnce(&mut TestableServer<T>)) {
        let mut testable = TestableServer {
            server: &mut self.inner,
        };

        test(&mut testable);
    }
}

pub struct TestableServer<'a, T: LanguageServer> {
    server: &'a mut T,
}

impl<'a, T: LanguageServer> TestableServer<'a, T> {
    // TODO: Add a method for opening a file.
    // TODO: Add a method for closing a file.
    // TODO: Add a method for changing a file.
    // TODO: Add a method for saving a file.
    // TODO: Add a method for sending a raw notification.
    // TODO: Add a method for sending a raw request.
    // TODO: Add a method for completions.
    // TODO: Add a method for document symbols.
    // TODO: Add a method for hovers.
}
