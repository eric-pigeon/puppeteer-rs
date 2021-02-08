// This file is auto-generated do not edit manually.

// Unique frame identifier.
pub type FrameId = String;
// Indicates whether a frame has been identified as an ad.
pub enum AdFrameType {
    None,
    Child,
    Root,
}
// Indicates whether the frame is a secure context and why it is the case.
pub enum SecureContextType {
    Secure,
    SecureLocalhost,
    InsecureScheme,
    InsecureAncestor,
}
// Indicates whether the frame is cross-origin isolated and why it is the case.
pub enum CrossOriginIsolatedContextType {
    Isolated,
    NotIsolated,
    NotIsolatedFeatureDisabled,
}
// Information about the Frame on the page.
pub struct Frame {
    // Frame unique identifier.
    pub id: FrameId,
    // Parent frame identifier.
    pub parent_id: Option<String>,
    // Identifier of the loader associated with this frame.
    pub loader_id: super::network::LoaderId,
    // Frame's name as specified in the tag.
    pub name: Option<String>,
    // Frame document's URL without fragment.
    pub url: String,
    // Frame document's URL fragment including the '#'.
    pub url_fragment: Option<String>,
    // Frame document's registered domain, taking the public suffixes list into account.
    // Extracted from the Frame's url.
    // Example URLs: http://www.google.com/file.html -> "google.com"
    //               http://a.b.co.uk/file.html      -> "b.co.uk"
    pub domain_and_registry: String,
    // Frame document's security origin.
    pub security_origin: String,
    // Frame document's mimeType as determined by the browser.
    pub mime_type: String,
    // If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.
    pub unreachable_url: Option<String>,
    // Indicates whether this frame was tagged as an ad.
    pub ad_frame_type: Option<AdFrameType>,
    // Indicates whether the main document is a secure context and explains why that is the case.
    pub secure_context_type: SecureContextType,
    // Indicates whether this is a cross origin isolated context.
    pub cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
}
// Information about the Resource on the page.
pub struct FrameResource {
    // Resource URL.
    pub url: String,
    // Type of this resource.
    pub r#type: super::network::ResourceType,
    // Resource mimeType as determined by the browser.
    pub mime_type: String,
    // last-modified timestamp as reported by server.
    pub last_modified: Option<super::network::TimeSinceEpoch>,
    // Resource content size.
    pub content_size: Option<f64>,
    // True if the resource failed to load.
    pub failed: Option<bool>,
    // True if the resource was canceled during loading.
    pub canceled: Option<bool>,
}
// Information about the Frame hierarchy along with their cached resources.
pub struct FrameResourceTree {
    // Frame information for this tree item.
    pub frame: Frame,
    // Child frames.
    pub child_frames: Option<Vec<FrameResourceTree>>,
    // Information about frame resources.
    pub resources: Vec<FrameResource>,
}
// Information about the Frame hierarchy.
pub struct FrameTree {
    // Frame information for this tree item.
    pub frame: Frame,
    // Child frames.
    pub child_frames: Option<Vec<FrameTree>>,
}
// Unique script identifier.
pub type ScriptIdentifier = String;
// Transition type.
pub enum TransitionType {
    Link,
    Typed,
    Addressbar,
    Autobookmark,
    Autosubframe,
    Manualsubframe,
    Generated,
    Autotoplevel,
    Formsubmit,
    Reload,
    Keyword,
    Keywordgenerated,
    Other,
}
// Navigation history entry.
pub struct NavigationEntry {
    // Unique id of the navigation history entry.
    pub id: i32,
    // URL of the navigation history entry.
    pub url: String,
    // URL that the user typed in the url bar.
    pub user_typed_url: String,
    // Title of the navigation history entry.
    pub title: String,
    // Transition type.
    pub transition_type: TransitionType,
}
// Screencast frame metadata.
pub struct ScreencastFrameMetadata {
    // Top offset in DIP.
    pub offset_top: f64,
    // Page scale factor.
    pub page_scale_factor: f64,
    // Device screen width in DIP.
    pub device_width: f64,
    // Device screen height in DIP.
    pub device_height: f64,
    // Position of horizontal scroll in CSS pixels.
    pub scroll_offset_x: f64,
    // Position of vertical scroll in CSS pixels.
    pub scroll_offset_y: f64,
    // Frame swap timestamp.
    pub timestamp: Option<super::network::TimeSinceEpoch>,
}
// Javascript dialog type.
pub enum DialogType {
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}
// Error while paring app manifest.
pub struct AppManifestError {
    // Error message.
    pub message: String,
    // If criticial, this is a non-recoverable parse error.
    pub critical: i32,
    // Error line.
    pub line: i32,
    // Error column.
    pub column: i32,
}
// Parsed app manifest properties.
pub struct AppManifestParsedProperties {
    // Computed scope value
    pub scope: String,
}
// Layout viewport position and dimensions.
pub struct LayoutViewport {
    // Horizontal offset relative to the document (CSS pixels).
    pub page_x: i32,
    // Vertical offset relative to the document (CSS pixels).
    pub page_y: i32,
    // Width (CSS pixels), excludes scrollbar if present.
    pub client_width: i32,
    // Height (CSS pixels), excludes scrollbar if present.
    pub client_height: i32,
}
// Visual viewport position, dimensions, and scale.
pub struct VisualViewport {
    // Horizontal offset relative to the layout viewport (CSS pixels).
    pub offset_x: f64,
    // Vertical offset relative to the layout viewport (CSS pixels).
    pub offset_y: f64,
    // Horizontal offset relative to the document (CSS pixels).
    pub page_x: f64,
    // Vertical offset relative to the document (CSS pixels).
    pub page_y: f64,
    // Width (CSS pixels), excludes scrollbar if present.
    pub client_width: f64,
    // Height (CSS pixels), excludes scrollbar if present.
    pub client_height: f64,
    // Scale relative to the ideal viewport (size at width=device-width).
    pub scale: f64,
    // Page zoom factor (CSS to device independent pixels ratio).
    pub zoom: Option<f64>,
}
// Viewport for capturing screenshot.
pub struct Viewport {
    // X offset in device independent pixels (dip).
    pub x: f64,
    // Y offset in device independent pixels (dip).
    pub y: f64,
    // Rectangle width in device independent pixels (dip).
    pub width: f64,
    // Rectangle height in device independent pixels (dip).
    pub height: f64,
    // Page scale factor.
    pub scale: f64,
}
// Generic font families collection.
pub struct FontFamilies {
    // The standard font-family.
    pub standard: Option<String>,
    // The fixed font-family.
    pub fixed: Option<String>,
    // The serif font-family.
    pub serif: Option<String>,
    // The sansSerif font-family.
    pub sans_serif: Option<String>,
    // The cursive font-family.
    pub cursive: Option<String>,
    // The fantasy font-family.
    pub fantasy: Option<String>,
    // The pictograph font-family.
    pub pictograph: Option<String>,
}
// Default font sizes.
pub struct FontSizes {
    // Default standard font size.
    pub standard: Option<i32>,
    // Default fixed font size.
    pub fixed: Option<i32>,
}
pub enum ClientNavigationReason {
    FormSubmissionGet,
    FormSubmissionPost,
    HttpHeaderRefresh,
    ScriptInitiated,
    MetaTagRefresh,
    PageBlockInterstitial,
    Reload,
    AnchorClick,
}
pub enum ClientNavigationDisposition {
    CurrentTab,
    NewTab,
    NewWindow,
    Download,
}
pub struct InstallabilityErrorArgument {
    // Argument name (e.g. name:'minimum-icon-size-in-pixels').
    pub name: String,
    // Argument value (e.g. value:'64').
    pub value: String,
}
// The installability error
pub struct InstallabilityError {
    // The error id (e.g. 'manifest-missing-suitable-icon').
    pub error_id: String,
    // The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    pub error_arguments: Vec<InstallabilityErrorArgument>,
}
// The referring-policy used for the navigation.
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

