// This file is auto-generated do not edit manually.

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
pub enum ConsoleMessageLevel {
    Log,
    Warning,
    Error,
    Debug,
    Info,
}
// Console message.
pub struct ConsoleMessage {
    // Message source.
    pub source: ConsoleMessageSource,
    // Message severity.
    pub level: ConsoleMessageLevel,
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
pub struct ClearMessages {}
pub struct ClearMessagesReturnObject {}
// Disables console domain, prevents further console messages from being reported to the client.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables console domain, sends the messages collected so far to the client by means of the
// `messageAdded` notification.
pub struct Enable {}
pub struct EnableReturnObject {}
