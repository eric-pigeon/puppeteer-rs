// This file is auto-generated do not edit manually.

pub enum LogEntrySource {
    Xml,
    Javascript,
    Network,
    Storage,
    Appcache,
    Rendering,
    Security,
    Deprecation,
    Worker,
    Violation,
    Intervention,
    Recommendation,
    Other,
}
pub enum LogEntryLevel {
    Verbose,
    Info,
    Warning,
    Error,
}
// Log entry.
pub struct LogEntry {
    // Log entry source.
    pub source: LogEntrySource,
    // Log entry severity.
    pub level: LogEntryLevel,
    // Logged text.
    pub text: String,
    // Timestamp when this entry was added.
    pub timestamp: super::runtime::Timestamp,
    // URL of the resource if known.
    pub url: Option<String>,
    // Line number in the resource.
    pub line_number: Option<i32>,
    // JavaScript stack trace.
    pub stack_trace: Option<super::runtime::StackTrace>,
    // Identifier of the network request associated with this entry.
    pub network_request_id: Option<super::network::RequestId>,
    // Identifier of the worker associated with this entry.
    pub worker_id: Option<String>,
    // Call arguments.
    pub args: Option<Vec<super::runtime::RemoteObject>>,
}
pub enum ViolationSettingName {
    LongTask,
    LongLayout,
    BlockedEvent,
    BlockedParser,
    DiscouragedAPIUse,
    Handler,
    RecurringHandler,
}
// Violation configuration setting.
pub struct ViolationSetting {
    // Violation type.
    pub name: ViolationSettingName,
    // Time threshold to trigger upon.
    pub threshold: f64,
}

// Clears the log.
pub struct Clear {}
pub struct ClearReturnObject {}
// Disables log domain, prevents further log entries from being reported to the client.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables log domain, sends the entries collected so far to the client by means of the
// `entryAdded` notification.
pub struct Enable {}
pub struct EnableReturnObject {}
// start violation reporting.
pub struct StartViolationsReport {
    // Configuration for violations.
    pub config: Vec<ViolationSetting>,
}
pub struct StartViolationsReportReturnObject {}
// Stop violation reporting.
pub struct StopViolationsReport {}
pub struct StopViolationsReportReturnObject {}
