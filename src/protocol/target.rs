// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

pub type TargetID = String;
// Unique identifier of attached debugging session.
pub type SessionID = String;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: TargetID,
    pub r#type: String,
    pub title: String,
    pub url: String,
    // Whether the target has an attached client.
    pub attached: bool,
    // Opener target Id
    pub opener_id: Option<TargetID>,
    // Whether the target has access to the originating window.
    pub can_access_opener: bool,
    // Frame id of originating window (is only set if target has an opener).
    pub opener_frame_id: Option<super::page::FrameId>,
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteLocation {
    pub host: String,
    pub port: i32,
}

// Activates (focuses) the target.
#[derive(Serialize, Debug)]
pub struct ActivateTarget {
    pub target_id: TargetID,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ActivateTargetReturnObject {}
impl super::Command for ActivateTarget {
    const NAME: &'static str = "Target.activateTarget";

    type ReturnObject = ActivateTargetReturnObject;
}
// Attaches to the target with given id.
#[derive(Serialize, Debug)]
pub struct AttachToTarget {
    pub target_id: TargetID,
    // Enables "flat" access to the session via specifying sessionId attribute in the commands.
    // We plan to make this the default, deprecate non-flattened mode,
    // and eventually retire it. See crbug.com/991325.
    pub flatten: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AttachToTargetReturnObject {
    // Id assigned to the session.
    pub session_id: SessionID,
}
impl super::Command for AttachToTarget {
    const NAME: &'static str = "Target.attachToTarget";

    type ReturnObject = AttachToTargetReturnObject;
}
// Attaches to the browser target, only uses flat sessionId mode.
#[derive(Serialize, Debug)]
pub struct AttachToBrowserTarget {}
#[derive(Deserialize, Debug, Clone)]
pub struct AttachToBrowserTargetReturnObject {
    // Id assigned to the session.
    pub session_id: SessionID,
}
impl super::Command for AttachToBrowserTarget {
    const NAME: &'static str = "Target.attachToBrowserTarget";

    type ReturnObject = AttachToBrowserTargetReturnObject;
}
// Closes the target. If the target is a page that gets closed too.
#[derive(Serialize, Debug)]
pub struct CloseTarget {
    pub target_id: TargetID,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CloseTargetReturnObject {
    // Always set to true. If an error occurs, the response indicates protocol error.
    pub success: bool,
}
impl super::Command for CloseTarget {
    const NAME: &'static str = "Target.closeTarget";

    type ReturnObject = CloseTargetReturnObject;
}
// Inject object to the target's main frame that provides a communication
// channel with browser target.
//
// Injected object will be available as `window[bindingName]`.
//
// The object has the follwing API:
// - `binding.send(json)` - a method to send messages over the remote debugging protocol
// - `binding.onmessage = json => handleMessage(json)` - a callback that will be called for the protocol notifications and command responses.
#[derive(Serialize, Debug)]
pub struct ExposeDevToolsProtocol {
    pub target_id: TargetID,
    // Binding name, 'cdp' if not specified.
    pub binding_name: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ExposeDevToolsProtocolReturnObject {}
impl super::Command for ExposeDevToolsProtocol {
    const NAME: &'static str = "Target.exposeDevToolsProtocol";

    type ReturnObject = ExposeDevToolsProtocolReturnObject;
}
// Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
// one.
#[derive(Serialize, Debug)]
pub struct CreateBrowserContext {
    // If specified, disposes this context when debugging session disconnects.
    pub dispose_on_detach: Option<bool>,
    // Proxy server, similar to the one passed to --proxy-server
    pub proxy_server: Option<String>,
    // Proxy bypass list, similar to the one passed to --proxy-bypass-list
    pub proxy_bypass_list: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CreateBrowserContextReturnObject {
    // The id of the context created.
    pub browser_context_id: super::browser::BrowserContextID,
}
impl super::Command for CreateBrowserContext {
    const NAME: &'static str = "Target.createBrowserContext";

    type ReturnObject = CreateBrowserContextReturnObject;
}
// Returns all browser contexts created with `Target.createBrowserContext` method.
#[derive(Serialize, Debug)]
pub struct GetBrowserContexts {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetBrowserContextsReturnObject {
    // An array of browser context ids.
    pub browser_context_ids: Vec<super::browser::BrowserContextID>,
}
impl super::Command for GetBrowserContexts {
    const NAME: &'static str = "Target.getBrowserContexts";