// Deprecated, please use addScriptToEvaluateOnNewDocument instead.
pub struct AddScriptToEvaluateOnLoad {
    pub script_source: String,
}
pub struct AddScriptToEvaluateOnLoadReturnObject {
    // Identifier of the added script.
    pub identifier: ScriptIdentifier,
}
// Evaluates given script in every frame upon creation (before loading frame's scripts).
pub struct AddScriptToEvaluateOnNewDocument {
    pub source: String,
    // If specified, creates an isolated world with the given name and evaluates given script in it.
    // This world name will be used as the ExecutionContextDescription::name when the corresponding
    // event is emitted.
    pub world_name: Option<String>,
}
pub struct AddScriptToEvaluateOnNewDocumentReturnObject {
    // Identifier of the added script.
    pub identifier: ScriptIdentifier,
}
// Brings page to front (activates tab).
pub struct BringToFront {}
pub struct BringToFrontReturnObject {}
pub enum Format {
    Jpeg,
    Png,
}
// Capture page screenshot.
pub struct CaptureScreenshot {
    // Image compression format (defaults to png).
    pub format: Option<Format>,
    // Compression quality from range [0..100] (jpeg only).
    pub quality: Option<i32>,
    // Capture the screenshot of a given region only.
    pub clip: Option<Viewport>,
    // Capture the screenshot from the surface, rather than the view. Defaults to true.
    pub from_surface: Option<bool>,
}
pub struct CaptureScreenshotReturnObject {
    // Base64-encoded image data.
    pub data: String,
}
// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
// iframes, shadow DOM, external resources, and element-inline styles.
pub struct CaptureSnapshot {
    // Format (defaults to mhtml).
    pub format: Option<Format>,
}
pub struct CaptureSnapshotReturnObject {
    // Serialized page data.
    pub data: String,
}
// Clears the overriden device metrics.
pub struct ClearDeviceMetricsOverride {}
pub struct ClearDeviceMetricsOverrideReturnObject {}
// Clears the overridden Device Orientation.
pub struct ClearDeviceOrientationOverride {}
pub struct ClearDeviceOrientationOverrideReturnObject {}
// Clears the overriden Geolocation Position and Error.
pub struct ClearGeolocationOverride {}
pub struct ClearGeolocationOverrideReturnObject {}
// Creates an isolated world for the given frame.
pub struct CreateIsolatedWorld {
    // Id of the frame in which the isolated world should be created.
    pub frame_id: FrameId,
    // An optional name which is reported in the Execution Context.
    pub world_name: Option<String>,
    // Whether or not universal access should be granted to the isolated world. This is a powerful
    // option, use with caution.
    pub grant_univeral_access: Option<bool>,
}
pub struct CreateIsolatedWorldReturnObject {
    // Execution context of the isolated world.
    pub execution_context_id: super::runtime::ExecutionContextId,
}
// Deletes browser cookie with given name, domain and path.
pub struct DeleteCookie {
    // Name of the cookie to remove.
    pub cookie_name: String,
    // URL to match cooke domain and path.
    pub url: String,
}
pub struct DeleteCookieReturnObject {}
// Disables page domain notifications.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables page domain notifications.
pub struct Enable {}
pub struct EnableReturnObject {}
pub struct GetAppManifest {}
pub struct GetAppManifestReturnObject {
    // Manifest location.
    pub url: String,
    pub errors: Vec<AppManifestError>,
    // Manifest content.
    pub data: Option<String>,
    // Parsed manifest properties
    pub parsed: Option<AppManifestParsedProperties>,
}
pub struct GetInstallabilityErrors {}
pub struct GetInstallabilityErrorsReturnObject {
    pub installability_errors: Vec<InstallabilityError>,
}
pub struct GetManifestIcons {}
pub struct GetManifestIconsReturnObject {
    pub primary_icon: Option<String>,
}
// Returns all browser cookies. Depending on the backend support, will return detailed cookie
// information in the `cookies` field.
pub struct GetCookies {}
pub struct GetCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<super::network::Cookie>,
}
// Returns present frame tree structure.
pub struct GetFrameTree {}
pub struct GetFrameTreeReturnObject {
    // Present frame tree structure.
    pub frame_tree: FrameTree,
}
// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.
pub struct GetLayoutMetrics {}
pub struct GetLayoutMetricsReturnObject {
    // Metrics relating to the layout viewport.
    pub layout_viewport: LayoutViewport,
    // Metrics relating to the visual viewport.
    pub visual_viewport: VisualViewport,
    // Size of scrollable area.
    pub content_size: super::dom::Rect,
}
// Returns navigation history for the current page.
pub struct GetNavigationHistory {}
pub struct GetNavigationHistoryReturnObject {
    // Index of the current navigation history entry.
    pub current_index: i32,
    // Array of navigation history entries.
    pub entries: Vec<NavigationEntry>,
}
// Resets navigation history for the current page.
pub struct ResetNavigationHistory {}
pub struct ResetNavigationHistoryReturnObject {}
// Returns content of the given resource.
pub struct GetResourceContent {
    // Frame id to get resource for.
    pub frame_id: FrameId,
    // URL of the resource to get content for.
    pub url: String,
}
pub struct GetResourceContentReturnObject {
    // Resource content.
    pub content: String,
    // True, if content was served as base64.
    pub base64_encoded: bool,
}
// Returns present frame / resource tree structure.
pub struct GetResourceTree {}
pub struct GetResourceTreeReturnObject {
    // Present frame / resource tree structure.
    pub frame_tree: FrameResourceTree,
}
// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).
pub struct HandleJavaScriptDialog {
    // Whether to accept or dismiss the dialog.
    pub accept: bool,
    // The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    // dialog.
    pub prompt_text: Option<String>,
}
pub struct HandleJavaScriptDialogReturnObject {}
// Navigates current page to the given URL.
pub struct Navigate {
    // URL to navigate the page to.
    pub url: String,
    // Referrer URL.
    pub referrer: Option<String>,
    // Intended transition type.
    pub transition_type: Option<TransitionType>,
    // Frame id to navigate, if not specified navigates the top frame.
    pub frame_id: Option<FrameId>,
    // Referrer-policy used for the navigation.
    pub referrer_policy: Option<ReferrerPolicy>,
}
pub struct NavigateReturnObject {
    // Frame id that has navigated (or failed to navigate)
    pub frame_id: FrameId,
    // Loader identifier.
    pub loader_id: Option<super::network::LoaderId>,
    // User friendly error message, present if and only if navigation has failed.
    pub error_text: Option<String>,
}
// Navigates current page to the given history entry.
pub struct NavigateToHistoryEntry {
    // Unique id of the entry to navigate to.
    pub entry_id: i32,
}
pub struct NavigateToHistoryEntryReturnObject {}
pub enum TransferMode {
    ReturnAsBase64,
    ReturnAsStream,
}
// Print page as PDF.
pub struct PrintToPDF {
    // Paper orientation. Defaults to false.
    pub landscape: Option<bool>,
    // Display header and footer. Defaults to false.
    pub display_header_footer: Option<bool>,
    // Print background graphics. Defaults to false.
    pub print_background: Option<bool>,
    // Scale of the webpage rendering. Defaults to 1.
    pub scale: Option<f64>,
    // Paper width in inches. Defaults to 8.5 inches.
    pub paper_width: Option<f64>,
    // Paper height in inches. Defaults to 11 inches.
    pub paper_height: Option<f64>,
    // Top margin in inches. Defaults to 1cm (~0.4 inches).
    pub margin_top: Option<f64>,
    // Bottom margin in inches. Defaults to 1cm (~0.4 inches).
    pub margin_bottom: Option<f64>,
    // Left margin in inches. Defaults to 1cm (~0.4 inches).
    pub margin_left: Option<f64>,
    // Right margin in inches. Defaults to 1cm (~0.4 inches).
    pub margin_right: Option<f64>,
    // Paper ranges to print, e.g., '1-5, 8, 11-13'. Defaults to the empty string, which means
    // print all pages.
    pub page_ranges: Option<String>,
    // Whether to silently ignore invalid but successfully parsed page ranges, such as '3-2'.
    // Defaults to false.
    pub ignore_invalid_page_ranges: Option<bool>,
    // HTML template for the print header. Should be valid HTML markup with following
    // classes used to inject printing values into them:
    // - `date`: formatted print date
    // - `title`: document title
    // - `url`: document location
    // - `pageNumber`: current page number
    // - `totalPages`: total pages in the document
    //
    // For example, `<span class=title></span>` would generate span containing the title.
    pub header_template: Option<String>,
    // HTML template for the print footer. Should use the same format as the `headerTemplate`.
    pub footer_template: Option<String>,
    // Whether or not to prefer page size as defined by css. Defaults to false,
    // in which case the content will be scaled to fit the paper size.
    pub prefer_css_page_size: Option<bool>,
    // return as stream
    pub transfer_mode: Option<TransferMode>,
}
pub struct PrintToPDFReturnObject {
    // Base64-encoded pdf data. Empty if |returnAsStream| is specified.
    pub data: String,
    // A handle of the stream that holds resulting PDF data.
    pub stream: Option<super::io::StreamHandle>,
}
// Reloads given page optionally ignoring the cache.
pub struct Reload {
    // If true, browser cache is ignored (as if the user pressed Shift+refresh).
    pub ignore_cache: Option<bool>,
    // If set, the script will be injected into all frames of the inspected page after reload.
    // Argument will be ignored if reloading dataURL origin.
    pub script_to_evaluate_on_load: Option<String>,
}
pub struct ReloadReturnObject {}
// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.
pub struct RemoveScriptToEvaluateOnLoad {
    pub identifier: ScriptIdentifier,
}
pub struct RemoveScriptToEvaluateOnLoadReturnObject {}
// Removes given script from the list.
pub struct RemoveScriptToEvaluateOnNewDocument {
    pub identifier: ScriptIdentifier,
}
pub struct RemoveScriptToEvaluateOnNewDocumentReturnObject {}
// Acknowledges that a screencast frame has been received by the frontend.
pub struct ScreencastFrameAck {
    // Frame number.
    pub session_id: i32,
}
pub struct ScreencastFrameAckReturnObject {}
// Searches for given string in resource content.
pub struct SearchInResource {
    // Frame id for resource to search in.
    pub frame_id: FrameId,
    // URL of the resource to search in.
    pub url: String,
    // String to search for.
    pub query: String,
    // If true, search is case sensitive.
    pub case_sensitive: Option<bool>,
    // If true, treats string parameter as regex.
    pub is_regex: Option<bool>,
}
pub struct SearchInResourceReturnObject {
    // List of search matches.
    pub result: Vec<super::debugger::SearchMatch>,
}
// Enable Chrome's experimental ad filter on all sites.
pub struct SetAdBlockingEnabled {
    // Whether to block ads.
    pub enabled: bool,
}
pub struct SetAdBlockingEnabledReturnObject {}
// Enable page Content Security Policy by-passing.
pub struct SetBypassCSP {
    // Whether to bypass page CSP.
    pub enabled: bool,
}
pub struct SetBypassCSPReturnObject {}
// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
// query results).
pub struct SetDeviceMetricsOverride {
    // Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub width: i32,
    // Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.
    pub height: i32,
    // Overriding device scale factor value. 0 disables the override.
    pub device_scale_factor: f64,
    // Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    // autosizing and more.
    pub mobile: bool,
    // Scale to apply to resulting view image.
    pub scale: Option<f64>,
    // Overriding screen width value in pixels (minimum 0, maximum 10000000).
    pub screen_width: Option<i32>,
    // Overriding screen height value in pixels (minimum 0, maximum 10000000).
    pub screen_height: Option<i32>,
    // Overriding view X position on screen in pixels (minimum 0, maximum 10000000).
    pub position_x: Option<i32>,
    // Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).
    pub position_y: Option<i32>,
    // Do not set visible view size, rely upon explicit setVisibleSize call.
    pub dont_set_visible_size: Option<bool>,
    // Screen orientation override.
    pub screen_orientation: Option<super::emulation::ScreenOrientation>,
    // The viewport dimensions and scale. If not set, the override is cleared.
    pub viewport: Option<Viewport>,
}
pub struct SetDeviceMetricsOverrideReturnObject {}
// Overrides the Device Orientation.
pub struct SetDeviceOrientationOverride {
    // Mock alpha
    pub alpha: f64,
    // Mock beta
    pub beta: f64,
    // Mock gamma
    pub gamma: f64,
}
pub struct SetDeviceOrientationOverrideReturnObject {}
// Set generic font families.
pub struct SetFontFamilies {
    // Specifies font families to set. If a font family is not specified, it won't be changed.
    pub font_families: FontFamilies,
}
pub struct SetFontFamiliesReturnObject {}
// Set default font sizes.
pub struct SetFontSizes {
    // Specifies font sizes to set. If a font size is not specified, it won't be changed.
    pub font_sizes: FontSizes,
}
pub struct SetFontSizesReturnObject {}
// Sets given markup as the document's HTML.
pub struct SetDocumentContent {
    // Frame id to set HTML for.
    pub frame_id: FrameId,
    // HTML content to set.
    pub html: String,
}
pub struct SetDocumentContentReturnObject {}
pub enum Behavior {
    Deny,
    Allow,
    Default,
}
// Set the behavior when downloading a file.
pub struct SetDownloadBehavior {
    // Whether to allow all or deny all download requests, or use default Chrome behavior if
    // available (otherwise deny).
    pub behavior: Behavior,
    // The default path to save downloaded files to. This is requred if behavior is set to 'allow'
    pub download_path: Option<String>,
}
pub struct SetDownloadBehaviorReturnObject {}
// Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
// unavailable.
pub struct SetGeolocationOverride {
    // Mock latitude
    pub latitude: Option<f64>,
    // Mock longitude
    pub longitude: Option<f64>,
    // Mock accuracy
    pub accuracy: Option<f64>,
}
pub struct SetGeolocationOverrideReturnObject {}
// Controls whether page will emit lifecycle events.
pub struct SetLifecycleEventsEnabled {
    // If true, starts emitting lifecycle events.
    pub enabled: bool,
}
pub struct SetLifecycleEventsEnabledReturnObject {}
pub enum Configuration {
    Mobile,
    Desktop,
}
// Toggles mouse event-based touch event emulation.
pub struct SetTouchEmulationEnabled {
    // Whether the touch event emulation should be enabled.
    pub enabled: bool,
    // Touch/gesture events configuration. Default: current platform.
    pub configuration: Option<Configuration>,
}
pub struct SetTouchEmulationEnabledReturnObject {}
// Starts sending each frame using the `screencastFrame` event.
pub struct StartScreencast {
    // Image compression format.
    pub format: Option<Format>,
    // Compression quality from range [0..100].
    pub quality: Option<i32>,
    // Maximum screenshot width.
    pub max_width: Option<i32>,
    // Maximum screenshot height.
    pub max_height: Option<i32>,
    // Send every n-th frame.
    pub every_nth_frame: Option<i32>,
}
pub struct StartScreencastReturnObject {}
// Force the page stop all navigations and pending resource fetches.
pub struct StopLoading {}
pub struct StopLoadingReturnObject {}
// Crashes renderer on the IO thread, generates minidumps.
pub struct Crash {}
pub struct CrashReturnObject {}
// Tries to close page, running its beforeunload hooks, if any.
pub struct Close {}
pub struct CloseReturnObject {}
pub enum State {
    Frozen,
    Active,
}
// Tries to update the web lifecycle state of the page.
// It will transition the page to the given state according to:
// https://github.com/WICG/web-lifecycle/
pub struct SetWebLifecycleState {
    // Target lifecycle state
    pub state: State,
}
pub struct SetWebLifecycleStateReturnObject {}
// Stops sending each frame in the `screencastFrame`.
pub struct StopScreencast {}
pub struct StopScreencastReturnObject {}
// Forces compilation cache to be generated for every subresource script.
pub struct SetProduceCompilationCache {
    pub enabled: bool,
}
pub struct SetProduceCompilationCacheReturnObject {}
// Seeds compilation cache for given url. Compilation cache does not survive
// cross-process navigation.
pub struct AddCompilationCache {
    pub url: String,
    // Base64-encoded data
    pub data: String,
}
pub struct AddCompilationCacheReturnObject {}
// Clears seeded compilation cache.
pub struct ClearCompilationCache {}
pub struct ClearCompilationCacheReturnObject {}
// Generates a report for testing.
pub struct GenerateTestReport {
    // Message to be displayed in the report.
    pub message: String,
    // Specifies the endpoint group to deliver the report to.
    pub group: Option<String>,
}
pub struct GenerateTestReportReturnObject {}
// Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.
pub struct WaitForDebugger {}
pub struct WaitForDebuggerReturnObject {}
// Intercept file chooser requests and transfer control to protocol clients.
// When file chooser interception is enabled, native file chooser dialog is not shown.
// Instead, a protocol event `Page.fileChooserOpened` is emitted.
pub struct SetInterceptFileChooserDialog {
    pub enabled: bool,
}
pub struct SetInterceptFileChooserDialogReturnObject {}
