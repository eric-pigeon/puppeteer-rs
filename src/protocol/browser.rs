// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

pub type BrowserContextID = String;
pub type WindowID = i32;
// The state of the browser window.
pub type WindowState = String;
// Browser window bounds information
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    // The offset from the left edge of the screen to the window in pixels.
    pub left: Option<i32>,
    // The offset from the top edge of the screen to the window in pixels.
    pub top: Option<i32>,
    // The window width in pixels.
    pub width: Option<i32>,
    // The window height in pixels.
    pub height: Option<i32>,
    // The window state. Default to normal.
    pub window_state: Option<WindowState>,
}
pub type PermissionType = String;
pub type PermissionSetting = String;
// Definition of PermissionDescriptor defined in the Permissions API:
// https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor {
    // Name of permission.
    // See https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names.
    pub name: String,
    // For "midi" permission, may also specify sysex control.
    pub sysex: Option<bool>,
    // For "push" permission, may specify userVisibleOnly.
    // Note that userVisibleOnly = true is the only currently supported type.
    pub user_visible_only: Option<bool>,
    // For "clipboard" permission, may specify allowWithoutSanitization.
    pub allow_without_sanitization: Option<bool>,
    // For "camera" permission, may specify panTiltZoom.
    pub pan_tilt_zoom: Option<bool>,
}
// Chrome histogram bucket.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    // Minimum value (inclusive).
    pub low: i32,
    // Maximum value (exclusive).
    pub high: i32,
    // Number of samples.
    pub count: i32,
}
// Chrome histogram.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    // Name.
    pub name: String,
    // Sum of sample values.
    pub sum: i32,
    // Total number of samples.
    pub count: i32,
    // Buckets.
    pub buckets: Vec<Bucket>,
}

// Set permission settings for given origin.
#[derive(Serialize, Debug)]
pub struct SetPermission {
    // Descriptor of permission to override.
    pub permission: PermissionDescriptor,
    // Setting of the permission.
    pub setting: PermissionSetting,
    // Origin the permission applies to, all origins if not specified.
    pub origin: Option<String>,
    // Context to override. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetPermissionReturnObject {}
impl super::Command for SetPermission {
    const NAME: &'static str = "Browser.setPermission";

    type ReturnObject = SetPermissionReturnObject;
}
// Grant specific permissions to the given origin and reject all others.
#[derive(Serialize, Debug)]
pub struct GrantPermissions {
    pub permissions: Vec<PermissionType>,
    // Origin the permission applies to, all origins if not specified.
    pub origin: Option<String>,
    // BrowserContext to override permissions. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GrantPermissionsReturnObject {}
impl super::Command for GrantPermissions {
    const NAME: &'static str = "Browser.grantPermissions";

    type ReturnObject = GrantPermissionsReturnObject;
}
// Reset all permission management for all origins.
#[derive(Serialize, Debug)]
pub struct ResetPermissions {
    // BrowserContext to reset permissions. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ResetPermissionsReturnObject {}
impl super::Command for ResetPermissions {
    const NAME: &'static str = "Browser.resetPermissions";

    type ReturnObject = ResetPermissionsReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Behavior {
    Deny,
    Allow,
    AllowAndName,
    Default,
}
// Set the behavior when downloading a file.
#[derive(Serialize, Debug)]
pub struct SetDownloadBehavior {
    // Whether to allow all or deny all download requests, or use default Chrome behavior if
    // available (otherwise deny). |allowAndName| allows download and names files according to
    // their dowmload guids.
    pub behavior: String,
    // BrowserContext to set download behavior. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
    // The default path to save downloaded files to. This is requred if behavior is set to 'allow'
    // or 'allowAndName'.
    pub download_path: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDownloadBehaviorReturnObject {}
impl super::Command for SetDownloadBehavior {
    const NAME: &'static str = "Browser.setDownloadBehavior";

    type ReturnObject = SetDownloadBehaviorReturnObject;
}
// Close browser gracefully.
#[derive(Serialize, Debug)]
pub struct Close {}
#[derive(Deserialize, Debug, Clone)]
pub struct CloseReturnObject {}
impl super::Command for Close {
    const NAME: &'static str = "Browser.close";

    type ReturnObject = CloseReturnObject;
}
// Crashes browser on the main thread.
#[derive(Serialize, Debug)]
pub struct Crash {}
#[derive(Deserialize, Debug, Clone)]
pub struct CrashReturnObject {}
impl super::Command for Crash {
    const NAME: &'static str = "Browser.crash";

