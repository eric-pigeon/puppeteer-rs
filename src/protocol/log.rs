// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LogEntryLevel {
    Verbose,
    Info,
    Warning,
    Error,
}
// Log entry.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViolationSetting {
    // Violation type.
    pub name: ViolationSettingName,
    // Time threshold to trigger upon.
    pub threshold: f64,
}

// Clears the log.
#[derive(Serialize, Debug)]
pub struct Clear {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearReturnObject {}
impl super::Command for Clear {
    const NAME: &'static str = "Log.clear";

    type ReturnObject = ClearReturnObject;
}
// Disables log domain, prevents further log entries from being reported to the client.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Log.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables log domain, sends the entries collected so far to the client by means of the
// `entryAdded` notification.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Log.enable";

    type ReturnObject = EnableReturnObject;
}
// start violation reporting.
#[derive(Serialize, Debug)]
pub struct StartViolationsReport {
    // Configuration for violations.
    pub config: Vec<ViolationSetting>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartViolationsReportReturnObject {}
impl super::Command for StartViolationsReport {
    const NAME: &'static str = "Log.startViolationsReport";

    type ReturnObject = StartViolationsReportReturnObject;
}
// Stop violation reporting.
#[derive(Serialize, Debug)]
pub struct StopViolationsReport {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopViolationsReportReturnObject {}
impl super::Command for StopViolationsReport {
    const NAME: &'static str = "Log.stopViolationsReport";

    type ReturnObject = StopViolationsReportReturnObject;
}
