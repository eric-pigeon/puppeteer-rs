// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ConsoleMessageSource {
    Xml,
    Javascript,
    Network,
    ConsoleApi,
    Storage,
    Appcache,
    Rendering,
    Security,
    Other,
    Deprecation,
    Worker,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ConsoleMessageLevel {
    Log,
    Warning,
    Error,
    Debug,
    Info,
}
// Console message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleMessage {
    // Message source.
    pub source: String,
    // Message severity.
    pub level: String,
    // Message text.
    pub text: String,
    // URL of the message origin.
    pub url: Option<String>,
    // Line number in the resource that generated this message (1-based).
    pub line: Option<i32>,
    // Column number in the resource that generated this message (1-based).
    pub column: Option<i32>,
}

// Does nothing.
#[derive(Serialize, Debug)]
pub struct ClearMessages {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearMessagesReturnObject {}
impl super::Command for ClearMessages {
    const NAME: &'static str = "Console.clearMessages";

    type ReturnObject = ClearMessagesReturnObject;
}
// Disables console domain, prevents further console messages from being reported to the client.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Console.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables console domain, sends the messages collected so far to the client by means of the
// `messageAdded` notification.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Console.enable";

    type ReturnObject = EnableReturnObject;
}

// Issued when new console message is added.
#[derive(Deserialize, Debug, Clone)]
pub struct MessageAdded {
    // Console message that has been added.
    pub message: ConsoleMessage,
}
