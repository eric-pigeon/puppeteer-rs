// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// DOM breakpoint type.
pub type DOMBreakpointType = String;
// Object event listener.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventListener {
    // `EventListener`'s type.
    pub r#type: String,
    // `EventListener`'s useCapture.
    pub use_capture: bool,
    // `EventListener`'s passive flag.
    pub passive: bool,
    // `EventListener`'s once flag.
    pub once: bool,
    // Script id of the handler code.
    pub script_id: super::runtime::ScriptId,
    // Line number in the script (0-based).
    pub line_number: i32,
    // Column number in the script (0-based).
    pub column_number: i32,
    // Event handler function value.
    pub handler: Option<super::runtime::RemoteObject>,
    // Event original handler function value.
    pub original_handler: Option<super::runtime::RemoteObject>,
    // Node the listener is added to (if any).
    pub backend_node_id: Option<super::dom::BackendNodeId>,
}

// Returns event listeners of the given object.
#[derive(Serialize, Debug)]
pub struct GetEventListeners {
    // Identifier of the object to return listeners for.
    pub object_id: super::runtime::RemoteObjectId,
    // The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the subtree
    // (default is false). Reports listeners for all contexts if pierce is enabled.
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetEventListenersReturnObject {
    // Array of relevant listeners.
    pub listeners: Vec<EventListener>,
}
impl super::Command for GetEventListeners {
    const NAME: &'static str = "DOMDebugger.getEventListeners";

    type ReturnObject = GetEventListenersReturnObject;
}
// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
#[derive(Serialize, Debug)]
pub struct RemoveDOMBreakpoint {
    // Identifier of the node to remove breakpoint from.
    pub node_id: super::dom::NodeId,
    // Type of the breakpoint to remove.
    pub r#type: DOMBreakpointType,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDOMBreakpointReturnObject {}
impl super::Command for RemoveDOMBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeDOMBreakpoint";

    type ReturnObject = RemoveDOMBreakpointReturnObject;
}
// Removes breakpoint on particular DOM event.
#[derive(Serialize, Debug)]
pub struct RemoveEventListenerBreakpoint {
    // Event name.
    pub event_name: String,
    // EventTarget interface name.
    pub target_name: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEventListenerBreakpointReturnObject {}
impl super::Command for RemoveEventListenerBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeEventListenerBreakpoint";

    type ReturnObject = RemoveEventListenerBreakpointReturnObject;
}
// Removes breakpoint on particular native event.
#[derive(Serialize, Debug)]
pub struct RemoveInstrumentationBreakpoint {
    // Instrumentation name to stop on.
    pub event_name: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstrumentationBreakpointReturnObject {}
impl super::Command for RemoveInstrumentationBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";

    type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
}
// Removes breakpoint from XMLHttpRequest.
#[derive(Serialize, Debug)]
pub struct RemoveXHRBreakpoint {
    // Resource URL substring.
    pub url: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveXHRBreakpointReturnObject {}
impl super::Command for RemoveXHRBreakpoint {
    const NAME: &'static str = "DOMDebugger.removeXHRBreakpoint";

    type ReturnObject = RemoveXHRBreakpointReturnObject;
}
// Sets breakpoint on particular operation with DOM.
#[derive(Serialize, Debug)]
pub struct SetDOMBreakpoint {
    // Identifier of the node to set breakpoint on.
    pub node_id: super::dom::NodeId,
    // Type of the operation to stop upon.
    pub r#type: DOMBreakpointType,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetDOMBreakpointReturnObject {}
impl super::Command for SetDOMBreakpoint {
    const NAME: &'static str = "DOMDebugger.setDOMBreakpoint";

    type ReturnObject = SetDOMBreakpointReturnObject;
}
// Sets breakpoint on particular DOM event.
#[derive(Serialize, Debug)]
pub struct SetEventListenerBreakpoint {
    // DOM Event name to stop on (any DOM event will do).
    pub event_name: String,
    // EventTarget interface name to stop on. If equal to `"*"` or not provided, will stop on any
    // EventTarget.
    pub target_name: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetEventListenerBreakpointReturnObject {}
impl super::Command for SetEventListenerBreakpoint {
    const NAME: &'static str = "DOMDebugger.setEventListenerBreakpoint";

    type ReturnObject = SetEventListenerBreakpointReturnObject;
}
// Sets breakpoint on particular native event.
#[derive(Serialize, Debug)]
pub struct SetInstrumentationBreakpoint {
    // Instrumentation name to stop on.
    pub event_name: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturnObject {}
impl super::Command for SetInstrumentationBreakpoint {
    const NAME: &'static str = "DOMDebugger.setInstrumentationBreakpoint";

    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
// Sets breakpoint on XMLHttpRequest.
#[derive(Serialize, Debug)]
pub struct SetXHRBreakpoint {
    // Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    pub url: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetXHRBreakpointReturnObject {}
impl super::Command for SetXHRBreakpoint {
    const NAME: &'static str = "DOMDebugger.setXHRBreakpoint";

    type ReturnObject = SetXHRBreakpointReturnObject;
}
