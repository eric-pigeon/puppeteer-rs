// This file is auto-generated do not edit manually.

// DOM breakpoint type.
pub enum DOMBreakpointType {
    SubtreeModified,
    AttributeModified,
    NodeRemoved,
}
// Object event listener.
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
pub struct GetEventListenersReturnObject {
    // Array of relevant listeners.
    pub listeners: Vec<EventListener>,
}
// Removes DOM breakpoint that was set using `setDOMBreakpoint`.
pub struct RemoveDOMBreakpoint {
    // Identifier of the node to remove breakpoint from.
    pub node_id: super::dom::NodeId,
    // Type of the breakpoint to remove.
    pub r#type: DOMBreakpointType,
}
pub struct RemoveDOMBreakpointReturnObject {}
// Removes breakpoint on particular DOM event.
pub struct RemoveEventListenerBreakpoint {
    // Event name.
    pub event_name: String,
    // EventTarget interface name.
    pub target_name: Option<String>,
}
pub struct RemoveEventListenerBreakpointReturnObject {}
// Removes breakpoint on particular native event.
pub struct RemoveInstrumentationBreakpoint {
    // Instrumentation name to stop on.
    pub event_name: String,
}
pub struct RemoveInstrumentationBreakpointReturnObject {}
// Removes breakpoint from XMLHttpRequest.
pub struct RemoveXHRBreakpoint {
    // Resource URL substring.
    pub url: String,
}
pub struct RemoveXHRBreakpointReturnObject {}
// Sets breakpoint on particular operation with DOM.
pub struct SetDOMBreakpoint {
    // Identifier of the node to set breakpoint on.
    pub node_id: super::dom::NodeId,
    // Type of the operation to stop upon.
    pub r#type: DOMBreakpointType,
}
pub struct SetDOMBreakpointReturnObject {}
// Sets breakpoint on particular DOM event.
pub struct SetEventListenerBreakpoint {
    // DOM Event name to stop on (any DOM event will do).
    pub event_name: String,
    // EventTarget interface name to stop on. If equal to `"*"` or not provided, will stop on any
    // EventTarget.
    pub target_name: Option<String>,
}
pub struct SetEventListenerBreakpointReturnObject {}
// Sets breakpoint on particular native event.
pub struct SetInstrumentationBreakpoint {
    // Instrumentation name to stop on.
    pub event_name: String,
}
pub struct SetInstrumentationBreakpointReturnObject {}
// Sets breakpoint on XMLHttpRequest.
pub struct SetXHRBreakpoint {
    // Resource URL substring. All XHRs having this substring in the URL will get stopped upon.
    pub url: String,
}
pub struct SetXHRBreakpointReturnObject {}
