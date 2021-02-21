// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Profile node. Holds callsite information, execution statistics and child nodes.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionTickInfo {
    // Source line number (1-based).
    pub line: i32,
    // Number of samples attributed to the source line.
    pub ticks: i32,
}
// Coverage data for a source range.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoverageRange {
    // JavaScript script source offset for the range start.
    pub start_offset: i32,
    // JavaScript script source offset for the range end.
    pub end_offset: i32,
    // Collected execution count of the source range.
    pub count: i32,
}
// Coverage data for a JavaScript function.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCoverage {
    // JavaScript function name.
    pub function_name: String,
    // Source ranges inside the function with coverage data.
    pub ranges: Vec<CoverageRange>,
    // Whether coverage data for this function has block granularity.
    pub is_block_coverage: bool,
}
// Coverage data for a JavaScript script.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptCoverage {
    // JavaScript script id.
    pub script_id: super::runtime::ScriptId,
    // JavaScript script name or url.
    pub url: String,
    // Functions contained in the script that has coverage data.
    pub functions: Vec<FunctionCoverage>,
}
// Describes a type collected during runtime.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeObject {
    // Name of a type collected with type profiling.
    pub name: String,
}
// Source offset and types for a parameter or return value.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeProfileEntry {
    // Source offset of the parameter or end of function for return values.
    pub offset: i32,
    // The types for this parameter or return value.
    pub types: Vec<TypeObject>,
}
// Type profile data collected during runtime for a JavaScript script.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTypeProfile {
    // JavaScript script id.
    pub script_id: super::runtime::ScriptId,
    // JavaScript script name or url.
    pub url: String,
    // Type profile entries for parameters and return values of the functions in the script.
    pub entries: Vec<TypeProfileEntry>,
}
// Collected counter information.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CounterInfo {
    // Counter name.
    pub name: String,
    // Counter value.
    pub value: i32,
}
// Runtime call counter information.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeCallCounterInfo {
    // Counter name.
    pub name: String,
    // Counter value.
    pub value: f64,
    // Counter time in seconds.
    pub time: f64,
}

#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Profiler.disable";

    type ReturnObject = DisableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Profiler.enable";

    type ReturnObject = EnableReturnObject;
}
// Collect coverage data for the current isolate. The coverage data may be incomplete due to
// garbage collection.
#[derive(Serialize, Debug)]
pub struct GetBestEffortCoverage {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetBestEffortCoverageReturnObject {
    // Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage>,
}
impl super::Command for GetBestEffortCoverage {
    const NAME: &'static str = "Profiler.getBestEffortCoverage";

    type ReturnObject = GetBestEffortCoverageReturnObject;
}
// Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.
#[derive(Serialize, Debug)]
pub struct SetSamplingInterval {
    // New sampling interval in microseconds.
    pub interval: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetSamplingIntervalReturnObject {}
impl super::Command for SetSamplingInterval {
    const NAME: &'static str = "Profiler.setSamplingInterval";

    type ReturnObject = SetSamplingIntervalReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Start {}
#[derive(Deserialize, Debug, Clone)]
pub struct StartReturnObject {}
impl super::Command for Start {
    const NAME: &'static str = "Profiler.start";

    type ReturnObject = StartReturnObject;
}
// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
// coverage may be incomplete. Enabling prevents running optimized code and resets execution
// counters.
#[derive(Serialize, Debug)]
pub struct StartPreciseCoverage {
    // Collect accurate call counts beyond simple 'covered' or 'not covered'.
    pub call_count: Option<bool>,
    // Collect block-based coverage.
    pub detailed: Option<bool>,
    // Allow the backend to send updates on its own initiative
    pub allow_triggered_updates: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartPreciseCoverageReturnObject {
    // Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
}
impl super::Command for StartPreciseCoverage {
    const NAME: &'static str = "Profiler.startPreciseCoverage";

    type ReturnObject = StartPreciseCoverageReturnObject;
}
// Enable type profile.
#[derive(Serialize, Debug)]
pub struct StartTypeProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct StartTypeProfileReturnObject {}
impl super::Command for StartTypeProfile {
    const NAME: &'static str = "Profiler.startTypeProfile";

