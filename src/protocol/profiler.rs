// This file is auto-generated do not edit manually.

// Profile node. Holds callsite information, execution statistics and child nodes.
pub struct ProfileNode {
    // Unique id of the node.
    pub id: i32,
    // Function location.
    pub call_frame: super::runtime::CallFrame,
    // Number of samples where this node was on top of the call stack.
    pub hit_count: Option<i32>,
    // Child node ids.
    pub children: Option<Vec<i32>>,
    // The reason of being not optimized. The function may be deoptimized or marked as don't
    // optimize.
    pub deopt_reason: Option<String>,
    // An array of source position ticks.
    pub position_ticks: Option<Vec<PositionTickInfo>>,
}
// Profile.
pub struct Profile {
    // The list of profile nodes. First item is the root node.
    pub nodes: Vec<ProfileNode>,
    // Profiling start timestamp in microseconds.
    pub start_time: f64,
    // Profiling end timestamp in microseconds.
    pub end_time: f64,
    // Ids of samples top nodes.
    pub samples: Option<Vec<i32>>,
    // Time intervals between adjacent samples in microseconds. The first delta is relative to the
    // profile startTime.
    pub time_deltas: Option<Vec<i32>>,
}
// Specifies a number of samples attributed to a certain source position.
pub struct PositionTickInfo {
    // Source line number (1-based).
    pub line: i32,
    // Number of samples attributed to the source line.
    pub ticks: i32,
}
// Coverage data for a source range.
pub struct CoverageRange {
    // JavaScript script source offset for the range start.
    pub start_offset: i32,
    // JavaScript script source offset for the range end.
    pub end_offset: i32,
    // Collected execution count of the source range.
    pub count: i32,
}
// Coverage data for a JavaScript function.
pub struct FunctionCoverage {
    // JavaScript function name.
    pub function_name: String,
    // Source ranges inside the function with coverage data.
    pub ranges: Vec<CoverageRange>,
    // Whether coverage data for this function has block granularity.
    pub is_block_coverage: bool,
}
// Coverage data for a JavaScript script.
pub struct ScriptCoverage {
    // JavaScript script id.
    pub script_id: super::runtime::ScriptId,
    // JavaScript script name or url.
    pub url: String,
    // Functions contained in the script that has coverage data.
    pub functions: Vec<FunctionCoverage>,
}
// Describes a type collected during runtime.
pub struct TypeObject {
    // Name of a type collected with type profiling.
    pub name: String,
}
// Source offset and types for a parameter or return value.
pub struct TypeProfileEntry {
    // Source offset of the parameter or end of function for return values.
    pub offset: i32,
    // The types for this parameter or return value.
    pub types: Vec<TypeObject>,
}
// Type profile data collected during runtime for a JavaScript script.
pub struct ScriptTypeProfile {
    // JavaScript script id.
    pub script_id: super::runtime::ScriptId,
    // JavaScript script name or url.
    pub url: String,
    // Type profile entries for parameters and return values of the functions in the script.
    pub entries: Vec<TypeProfileEntry>,
}
// Collected counter information.
pub struct CounterInfo {
    // Counter name.
    pub name: String,
    // Counter value.
    pub value: i32,
}
// Runtime call counter information.
pub struct RuntimeCallCounterInfo {
    // Counter name.
    pub name: String,
    // Counter value.
    pub value: f64,
    // Counter time in seconds.
    pub time: f64,
}

pub struct Disable {}
pub struct DisableReturnObject {}
pub struct Enable {}
pub struct EnableReturnObject {}
// Collect coverage data for the current isolate. The coverage data may be incomplete due to
// garbage collection.
pub struct GetBestEffortCoverage {}
pub struct GetBestEffortCoverageReturnObject {
    // Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage>,
}
// Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.
pub struct SetSamplingInterval {
    // New sampling interval in microseconds.
    pub interval: i32,
}
pub struct SetSamplingIntervalReturnObject {}
pub struct Start {}
pub struct StartReturnObject {}
// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
// coverage may be incomplete. Enabling prevents running optimized code and resets execution
// counters.
pub struct StartPreciseCoverage {
    // Collect accurate call counts beyond simple 'covered' or 'not covered'.
    pub call_count: Option<bool>,
    // Collect block-based coverage.
    pub detailed: Option<bool>,
    // Allow the backend to send updates on its own initiative
    pub allow_triggered_updates: Option<bool>,
}
pub struct StartPreciseCoverageReturnObject {
    // Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
}
// Enable type profile.
pub struct StartTypeProfile {}
pub struct StartTypeProfileReturnObject {}
pub struct Stop {}
pub struct StopReturnObject {
    // Recorded profile.
    pub profile: Profile,
}
// Disable precise code coverage. Disabling releases unnecessary execution count records and allows
// executing optimized code.
pub struct StopPreciseCoverage {}
pub struct StopPreciseCoverageReturnObject {}
// Disable type profile. Disabling releases type profile data collected so far.
pub struct StopTypeProfile {}
pub struct StopTypeProfileReturnObject {}
// Collect coverage data for the current isolate, and resets execution counters. Precise code
// coverage needs to have started.
pub struct TakePreciseCoverage {}
pub struct TakePreciseCoverageReturnObject {
    // Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage>,
    // Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
}
// Collect type profile.
pub struct TakeTypeProfile {}
pub struct TakeTypeProfileReturnObject {
    // Type profile for all scripts since startTypeProfile() was turned on.
    pub result: Vec<ScriptTypeProfile>,
}
// Enable counters collection.
pub struct EnableCounters {}
pub struct EnableCountersReturnObject {}
// Disable counters collection.
pub struct DisableCounters {}
pub struct DisableCountersReturnObject {}
// Retrieve counters.
pub struct GetCounters {}
pub struct GetCountersReturnObject {
    // Collected counters information.
    pub result: Vec<CounterInfo>,
}
// Enable run time call stats collection.
pub struct EnableRuntimeCallStats {}
pub struct EnableRuntimeCallStatsReturnObject {}
// Disable run time call stats collection.
pub struct DisableRuntimeCallStats {}
pub struct DisableRuntimeCallStatsReturnObject {}
// Retrieve run time call stats.
pub struct GetRuntimeCallStats {}
pub struct GetRuntimeCallStatsReturnObject {
    // Collected runtime call counter information.
    pub result: Vec<RuntimeCallCounterInfo>,
}
