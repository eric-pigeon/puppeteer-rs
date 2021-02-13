// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Unique script identifier.
pub type ScriptId = String;
// Unique object identifier.
pub type RemoteObjectId = String;
// Primitive value which cannot be JSON-stringified. Includes values `-0`, `NaN`, `Infinity`,
// `-Infinity`, and bigint literals.
pub type UnserializableValue = String;
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RemoteObjectType {
    Object,
    Function,
    Undefined,
    String,
    Number,
    Boolean,
    Symbol,
    Bigint,
    Wasm,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RemoteObjectSubtype {
    Array,
    Null,
    Node,
    Regexp,
    Date,
    Map,
    Set,
    Weakmap,
    Weakset,
    Iterator,
    Generator,
    Error,
    Proxy,
    Promise,
    Typedarray,
    Arraybuffer,
    Dataview,
    I32,
    I64,
    F32,
    F64,
    V128,
    Externref,
}
// Mirror object referencing original JavaScript object.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    // Object type.
    pub r#type: RemoteObjectType,
    // Object subtype hint. Specified for `object` or `wasm` type values only.
    pub subtype: Option<RemoteObjectSubtype>,
    // Object class (constructor) name. Specified for `object` type values only.
    pub class_name: Option<String>,
    // Remote object value in case of primitive values or JSON values (if it was requested).
    pub value: Option<serde_json::Value>,
    // Primitive value which can not be JSON-stringified does not have `value`, but gets this
    // property.
    pub unserializable_value: Option<UnserializableValue>,
    // String representation of the object.
    pub description: Option<String>,
    // Unique object identifier (for non-primitive values).
    pub object_id: Option<RemoteObjectId>,
    // Preview containing abbreviated property values. Specified for `object` type values only.
    pub preview: Option<ObjectPreview>,
    pub custom_preview: Option<CustomPreview>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomPreview {
    // The JSON-stringified result of formatter.header(object, config) call.
    // It contains json ML array that represents RemoteObject.
    pub header: String,
    // If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    // contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    // The result value is json ML array.
    pub body_getter_id: Option<RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ObjectPreviewType {
    Object,
    Function,
    Undefined,
    String,
    Number,
    Boolean,
    Symbol,
    Bigint,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ObjectPreviewSubtype {
    Array,
    Null,
    Node,
    Regexp,
    Date,
    Map,
    Set,
    Weakmap,
    Weakset,
    Iterator,
    Generator,
    Error,
}
// Object containing abbreviated remote object value.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    // Object type.
    pub r#type: ObjectPreviewType,
    // Object subtype hint. Specified for `object` type values only.
    pub subtype: Option<ObjectPreviewSubtype>,
    // String representation of the object.
    pub description: Option<String>,
    // True iff some of the properties or entries of the original object did not fit.
    pub overflow: bool,
    // List of the properties.
    pub properties: Vec<PropertyPreview>,
    // List of the entries. Specified for `map` and `set` subtype values only.
    pub entries: Option<Vec<EntryPreview>>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PropertyPreviewType {
    Object,
    Function,
    Undefined,
    String,
    Number,
    Boolean,
    Symbol,
    Accessor,
    Bigint,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PropertyPreviewSubtype {
    Array,
    Null,
    Node,
    Regexp,
    Date,
    Map,
    Set,
    Weakmap,
    Weakset,
    Iterator,
    Generator,
    Error,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview {
    // Property name.
    pub name: String,
    // Object type. Accessor means that the property itself is an accessor property.
    pub r#type: PropertyPreviewType,
    // User-friendly property value string.
    pub value: Option<String>,
    // Nested value preview.
    pub value_preview: Option<ObjectPreview>,
    // Object subtype hint. Specified for `object` type values only.
    pub subtype: Option<PropertyPreviewSubtype>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EntryPreview {
    // Preview of the key. Specified for map-like collection entries.
    pub key: Option<ObjectPreview>,
    // Preview of the value.
    pub value: ObjectPreview,
}
// Object property descriptor.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor {
    // Property name or symbol description.
    pub name: String,
    // The value associated with the property.
    pub value: Option<RemoteObject>,
    // True if the value associated with the property may be changed (data descriptors only).
    pub writable: Option<bool>,
    // A function which serves as a getter for the property, or `undefined` if there is no getter
    // (accessor descriptors only).
    pub get: Option<RemoteObject>,
    // A function which serves as a setter for the property, or `undefined` if there is no setter
    // (accessor descriptors only).
    pub set: Option<RemoteObject>,
    // True if the type of this property descriptor may be changed and if the property may be
    // deleted from the corresponding object.
    pub configurable: bool,
    // True if this property shows up during enumeration of the properties on the corresponding
    // object.
    pub enumerable: bool,
    // True if the result was thrown during the evaluation.
    pub was_thrown: Option<bool>,
    // True if the property is owned for the object.
    pub is_own: Option<bool>,
    // Property symbol object, if the property is of the `symbol` type.
    pub symbol: Option<RemoteObject>,
}
// Object internal property descriptor. This property isn't normally visible in JavaScript code.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InternalPropertyDescriptor {
    // Conventional property name.
    pub name: String,
    // The value associated with the property.
    pub value: Option<RemoteObject>,
}
// Object private field descriptor.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePropertyDescriptor {
    // Private property name.
    pub name: String,
    // The value associated with the private property.
    pub value: Option<RemoteObject>,
    // A function which serves as a getter for the private property,
    // or `undefined` if there is no getter (accessor descriptors only).
    pub get: Option<RemoteObject>,
    // A function which serves as a setter for the private property,
    // or `undefined` if there is no setter (accessor descriptors only).
    pub set: Option<RemoteObject>,
}
// Represents function call argument. Either remote object id `objectId`, primitive `value`,
// unserializable primitive value or neither of (for undefined) them should be specified.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallArgument {
    // Primitive value or serializable javascript object.
    pub value: Option<serde_json::Value>,
    // Primitive value which can not be JSON-stringified.
    pub unserializable_value: Option<UnserializableValue>,
    // Remote object handle.
    pub object_id: Option<RemoteObjectId>,
}
// Id of an execution context.
pub type ExecutionContextId = i32;
// Description of an isolated world.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDescription {
    // Unique id of the execution context. It can be used to specify in which execution context
    // script evaluation should be performed.
    pub id: ExecutionContextId,
    // Execution context origin.
    pub origin: String,
    // Human readable name describing given context.
    pub name: String,
    // Embedder-specific auxiliary data.
    // TODO objectProperty
}
// Detailed information about exception (or error) that was thrown during script compilation or
// execution.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    // Exception id.
    pub exception_id: i32,
    // Exception text, which should be used together with exception object when available.
    pub text: String,
    // Line number of the exception location (0-based).
    pub line_number: i32,
    // Column number of the exception location (0-based).
    pub column_number: i32,
    // Script ID of the exception location.
    pub script_id: Option<ScriptId>,
    // URL of the exception location, to be used when the script was not reported.
    pub url: Option<String>,
    // JavaScript stack trace if available.
    pub stack_trace: Option<StackTrace>,
    // Exception object if available.
    pub exception: Option<RemoteObject>,
    // Identifier of the context where exception happened.
    pub execution_context_id: Option<ExecutionContextId>,
}
// Number of milliseconds since epoch.
pub type Timestamp = f64;
// Number of milliseconds.
pub type TimeDelta = f64;
// Stack entry for runtime errors and assertions.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    // JavaScript function name.
    pub function_name: String,
    // JavaScript script id.
    pub script_id: ScriptId,
    // JavaScript script name or url.
    pub url: String,
    // JavaScript script line number (0-based).
    pub line_number: i32,
    // JavaScript script column number (0-based).
    pub column_number: i32,
}
// Call frames for assertions or error messages.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace {
    // String label of this stack trace. For async traces this may be a name of the function that
    // initiated the async call.
    pub description: Option<String>,
    // JavaScript function name.
    pub call_frames: Vec<CallFrame>,
    // Asynchronous JavaScript stack trace that preceded this stack, if available.
    pub parent: Option<Box<StackTrace>>,
    // Asynchronous JavaScript stack trace that preceded this stack, if available.
    pub parent_id: Option<StackTraceId>,
}
// Unique identifier of current debugger.
pub type UniqueDebuggerId = String;
// If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This
// allows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceId {
    pub id: String,
    pub debugger_id: Option<UniqueDebuggerId>,
}

