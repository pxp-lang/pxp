use std::{error::Error, thread, time::Duration};

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use lsp_types::{
    notification::{LogMessage, Notification as NotificationTrait, Progress, PublishDiagnostics},
    request::{Request as _, WorkDoneProgressCreate},
    Diagnostic, LogMessageParams, MessageType, ProgressParams, ProgressParamsValue, ProgressToken,
    PublishDiagnosticsParams, Uri, WorkDoneProgress, WorkDoneProgressBegin,
    WorkDoneProgressCreateParams, WorkDoneProgressEnd, WorkDoneProgressReport,
};
use serde_json::to_value;

pub struct Client<'a> {
    pub(super) connection: &'a Connection,
}

impl Client<'_> {
    pub fn log_message(
        &self,
        typ: MessageType,
        message: String,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.connection
            .sender
            .send(Message::Notification(Notification {
                method: LogMessage::METHOD.to_string(),
                params: to_value(LogMessageParams { typ, message })?,
            }))
            .map_err(|err| err.into())
    }

    pub fn with_progress(
        &self,
        title: impl Into<String>,
        cb: impl FnOnce(&ProgressReporter) -> Result<(), Box<dyn Error + Sync + Send>>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let title = title.into();
        let token_string = format!("pls/{}", title);
        let token = ProgressToken::String(token_string.clone());

        self.connection.sender.send(Message::Request(Request {
            id: token_string.into(),
            method: WorkDoneProgressCreate::METHOD.to_string(),
            params: to_value(WorkDoneProgressCreateParams {
                token: token.clone(),
            })?,
        }))?;

        self.connection
            .sender
            .send(Message::Notification(Notification {
                method: Progress::METHOD.to_string(),
                params: to_value(ProgressParams {
                    token: token.clone(),
                    value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(
                        WorkDoneProgressBegin {
                            title,
                            cancellable: Some(false),
                            message: None,
                            percentage: Some(0),
                        },
                    )),
                })?,
            }))?;

        let progress_reporter = ProgressReporter::new(self, token.clone());

        cb(&progress_reporter)?;

        self.connection
            .sender
            .send(Message::Notification(Notification {
                method: Progress::METHOD.to_string(),
                params: to_value(ProgressParams {
                    token: token.clone(),
                    value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(
                        WorkDoneProgressEnd { message: None },
                    )),
                })?,
            }))?;

        Ok(())
    }

    pub fn publish_diagnostics(
        &self,
        uri: &Uri,
        diagnostics: Vec<Diagnostic>,
        version: i32,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.connection
            .sender
            .send(Message::Notification(Notification {
                method: PublishDiagnostics::METHOD.to_string(),
                params: to_value(PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics,
                    version: Some(version),
                })?,
            }))
            .map_err(|err| err.into())
    }
}

pub struct ProgressReporter<'a> {
    client: &'a Client<'a>,
    token: ProgressToken,
}

impl<'a> ProgressReporter<'a> {
    fn new(client: &'a Client<'a>, token: ProgressToken) -> Self {
        Self { client, token }
    }

    pub fn report(
        &self,
        progress: u32,
        message: Option<String>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.client
            .connection
            .sender
            .send(Message::Notification(Notification {
                method: Progress::METHOD.to_string(),
                params: to_value(ProgressParams {
                    token: self.token.clone(),
                    value: ProgressParamsValue::WorkDone(WorkDoneProgress::Report(
                        WorkDoneProgressReport {
                            cancellable: Some(false),
                            message,
                            percentage: Some(progress),
                        },
                    )),
                })?,
            }))
            .map_err(|err| err.into())
    }
}
