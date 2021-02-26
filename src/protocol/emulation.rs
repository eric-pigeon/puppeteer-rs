// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScreenOrientationType {
    PortraitPrimary,
    PortraitSecondary,
    LandscapePrimary,
    LandscapeSecondary,
}
// Screen orientation.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenOrientation {
    // Orientation type.
    pub r#type: String,
    // Orientation angle.
    pub angle: i32,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DisplayFeatureOrientation {
    Vertical,
    Horizontal,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisplayFeature {
    // Orientation of a display feature in relation to screen
    pub orientation: String,
    // The offset from the screen origin in either the x (for vertical
    // orientation) or y (for horizontal orientation) direction.
    pub offset: i32,
    // A display feature may mask content such that it is not physically
    // displayed - this length along with the offset describes this area.
    // A display feature that only splits content will have a 0 mask_length.
    pub mask_length: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MediaFeature {
    pub name: String,
    pub value: String,
}
// advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
// allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
// pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
// resource fetches.
pub type VirtualTimePolicy = String;
// Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentBrandVersion {
    pub brand: String,
    pub version: String,
}
// Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserAgentMetadata {
    pub brands: Vec<UserAgentBrandVersion>,
    pub full_version: String,
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub model: String,
    pub mobile: bool,
}

// Tells whether emulation is supported.
#[derive(Serialize, Debug)]
pub struct CanEmulate {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateReturnObject {
    // True if emulation is supported.
    pub result: bool,
}
impl super::Command for CanEmulate {
    const NAME: &'static str = "Emulation.canEmulate";

    type ReturnObject = CanEmulateReturnObject;
}
// Clears the overriden device metrics.
#[derive(Serialize, Debug)]
pub struct ClearDeviceMetricsOverride {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceMetricsOverrideReturnObject {}
impl super::Command for ClearDeviceMetricsOverride {
    const NAME: &'static str = "Emulation.clearDeviceMetricsOverride";

    type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
}
// Clears the overriden Geolocation Position and Error.
#[derive(Serialize, Debug)]
pub struct ClearGeolocationOverride {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearGeolocationOverrideReturnObject {}
impl super::Command for ClearGeolocationOverride {
    const NAME: &'static str = "Emulation.clearGeolocationOverride";

    type ReturnObject = ClearGeolocationOverrideReturnObject;
}
// Requests that page scale factor is reset to initial values.
#[derive(Serialize, Debug)]
pub struct ResetPageScaleFactor {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResetPageScaleFactorReturnObject {}
impl super::Command for ResetPageScaleFactor {
    const NAME: &'static str = "Emulation.resetPageScaleFactor";

    type ReturnObject = ResetPageScaleFactorReturnObject;
}
// Enables or disables simulating a focused and active page.
#[derive(Serialize, Debug)]
pub struct SetFocusEmulationEnabled {
    // Whether to enable to disable focus emulation.
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetFocusEmulationEnabledReturnObject {}
impl super::Command for SetFocusEmulationEnabled {
    const NAME: &'static str = "Emulation.setFocusEmulationEnabled";

    type ReturnObject = SetFocusEmulationEnabledReturnObject;
}
// Enables CPU throttling to emulate slow CPUs.
#[derive(Serialize, Debug)]
pub struct SetCPUThrottlingRate {
    // Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).
    pub rate: f64,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCPUThrottlingRateReturnObject {}
impl super::Command for SetCPUThrottlingRate {
    const NAME: &'static str = "Emulation.setCPUThrottlingRate";

    type ReturnObject = SetCPUThrottlingRateReturnObject;
}
// Sets or clears an override of the default background color of the frame. This override is used
// if the content does not specify one.
#[derive(Serialize, Debug)]
pub struct SetDefaultBackgroundColorOverride {
    // RGBA of the default background color. If not specified, any existing override will be
    // cleared.
    pub color: Option<super::dom::RGBA>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultBackgroundColorOverrideReturnObject {}
impl super::Command for SetDefaultBackgroundColorOverride {
    const NAME: &'static str = "Emulation.setDefaultBackgroundColorOverride";

    type ReturnObject = SetDefaultBackgroundColorOverrideReturnObject;
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
    pub screen_orientation: Option<ScreenOrientation>,
    // If set, the visible area of the page will be overridden to this viewport. This viewport
    // change is not observed by the page, e.g. viewport-relative elements do not change positions.
    pub viewport: Option<super::page::Viewport>,
    // If set, the display feature of a multi-segment screen. If not set, multi-segment support
    // is turned-off.
    pub display_feature: Option<DisplayFeature>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideReturnObject {}
impl super::Command for SetDeviceMetricsOverride {
    const NAME: &'static str = "Emulation.setDeviceMetricsOverride";