// Add handler to promise with given promise object id.
#[derive(Serialize, Debug)]
pub struct AwaitPromise {
    // Identifier of the promise.
    pub promise_object_id: RemoteObjectId,
    // Whether the result is expected to be a JSON object that should be sent by value.
    pub return_by_value: Option<bool>,
    // Whether preview should be generated for the result.
    pub generate_preview: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AwaitPromiseReturnObject {
    // Promise result. Will contain rejected value if promise was rejected.
    pub result: RemoteObject,
    // Exception details if stack strace is available.
    pub exception_details: Option<ExceptionDetails>,
}
impl super::Command for AwaitPromise {
    const NAME: &'static str = "Runtime.awaitPromise";

    type ReturnObject = AwaitPromiseReturnObject;
}
// Calls function with given declaration on the given object. Object group of the result is
// inherited from the target object.
#[derive(Serialize, Debug)]
pub struct CallFunctionOn {
    // Declaration of the function to call.
    pub function_declaration: String,
    // Identifier of the object to call function on. Either objectId or executionContextId should
    // be specified.
    pub object_id: Option<RemoteObjectId>,
    // Call arguments. All call arguments must belong to the same JavaScript world as the target
    // object.
    pub arguments: Option<Vec<CallArgument>>,
    // In silent mode exceptions thrown during evaluation are not reported and do not pause
    // execution. Overrides `setPauseOnException` state.
    pub silent: Option<bool>,
    // Whether the result is expected to be a JSON object which should be sent by value.
    pub return_by_value: Option<bool>,
    // Whether preview should be generated for the result.
    pub generate_preview: Option<bool>,
    // Whether execution should be treated as initiated by user in the UI.
    pub user_gesture: Option<bool>,
    // Whether execution should `await` for resulting value and return once awaited promise is
    // resolved.
    pub await_promise: Option<bool>,
    // Specifies execution context which global object will be used to call function on. Either
    // executionContextId or objectId should be specified.
    pub execution_context_id: Option<ExecutionContextId>,
    // Symbolic group name that can be used to release multiple objects. If objectGroup is not
    // specified and objectId is, objectGroup will be inherited from object.
    pub object_group: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CallFunctionOnReturnObject {
    // Call result.
    pub result: RemoteObject,
    // Exception details.
    pub exception_details: Option<ExceptionDetails>,
}
impl super::Command for CallFunctionOn {
    const NAME: &'static str = "Runtime.callFunctionOn";

