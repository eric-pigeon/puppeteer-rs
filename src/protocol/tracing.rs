// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Configuration for memory dump. Used only when "memory-infra" category is enabled.
pub type MemoryDumpConfig = std::collections::HashMap<String, serde_json::Value>;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TraceConfigRecordMode {
    RecordUntilFull,
    RecordContinuously,
    RecordAsMuchAsPossible,
    EchoToConsole,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TraceConfig {
    // Controls how the trace buffer stores data.
    pub record_mode: Option<String>,
    // Turns on JavaScript stack sampling.
    pub enable_sampling: Option<bool>,
    // Turns on system tracing.
    pub enable_systrace: Option<bool>,
    // Turns on argument filter.
    pub enable_argument_filter: Option<bool>,
    // Included category filters.
    pub included_categories: Option<Vec<String>>,
    // Excluded category filters.
    pub excluded_categories: Option<Vec<String>>,
    // Configuration to synthesize the delays in tracing.
    pub synthetic_delays: Option<Vec<String>>,
    // Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.
    pub memory_dump_config: Option<MemoryDumpConfig>,
}
// Data format of a trace. Can be either the legacy JSON format or the
// protocol buffer format. Note that the JSON format will be deprecated soon.
pub type StreamFormat = String;
// Compression type to use for traces returned via streams.
pub type StreamCompression = String;

// Stop trace events collection.
#[derive(Serialize, Debug)]
pub struct End {}
#[derive(Deserialize, Debug, Clone)]
pub struct EndReturnObject {}
impl super::Command for End {
    const NAME: &'static str = "Tracing.end";

    type ReturnObject = EndReturnObject;
}
// Gets supported tracing categories.
#[derive(Serialize, Debug)]
pub struct GetCategories {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetCategoriesReturnObject {
    // A list of supported tracing categories.
    pub categories: Vec<String>,
}
impl super::Command for GetCategories {
    const NAME: &'static str = "Tracing.getCategories";

    type ReturnObject = GetCategoriesReturnObject;
}
// Record a clock sync marker in the trace.
#[derive(Serialize, Debug)]
pub struct RecordClockSyncMarker {
    // The ID of this clock sync marker
    pub sync_id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RecordClockSyncMarkerReturnObject {}
impl super::Command for RecordClockSyncMarker {
    const NAME: &'static str = "Tracing.recordClockSyncMarker";

    type ReturnObject = RecordClockSyncMarkerReturnObject;
}
// Request a global memory dump.
#[derive(Serialize, Debug)]
pub struct RequestMemoryDump {
    // Enables more deterministic results by forcing garbage collection
    pub deterministic: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestMemoryDumpReturnObject {
    // GUID of the resulting global memory dump.
    pub dump_guid: String,
    // True iff the global memory dump succeeded.
    pub success: bool,
}
impl super::Command for RequestMemoryDump {
    const NAME: &'static str = "Tracing.requestMemoryDump";

    type ReturnObject = RequestMemoryDumpReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TransferMode {
    ReportEvents,
    ReturnAsStream,
}
// Start trace events collection.
#[derive(Serialize, Debug)]
pub struct Start {
    // Category/tag filter
    pub categories: Option<String>,
    // Tracing options
    pub options: Option<String>,
    // If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    pub buffer_usage_reporting_interval: Option<f64>,
    // Whether to report trace events as series of dataCollected events or to save trace to a
    // stream (defaults to `ReportEvents`).
    pub transfer_mode: Option<String>,
    // Trace data format to use. This only applies when using `ReturnAsStream`
    // transfer mode (defaults to `json`).
    pub stream_format: Option<StreamFormat>,
    // Compression format to use. This only applies when using `ReturnAsStream`
    // transfer mode (defaults to `none`)
    pub stream_compression: Option<StreamCompression>,
    pub trace_config: Option<TraceConfig>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartReturnObject {}
impl super::Command for Start {
    const NAME: &'static str = "Tracing.start";

    type ReturnObject = StartReturnObject;
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BufferUsageEvent {
    pub params: BufferUsageParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BufferUsageParams {
    // A number in range [0..1] that indicates the used size of event buffer as a fraction of its
    // total size.
    pub percent_full: Option<f64>,
    // An approximate number of events in the trace log.
    pub event_count: Option<f64>,
    // A number in range [0..1] that indicates the used size of event buffer as a fraction of its
    // total size.
    pub value: Option<f64>,
}
// Contains an bucket of collected trace events. When tracing is stopped collected events will be
// send as a sequence of dataCollected events followed by tracingComplete event.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DataCollectedEvent {
    pub params: DataCollectedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataCollectedParams {
    pub value: Vec<serde_json::map::Map<String, serde_json::Value>>,
}
// Signals that tracing is stopped and there is no trace buffers pending flush, all data were
// delivered via dataCollected events.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TracingCompleteEvent {
    pub params: TracingCompleteParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TracingCompleteParams {
    // Indicates whether some trace data is known to have been lost, e.g. because the trace ring
    // buffer wrapped around.
    pub data_loss_occurred: bool,
    // A handle of the stream that holds resulting trace data.
    pub stream: Option<super::io::StreamHandle>,
    // Trace data format of returned stream.
    pub trace_format: Option<StreamFormat>,
    // Compression format of returned stream.
    pub stream_compression: Option<StreamCompression>,
}
