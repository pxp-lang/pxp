use tower_lsp::lsp_types::{
    CompletionOptions, 
    DeclarationCapability, 
    DiagnosticOptions, 
    DocumentSymbolOptions,
    FileOperationRegistrationOptions, 
    HoverProviderCapability, 
    ImplementationProviderCapability,
    InitializeParams, 
    OneOf, 
    ReferencesOptions, 
    RenameOptions, 
    SelectionRangeProviderCapability,
    ServerCapabilities, 
    SignatureHelpOptions, 
    TextDocumentSyncCapability, 
    TextDocumentSyncKind,
    TypeDefinitionProviderCapability, 
    WorkDoneProgressOptions,
    WorkspaceFileOperationsServerCapabilities, 
    WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
    DiagnosticServerCapabilities,
};

pub(crate) fn get_server_capabilities(_params: InitializeParams) -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: None,
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: None,
            work_done_progress_options: Default::default(),
            all_commit_characters: None,
            completion_item: None,
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: None,
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        definition_provider: Some(OneOf::Left(true)),
        type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        references_provider: Some(OneOf::Right(ReferencesOptions {
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        document_highlight_provider: None,
        document_symbol_provider: Some(OneOf::Right(DocumentSymbolOptions {
            label: Some("Document Symbol".to_string()),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        code_action_provider: None,
        code_lens_provider: None,
        document_formatting_provider: None,
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        declaration_provider: Some(DeclarationCapability::Simple(true)),
        execute_command_provider: None,
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
        call_hierarchy_provider: None,
        semantic_tokens_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        inline_value_provider: None,
        inlay_hint_provider: None,
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("pxp_language_server".to_string()),
            inter_file_dependencies: true,
            workspace_diagnostics: true,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        experimental: None,
    }
}
