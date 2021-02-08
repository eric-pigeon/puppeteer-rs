// This file is auto-generated do not edit manually.

// Configuration for memory dump. Used only when "memory-infra" category is enabled.
pub type MemoryDumpConfig = std::collections::HashMap<String, serde_json::Value>;
pub enum TraceConfigRecordMode {
    RecordUntilFull,
    RecordContinuously,
    RecordAsMuchAsPossible,
    EchoToConsole,
}
pub struct TraceConfig {
    // Controls how the trace buffer stores data.
    pub record_mode: Option<TraceConfigRecordMode>,
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
pub enum StreamFormat {
    Json,
    Proto,
}
// Compression type to use for traces returned via streams.
pub enum StreamCompression {
    None,
    Gzip,
}

// Stop trace events collection.
pub struct End {}
pub struct EndReturnObject {}
// Gets supported tracing categories.
pub struct GetCategories {}
pub struct GetCategoriesReturnObject {
    // A list of supported tracing categories.
    pub categories: Vec<String>,
}
// Record a clock sync marker in the trace.
pub struct RecordClockSyncMarker {
    // The ID of this clock sync marker
    pub sync_id: String,
}
pub struct RecordClockSyncMarkerReturnObject {}
// Request a global memory dump.
pub struct RequestMemoryDump {
    // Enables more deterministic results by forcing garbage collection
    pub deterministic: Option<bool>,
}
pub struct RequestMemoryDumpReturnObject {
    // GUID of the resulting global memory dump.
    pub dump_guid: String,
    // True iff the global memory dump succeeded.
    pub success: bool,
}
pub enum TransferMode {
    ReportEvents,
    ReturnAsStream,
}
// Start trace events collection.
pub struct Start {
    // Category/tag filter
    pub categories: Option<String>,
    // Tracing options
    pub options: Option<String>,
    // If set, the agent will issue bufferUsage events at this interval, specified in milliseconds
    pub buffer_usage_reporting_interval: Option<f64>,
    // Whether to report trace events as series of dataCollected events or to save trace to a
    // stream (defaults to `ReportEvents`).
    pub transfer_mode: Option<TransferMode>,
    // Trace data format to use. This only applies when using `ReturnAsStream`
    // transfer mode (defaults to `json`).
    pub stream_format: Option<StreamFormat>,
    // Compression format to use. This only applies when using `ReturnAsStream`
    // transfer mode (defaults to `none`)
    pub stream_compression: Option<StreamCompression>,
    pub trace_config: Option<TraceConfig>,
}
pub struct StartReturnObject {}