    type ReturnObject = CrashReturnObject;
}
// Crashes GPU process.
#[derive(Serialize, Debug)]
pub struct CrashGpuProcess {}
#[derive(Deserialize, Debug, Clone)]
pub struct CrashGpuProcessReturnObject {}
impl super::Command for CrashGpuProcess {
    const NAME: &'static str = "Browser.crashGpuProcess";

    type ReturnObject = CrashGpuProcessReturnObject;
}
// Returns version information.
#[derive(Serialize, Debug)]
pub struct GetVersion {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetVersionReturnObject {
    // Protocol version.
    pub protocol_version: String,
    // Product name.
    pub product: String,
    // Product revision.
    pub revision: String,
    // User-Agent.
    pub user_agent: String,
    // V8 version.
    pub js_version: String,
}
impl super::Command for GetVersion {
    const NAME: &'static str = "Browser.getVersion";

    type ReturnObject = GetVersionReturnObject;
}
// Returns the command line switches for the browser process if, and only if
// --enable-automation is on the commandline.
#[derive(Serialize, Debug)]
pub struct GetBrowserCommandLine {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetBrowserCommandLineReturnObject {
    // Commandline parameters
    pub arguments: Vec<String>,
}
impl super::Command for GetBrowserCommandLine {
    const NAME: &'static str = "Browser.getBrowserCommandLine";

    type ReturnObject = GetBrowserCommandLineReturnObject;
}
// Get Chrome histograms.
#[derive(Serialize, Debug)]
pub struct GetHistograms {
    // Requested substring in name. Only histograms which have query as a
    // substring in their name are extracted. An empty or absent query returns
    // all histograms.
    pub query: Option<String>,
    // If true, retrieve delta since last call.
    pub delta: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetHistogramsReturnObject {
    // Histograms.
    pub histograms: Vec<Histogram>,
}
impl super::Command for GetHistograms {
    const NAME: &'static str = "Browser.getHistograms";

    type ReturnObject = GetHistogramsReturnObject;
}
// Get a Chrome histogram by name.
#[derive(Serialize, Debug)]
pub struct GetHistogram {
    // Requested histogram name.
    pub name: String,
    // If true, retrieve delta since last call.
    pub delta: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetHistogramReturnObject {
    // Histogram.
    pub histogram: Histogram,
}
impl super::Command for GetHistogram {
    const NAME: &'static str = "Browser.getHistogram";

    type ReturnObject = GetHistogramReturnObject;
}
// Get position and size of the browser window.
#[derive(Serialize, Debug)]
pub struct GetWindowBounds {
    // Browser window id.
    pub window_id: WindowID,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetWindowBoundsReturnObject {
    // Bounds information of the window. When window state is 'minimized', the restored window
    // position and size are returned.
    pub bounds: Bounds,
}
impl super::Command for GetWindowBounds {
    const NAME: &'static str = "Browser.getWindowBounds";

    type ReturnObject = GetWindowBoundsReturnObject;
}
// Get the browser window that contains the devtools target.
#[derive(Serialize, Debug)]
pub struct GetWindowForTarget {
    // Devtools agent host id. If called as a part of the session, associated targetId is used.
    pub target_id: Option<super::target::TargetID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetWindowForTargetReturnObject {
    // Browser window id.
    pub window_id: WindowID,
    // Bounds information of the window. When window state is 'minimized', the restored window
    // position and size are returned.
    pub bounds: Bounds,
}
impl super::Command for GetWindowForTarget {
    const NAME: &'static str = "Browser.getWindowForTarget";

    type ReturnObject = GetWindowForTargetReturnObject;
}
// Set position and/or size of the browser window.
#[derive(Serialize, Debug)]
pub struct SetWindowBounds {
    // Browser window id.
    pub window_id: WindowID,
    // New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    // with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    pub bounds: Bounds,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetWindowBoundsReturnObject {}
impl super::Command for SetWindowBounds {
    const NAME: &'static str = "Browser.setWindowBounds";

    type ReturnObject = SetWindowBoundsReturnObject;
}
// Set dock tile details, platform-specific.
#[derive(Serialize, Debug)]
pub struct SetDockTile {
    pub badge_label: Option<String>,
    // Png encoded image.
    pub image: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDockTileReturnObject {}
impl super::Command for SetDockTile {
    const NAME: &'static str = "Browser.setDockTile";

    type ReturnObject = SetDockTileReturnObject;
}