    type ReturnObject = CallFunctionOnReturnObject;
}
// Compiles expression.
#[derive(Serialize, Debug)]
pub struct CompileScript {
    // Expression to compile.
    pub expression: String,
    // Source url to be set for the script.
    pub source_url: String,
    // Specifies whether the compiled script should be persisted.
    pub persist_script: bool,
    // Specifies in which execution context to perform script run. If the parameter is omitted the
    // evaluation will be performed in the context of the inspected page.
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CompileScriptReturnObject {
    // Id of the script.
    pub script_id: Option<ScriptId>,
    // Exception details.
    pub exception_details: Option<ExceptionDetails>,
}
impl super::Command for CompileScript {
    const NAME: &'static str = "Runtime.compileScript";

    type ReturnObject = CompileScriptReturnObject;
}
// Disables reporting of execution contexts creation.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Runtime.disable";

    type ReturnObject = DisableReturnObject;
}
// Discards collected exceptions and console API calls.
#[derive(Serialize, Debug)]
pub struct DiscardConsoleEntries {}
#[derive(Deserialize, Debug, Clone)]
pub struct DiscardConsoleEntriesReturnObject {}
impl super::Command for DiscardConsoleEntries {
    const NAME: &'static str = "Runtime.discardConsoleEntries";

    type ReturnObject = DiscardConsoleEntriesReturnObject;
}
// Enables reporting of execution contexts creation by means of `executionContextCreated` event.
// When the reporting gets enabled the event will be sent immediately for each existing execution
// context.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Runtime.enable";

    type ReturnObject = EnableReturnObject;
}
// Evaluates expression on global object.
#[derive(Serialize, Debug)]
pub struct Evaluate {
    // Expression to evaluate.
    pub expression: String,
    // Symbolic group name that can be used to release multiple objects.
    pub object_group: Option<String>,
    // Determines whether Command Line API should be available during the evaluation.
    pub include_command_line_api: Option<bool>,
    // In silent mode exceptions thrown during evaluation are not reported and do not pause
    // execution. Overrides `setPauseOnException` state.
    pub silent: Option<bool>,
    // Specifies in which execution context to perform evaluation. If the parameter is omitted the
    // evaluation will be performed in the context of the inspected page.
    pub context_id: Option<ExecutionContextId>,
    // Whether the result is expected to be a JSON object that should be sent by value.
    pub return_by_value: Option<bool>,
    // Whether preview should be generated for the result.
    pub generate_preview: Option<bool>,
    // Whether execution should be treated as initiated by user in the UI.
    pub user_gesture: Option<bool>,
    // Whether execution should `await` for resulting value and return once awaited promise is
    // resolved.
    pub await_promise: Option<bool>,
    // Whether to throw an exception if side effect cannot be ruled out during evaluation.
    // This implies `disableBreaks` below.
    pub throw_on_side_effect: Option<bool>,
    // Terminate execution after timing out (number of milliseconds).
    pub timeout: Option<TimeDelta>,
    // Disable breakpoints during execution.
    pub disable_breaks: Option<bool>,
    // Setting this flag to true enables `let` re-declaration and top-level `await`.
    // Note that `let` variables can only be re-declared if they originate from
    // `replMode` themselves.
    pub repl_mode: Option<bool>,
    // The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    // which includes eval(), Function(), setTimeout() and setInterval()
    // when called with non-callable arguments. This flag bypasses CSP for this
    // evaluation and allows unsafe-eval. Defaults to true.
    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct EvaluateReturnObject {
    // Evaluation result.
    pub result: RemoteObject,
    // Exception details.
    pub exception_details: Option<ExceptionDetails>,
}
impl super::Command for Evaluate {
    const NAME: &'static str = "Runtime.evaluate";

