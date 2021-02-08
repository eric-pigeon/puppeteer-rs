// This file is auto-generated do not edit manually.

// Breakpoint identifier.
pub type BreakpointId = String;
// Call frame identifier.
pub type CallFrameId = String;
// Location in the source code.
pub struct Location {
    // Script identifier as reported in the `Debugger.scriptParsed`.
    pub script_id: super::runtime::ScriptId,
    // Line number in the script (0-based).
    pub line_number: i32,
    // Column number in the script (0-based).
    pub column_number: Option<i32>,
}
// Location in the source code.
pub struct ScriptPosition {
    pub line_number: i32,
    pub column_number: i32,
}
// Location range within one script.
pub struct LocationRange {
    pub script_id: super::runtime::ScriptId,
    pub start: ScriptPosition,
    pub end: ScriptPosition,
}
// JavaScript call frame. Array of call frames form the call stack.
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
pub struct Scope {
    // Scope type.
    pub r#type: ScopeType,
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
pub struct SearchMatch {
    // Line number in resource content.
    pub line_number: f64,
    // Line with match content.
    pub line_content: String,
}
pub enum BreakLocationType {
    DebuggerStatement,
    Call,
    Return,
}
pub struct BreakLocation {
    // Script identifier as reported in the `Debugger.scriptParsed`.
    pub script_id: super::runtime::ScriptId,
    // Line number in the script (0-based).
    pub line_number: i32,
    // Column number in the script (0-based).
    pub column_number: Option<i32>,
    pub r#type: Option<BreakLocationType>,
}
// Enum of possible script languages.
pub enum ScriptLanguage {
    JavaScript,
    WebAssembly,
}
pub enum DebugSymbolsType {
    None,
    SourceMap,
    EmbeddedDWARF,
    ExternalDWARF,
}
// Debug symbols available for a wasm script.
pub struct DebugSymbols {
    // Type of the debug symbols.
    pub r#type: DebugSymbolsType,
    // URL of the external symbol source.
    pub external_url: Option<String>,
}

pub enum TargetCallFrames {
    Any,
    Current,
}
// Continues execution until specific location is reached.
pub struct ContinueToLocation {
    // Location to continue to.
    pub location: Location,
    pub target_call_frames: Option<TargetCallFrames>,
}
pub struct ContinueToLocationReturnObject {}
// Disables debugger for given page.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables debugger for the given page. Clients should not assume that the debugging has been
// enabled until the result for this command is received.
pub struct Enable {
    // The maximum size in bytes of collected scripts (not referenced by other heap objects)
    // the debugger can hold. Puts no limit if paramter is omitted.
    pub max_scripts_cache_size: Option<f64>,
}
pub struct EnableReturnObject {
    // Unique identifier of the debugger.
    pub debugger_id: super::runtime::UniqueDebuggerId,
}
// Evaluates expression on a given call frame.
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
pub struct EvaluateOnCallFrameReturnObject {
    // Object wrapper for the evaluation result.
    pub result: super::runtime::RemoteObject,
    // Exception details.
    pub exception_details: Option<super::runtime::ExceptionDetails>,
}
// Execute a Wasm Evaluator module on a given call frame.
pub struct ExecuteWasmEvaluator {
    // WebAssembly call frame identifier to evaluate on.
    pub call_frame_id: CallFrameId,
    // Code of the evaluator module.
    pub evaluator: String,
    // Terminate execution after timing out (number of milliseconds).
    pub timeout: Option<super::runtime::TimeDelta>,
}
pub struct ExecuteWasmEvaluatorReturnObject {
    // Object wrapper for the evaluation result.
    pub result: super::runtime::RemoteObject,
    // Exception details.
    pub exception_details: Option<super::runtime::ExceptionDetails>,
}
// Returns possible locations for breakpoint. scriptId in start and end range locations should be
// the same.
pub struct GetPossibleBreakpoints {
    // Start of range to search possible breakpoint locations in.
    pub start: Location,
    // End of range to search possible breakpoint locations in (excluding). When not specified, end
    // of scripts is used as end of range.
    pub end: Option<Location>,
    // Only consider locations which are in the same (non-nested) function as start.
    pub restrict_to_function: Option<bool>,
}
pub struct GetPossibleBreakpointsReturnObject {
    // List of the possible breakpoint locations.
    pub locations: Vec<BreakLocation>,
}
// Returns source for the script with given id.
pub struct GetScriptSource {
    // Id of the script to get source for.
    pub script_id: super::runtime::ScriptId,
}
pub struct GetScriptSourceReturnObject {
    // Script source (empty in case of Wasm bytecode).
    pub script_source: String,
    // Wasm bytecode.
    pub bytecode: Option<String>,
}
// This command is deprecated. Use getScriptSource instead.
pub struct GetWasmBytecode {
    // Id of the Wasm script to get source for.
    pub script_id: super::runtime::ScriptId,
}
pub struct GetWasmBytecodeReturnObject {
    // Script source.
    pub bytecode: String,
}
// Returns stack trace with given `stackTraceId`.
pub struct GetStackTrace {
    pub stack_trace_id: super::runtime::StackTraceId,
}
pub struct GetStackTraceReturnObject {
    pub stack_trace: super::runtime::StackTrace,
}
// Stops on the next JavaScript statement.
pub struct Pause {}
pub struct PauseReturnObject {}
pub struct PauseOnAsyncCall {
    // Debugger will pause when async call with given stack trace is started.
    pub parent_stack_trace_id: super::runtime::StackTraceId,
}
pub struct PauseOnAsyncCallReturnObject {}
// Removes JavaScript breakpoint.
pub struct RemoveBreakpoint {
    pub breakpoint_id: BreakpointId,
}
pub struct RemoveBreakpointReturnObject {}
// Restarts particular call frame from the beginning.
pub struct RestartFrame {
    // Call frame identifier to evaluate on.
    pub call_frame_id: CallFrameId,
}
pub struct RestartFrameReturnObject {
    // New stack trace.
    pub call_frames: Vec<CallFrame>,
    // Async stack trace, if any.
    pub async_stack_trace: Option<super::runtime::StackTrace>,
    // Async stack trace, if any.
    pub async_stack_trace_id: Option<super::runtime::StackTraceId>,
}
// Resumes JavaScript execution.
pub struct Resume {
    // Set to true to terminate execution upon resuming execution. In contrast
    // to Runtime.terminateExecution, this will allows to execute further
    // JavaScript (i.e. via evaluation) until execution of the paused code
    // is actually resumed, at which point termination is triggered.
    // If execution is currently not paused, this parameter has no effect.
    pub terminate_on_resume: Option<bool>,
}
pub struct ResumeReturnObject {}
// Searches for given string in script content.
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
pub struct SearchInContentReturnObject {
    // List of search matches.
    pub result: Vec<SearchMatch>,
}
// Enables or disables async call stacks tracking.
pub struct SetAsyncCallStackDepth {
    // Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
    // call stacks (default).
    pub max_depth: i32,
}
pub struct SetAsyncCallStackDepthReturnObject {}
// Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
// scripts with url matching one of the patterns. VM will try to leave blackboxed script by
// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
pub struct SetBlackboxPatterns {
    // Array of regexps that will be used to check script url for blackbox state.
    pub patterns: Vec<String>,
}
pub struct SetBlackboxPatternsReturnObject {}
// Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
// scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
// Positions array contains positions where blackbox state is changed. First interval isn't
// blackboxed. Array should be sorted.
pub struct SetBlackboxedRanges {
    // Id of the script.
    pub script_id: super::runtime::ScriptId,
    pub positions: Vec<ScriptPosition>,
}
pub struct SetBlackboxedRangesReturnObject {}
// Sets JavaScript breakpoint at a given location.
pub struct SetBreakpoint {
    // Location to set breakpoint in.
    pub location: Location,
    // Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    // breakpoint if this expression evaluates to true.
    pub condition: Option<String>,
}
pub struct SetBreakpointReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
    // Location this breakpoint resolved into.
    pub actual_location: Location,
}
pub enum Instrumentation {
    BeforeScriptExecution,
    BeforeScriptWithSourceMapExecution,
}
// Sets instrumentation breakpoint.
pub struct SetInstrumentationBreakpoint {
    // Instrumentation name.
    pub instrumentation: Instrumentation,
}
pub struct SetInstrumentationBreakpointReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
}
// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
// `locations` property. Further matching script parsing will result in subsequent
// `breakpointResolved` events issued. This logical breakpoint will survive page reloads.
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
pub struct SetBreakpointByUrlReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
    // List of the locations this breakpoint resolved into upon addition.
    pub locations: Vec<Location>,
}
// Sets JavaScript breakpoint before each call to the given function.
// If another function was created from the same source as a given one,
// calling it will also trigger the breakpoint.
pub struct SetBreakpointOnFunctionCall {
    // Function object id.
    pub object_id: super::runtime::RemoteObjectId,
    // Expression to use as a breakpoint condition. When specified, debugger will
    // stop on the breakpoint if this expression evaluates to true.
    pub condition: Option<String>,
}
pub struct SetBreakpointOnFunctionCallReturnObject {
    // Id of the created breakpoint for further reference.
    pub breakpoint_id: BreakpointId,
}
// Activates / deactivates all breakpoints on the page.
pub struct SetBreakpointsActive {
    // New value for breakpoints active state.
    pub active: bool,
}
pub struct SetBreakpointsActiveReturnObject {}
pub enum State {
    None,
    Uncaught,
    All,
}
// Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions or
// no exceptions. Initial pause on exceptions state is `none`.
pub struct SetPauseOnExceptions {
    // Pause on exceptions mode.
    pub state: State,
}
pub struct SetPauseOnExceptionsReturnObject {}
// Changes return value in top frame. Available only at return break position.
pub struct SetReturnValue {
    // New return value.
    pub new_value: super::runtime::CallArgument,
}
pub struct SetReturnValueReturnObject {}
// Edits JavaScript source live.
pub struct SetScriptSource {
    // Id of the script to edit.
    pub script_id: super::runtime::ScriptId,
    // New content of the script.
    pub script_source: String,
    // If true the change will not actually be applied. Dry run may be used to get result
    // description without actually modifying the code.
    pub dry_run: Option<bool>,
}
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
// Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).
pub struct SetSkipAllPauses {
    // New value for skip pauses state.
    pub skip: bool,
}
pub struct SetSkipAllPausesReturnObject {}
// Changes value of variable in a callframe. Object-based scopes are not supported and must be
// mutated manually.
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
pub struct SetVariableValueReturnObject {}
// Steps into the function call.
pub struct StepInto {
    // Debugger will pause on the execution of the first async task which was scheduled
    // before next pause.
    pub break_on_async_call: Option<bool>,
    // The skipList specifies location ranges that should be skipped on step into.
    pub skip_list: Option<Vec<LocationRange>>,
}
pub struct StepIntoReturnObject {}
// Steps out of the function call.
pub struct StepOut {}
pub struct StepOutReturnObject {}
// Steps over the statement.
pub struct StepOver {
    // The skipList specifies location ranges that should be skipped on step over.
    pub skip_list: Option<Vec<LocationRange>>,
}
pub struct StepOverReturnObject {}
