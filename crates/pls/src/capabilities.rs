use lsp_types::{
    DiagnosticOptions, DiagnosticServerCapabilities, DocumentSymbolOptions, FileOperationRegistrationOptions, OneOf, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions, WorkspaceFileOperationsServerCapabilities, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities
};

pub(crate) fn get_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: None,
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("pls".to_string()),
            inter_file_dependencies: false,
            workspace_diagnostics: false,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                did_create: Some(FileOperationRegistrationOptions { filters: vec![] }),
                did_rename: Some(FileOperationRegistrationOptions { filters: vec![] }),
                did_delete: Some(FileOperationRegistrationOptions { filters: vec![] }),
                will_rename: None,
                will_create: None,
                will_delete: None,
            }),
        }),
        ..Default::default()
    }
}