    type ReturnObject = EvaluateReturnObject;
}
// Returns the isolate id.
#[derive(Serialize, Debug)]
pub struct GetIsolateId {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetIsolateIdReturnObject {
    // The isolate id.
    pub id: String,
}
impl super::Command for GetIsolateId {
    const NAME: &'static str = "Runtime.getIsolateId";

    type ReturnObject = GetIsolateIdReturnObject;
}
// Returns the JavaScript heap usage.
// It is the total usage of the corresponding isolate not scoped to a particular Runtime.
#[derive(Serialize, Debug)]
pub struct GetHeapUsage {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetHeapUsageReturnObject {
    // Used heap size in bytes.
    pub used_size: f64,
    // Allocated heap size in bytes.
    pub total_size: f64,
}
impl super::Command for GetHeapUsage {
    const NAME: &'static str = "Runtime.getHeapUsage";

    type ReturnObject = GetHeapUsageReturnObject;
}
// Returns properties of a given object. Object group of the result is inherited from the target
// object.
#[derive(Serialize, Debug)]
pub struct GetProperties {
    // Identifier of the object to return properties for.
    pub object_id: RemoteObjectId,
    // If true, returns properties belonging only to the element itself, not to its prototype
    // chain.
    pub own_properties: Option<bool>,
    // If true, returns accessor properties (with getter/setter) only; internal properties are not
    // returned either.
    pub accessor_properties_only: Option<bool>,
    // Whether preview should be generated for the results.
    pub generate_preview: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetPropertiesReturnObject {
    // Object properties.
    pub result: Vec<PropertyDescriptor>,
    // Internal object properties (only of the element itself).
    pub internal_properties: Option<Vec<InternalPropertyDescriptor>>,
    // Object private properties.
    pub private_properties: Option<Vec<PrivatePropertyDescriptor>>,
    // Exception details.
    pub exception_details: Option<ExceptionDetails>,
}
impl super::Command for GetProperties {
    const NAME: &'static str = "Runtime.getProperties";

    type ReturnObject = GetPropertiesReturnObject;
}
// Returns all let, const and class variables from global scope.
#[derive(Serialize, Debug)]
pub struct GlobalLexicalScopeNames {
    // Specifies in which execution context to lookup global scope variables.
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GlobalLexicalScopeNamesReturnObject {
    pub names: Vec<String>,
}
impl super::Command for GlobalLexicalScopeNames {
    const NAME: &'static str = "Runtime.globalLexicalScopeNames";

    type ReturnObject = GlobalLexicalScopeNamesReturnObject;
}
#[derive(Serialize, Debug)]
pub struct QueryObjects {
    // Identifier of the prototype to return objects for.
    pub prototype_object_id: RemoteObjectId,
    // Symbolic group name that can be used to release the results.
    pub object_group: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct QueryObjectsReturnObject {
    // Array with objects.
    pub objects: RemoteObject,
}
impl super::Command for QueryObjects {
    const NAME: &'static str = "Runtime.queryObjects";

    type ReturnObject = QueryObjectsReturnObject;
}
// Releases remote object with given id.
#[derive(Serialize, Debug)]
pub struct ReleaseObject {
    // Identifier of the object to release.
    pub object_id: RemoteObjectId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ReleaseObjectReturnObject {}
impl super::Command for ReleaseObject {
    const NAME: &'static str = "Runtime.releaseObject";

    type ReturnObject = ReleaseObjectReturnObject;
}
// Releases all remote objects that belong to a given group.
#[derive(Serialize, Debug)]
pub struct ReleaseObjectGroup {
    // Symbolic object group name.
    pub object_group: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ReleaseObjectGroupReturnObject {}
impl super::Command for ReleaseObjectGroup {
    const NAME: &'static str = "Runtime.releaseObjectGroup";