    type ReturnObject = GetBrowserContextsReturnObject;
}
// Creates a new page.
#[derive(Serialize, Debug)]
pub struct CreateTarget {
    // The initial URL the page will be navigated to.
    pub url: String,
    // Frame width in DIP (headless chrome only).
    pub width: Option<i32>,
    // Frame height in DIP (headless chrome only).
    pub height: Option<i32>,
    // The browser context to create the page in.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
    // Whether BeginFrames for this target will be controlled via DevTools (headless chrome only,
    // not supported on MacOS yet, false by default).
    pub enable_begin_frame_control: Option<bool>,
    // Whether to create a new Window or Tab (chrome-only, false by default).
    pub new_window: Option<bool>,
    // Whether to create the target in background or foreground (chrome-only,
    // false by default).
    pub background: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CreateTargetReturnObject {
    // The id of the page opened.
    pub target_id: TargetID,
}
impl super::Command for CreateTarget {
    const NAME: &'static str = "Target.createTarget";

    type ReturnObject = CreateTargetReturnObject;
}
// Detaches session with given id.
#[derive(Serialize, Debug)]
pub struct DetachFromTarget {
    // Session to detach.
    pub session_id: Option<SessionID>,
    // Deprecated.
    pub target_id: Option<TargetID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DetachFromTargetReturnObject {}
impl super::Command for DetachFromTarget {
    const NAME: &'static str = "Target.detachFromTarget";

    type ReturnObject = DetachFromTargetReturnObject;
}
// Deletes a BrowserContext. All the belonging pages will be closed without calling their
// beforeunload hooks.
#[derive(Serialize, Debug)]
pub struct DisposeBrowserContext {
    pub browser_context_id: super::browser::BrowserContextID,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DisposeBrowserContextReturnObject {}
impl super::Command for DisposeBrowserContext {
    const NAME: &'static str = "Target.disposeBrowserContext";

    type ReturnObject = DisposeBrowserContextReturnObject;
}
// Returns information about a target.
#[derive(Serialize, Debug)]
pub struct GetTargetInfo {
    pub target_id: Option<TargetID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetTargetInfoReturnObject {
    pub target_info: TargetInfo,
}
impl super::Command for GetTargetInfo {
    const NAME: &'static str = "Target.getTargetInfo";

    type ReturnObject = GetTargetInfoReturnObject;
}
// Retrieves a list of available targets.
#[derive(Serialize, Debug)]
pub struct GetTargets {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetTargetsReturnObject {
    // The list of targets.
    pub target_infos: Vec<TargetInfo>,
}
impl super::Command for GetTargets {
    const NAME: &'static str = "Target.getTargets";

    type ReturnObject = GetTargetsReturnObject;
}
// Sends protocol message over session with given id.
// Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
// and crbug.com/991325.
#[derive(Serialize, Debug)]
pub struct SendMessageToTarget {
    pub message: String,
    // Identifier of the session.
    pub session_id: Option<SessionID>,
    // Deprecated.
    pub target_id: Option<TargetID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SendMessageToTargetReturnObject {}
impl super::Command for SendMessageToTarget {
    const NAME: &'static str = "Target.sendMessageToTarget";

    type ReturnObject = SendMessageToTargetReturnObject;
}
// Controls whether to automatically attach to new targets which are considered to be related to
// this one. When turned on, attaches to all existing related targets as well. When turned off,
// automatically detaches from all currently attached targets.
#[derive(Serialize, Debug)]
pub struct SetAutoAttach {
    // Whether to auto-attach to related targets.
    pub auto_attach: bool,
    // Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`
    // to run paused targets.
    pub wait_for_debugger_on_start: bool,
    // Enables "flat" access to the session via specifying sessionId attribute in the commands.
    // We plan to make this the default, deprecate non-flattened mode,
    // and eventually retire it. See crbug.com/991325.
    pub flatten: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetAutoAttachReturnObject {}
impl super::Command for SetAutoAttach {
    const NAME: &'static str = "Target.setAutoAttach";

    type ReturnObject = SetAutoAttachReturnObject;
}
// Controls whether to discover available targets and notify via
// `targetCreated/targetInfoChanged/targetDestroyed` events.
#[derive(Serialize, Debug)]
pub struct SetDiscoverTargets {
    // Whether to discover available targets.
    pub discover: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDiscoverTargetsReturnObject {}
impl super::Command for SetDiscoverTargets {
    const NAME: &'static str = "Target.setDiscoverTargets";

    type ReturnObject = SetDiscoverTargetsReturnObject;
}
// Enables target discovery for the specified locations, when `setDiscoverTargets` was set to
// `true`.
#[derive(Serialize, Debug)]
pub struct SetRemoteLocations {
    // List of remote locations.
    pub locations: Vec<RemoteLocation>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetRemoteLocationsReturnObject {}
impl super::Command for SetRemoteLocations {
    const NAME: &'static str = "Target.setRemoteLocations";

    type ReturnObject = SetRemoteLocationsReturnObject;
}
