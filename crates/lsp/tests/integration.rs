use std::path::PathBuf;

use lsp_types::{InitializeParams, InitializeResult};
use pxp_lsp::{testing::TestServerManager, Client, LanguageServer};

#[test]
#[should_panic(expected = "Initialized!")]
fn it_initializes() {
    server().go(|_| {
        assert!(false, "Initialized!");
    });
}

fn server() -> TestServerManager<TestServer> {
    TestServerManager::new(TestServer::new).workspace(PathBuf::from("./tests/workspace"))
}

struct TestServer;

impl TestServer {
    pub fn new() -> Self {
        Self
    }
}

impl LanguageServer for TestServer {
    fn initialize(&mut self, _: &Client, _: &InitializeParams) -> InitializeResult {
        InitializeResult::default()
    }
}
