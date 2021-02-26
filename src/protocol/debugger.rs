// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Breakpoint identifier.
pub type BreakpointId = String;
// Call frame identifier.
pub type CallFrameId = String;
// Location in the source code.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    // Script identifier as reported in the `Debugger.scriptParsed`.
    pub script_id: super::runtime::ScriptId,
    // Line number in the script (0-based).
    pub line_number: i32,
    // Column number in the script (0-based).
    pub column_number: Option<i32>,
}
// Location in the source code.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPosition {
    pub line_number: i32,
    pub column_number: i32,
}
// Location range within one script.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocationRange {
    pub script_id: super::runtime::ScriptId,
    pub start: ScriptPosition,
    pub end: ScriptPosition,
}
// JavaScript call frame. Array of call frames form the call stack.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    // Call frame identifier. This identifier is only valid while the virtual machine is paused.
    pub call_frame_id: CallFrameId,
    // Name of the JavaScript function called on this call frame.
    pub function_name: String,
    // Location in the source code.
    pub function_location: Option<Location>,
    // Location in the source code.
    pub location: Location,
    // JavaScript script name or url.
    pub url: String,
    // Scope chain for this call frame.
    pub scope_chain: Vec<Scope>,
    // `this` object for this call frame.
    pub this: super::runtime::RemoteObject,
    // The value being returned, if the function is at return point.
    pub return_value: Option<super::runtime::RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScopeType {
    Global,
    Local,
    With,
    Closure,
    Catch,
    Block,
    Script,
    Eval,
    Module,
    WasmExpressionStack,
}
// Scope description.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    // Scope type.
    pub r#type: String,
    // Object representing the scope. For `global` and `with` scopes it represents the actual
    // object; for the rest of the scopes, it is artificial transient object enumerating scope
    // variables as its properties.
    pub object: super::runtime::RemoteObject,
    pub name: Option<String>,
    // Location in the source code where scope starts
    pub start_location: Option<Location>,
    // Location in the source code where scope ends
    pub end_location: Option<Location>,
}
// Search match for resource.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    // Line number in resource content.
    pub line_number: f64,
    // Line with match content.
    pub line_content: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BreakLocationType {
    DebuggerStatement,
    Call,
    Return,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BreakLocation {
    // Script identifier as reported in the `Debugger.scriptParsed`.
    pub script_id: super::runtime::ScriptId,
    // Line number in the script (0-based).
    pub line_number: i32,
    // Column number in the script (0-based).
    pub column_number: Option<i32>,
    pub r#type: Option<String>,
}
// Enum of possible script languages.
pub type ScriptLanguage = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DebugSymbolsType {
    None,
    SourceMap,
    EmbeddedDWARF,
    ExternalDWARF,
}
// Debug symbols available for a wasm script.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DebugSymbols {
    // Type of the debug symbols.
    pub r#type: String,
    // URL of the external symbol source.
    pub external_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetCallFrames {
    Any,
    Current,
}
// Continues execution until specific location is reached.
#[derive(Serialize, Debug)]
pub struct ContinueToLocation {
    // Location to continue to.
    pub location: Location,
    pub target_call_frames: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocationReturnObject {}
impl super::Command for ContinueToLocation {
    const NAME: &'static str = "Debugger.continueToLocation";

    type ReturnObject = ContinueToLocationReturnObject;
}
// Disables debugger for given page.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Debugger.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables debugger for the given page. Clients should not assume that the debugging has been
// enabled until the result for this command is received.
#[derive(Serialize, Debug)]
pub struct Enable {
    // The maximum size in bytes of collected scripts (not referenced by other heap objects)
    // the debugger can hold. Puts no limit if paramter is omitted.
    pub max_scripts_cache_size: Option<f64>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {
    // Unique identifier of the debugger.
    pub debugger_id: super::runtime::UniqueDebuggerId,
}
impl super::Command for Enable {
    const NAME: &'static str = "Debugger.enable";

    type ReturnObject = EnableReturnObject;
}
// Evaluates expression on a given call frame.
#[derive(Serialize, Debug)]
pub struct EvaluateOnCallFrame {
    // Call frame identifier to evaluate on.
    pub call_frame_id: CallFrameId,
    // Expression to evaluate.
    pub expression: String,
    // String object group name to put result into (allows rapid releasing resulting object handles
    // using `releaseObjectGroup`).
    pub object_group: Option<String>,
    // Specifies whether command line API should be available to the evaluated expression, defaults
    // to false.
    pub include_command_line_api: Option<bool>,
    // In silent mode exceptions thrown during evaluation are not reported and do not pause
    // execution. Overrides `setPauseOnException` state.
    pub silent: Option<bool>,
    // Whether the result is expected to be a JSON object that should be sent by value.
    pub return_by_value: Option<bool>,
    // Whether preview should be generated for the result.
    pub generate_preview: Option<bool>,
    // Whether to throw an exception if side effect cannot be ruled out during evaluation.
    pub throw_on_side_effect: Option<bool>,
    // Terminate execution after timing out (number of milliseconds).
    pub timeout: Option<super::runtime::TimeDelta>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameReturnObject {
    // Object wrapper for the evaluation result.
    pub result: super::runtime::RemoteObject,
    // Exception details.
    pub exception_details: Option<super::runtime::ExceptionDetails>,
}
impl super::Command for EvaluateOnCallFrame {
    const NAME: &'static str = "Debugger.evaluateOnCallFrame";

    type ReturnObject = EvaluateOnCallFrameReturnObject;
}
// Execute a Wasm Evaluator module on a given call frame.
#[derive(Serialize, Debug)]
pub struct ExecuteWasmEvaluator {
    // WebAssembly call frame identifier to evaluate on.
    pub call_frame_id: CallFrameId,
    // Code of the evaluator module.
    pub evaluator: String,
    // Terminate execution after timing out (number of milliseconds).
    pub timeout: Option<super::runtime::TimeDelta>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteWasmEvaluatorReturnObject {
    // Object wrapper for the evaluation result.
    pub result: super::runtime::RemoteObject,
    // Exception details.
    pub exception_details: Option<super::runtime::ExceptionDetails>,
}
impl super::Command for ExecuteWasmEvaluator {
    const NAME: &'static str = "Debugger.executeWasmEvaluator";

    type ReturnObject = ExecuteWasmEvaluatorReturnObject;
}
// Returns possible locations for breakpoint. scriptId in start and end range locations should be
// the same.
#[derive(Serialize, Debug)]
pub struct GetPossibleBreakpoints {
    // Start of range to search possible breakpoint locations in.
    pub start: Location,
    // End of range to search possible breakpoint locations in (excluding). When not specified, end
    // of scripts is used as end of range.
    pub end: Option<Location>,
    // Only consider locations which are in the same (non-nested) function as start.
    pub restrict_to_function: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsReturnObject {
    // List of the possible breakpoint locations.
    pub locations: Vec<BreakLocation>,
}
impl super::Command for GetPossibleBreakpoints {
    const NAME: &'static str = "Debugger.getPossibleBreakpoints";

    type ReturnObject = GetPossibleBreakpointsReturnObject;
}
// Returns source for the script with given id.
#[derive(Serialize, Debug)]
pub struct GetScriptSource {
    // Id of the script to get source for.
    pub script_id: super::runtime::ScriptId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptSourceReturnObject {
    // Script source (empty in case of Wasm bytecode).
    pub script_source: String,
    // Wasm bytecode.
    pub bytecode: Option<String>,
}
impl super::Command for GetScriptSource {
    const NAME: &'static str = "Debugger.getScriptSource";

    type ReturnObject = GetScriptSourceReturnObject;
}
// This command is deprecated. Use getScriptSource instead.
#[derive(Serialize, Debug)]
pub struct GetWasmBytecode {
    // Id of the Wasm script to get source for.
    pub script_id: super::runtime::ScriptId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetWasmBytecodeReturnObject {
    // Script source.
    pub bytecode: String,
}
impl super::Command for GetWasmBytecode {
    const NAME: &'static str = "Debugger.getWasmBytecode";

    type ReturnObject = GetWasmBytecodeReturnObject;
}
// Returns stack trace with given `stackTraceId`.
#[derive(Serialize, Debug)]
pub struct GetStackTrace {
    pub stack_trace_id: super::runtime::StackTraceId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetStackTraceReturnObject {
    pub stack_trace: super::runtime::StackTrace,
}
impl super::Command for GetStackTrace {
    const NAME: &'static str = "Debugger.getStackTrace";

    type ReturnObject = GetStackTraceReturnObject;
}
// Stops on the next JavaScript statement.
#[derive(Serialize, Debug)]
pub struct Pause {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PauseReturnObject {}
impl super::Command for Pause {
    const NAME: &'static str = "Debugger.pause";

    type ReturnObject = PauseReturnObject;
}
#[derive(Serialize, Debug)]
pub struct PauseOnAsyncCall {
    // Debugger will pause when async call with given stack trace is started.
    pub parent_stack_trace_id: super::runtime::StackTraceId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PauseOnAsyncCallReturnObject {}
impl super::Command for PauseOnAsyncCall {
    const NAME: &'static str = "Debugger.pauseOnAsyncCall";

    type ReturnObject = PauseOnAsyncCallReturnObject;
}
// Removes JavaScript breakpoint.
#[derive(Serialize, Debug)]
pub struct RemoveBreakpoint {
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBreakpointReturnObject {}
impl super::Command for RemoveBreakpoint {
    const NAME: &'static str = "Debugger.removeBreakpoint";

    type ReturnObject = RemoveBreakpointReturnObject;
}
// Restarts particular call frame from the beginning.
#[derive(Serialize, Debug)]
pub struct RestartFrame {
    // Call frame identifier to evaluate on.
    pub call_frame_id: CallFrameId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameReturnObject {
    // New stack trace.
    pub call_frames: Vec<CallFrame>,
    // Async stack trace, if any.
    pub async_stack_trace: Option<super::runtime::StackTrace>,
    // Async stack trace, if any.
    pub async_stack_trace_id: Option<super::runtime::StackTraceId>,
}
impl super::Command for RestartFrame {
    const NAME: &'static str = "Debugger.restartFrame";

    type ReturnObject = RestartFrameReturnObject;
}
// Resumes JavaScript execution.
#[derive(Serialize, Debug)]
pub struct Resume {
    // Set to true to terminate execution upon resuming execution. In contrast
    // to Runtime.terminateExecution, this will allows to execute further
    // JavaScript (i.e. via evaluation) until execution of the paused code
    // is actually resumed, at which point termination is triggered.
    // If execution is currently not paused, this parameter has no effect.
    pub terminate_on_resume: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResumeReturnObject {}
impl super::Command for Resume {
    const NAME: &'static str = "Debugger.resume";

    type ReturnObject = ResumeReturnObject;
}
// Searches for given string in script content.
#[derive(Serialize, Debug)]
pub struct SearchInContent {
    // Id of the script to search in.
    pub script_id: super::runtime::ScriptId,
    // String to search for.
    pub query: String,
    // If true, search is case sensitive.
    pub case_sensitive: Option<bool>,
    // If true, treats string parameter as regex.
    pub is_regex: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchInContentReturnObject {
    // List of search matches.
    pub result: Vec<SearchMatch>,
}
impl super::Command for SearchInContent {
    const NAME: &'static str = "Debugger.searchInContent";

    type ReturnObject = SearchInContentReturnObject;
}
// Enables or disables async call stacks tracking.
#[derive(Serialize, Debug)]
pub struct SetAsyncCallStackDepth {
    // Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
    // call stacks (default).
    pub max_depth: i32,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAsyncCallStackDepthReturnObject {}
impl super::Command for SetAsyncCallStackDepth {
    const NAME: &'static str = "Debugger.setAsyncCallStackDepth";

    type ReturnObject = SetAsyncCallStackDepthReturnObject;
}
// Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
// scripts with url matching one of the patterns. VM will try to leave blackboxed script by
// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
#[derive(Serialize, Debug)]
pub struct SetBlackboxPatterns {
    // Array of regexps that will be used to check script url for blackbox state.
    pub patterns: Vec<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxPatternsReturnObject {}
impl super::Command for SetBlackboxPatterns {
    const NAME: &'static str = "Debugger.setBlackboxPatterns";

    type ReturnObject = SetBlackboxPatternsReturnObject;
}
// Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
// scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
// Positions array contains positions where blackbox state is changed. First interval isn't
// blackboxed. Array should be sorted.
#[derive(Serialize, Debug)]
pub struct SetBlackboxedRanges {
    // Id of the script.
    pub script_id: super::runtime::ScriptId,
    pub positions: Vec<ScriptPosition>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxedRangesReturnObject {}
impl super::Command for SetBlackboxedRanges {
    const NAME: &'static str = "Debugger.setBlackboxedRanges";

    type ReturnObject = SetBlackboxedRangesReturnObject;
}
// Sets JavaScript breakpoint at a given location.
#[derive(Serialize, Debug)]
pub struct SetBreakpoint {
    // Location to set breakpoint in.
    pub location: Location,
    // Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    // breakpoint if this expression evaluates to true.
    pub condition: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
    // Location this breakpoint resolved into.
    pub actual_location: Location,
}
impl super::Command for SetBreakpoint {
    const NAME: &'static str = "Debugger.setBreakpoint";

    type ReturnObject = SetBreakpointReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Instrumentation {
    BeforeScriptExecution,
    BeforeScriptWithSourceMapExecution,
}
// Sets instrumentation breakpoint.
#[derive(Serialize, Debug)]
pub struct SetInstrumentationBreakpoint {
    // Instrumentation name.
    pub instrumentation: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
}
impl super::Command for SetInstrumentationBreakpoint {
    const NAME: &'static str = "Debugger.setInstrumentationBreakpoint";

    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
// `locations` property. Further matching script parsing will result in subsequent
// `breakpointResolved` events issued. This logical breakpoint will survive page reloads.
#[derive(Serialize, Debug)]
pub struct SetBreakpointByUrl {
    // Line number to set breakpoint at.
    pub line_number: i32,
    // URL of the resources to set breakpoint on.
    pub url: Option<String>,
    // Regex pattern for the URLs of the resources to set breakpoints on. Either `url` or
    // `urlRegex` must be specified.
    pub url_regex: Option<String>,
    // Script hash of the resources to set breakpoint on.
    pub script_hash: Option<String>,
    // Offset in the line to set breakpoint at.
    pub column_number: Option<i32>,
    // Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    // breakpoint if this expression evaluates to true.
    pub condition: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointByUrlReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
    // List of the locations this breakpoint resolved into upon addition.
    pub locations: Vec<Location>,
}
impl super::Command for SetBreakpointByUrl {
    const NAME: &'static str = "Debugger.setBreakpointByUrl";

    type ReturnObject = SetBreakpointByUrlReturnObject;
}
// Sets JavaScript breakpoint before each call to the given function.
// If another function was created from the same source as a given one,
// calling it will also trigger the breakpoint.
#[derive(Serialize, Debug)]
pub struct SetBreakpointOnFunctionCall {
    // Function object id.
    pub object_id: super::runtime::RemoteObjectId,
    // Expression to use as a breakpoint condition. When specified, debugger will
    // stop on the breakpoint if this expression evaluates to true.
    pub condition: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointOnFunctionCallReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
}
impl super::Command for SetBreakpointOnFunctionCall {
    const NAME: &'static str = "Debugger.setBreakpointOnFunctionCall";

    type ReturnObject = SetBreakpointOnFunctionCallReturnObject;
}
// Activates / deactivates all breakpoints on the page.
#[derive(Serialize, Debug)]
pub struct SetBreakpointsActive {
    // New value for breakpoints active state.
    pub active: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsActiveReturnObject {}
impl super::Command for SetBreakpointsActive {
    const NAME: &'static str = "Debugger.setBreakpointsActive";

    type ReturnObject = SetBreakpointsActiveReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum State {
    None,
    Uncaught,
    All,
}
// Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions or
// no exceptions. Initial pause on exceptions state is `none`.
#[derive(Serialize, Debug)]
pub struct SetPauseOnExceptions {
    // Pause on exceptions mode.
    pub state: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetPauseOnExceptionsReturnObject {}
impl super::Command for SetPauseOnExceptions {
    const NAME: &'static str = "Debugger.setPauseOnExceptions";

    type ReturnObject = SetPauseOnExceptionsReturnObject;
}
// Changes return value in top frame. Available only at return break position.
#[derive(Serialize, Debug)]
pub struct SetReturnValue {
    // New return value.
    pub new_value: super::runtime::CallArgument,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReturnValueReturnObject {}
impl super::Command for SetReturnValue {
    const NAME: &'static str = "Debugger.setReturnValue";

    type ReturnObject = SetReturnValueReturnObject;
}
// Edits JavaScript source live.
#[derive(Serialize, Debug)]
pub struct SetScriptSource {
    // Id of the script to edit.
    pub script_id: super::runtime::ScriptId,
    // New content of the script.
    pub script_source: String,
    // If true the change will not actually be applied. Dry run may be used to get result
    // description without actually modifying the code.
    pub dry_run: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptSourceReturnObject {
    // New stack trace in case editing has happened while VM was stopped.
    pub call_frames: Option<Vec<CallFrame>>,
    // Whether current call stack  was modified after applying the changes.
    pub stack_changed: Option<bool>,
    // Async stack trace, if any.
    pub async_stack_trace: Option<super::runtime::StackTrace>,
    // Async stack trace, if any.
    pub async_stack_trace_id: Option<super::runtime::StackTraceId>,
    // Exception details if any.
    pub exception_details: Option<super::runtime::ExceptionDetails>,
}
impl super::Command for SetScriptSource {
    const NAME: &'static str = "Debugger.setScriptSource";

    type ReturnObject = SetScriptSourceReturnObject;
}
// Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).
#[derive(Serialize, Debug)]
pub struct SetSkipAllPauses {
    // New value for skip pauses state.
    pub skip: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetSkipAllPausesReturnObject {}
impl super::Command for SetSkipAllPauses {
    const NAME: &'static str = "Debugger.setSkipAllPauses";

    type ReturnObject = SetSkipAllPausesReturnObject;
}
// Changes value of variable in a callframe. Object-based scopes are not supported and must be
// mutated manually.
#[derive(Serialize, Debug)]
pub struct SetVariableValue {
    // 0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
    // scope types are allowed. Other scopes could be manipulated manually.
    pub scope_number: i32,
    // Variable name.
    pub variable_name: String,
    // New variable value.
    pub new_value: super::runtime::CallArgument,
    // Id of callframe that holds variable.
    pub call_frame_id: CallFrameId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableValueReturnObject {}
impl super::Command for SetVariableValue {
    const NAME: &'static str = "Debugger.setVariableValue";

    type ReturnObject = SetVariableValueReturnObject;
}
// Steps into the function call.
#[derive(Serialize, Debug)]
pub struct StepInto {
    // Debugger will pause on the execution of the first async task which was scheduled
    // before next pause.
    pub break_on_async_call: Option<bool>,
    // The skipList specifies location ranges that should be skipped on step into.
    pub skip_list: Option<Vec<LocationRange>>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepIntoReturnObject {}
impl super::Command for StepInto {
    const NAME: &'static str = "Debugger.stepInto";

    type ReturnObject = StepIntoReturnObject;
}
// Steps out of the function call.
#[derive(Serialize, Debug)]
pub struct StepOut {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepOutReturnObject {}
impl super::Command for StepOut {
    const NAME: &'static str = "Debugger.stepOut";

    type ReturnObject = StepOutReturnObject;
}
// Steps over the statement.
#[derive(Serialize, Debug)]
pub struct StepOver {
    // The skipList specifies location ranges that should be skipped on step over.
    pub skip_list: Option<Vec<LocationRange>>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepOverReturnObject {}
impl super::Command for StepOver {
    const NAME: &'static str = "Debugger.stepOver";

    type ReturnObject = StepOverReturnObject;
}

// Fired when breakpoint is resolved to an actual script and location.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BreakpointResolvedEvent {
    pub params: BreakpointResolvedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointResolvedParams {
    // Breakpoint unique identifier.
    pub breakpoint_id: BreakpointId,
    // Actual breakpoint location.
    pub location: Location,
}
// Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PausedEvent {
    pub params: PausedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PausedParams {
    // Call stack the virtual machine stopped on.
    pub call_frames: Vec<CallFrame>,
    // Pause reason.
    pub reason: String,
    // Object containing break-specific auxiliary properties.
    // TODO objectProperty
    // Hit breakpoints IDs
    pub hit_breakpoints: Option<Vec<String>>,
    // Async stack trace, if any.
    pub async_stack_trace: Option<super::runtime::StackTrace>,
    // Async stack trace, if any.
    pub async_stack_trace_id: Option<super::runtime::StackTraceId>,
    // Never present, will be removed.
    pub async_call_stack_trace_id: Option<super::runtime::StackTraceId>,
}
// Fired when the virtual machine resumed execution.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ResumedEvent {
    pub params: ResumedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResumedParams {}
// Fired when virtual machine fails to parse the script.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptFailedToParseEvent {
    pub params: ScriptFailedToParseParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptFailedToParseParams {
    // Identifier of the script parsed.
    pub script_id: super::runtime::ScriptId,
    // URL or name of the script parsed (if any).
    pub url: String,
    // Line offset of the script within the resource with given URL (for script tags).
    pub start_line: i32,
    // Column offset of the script within the resource with given URL.
    pub start_column: i32,
    // Last line of the script.
    pub end_line: i32,
    // Length of the last line of the script.
    pub end_column: i32,
    // Specifies script creation context.
    pub execution_context_id: super::runtime::ExecutionContextId,
    // Content hash of the script.
    pub hash: String,
    // Embedder-specific auxiliary data.
    // TODO objectProperty
    // URL of source map associated with script (if any).
    pub source_map_url: Option<String>,
    // True, if this script has sourceURL.
    pub has_source_url: Option<bool>,
    // True, if this script is ES6 module.
    pub is_module: Option<bool>,
    // This script length.
    pub length: Option<i32>,
    // JavaScript top stack frame of where the script parsed event was triggered if available.
    pub stack_trace: Option<super::runtime::StackTrace>,
    // If the scriptLanguage is WebAssembly, the code section offset in the module.
    pub code_offset: Option<i32>,
    // The language of the script.
    pub script_language: Option<super::debugger::ScriptLanguage>,
    // The name the embedder supplied for this script.
    pub embedder_name: Option<String>,
}
// Fired when virtual machine parses script. This event is also fired for all known and uncollected
// scripts upon enabling debugger.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptParsedEvent {
    pub params: ScriptParsedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScriptParsedParams {
    // Identifier of the script parsed.
    pub script_id: super::runtime::ScriptId,
    // URL or name of the script parsed (if any).
    pub url: String,
    // Line offset of the script within the resource with given URL (for script tags).
    pub start_line: i32,
    // Column offset of the script within the resource with given URL.
    pub start_column: i32,
    // Last line of the script.
    pub end_line: i32,
    // Length of the last line of the script.
    pub end_column: i32,
    // Specifies script creation context.
    pub execution_context_id: super::runtime::ExecutionContextId,
    // Content hash of the script.
    pub hash: String,
    // Embedder-specific auxiliary data.
    // TODO objectProperty
    // True, if this script is generated as a result of the live edit operation.
    pub is_live_edit: Option<bool>,
    // URL of source map associated with script (if any).
    pub source_map_url: Option<String>,
    // True, if this script has sourceURL.
    pub has_source_url: Option<bool>,
    // True, if this script is ES6 module.
    pub is_module: Option<bool>,
    // This script length.
    pub length: Option<i32>,
    // JavaScript top stack frame of where the script parsed event was triggered if available.
    pub stack_trace: Option<super::runtime::StackTrace>,
    // If the scriptLanguage is WebAssembly, the code section offset in the module.
    pub code_offset: Option<i32>,
    // The language of the script.
    pub script_language: Option<super::debugger::ScriptLanguage>,
    // If the scriptLanguage is WebASsembly, the source of debug symbols for the module.
    pub debug_symbols: Option<super::debugger::DebugSymbols>,
    // The name the embedder supplied for this script.
    pub embedder_name: Option<String>,
}
