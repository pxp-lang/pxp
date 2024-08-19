use std::error::Error;

use lsp_server::{Connection, Message, Notification};
use lsp_types::{notification::{LogMessage, Notification as NotificationTrait}, LogMessageParams, MessageType};
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
}