    type ReturnObject = StartTypeProfileReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Stop {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopReturnObject {
    // Recorded profile.
    pub profile: Profile,
}
impl super::Command for Stop {
    const NAME: &'static str = "Profiler.stop";

    type ReturnObject = StopReturnObject;
}
// Disable precise code coverage. Disabling releases unnecessary execution count records and allows
// executing optimized code.
#[derive(Serialize, Debug)]
pub struct StopPreciseCoverage {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopPreciseCoverageReturnObject {}
impl super::Command for StopPreciseCoverage {
    const NAME: &'static str = "Profiler.stopPreciseCoverage";

    type ReturnObject = StopPreciseCoverageReturnObject;
}
// Disable type profile. Disabling releases type profile data collected so far.
#[derive(Serialize, Debug)]
pub struct StopTypeProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopTypeProfileReturnObject {}
impl super::Command for StopTypeProfile {
    const NAME: &'static str = "Profiler.stopTypeProfile";

    type ReturnObject = StopTypeProfileReturnObject;
}
// Collect coverage data for the current isolate, and resets execution counters. Precise code
// coverage needs to have started.
#[derive(Serialize, Debug)]
pub struct TakePreciseCoverage {}
#[derive(Deserialize, Debug, Clone)]
pub struct TakePreciseCoverageReturnObject {
    // Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage>,
    // Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
}
impl super::Command for TakePreciseCoverage {
    const NAME: &'static str = "Profiler.takePreciseCoverage";

    type ReturnObject = TakePreciseCoverageReturnObject;
}
// Collect type profile.
#[derive(Serialize, Debug)]
pub struct TakeTypeProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct TakeTypeProfileReturnObject {
    // Type profile for all scripts since startTypeProfile() was turned on.
    pub result: Vec<ScriptTypeProfile>,
}
impl super::Command for TakeTypeProfile {
    const NAME: &'static str = "Profiler.takeTypeProfile";

    type ReturnObject = TakeTypeProfileReturnObject;
}
// Enable counters collection.
#[derive(Serialize, Debug)]
pub struct EnableCounters {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableCountersReturnObject {}
impl super::Command for EnableCounters {
    const NAME: &'static str = "Profiler.enableCounters";

    type ReturnObject = EnableCountersReturnObject;
}
// Disable counters collection.
#[derive(Serialize, Debug)]
pub struct DisableCounters {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableCountersReturnObject {}
impl super::Command for DisableCounters {
    const NAME: &'static str = "Profiler.disableCounters";

    type ReturnObject = DisableCountersReturnObject;
}
// Retrieve counters.
#[derive(Serialize, Debug)]
pub struct GetCounters {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetCountersReturnObject {
    // Collected counters information.
    pub result: Vec<CounterInfo>,
}
impl super::Command for GetCounters {
    const NAME: &'static str = "Profiler.getCounters";

    type ReturnObject = GetCountersReturnObject;
}
// Enable run time call stats collection.
#[derive(Serialize, Debug)]
pub struct EnableRuntimeCallStats {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableRuntimeCallStatsReturnObject {}
impl super::Command for EnableRuntimeCallStats {
    const NAME: &'static str = "Profiler.enableRuntimeCallStats";

    type ReturnObject = EnableRuntimeCallStatsReturnObject;
}
// Disable run time call stats collection.
#[derive(Serialize, Debug)]
pub struct DisableRuntimeCallStats {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableRuntimeCallStatsReturnObject {}
impl super::Command for DisableRuntimeCallStats {
    const NAME: &'static str = "Profiler.disableRuntimeCallStats";

    type ReturnObject = DisableRuntimeCallStatsReturnObject;
}
// Retrieve run time call stats.
#[derive(Serialize, Debug)]
pub struct GetRuntimeCallStats {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetRuntimeCallStatsReturnObject {
    // Collected runtime call counter information.
    pub result: Vec<RuntimeCallCounterInfo>,
}
impl super::Command for GetRuntimeCallStats {
    const NAME: &'static str = "Profiler.getRuntimeCallStats";

    type ReturnObject = GetRuntimeCallStatsReturnObject;
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConsoleProfileFinished {
    pub id: String,
    // Location of console.profileEnd().
    pub location: super::debugger::Location,
    pub profile: Profile,
    // Profile title passed as an argument to console.profile().
    pub title: Option<String>,
}
// Sent when new profile recording is started using console.profile() call.
#[derive(Deserialize, Debug, Clone)]
pub struct ConsoleProfileStarted {
    pub id: String,
    // Location of console.profile().
    pub location: super::debugger::Location,
    // Profile title passed as an argument to console.profile().
    pub title: Option<String>,
}
// Reports coverage delta since the last poll (either from an event like this, or from
// `takePreciseCoverage` for the current isolate. May only be sent if precise code
// coverage has been started. This event can be trigged by the embedder to, for example,
// trigger collection of coverage data immediatelly at a certain point in time.
#[derive(Deserialize, Debug, Clone)]
pub struct PreciseCoverageDeltaUpdate {
    // Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
    // Identifier for distinguishing coverage events.
    pub occassion: String,
    // Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage>,
}
