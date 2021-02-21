// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Unique frame identifier.
pub type FrameId = String;
// Indicates whether a frame has been identified as an ad.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AdFrameType {
    None,
    Child,
    Root,
}
// Indicates whether the frame is a secure context and why it is the case.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SecureContextType {
    Secure,
    SecureLocalhost,
    InsecureScheme,
    InsecureAncestor,
}
// Indicates whether the frame is cross-origin isolated and why it is the case.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CrossOriginIsolatedContextType {
    Isolated,
    NotIsolated,
    NotIsolatedFeatureDisabled,
}
// Information about the Frame on the page.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameResourceTree {
    // Frame information for this tree item.
    pub frame: Frame,
    // Child frames.
    pub child_frames: Option<Vec<FrameResourceTree>>,
    // Information about frame resources.
    pub resources: Vec<FrameResource>,
}
// Information about the Frame hierarchy.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree {
    // Frame information for this tree item.
    pub frame: Frame,
    // Child frames.
    pub child_frames: Option<Vec<FrameTree>>,
}
// Unique script identifier.
pub type ScriptIdentifier = String;
// Transition type.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DialogType {
    Alert,
    Confirm,
    Prompt,
    Beforeunload,
}
// Error while paring app manifest.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppManifestParsedProperties {
    // Computed scope value
    pub scope: String,
}
// Layout viewport position and dimensions.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    // Default standard font size.
    pub standard: Option<i32>,
    // Default fixed font size.
    pub fixed: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ClientNavigationDisposition {
    CurrentTab,
    NewTab,
    NewWindow,
    Download,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityErrorArgument {
    // Argument name (e.g. name:'minimum-icon-size-in-pixels').
    pub name: String,
    // Argument value (e.g. value:'64').
    pub value: String,
}
// The installability error
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityError {
    // The error id (e.g. 'manifest-missing-suitable-icon').
    pub error_id: String,
    // The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).
    pub error_arguments: Vec<InstallabilityErrorArgument>,
}
// The referring-policy used for the navigation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Debug)]
pub struct AddScriptToEvaluateOnLoad {
    pub script_source: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AddScriptToEvaluateOnLoadReturnObject {
    // Identifier of the added script.
    pub identifier: ScriptIdentifier,
}
impl super::Command for AddScriptToEvaluateOnLoad {
    const NAME: &'static str = "Page.addScriptToEvaluateOnLoad";

    type ReturnObject = AddScriptToEvaluateOnLoadReturnObject;
}
// Evaluates given script in every frame upon creation (before loading frame's scripts).
#[derive(Serialize, Debug)]
pub struct AddScriptToEvaluateOnNewDocument {
    pub source: String,
    // If specified, creates an isolated world with the given name and evaluates given script in it.
    // This world name will be used as the ExecutionContextDescription::name when the corresponding
    // event is emitted.
    pub world_name: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AddScriptToEvaluateOnNewDocumentReturnObject {
    // Identifier of the added script.
    pub identifier: ScriptIdentifier,
}
impl super::Command for AddScriptToEvaluateOnNewDocument {
    const NAME: &'static str = "Page.addScriptToEvaluateOnNewDocument";

    type ReturnObject = AddScriptToEvaluateOnNewDocumentReturnObject;
}
// Brings page to front (activates tab).
#[derive(Serialize, Debug)]
pub struct BringToFront {}
#[derive(Deserialize, Debug, Clone)]
pub struct BringToFrontReturnObject {}
impl super::Command for BringToFront {
    const NAME: &'static str = "Page.bringToFront";

    type ReturnObject = BringToFrontReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    Jpeg,
    Png,
}
// Capture page screenshot.
#[derive(Serialize, Debug)]
pub struct CaptureScreenshot {
    // Image compression format (defaults to png).
    pub format: Option<String>,
    // Compression quality from range [0..100] (jpeg only).
    pub quality: Option<i32>,
    // Capture the screenshot of a given region only.
    pub clip: Option<Viewport>,
    // Capture the screenshot from the surface, rather than the view. Defaults to true.
    pub from_surface: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CaptureScreenshotReturnObject {
    // Base64-encoded image data.
    pub data: String,
}
impl super::Command for CaptureScreenshot {
    const NAME: &'static str = "Page.captureScreenshot";

    type ReturnObject = CaptureScreenshotReturnObject;
}
// Returns a snapshot of the page as a string. For MHTML format, the serialization includes
// iframes, shadow DOM, external resources, and element-inline styles.
#[derive(Serialize, Debug)]
pub struct CaptureSnapshot {
    // Format (defaults to mhtml).
    pub format: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CaptureSnapshotReturnObject {
    // Serialized page data.
    pub data: String,
}
impl super::Command for CaptureSnapshot {
    const NAME: &'static str = "Page.captureSnapshot";

    type ReturnObject = CaptureSnapshotReturnObject;
}
// Clears the overriden device metrics.
#[derive(Serialize, Debug)]
pub struct ClearDeviceMetricsOverride {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearDeviceMetricsOverrideReturnObject {}
impl super::Command for ClearDeviceMetricsOverride {
    const NAME: &'static str = "Page.clearDeviceMetricsOverride";

    type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
}
// Clears the overridden Device Orientation.
#[derive(Serialize, Debug)]
pub struct ClearDeviceOrientationOverride {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearDeviceOrientationOverrideReturnObject {}
impl super::Command for ClearDeviceOrientationOverride {
    const NAME: &'static str = "Page.clearDeviceOrientationOverride";

    type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
}
// Clears the overriden Geolocation Position and Error.
#[derive(Serialize, Debug)]
pub struct ClearGeolocationOverride {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearGeolocationOverrideReturnObject {}
impl super::Command for ClearGeolocationOverride {
    const NAME: &'static str = "Page.clearGeolocationOverride";

    type ReturnObject = ClearGeolocationOverrideReturnObject;
}
// Creates an isolated world for the given frame.
#[derive(Serialize, Debug)]
pub struct CreateIsolatedWorld {
    // Id of the frame in which the isolated world should be created.
    pub frame_id: FrameId,
    // An optional name which is reported in the Execution Context.
    pub world_name: Option<String>,
    // Whether or not universal access should be granted to the isolated world. This is a powerful
    // option, use with caution.
    pub grant_univeral_access: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CreateIsolatedWorldReturnObject {
    // Execution context of the isolated world.
    pub execution_context_id: super::runtime::ExecutionContextId,
}
impl super::Command for CreateIsolatedWorld {
    const NAME: &'static str = "Page.createIsolatedWorld";

    type ReturnObject = CreateIsolatedWorldReturnObject;
}
// Deletes browser cookie with given name, domain and path.
#[derive(Serialize, Debug)]
pub struct DeleteCookie {
    // Name of the cookie to remove.
    pub cookie_name: String,
    // URL to match cooke domain and path.
    pub url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DeleteCookieReturnObject {}
impl super::Command for DeleteCookie {
    const NAME: &'static str = "Page.deleteCookie";

    type ReturnObject = DeleteCookieReturnObject;
}
// Disables page domain notifications.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Page.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables page domain notifications.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Page.enable";

    type ReturnObject = EnableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetAppManifest {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetAppManifestReturnObject {
    // Manifest location.
    pub url: String,
    pub errors: Vec<AppManifestError>,
    // Manifest content.
    pub data: Option<String>,
    // Parsed manifest properties
    pub parsed: Option<AppManifestParsedProperties>,
}
impl super::Command for GetAppManifest {
    const NAME: &'static str = "Page.getAppManifest";

    type ReturnObject = GetAppManifestReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetInstallabilityErrors {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetInstallabilityErrorsReturnObject {
    pub installability_errors: Vec<InstallabilityError>,
}
impl super::Command for GetInstallabilityErrors {
    const NAME: &'static str = "Page.getInstallabilityErrors";

    type ReturnObject = GetInstallabilityErrorsReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetManifestIcons {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetManifestIconsReturnObject {
    pub primary_icon: Option<String>,
}
impl super::Command for GetManifestIcons {
    const NAME: &'static str = "Page.getManifestIcons";

    type ReturnObject = GetManifestIconsReturnObject;
}
// Returns all browser cookies. Depending on the backend support, will return detailed cookie
// information in the `cookies` field.
#[derive(Serialize, Debug)]
pub struct GetCookies {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<super::network::Cookie>,
}
impl super::Command for GetCookies {
    const NAME: &'static str = "Page.getCookies";

    type ReturnObject = GetCookiesReturnObject;
}
// Returns present frame tree structure.
#[derive(Serialize, Debug)]
pub struct GetFrameTree {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetFrameTreeReturnObject {
    // Present frame tree structure.
    pub frame_tree: FrameTree,
}
impl super::Command for GetFrameTree {
    const NAME: &'static str = "Page.getFrameTree";

    type ReturnObject = GetFrameTreeReturnObject;
}
// Returns metrics relating to the layouting of the page, such as viewport bounds/scale.
#[derive(Serialize, Debug)]
pub struct GetLayoutMetrics {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetLayoutMetricsReturnObject {
    // Metrics relating to the layout viewport.
    pub layout_viewport: LayoutViewport,
    // Metrics relating to the visual viewport.
    pub visual_viewport: VisualViewport,
    // Size of scrollable area.
    pub content_size: super::dom::Rect,
}
impl super::Command for GetLayoutMetrics {
    const NAME: &'static str = "Page.getLayoutMetrics";

    type ReturnObject = GetLayoutMetricsReturnObject;
}
// Returns navigation history for the current page.
#[derive(Serialize, Debug)]
pub struct GetNavigationHistory {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetNavigationHistoryReturnObject {
    // Index of the current navigation history entry.
    pub current_index: i32,
    // Array of navigation history entries.
    pub entries: Vec<NavigationEntry>,
}
impl super::Command for GetNavigationHistory {
    const NAME: &'static str = "Page.getNavigationHistory";

    type ReturnObject = GetNavigationHistoryReturnObject;
}
// Resets navigation history for the current page.
#[derive(Serialize, Debug)]
pub struct ResetNavigationHistory {}
#[derive(Deserialize, Debug, Clone)]
pub struct ResetNavigationHistoryReturnObject {}
impl super::Command for ResetNavigationHistory {
    const NAME: &'static str = "Page.resetNavigationHistory";

    type ReturnObject = ResetNavigationHistoryReturnObject;
}
// Returns content of the given resource.
#[derive(Serialize, Debug)]
pub struct GetResourceContent {
    // Frame id to get resource for.
    pub frame_id: FrameId,
    // URL of the resource to get content for.
    pub url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetResourceContentReturnObject {
    // Resource content.
    pub content: String,
    // True, if content was served as base64.
    pub base64_encoded: bool,
}
impl super::Command for GetResourceContent {
    const NAME: &'static str = "Page.getResourceContent";

    type ReturnObject = GetResourceContentReturnObject;
}
// Returns present frame / resource tree structure.
#[derive(Serialize, Debug)]
pub struct GetResourceTree {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetResourceTreeReturnObject {
    // Present frame / resource tree structure.
    pub frame_tree: FrameResourceTree,
}
impl super::Command for GetResourceTree {
    const NAME: &'static str = "Page.getResourceTree";

    type ReturnObject = GetResourceTreeReturnObject;
}
// Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).
#[derive(Serialize, Debug)]
pub struct HandleJavaScriptDialog {
    // Whether to accept or dismiss the dialog.
    pub accept: bool,
    // The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    // dialog.
    pub prompt_text: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct HandleJavaScriptDialogReturnObject {}
impl super::Command for HandleJavaScriptDialog {
    const NAME: &'static str = "Page.handleJavaScriptDialog";

    type ReturnObject = HandleJavaScriptDialogReturnObject;
}
// Navigates current page to the given URL.
#[derive(Serialize, Debug)]
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
#[derive(Deserialize, Debug, Clone)]
pub struct NavigateReturnObject {
    // Frame id that has navigated (or failed to navigate)
    pub frame_id: FrameId,
    // Loader identifier.
    pub loader_id: Option<super::network::LoaderId>,
    // User friendly error message, present if and only if navigation has failed.
    pub error_text: Option<String>,
}
impl super::Command for Navigate {
    const NAME: &'static str = "Page.navigate";

    type ReturnObject = NavigateReturnObject;
}
// Navigates current page to the given history entry.
#[derive(Serialize, Debug)]
pub struct NavigateToHistoryEntry {
    // Unique id of the entry to navigate to.
    pub entry_id: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct NavigateToHistoryEntryReturnObject {}
impl super::Command for NavigateToHistoryEntry {
    const NAME: &'static str = "Page.navigateToHistoryEntry";

    type ReturnObject = NavigateToHistoryEntryReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TransferMode {
    ReturnAsBase64,
    ReturnAsStream,
}
// Print page as PDF.
#[derive(Serialize, Debug)]
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
    pub transfer_mode: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PrintToPDFReturnObject {
    // Base64-encoded pdf data. Empty if |returnAsStream| is specified.
    pub data: String,
    // A handle of the stream that holds resulting PDF data.
    pub stream: Option<super::io::StreamHandle>,
}
impl super::Command for PrintToPDF {
    const NAME: &'static str = "Page.printToPDF";

    type ReturnObject = PrintToPDFReturnObject;
}
// Reloads given page optionally ignoring the cache.
#[derive(Serialize, Debug)]
pub struct Reload {
    // If true, browser cache is ignored (as if the user pressed Shift+refresh).
    pub ignore_cache: Option<bool>,
    // If set, the script will be injected into all frames of the inspected page after reload.
    // Argument will be ignored if reloading dataURL origin.
    pub script_to_evaluate_on_load: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ReloadReturnObject {}
impl super::Command for Reload {
    const NAME: &'static str = "Page.reload";

    type ReturnObject = ReloadReturnObject;
}
// Deprecated, please use removeScriptToEvaluateOnNewDocument instead.
#[derive(Serialize, Debug)]
pub struct RemoveScriptToEvaluateOnLoad {
    pub identifier: ScriptIdentifier,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RemoveScriptToEvaluateOnLoadReturnObject {}
impl super::Command for RemoveScriptToEvaluateOnLoad {
    const NAME: &'static str = "Page.removeScriptToEvaluateOnLoad";

    type ReturnObject = RemoveScriptToEvaluateOnLoadReturnObject;
}
// Removes given script from the list.
#[derive(Serialize, Debug)]
pub struct RemoveScriptToEvaluateOnNewDocument {
    pub identifier: ScriptIdentifier,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RemoveScriptToEvaluateOnNewDocumentReturnObject {}
impl super::Command for RemoveScriptToEvaluateOnNewDocument {
    const NAME: &'static str = "Page.removeScriptToEvaluateOnNewDocument";

    type ReturnObject = RemoveScriptToEvaluateOnNewDocumentReturnObject;
}
// Acknowledges that a screencast frame has been received by the frontend.
#[derive(Serialize, Debug)]
pub struct ScreencastFrameAck {
    // Frame number.
    pub session_id: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ScreencastFrameAckReturnObject {}
impl super::Command for ScreencastFrameAck {
    const NAME: &'static str = "Page.screencastFrameAck";

    type ReturnObject = ScreencastFrameAckReturnObject;
}
// Searches for given string in resource content.
#[derive(Serialize, Debug)]
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
#[derive(Deserialize, Debug, Clone)]
pub struct SearchInResourceReturnObject {
    // List of search matches.
    pub result: Vec<super::debugger::SearchMatch>,
}
impl super::Command for SearchInResource {
    const NAME: &'static str = "Page.searchInResource";

    type ReturnObject = SearchInResourceReturnObject;
}
// Enable Chrome's experimental ad filter on all sites.
#[derive(Serialize, Debug)]
pub struct SetAdBlockingEnabled {
    // Whether to block ads.
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetAdBlockingEnabledReturnObject {}
impl super::Command for SetAdBlockingEnabled {
    const NAME: &'static str = "Page.setAdBlockingEnabled";

    type ReturnObject = SetAdBlockingEnabledReturnObject;
}
// Enable page Content Security Policy by-passing.
#[derive(Serialize, Debug)]
pub struct SetBypassCSP {
    // Whether to bypass page CSP.
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetBypassCSPReturnObject {}
impl super::Command for SetBypassCSP {
    const NAME: &'static str = "Page.setBypassCSP";

    type ReturnObject = SetBypassCSPReturnObject;
}
// Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
// window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
// query results).
#[derive(Serialize, Debug)]
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
#[derive(Deserialize, Debug, Clone)]
pub struct SetDeviceMetricsOverrideReturnObject {}
impl super::Command for SetDeviceMetricsOverride {
    const NAME: &'static str = "Page.setDeviceMetricsOverride";

    type ReturnObject = SetDeviceMetricsOverrideReturnObject;
}
// Overrides the Device Orientation.
#[derive(Serialize, Debug)]
pub struct SetDeviceOrientationOverride {
    // Mock alpha
    pub alpha: f64,
    // Mock beta
    pub beta: f64,
    // Mock gamma
    pub gamma: f64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDeviceOrientationOverrideReturnObject {}
impl super::Command for SetDeviceOrientationOverride {
    const NAME: &'static str = "Page.setDeviceOrientationOverride";

    type ReturnObject = SetDeviceOrientationOverrideReturnObject;
}
// Set generic font families.
#[derive(Serialize, Debug)]
pub struct SetFontFamilies {
    // Specifies font families to set. If a font family is not specified, it won't be changed.
    pub font_families: FontFamilies,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetFontFamiliesReturnObject {}
impl super::Command for SetFontFamilies {
    const NAME: &'static str = "Page.setFontFamilies";

    type ReturnObject = SetFontFamiliesReturnObject;
}
// Set default font sizes.
#[derive(Serialize, Debug)]
pub struct SetFontSizes {
    // Specifies font sizes to set. If a font size is not specified, it won't be changed.
    pub font_sizes: FontSizes,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetFontSizesReturnObject {}
impl super::Command for SetFontSizes {
    const NAME: &'static str = "Page.setFontSizes";

    type ReturnObject = SetFontSizesReturnObject;
}
// Sets given markup as the document's HTML.
#[derive(Serialize, Debug)]
pub struct SetDocumentContent {
    // Frame id to set HTML for.
    pub frame_id: FrameId,
    // HTML content to set.
    pub html: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDocumentContentReturnObject {}
impl super::Command for SetDocumentContent {
    const NAME: &'static str = "Page.setDocumentContent";

    type ReturnObject = SetDocumentContentReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Behavior {
    Deny,
    Allow,
    Default,
}
// Set the behavior when downloading a file.
#[derive(Serialize, Debug)]
pub struct SetDownloadBehavior {
    // Whether to allow all or deny all download requests, or use default Chrome behavior if
    // available (otherwise deny).
    pub behavior: String,
    // The default path to save downloaded files to. This is requred if behavior is set to 'allow'
    pub download_path: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDownloadBehaviorReturnObject {}
impl super::Command for SetDownloadBehavior {
    const NAME: &'static str = "Page.setDownloadBehavior";

    type ReturnObject = SetDownloadBehaviorReturnObject;
}
// Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
// unavailable.
#[derive(Serialize, Debug)]
pub struct SetGeolocationOverride {
    // Mock latitude
    pub latitude: Option<f64>,
    // Mock longitude
    pub longitude: Option<f64>,
    // Mock accuracy
    pub accuracy: Option<f64>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetGeolocationOverrideReturnObject {}
impl super::Command for SetGeolocationOverride {
    const NAME: &'static str = "Page.setGeolocationOverride";

    type ReturnObject = SetGeolocationOverrideReturnObject;
}
// Controls whether page will emit lifecycle events.
#[derive(Serialize, Debug)]
pub struct SetLifecycleEventsEnabled {
    // If true, starts emitting lifecycle events.
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetLifecycleEventsEnabledReturnObject {}
impl super::Command for SetLifecycleEventsEnabled {
    const NAME: &'static str = "Page.setLifecycleEventsEnabled";

    type ReturnObject = SetLifecycleEventsEnabledReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Configuration {
    Mobile,
    Desktop,
}
// Toggles mouse event-based touch event emulation.
#[derive(Serialize, Debug)]
pub struct SetTouchEmulationEnabled {
    // Whether the touch event emulation should be enabled.
    pub enabled: bool,
    // Touch/gesture events configuration. Default: current platform.
    pub configuration: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetTouchEmulationEnabledReturnObject {}
impl super::Command for SetTouchEmulationEnabled {
    const NAME: &'static str = "Page.setTouchEmulationEnabled";

    type ReturnObject = SetTouchEmulationEnabledReturnObject;
}
// Starts sending each frame using the `screencastFrame` event.
#[derive(Serialize, Debug)]
pub struct StartScreencast {
    // Image compression format.
    pub format: Option<String>,
    // Compression quality from range [0..100].
    pub quality: Option<i32>,
    // Maximum screenshot width.
    pub max_width: Option<i32>,
    // Maximum screenshot height.
    pub max_height: Option<i32>,
    // Send every n-th frame.
    pub every_nth_frame: Option<i32>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartScreencastReturnObject {}
impl super::Command for StartScreencast {
    const NAME: &'static str = "Page.startScreencast";

    type ReturnObject = StartScreencastReturnObject;
}
// Force the page stop all navigations and pending resource fetches.
#[derive(Serialize, Debug)]
pub struct StopLoading {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopLoadingReturnObject {}
impl super::Command for StopLoading {
    const NAME: &'static str = "Page.stopLoading";

    type ReturnObject = StopLoadingReturnObject;
}
// Crashes renderer on the IO thread, generates minidumps.
#[derive(Serialize, Debug)]
pub struct Crash {}
#[derive(Deserialize, Debug, Clone)]
pub struct CrashReturnObject {}
impl super::Command for Crash {
    const NAME: &'static str = "Page.crash";

    type ReturnObject = CrashReturnObject;
}
// Tries to close page, running its beforeunload hooks, if any.
#[derive(Serialize, Debug)]
pub struct Close {}
#[derive(Deserialize, Debug, Clone)]
pub struct CloseReturnObject {}
impl super::Command for Close {
    const NAME: &'static str = "Page.close";

    type ReturnObject = CloseReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum State {
    Frozen,
    Active,
}
// Tries to update the web lifecycle state of the page.
// It will transition the page to the given state according to:
// https://github.com/WICG/web-lifecycle/
#[derive(Serialize, Debug)]
pub struct SetWebLifecycleState {
    // Target lifecycle state
    pub state: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetWebLifecycleStateReturnObject {}
impl super::Command for SetWebLifecycleState {
    const NAME: &'static str = "Page.setWebLifecycleState";

    type ReturnObject = SetWebLifecycleStateReturnObject;
}
// Stops sending each frame in the `screencastFrame`.
#[derive(Serialize, Debug)]
pub struct StopScreencast {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopScreencastReturnObject {}
impl super::Command for StopScreencast {
    const NAME: &'static str = "Page.stopScreencast";

    type ReturnObject = StopScreencastReturnObject;
}
// Forces compilation cache to be generated for every subresource script.
#[derive(Serialize, Debug)]
pub struct SetProduceCompilationCache {
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetProduceCompilationCacheReturnObject {}
impl super::Command for SetProduceCompilationCache {
    const NAME: &'static str = "Page.setProduceCompilationCache";

    type ReturnObject = SetProduceCompilationCacheReturnObject;
}
// Seeds compilation cache for given url. Compilation cache does not survive
// cross-process navigation.
#[derive(Serialize, Debug)]
pub struct AddCompilationCache {
    pub url: String,
    // Base64-encoded data
    pub data: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AddCompilationCacheReturnObject {}
impl super::Command for AddCompilationCache {
    const NAME: &'static str = "Page.addCompilationCache";

    type ReturnObject = AddCompilationCacheReturnObject;
}
// Clears seeded compilation cache.
#[derive(Serialize, Debug)]
pub struct ClearCompilationCache {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearCompilationCacheReturnObject {}
impl super::Command for ClearCompilationCache {
    const NAME: &'static str = "Page.clearCompilationCache";

    type ReturnObject = ClearCompilationCacheReturnObject;
}
// Generates a report for testing.
#[derive(Serialize, Debug)]
pub struct GenerateTestReport {
    // Message to be displayed in the report.
    pub message: String,
    // Specifies the endpoint group to deliver the report to.
    pub group: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GenerateTestReportReturnObject {}
impl super::Command for GenerateTestReport {
    const NAME: &'static str = "Page.generateTestReport";

    type ReturnObject = GenerateTestReportReturnObject;
}
// Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.
#[derive(Serialize, Debug)]
pub struct WaitForDebugger {}
#[derive(Deserialize, Debug, Clone)]
pub struct WaitForDebuggerReturnObject {}
impl super::Command for WaitForDebugger {
    const NAME: &'static str = "Page.waitForDebugger";

    type ReturnObject = WaitForDebuggerReturnObject;
}
// Intercept file chooser requests and transfer control to protocol clients.
// When file chooser interception is enabled, native file chooser dialog is not shown.
// Instead, a protocol event `Page.fileChooserOpened` is emitted.
#[derive(Serialize, Debug)]
pub struct SetInterceptFileChooserDialog {
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetInterceptFileChooserDialogReturnObject {}
impl super::Command for SetInterceptFileChooserDialog {
    const NAME: &'static str = "Page.setInterceptFileChooserDialog";

    type ReturnObject = SetInterceptFileChooserDialogReturnObject;
}

#[derive(Deserialize, Debug, Clone)]
pub struct DomContentEventFired {
    pub timestamp: super::network::MonotonicTime,
}
// Emitted only when `page.interceptFileChooser` is enabled.
#[derive(Deserialize, Debug, Clone)]
pub struct FileChooserOpened {
    // Id of the frame containing input node.
    pub frame_id: FrameId,
    // Input node id.
    pub backend_node_id: super::dom::BackendNodeId,
    // Input mode.
    pub mode: String,
}
// Fired when frame has been attached to its parent.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameAttached {
    // Id of the frame that has been attached.
    pub frame_id: FrameId,
    // Parent frame identifier.
    pub parent_frame_id: FrameId,
    // JavaScript stack trace of when frame was attached, only set if frame initiated from script.
    pub stack: Option<super::runtime::StackTrace>,
}
// Fired when frame no longer has a scheduled navigation.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameClearedScheduledNavigation {
    // Id of the frame that has cleared its scheduled navigation.
    pub frame_id: FrameId,
}
// Fired when frame has been detached from its parent.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameDetached {
    // Id of the frame that has been detached.
    pub frame_id: FrameId,
}
// Fired once navigation of the frame has completed. Frame is now associated with the new loader.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameNavigated {
    // Frame object.
    pub frame: Frame,
}
#[derive(Deserialize, Debug, Clone)]
pub struct FrameResized {}
// Fired when a renderer-initiated navigation is requested.
// Navigation may still be cancelled after the event is issued.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameRequestedNavigation {
    // Id of the frame that is being navigated.
    pub frame_id: FrameId,
    // The reason for the navigation.
    pub reason: ClientNavigationReason,
    // The destination URL for the requested navigation.
    pub url: String,
    // The disposition for the navigation.
    pub disposition: ClientNavigationDisposition,
}
// Fired when frame schedules a potential navigation.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameScheduledNavigation {
    // Id of the frame that has scheduled a navigation.
    pub frame_id: FrameId,
    // Delay (in seconds) until the navigation is scheduled to begin. The navigation is not
    // guaranteed to start.
    pub delay: f64,
    // The reason for the navigation.
    pub reason: ClientNavigationReason,
    // The destination URL for the scheduled navigation.
    pub url: String,
}
// Fired when frame has started loading.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameStartedLoading {
    // Id of the frame that has started loading.
    pub frame_id: FrameId,
}
// Fired when frame has stopped loading.
#[derive(Deserialize, Debug, Clone)]
pub struct FrameStoppedLoading {
    // Id of the frame that has stopped loading.
    pub frame_id: FrameId,
}
// Fired when page is about to start a download.
#[derive(Deserialize, Debug, Clone)]
pub struct DownloadWillBegin {
    // Id of the frame that caused download to begin.
    pub frame_id: FrameId,
    // Global unique identifier of the download.
    pub guid: String,
    // URL of the resource being downloaded.
    pub url: String,
    // Suggested file name of the resource (the actual name of the file saved on disk may differ).
    pub suggested_filename: String,
}
// Fired when download makes progress. Last call has |done| == true.
#[derive(Deserialize, Debug, Clone)]
pub struct DownloadProgress {
    // Global unique identifier of the download.
    pub guid: String,
    // Total expected bytes to download.
    pub total_bytes: f64,
    // Total bytes received.
    pub received_bytes: f64,
    // Download status.
    pub state: String,
}
// Fired when interstitial page was hidden
#[derive(Deserialize, Debug, Clone)]
pub struct InterstitialHidden {}
// Fired when interstitial page was shown
#[derive(Deserialize, Debug, Clone)]
pub struct InterstitialShown {}
// Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been
// closed.
#[derive(Deserialize, Debug, Clone)]
pub struct JavascriptDialogClosed {
    // Whether dialog was confirmed.
    pub result: bool,
    // User input in case of prompt.
    pub user_input: String,
}
// Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
// open.
#[derive(Deserialize, Debug, Clone)]
pub struct JavascriptDialogOpening {
    // Frame url.
    pub url: String,
    // Message that will be displayed by the dialog.
    pub message: String,
    // Dialog type.
    pub r#type: DialogType,
    // True iff browser is capable showing or acting on the given dialog. When browser has no
    // dialog handler for given target, calling alert while Page domain is engaged will stall
    // the page execution. Execution can be resumed via calling Page.handleJavaScriptDialog.
    pub has_browser_handler: bool,
    // Default dialog prompt.
    pub default_prompt: Option<String>,
}
// Fired for top level page lifecycle events such as navigation, load, paint, etc.
#[derive(Deserialize, Debug, Clone)]
pub struct LifecycleEvent {
    // Id of the frame.
    pub frame_id: FrameId,
    // Loader identifier. Empty string if the request is fetched from worker.
    pub loader_id: super::network::LoaderId,
    pub name: String,
    pub timestamp: super::network::MonotonicTime,
}
#[derive(Deserialize, Debug, Clone)]
pub struct LoadEventFired {
    pub timestamp: super::network::MonotonicTime,
}
// Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.
#[derive(Deserialize, Debug, Clone)]
pub struct NavigatedWithinDocument {
    // Id of the frame.
    pub frame_id: FrameId,
    // Frame's new url.
    pub url: String,
}
// Compressed image data requested by the `startScreencast`.
#[derive(Deserialize, Debug, Clone)]
pub struct ScreencastFrame {
    // Base64-encoded compressed image.
    pub data: String,
    // Screencast frame metadata.
    pub metadata: ScreencastFrameMetadata,
    // Frame number.
    pub session_id: i32,
}
// Fired when the page with currently enabled screencast was shown or hidden `.
#[derive(Deserialize, Debug, Clone)]
pub struct ScreencastVisibilityChanged {
    // True if the page is visible.
    pub visible: bool,
}
// Fired when a new window is going to be opened, via window.open(), link click, form submission,
// etc.
#[derive(Deserialize, Debug, Clone)]
pub struct WindowOpen {
    // The URL for the new window.
    pub url: String,
    // Window name.
    pub window_name: String,
    // An array of enabled window features.
    pub window_features: Vec<String>,
    // Whether or not it was triggered by user gesture.
    pub user_gesture: bool,
}
// Issued for every compilation cache generated. Is only available
// if Page.setGenerateCompilationCache is enabled.
#[derive(Deserialize, Debug, Clone)]
pub struct CompilationCacheProduced {
    pub url: String,
    // Base64-encoded data
    pub data: String,
}
