use std::error::Error;

use lsp_server::{Connection, Message, Notification};
use lsp_types::{notification::{LogMessage, Notification as NotificationTrait, PublishDiagnostics}, Diagnostic, LogMessageParams, MessageType, PublishDiagnosticsParams, Uri};
use serde_json::to_value;

pub struct Client<'a> {
    pub(super) connection: &'a Connection,
}

impl Client<'_> {
    pub fn log_message(&self, typ: MessageType, message: String) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.connection.sender.send(Message::Notification(Notification {
            method: LogMessage::METHOD.to_string(),
            params: to_value(LogMessageParams { typ, message })?,
        })).map_err(|err| err.into())
    }

    pub fn publish_diagnostics(&self, uri: &Uri, diagnostics: Vec<Diagnostic>, version: i32) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.connection.sender.send(Message::Notification(Notification {
            method: PublishDiagnostics::METHOD.to_string(),
            params: to_value(PublishDiagnosticsParams {
                uri: uri.clone(),
                diagnostics,
                version: Some(version),
            })?,
        })).map_err(|err| err.into())
    }
}