    type ReturnObject = SetDeviceMetricsOverrideReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SetScrollbarsHidden {
    // Whether scrollbars should be always hidden.
    pub hidden: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetScrollbarsHiddenReturnObject {}
impl super::Command for SetScrollbarsHidden {
    const NAME: &'static str = "Emulation.setScrollbarsHidden";

    type ReturnObject = SetScrollbarsHiddenReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SetDocumentCookieDisabled {
    // Whether document.coookie API should be disabled.
    pub disabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentCookieDisabledReturnObject {}
impl super::Command for SetDocumentCookieDisabled {
    const NAME: &'static str = "Emulation.setDocumentCookieDisabled";

    type ReturnObject = SetDocumentCookieDisabledReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Configuration {
    Mobile,
    Desktop,
}
#[derive(Serialize, Debug)]
pub struct SetEmitTouchEventsForMouse {
    // Whether touch emulation based on mouse input should be enabled.
    pub enabled: bool,
    // Touch/gesture events configuration. Default: current platform.
    pub configuration: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetEmitTouchEventsForMouseReturnObject {}
impl super::Command for SetEmitTouchEventsForMouse {
    const NAME: &'static str = "Emulation.setEmitTouchEventsForMouse";

    type ReturnObject = SetEmitTouchEventsForMouseReturnObject;
}
// Emulates the given media type or media feature for CSS media queries.
#[derive(Serialize, Debug)]
pub struct SetEmulatedMedia {
    // Media type to emulate. Empty string disables the override.
    pub media: Option<String>,
    // Media features to emulate.
    pub features: Option<Vec<MediaFeature>>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedMediaReturnObject {}
impl super::Command for SetEmulatedMedia {
    const NAME: &'static str = "Emulation.setEmulatedMedia";

    type ReturnObject = SetEmulatedMediaReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    None,
    Achromatopsia,
    BlurredVision,
    Deuteranopia,
    Protanopia,
    Tritanopia,
}
// Emulates the given vision deficiency.
#[derive(Serialize, Debug)]
pub struct SetEmulatedVisionDeficiency {
    // Vision deficiency to emulate.
    pub r#type: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedVisionDeficiencyReturnObject {}
impl super::Command for SetEmulatedVisionDeficiency {
    const NAME: &'static str = "Emulation.setEmulatedVisionDeficiency";

    type ReturnObject = SetEmulatedVisionDeficiencyReturnObject;
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
#[serde(rename_all = "camelCase")]
pub struct SetGeolocationOverrideReturnObject {}
impl super::Command for SetGeolocationOverride {
    const NAME: &'static str = "Emulation.setGeolocationOverride";

    type ReturnObject = SetGeolocationOverrideReturnObject;
}
// Overrides the Idle state.
#[derive(Serialize, Debug)]
pub struct SetIdleOverride {
    // Mock isUserActive
    pub is_user_active: bool,
    // Mock isScreenUnlocked
    pub is_screen_unlocked: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetIdleOverrideReturnObject {}
impl super::Command for SetIdleOverride {
    const NAME: &'static str = "Emulation.setIdleOverride";

    type ReturnObject = SetIdleOverrideReturnObject;
}
// Clears Idle state overrides.
#[derive(Serialize, Debug)]
pub struct ClearIdleOverride {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearIdleOverrideReturnObject {}
impl super::Command for ClearIdleOverride {
    const NAME: &'static str = "Emulation.clearIdleOverride";

    type ReturnObject = ClearIdleOverrideReturnObject;
}
// Overrides value returned by the javascript navigator object.
#[derive(Serialize, Debug)]
pub struct SetNavigatorOverrides {
    // The platform navigator.platform should return.
    pub platform: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigatorOverridesReturnObject {}
impl super::Command for SetNavigatorOverrides {
    const NAME: &'static str = "Emulation.setNavigatorOverrides";

    type ReturnObject = SetNavigatorOverridesReturnObject;
}
// Sets a specified page scale factor.
#[derive(Serialize, Debug)]
pub struct SetPageScaleFactor {
    // Page scale factor.
    pub page_scale_factor: f64,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetPageScaleFactorReturnObject {}
impl super::Command for SetPageScaleFactor {
    const NAME: &'static str = "Emulation.setPageScaleFactor";

    type ReturnObject = SetPageScaleFactorReturnObject;
}
// Switches script execution in the page.
#[derive(Serialize, Debug)]
pub struct SetScriptExecutionDisabled {
    // Whether script execution should be disabled in the page.
    pub value: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptExecutionDisabledReturnObject {}
impl super::Command for SetScriptExecutionDisabled {
    const NAME: &'static str = "Emulation.setScriptExecutionDisabled";

    type ReturnObject = SetScriptExecutionDisabledReturnObject;
}
// Enables touch on platforms which do not support them.
#[derive(Serialize, Debug)]
pub struct SetTouchEmulationEnabled {
    // Whether the touch event emulation should be enabled.
    pub enabled: bool,
    // Maximum touch points supported. Defaults to one.
    pub max_touch_points: Option<i32>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledReturnObject {}
impl super::Command for SetTouchEmulationEnabled {
    const NAME: &'static str = "Emulation.setTouchEmulationEnabled";

    type ReturnObject = SetTouchEmulationEnabledReturnObject;
}
// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
// the current virtual time policy.  Note this supersedes any previous time budget.
#[derive(Serialize, Debug)]
pub struct SetVirtualTimePolicy {
    pub policy: VirtualTimePolicy,
    // If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    // virtualTimeBudgetExpired event is sent.
    pub budget: Option<f64>,
    // If set this specifies the maximum number of tasks that can be run before virtual is forced
    // forwards to prevent deadlock.
    pub max_virtual_time_task_starvation_count: Option<i32>,
    // If set the virtual time policy change should be deferred until any frame starts navigating.
    // Note any previous deferred policy change is superseded.
    pub wait_for_navigation: Option<bool>,
    // If set, base::Time::Now will be overriden to initially return this value.
    pub initial_virtual_time: Option<super::network::TimeSinceEpoch>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVirtualTimePolicyReturnObject {
    // Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    pub virtual_time_ticks_base: f64,
}
impl super::Command for SetVirtualTimePolicy {
    const NAME: &'static str = "Emulation.setVirtualTimePolicy";

    type ReturnObject = SetVirtualTimePolicyReturnObject;
}
// Overrides default host system locale with the specified one.
#[derive(Serialize, Debug)]
pub struct SetLocaleOverride {
    // ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    // restores default host system locale.
    pub locale: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetLocaleOverrideReturnObject {}
impl super::Command for SetLocaleOverride {
    const NAME: &'static str = "Emulation.setLocaleOverride";

    type ReturnObject = SetLocaleOverrideReturnObject;
}
// Overrides default host system timezone with the specified one.
#[derive(Serialize, Debug)]
pub struct SetTimezoneOverride {
    // The timezone identifier. If empty, disables the override and
    // restores default host system timezone.
    pub timezone_id: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetTimezoneOverrideReturnObject {}
impl super::Command for SetTimezoneOverride {
    const NAME: &'static str = "Emulation.setTimezoneOverride";

    type ReturnObject = SetTimezoneOverrideReturnObject;
}
// Resizes the frame/viewport of the page. Note that this does not affect the frame's container
// (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
// on Android.
#[derive(Serialize, Debug)]
pub struct SetVisibleSize {
    // Frame width (DIP).
    pub width: i32,
    // Frame height (DIP).
    pub height: i32,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVisibleSizeReturnObject {}
impl super::Command for SetVisibleSize {
    const NAME: &'static str = "Emulation.setVisibleSize";

    type ReturnObject = SetVisibleSizeReturnObject;
}
// Allows overriding user agent with the given string.
#[derive(Serialize, Debug)]
pub struct SetUserAgentOverride {
    // User agent to use.
    pub user_agent: String,
    // Browser langugage to emulate.
    pub accept_language: Option<String>,
    // The platform navigator.platform should return.
    pub platform: Option<String>,
    // To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub user_agent_metadata: Option<UserAgentMetadata>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideReturnObject {}
impl super::Command for SetUserAgentOverride {
    const NAME: &'static str = "Emulation.setUserAgentOverride";

    type ReturnObject = SetUserAgentOverrideReturnObject;
}

// Notification sent after the virtual time budget for the current VirtualTimePolicy has run out.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct VirtualTimeBudgetExpiredEvent {
    pub params: VirtualTimeBudgetExpiredParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VirtualTimeBudgetExpiredParams {}
