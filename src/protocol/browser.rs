// This file is auto-generated do not edit manually.

pub type BrowserContextID = String;
pub type WindowID = i32;
// The state of the browser window.
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}
// Browser window bounds information
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
pub enum PermissionType {
    AccessibilityEvents,
    AudioCapture,
    BackgroundSync,
    BackgroundFetch,
    ClipboardReadWrite,
    ClipboardSanitizedWrite,
    DurableStorage,
    Flash,
    Geolocation,
    Midi,
    MidiSysex,
    Nfc,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
    ProtectedMediaIdentifier,
    Sensors,
    VideoCapture,
    VideoCapturePanTiltZoom,
    IdleDetection,
    WakeLockScreen,
    WakeLockSystem,
}
pub enum PermissionSetting {
    Granted,
    Denied,
    Prompt,
}
// Definition of PermissionDescriptor defined in the Permissions API:
// https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
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
pub struct Bucket {
    // Minimum value (inclusive).
    pub low: i32,
    // Maximum value (exclusive).
    pub high: i32,
    // Number of samples.
    pub count: i32,
}
// Chrome histogram.
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
pub struct SetPermissionReturnObject {}
// Grant specific permissions to the given origin and reject all others.
pub struct GrantPermissions {
    pub permissions: Vec<PermissionType>,
    // Origin the permission applies to, all origins if not specified.
    pub origin: Option<String>,
    // BrowserContext to override permissions. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
}
pub struct GrantPermissionsReturnObject {}
// Reset all permission management for all origins.
pub struct ResetPermissions {
    // BrowserContext to reset permissions. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
}
pub struct ResetPermissionsReturnObject {}
pub enum Behavior {
    Deny,
    Allow,
    AllowAndName,
    Default,
}
// Set the behavior when downloading a file.
pub struct SetDownloadBehavior {
    // Whether to allow all or deny all download requests, or use default Chrome behavior if
    // available (otherwise deny). |allowAndName| allows download and names files according to
    // their dowmload guids.
    pub behavior: Behavior,
    // BrowserContext to set download behavior. When omitted, default browser context is used.
    pub browser_context_id: Option<BrowserContextID>,
    // The default path to save downloaded files to. This is requred if behavior is set to 'allow'
    // or 'allowAndName'.
    pub download_path: Option<String>,
}
pub struct SetDownloadBehaviorReturnObject {}
// Close browser gracefully.
pub struct Close {}
pub struct CloseReturnObject {}
// Crashes browser on the main thread.
pub struct Crash {}
pub struct CrashReturnObject {}
// Crashes GPU process.
pub struct CrashGpuProcess {}
pub struct CrashGpuProcessReturnObject {}
// Returns version information.
pub struct GetVersion {}
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
// Returns the command line switches for the browser process if, and only if
// --enable-automation is on the commandline.
pub struct GetBrowserCommandLine {}
pub struct GetBrowserCommandLineReturnObject {
    // Commandline parameters
    pub arguments: Vec<String>,
}
// Get Chrome histograms.
pub struct GetHistograms {
    // Requested substring in name. Only histograms which have query as a
    // substring in their name are extracted. An empty or absent query returns
    // all histograms.
    pub query: Option<String>,
    // If true, retrieve delta since last call.
    pub delta: Option<bool>,
}
pub struct GetHistogramsReturnObject {
    // Histograms.
    pub histograms: Vec<Histogram>,
}
// Get a Chrome histogram by name.
pub struct GetHistogram {
    // Requested histogram name.
    pub name: String,
    // If true, retrieve delta since last call.
    pub delta: Option<bool>,
}
pub struct GetHistogramReturnObject {
    // Histogram.
    pub histogram: Histogram,
}
// Get position and size of the browser window.
pub struct GetWindowBounds {
    // Browser window id.
    pub window_id: WindowID,
}
pub struct GetWindowBoundsReturnObject {
    // Bounds information of the window. When window state is 'minimized', the restored window
    // position and size are returned.
    pub bounds: Bounds,
}
// Get the browser window that contains the devtools target.
pub struct GetWindowForTarget {
    // Devtools agent host id. If called as a part of the session, associated targetId is used.
    pub target_id: Option<super::target::TargetID>,
}
pub struct GetWindowForTargetReturnObject {
    // Browser window id.
    pub window_id: WindowID,
    // Bounds information of the window. When window state is 'minimized', the restored window
    // position and size are returned.
    pub bounds: Bounds,
}
// Set position and/or size of the browser window.
pub struct SetWindowBounds {
    // Browser window id.
    pub window_id: WindowID,
    // New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined
    // with 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged.
    pub bounds: Bounds,
}
pub struct SetWindowBoundsReturnObject {}
// Set dock tile details, platform-specific.
pub struct SetDockTile {
    pub badge_label: Option<String>,
    // Png encoded image.
    pub image: Option<String>,
}
pub struct SetDockTileReturnObject {}