    type ReturnObject = ReleaseObjectGroupReturnObject;
}
// Tells inspected instance to run if it was waiting for debugger to attach.
#[derive(Serialize, Debug)]
pub struct RunIfWaitingForDebugger {}
#[derive(Deserialize, Debug, Clone)]
pub struct RunIfWaitingForDebuggerReturnObject {}
impl super::Command for RunIfWaitingForDebugger {
    const NAME: &'static str = "Runtime.runIfWaitingForDebugger";

    type ReturnObject = RunIfWaitingForDebuggerReturnObject;
}
// Runs script with given id in a given context.
#[derive(Serialize, Debug)]
pub struct RunScript {
    // Id of the script to run.
    pub script_id: ScriptId,
    // Specifies in which execution context to perform script run. If the parameter is omitted the
    // evaluation will be performed in the context of the inspected page.
    pub execution_context_id: Option<ExecutionContextId>,
    // Symbolic group name that can be used to release multiple objects.
    pub object_group: Option<String>,
    // In silent mode exceptions thrown during evaluation are not reported and do not pause
    // execution. Overrides `setPauseOnException` state.
    pub silent: Option<bool>,
    // Determines whether Command Line API should be available during the evaluation.
    pub include_command_line_api: Option<bool>,
    // Whether the result is expected to be a JSON object which should be sent by value.
    pub return_by_value: Option<bool>,
    // Whether preview should be generated for the result.
    pub generate_preview: Option<bool>,
    // Whether execution should `await` for resulting value and return once awaited promise is
    // resolved.
    pub await_promise: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RunScriptReturnObject {
    // Run result.
    pub result: RemoteObject,
    // Exception details.
    pub exception_details: Option<ExceptionDetails>,
}
impl super::Command for RunScript {
    const NAME: &'static str = "Runtime.runScript";

    type ReturnObject = RunScriptReturnObject;
}
// Enables or disables async call stacks tracking.
#[derive(Serialize, Debug)]
pub struct SetAsyncCallStackDepth {
    // Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
    // call stacks (default).
    pub max_depth: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetAsyncCallStackDepthReturnObject {}
impl super::Command for SetAsyncCallStackDepth {
    const NAME: &'static str = "Runtime.setAsyncCallStackDepth";

    type ReturnObject = SetAsyncCallStackDepthReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SetCustomObjectFormatterEnabled {
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetCustomObjectFormatterEnabledReturnObject {}
impl super::Command for SetCustomObjectFormatterEnabled {
    const NAME: &'static str = "Runtime.setCustomObjectFormatterEnabled";

    type ReturnObject = SetCustomObjectFormatterEnabledReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SetMaxCallStackSizeToCapture {
    pub size: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetMaxCallStackSizeToCaptureReturnObject {}
impl super::Command for SetMaxCallStackSizeToCapture {
    const NAME: &'static str = "Runtime.setMaxCallStackSizeToCapture";

    type ReturnObject = SetMaxCallStackSizeToCaptureReturnObject;
}
// Terminate current or next JavaScript execution.
// Will cancel the termination when the outer-most script execution ends.
#[derive(Serialize, Debug)]
pub struct TerminateExecution {}
#[derive(Deserialize, Debug, Clone)]
pub struct TerminateExecutionReturnObject {}
impl super::Command for TerminateExecution {
    const NAME: &'static str = "Runtime.terminateExecution";

    type ReturnObject = TerminateExecutionReturnObject;
}
// If executionContextId is empty, adds binding with the given name on the
// global objects of all inspected contexts, including those created later,
// bindings survive reloads.
// If executionContextId is specified, adds binding only on global object of
// given execution context.
// Binding function takes exactly one argument, this argument should be string,
// in case of any other input, function throws an exception.
// Each binding function call produces Runtime.bindingCalled notification.
#[derive(Serialize, Debug)]
pub struct AddBinding {
    pub name: String,
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AddBindingReturnObject {}
impl super::Command for AddBinding {
    const NAME: &'static str = "Runtime.addBinding";

    type ReturnObject = AddBindingReturnObject;
}
// This method does not remove binding function from global object but
// unsubscribes current runtime agent from Runtime.bindingCalled notifications.
#[derive(Serialize, Debug)]
pub struct RemoveBinding {
    pub name: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RemoveBindingReturnObject {}
impl super::Command for RemoveBinding {
    const NAME: &'static str = "Runtime.removeBinding";

    type ReturnObject = RemoveBindingReturnObject;
}
