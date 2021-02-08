// This file is auto-generated do not edit manually.

pub enum ScreenOrientationType {
    PortraitPrimary,
    PortraitSecondary,
    LandscapePrimary,
    LandscapeSecondary,
}
// Screen orientation.
pub struct ScreenOrientation {
    // Orientation type.
    pub r#type: ScreenOrientationType,
    // Orientation angle.
    pub angle: i32,
}
pub enum DisplayFeatureOrientation {
    Vertical,
    Horizontal,
}
pub struct DisplayFeature {
    // Orientation of a display feature in relation to screen
    pub orientation: DisplayFeatureOrientation,
    // The offset from the screen origin in either the x (for vertical
    // orientation) or y (for horizontal orientation) direction.
    pub offset: i32,
    // A display feature may mask content such that it is not physically
    // displayed - this length along with the offset describes this area.
    // A display feature that only splits content will have a 0 mask_length.
    pub mask_length: i32,
}
pub struct MediaFeature {
    pub name: String,
    pub value: String,
}
// advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
// allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
// pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
// resource fetches.
pub enum VirtualTimePolicy {
    Advance,
    Pause,
    PauseIfNetworkFetchesPending,
}
// Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
pub struct UserAgentBrandVersion {
    pub brand: String,
    pub version: String,
}
// Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
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
pub struct CanEmulate {}
pub struct CanEmulateReturnObject {
    // True if emulation is supported.
    pub result: bool,
}
// Clears the overriden device metrics.
pub struct ClearDeviceMetricsOverride {}
pub struct ClearDeviceMetricsOverrideReturnObject {}
// Clears the overriden Geolocation Position and Error.
pub struct ClearGeolocationOverride {}
pub struct ClearGeolocationOverrideReturnObject {}
// Requests that page scale factor is reset to initial values.
pub struct ResetPageScaleFactor {}
pub struct ResetPageScaleFactorReturnObject {}
// Enables or disables simulating a focused and active page.
pub struct SetFocusEmulationEnabled {
    // Whether to enable to disable focus emulation.
    pub enabled: bool,
}
pub struct SetFocusEmulationEnabledReturnObject {}
// Enables CPU throttling to emulate slow CPUs.
pub struct SetCPUThrottlingRate {
    // Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).
    pub rate: f64,
}
pub struct SetCPUThrottlingRateReturnObject {}
// Sets or clears an override of the default background color of the frame. This override is used
// if the content does not specify one.
pub struct SetDefaultBackgroundColorOverride {
    // RGBA of the default background color. If not specified, any existing override will be
    // cleared.
    pub color: Option<super::dom::RGBA>,
}
pub struct SetDefaultBackgroundColorOverrideReturnObject {}
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
    pub screen_orientation: Option<ScreenOrientation>,
    // If set, the visible area of the page will be overridden to this viewport. This viewport
    // change is not observed by the page, e.g. viewport-relative elements do not change positions.
    pub viewport: Option<super::page::Viewport>,
    // If set, the display feature of a multi-segment screen. If not set, multi-segment support
    // is turned-off.
    pub display_feature: Option<DisplayFeature>,
}
pub struct SetDeviceMetricsOverrideReturnObject {}
pub struct SetScrollbarsHidden {
    // Whether scrollbars should be always hidden.
    pub hidden: bool,
}
pub struct SetScrollbarsHiddenReturnObject {}
pub struct SetDocumentCookieDisabled {
    // Whether document.coookie API should be disabled.
    pub disabled: bool,
}
pub struct SetDocumentCookieDisabledReturnObject {}
pub enum Configuration {
    Mobile,
    Desktop,
}
pub struct SetEmitTouchEventsForMouse {
    // Whether touch emulation based on mouse input should be enabled.
    pub enabled: bool,
    // Touch/gesture events configuration. Default: current platform.
    pub configuration: Option<Configuration>,
}
pub struct SetEmitTouchEventsForMouseReturnObject {}
// Emulates the given media type or media feature for CSS media queries.
pub struct SetEmulatedMedia {
    // Media type to emulate. Empty string disables the override.
    pub media: Option<String>,
    // Media features to emulate.
    pub features: Option<Vec<MediaFeature>>,
}
pub struct SetEmulatedMediaReturnObject {}
pub enum Type {
    None,
    Achromatopsia,
    BlurredVision,
    Deuteranopia,
    Protanopia,
    Tritanopia,
}
// Emulates the given vision deficiency.
pub struct SetEmulatedVisionDeficiency {
    // Vision deficiency to emulate.
    pub r#type: Type,
}
pub struct SetEmulatedVisionDeficiencyReturnObject {}
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
// Overrides the Idle state.
pub struct SetIdleOverride {
    // Mock isUserActive
    pub is_user_active: bool,
    // Mock isScreenUnlocked
    pub is_screen_unlocked: bool,
}
pub struct SetIdleOverrideReturnObject {}
// Clears Idle state overrides.
pub struct ClearIdleOverride {}
pub struct ClearIdleOverrideReturnObject {}
// Overrides value returned by the javascript navigator object.
pub struct SetNavigatorOverrides {
    // The platform navigator.platform should return.
    pub platform: String,
}
pub struct SetNavigatorOverridesReturnObject {}
// Sets a specified page scale factor.
pub struct SetPageScaleFactor {
    // Page scale factor.
    pub page_scale_factor: f64,
}
pub struct SetPageScaleFactorReturnObject {}
// Switches script execution in the page.
pub struct SetScriptExecutionDisabled {
    // Whether script execution should be disabled in the page.
    pub value: bool,
}
pub struct SetScriptExecutionDisabledReturnObject {}
// Enables touch on platforms which do not support them.
pub struct SetTouchEmulationEnabled {
    // Whether the touch event emulation should be enabled.
    pub enabled: bool,
    // Maximum touch points supported. Defaults to one.
    pub max_touch_points: Option<i32>,
}
pub struct SetTouchEmulationEnabledReturnObject {}
// Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
// the current virtual time policy.  Note this supersedes any previous time budget.
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
pub struct SetVirtualTimePolicyReturnObject {
    // Absolute timestamp at which virtual time was first enabled (up time in milliseconds).
    pub virtual_time_ticks_base: f64,
}
// Overrides default host system locale with the specified one.
pub struct SetLocaleOverride {
    // ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    // restores default host system locale.
    pub locale: Option<String>,
}
pub struct SetLocaleOverrideReturnObject {}
// Overrides default host system timezone with the specified one.
pub struct SetTimezoneOverride {
    // The timezone identifier. If empty, disables the override and
    // restores default host system timezone.
    pub timezone_id: String,
}
pub struct SetTimezoneOverrideReturnObject {}
// Resizes the frame/viewport of the page. Note that this does not affect the frame's container
// (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
// on Android.
pub struct SetVisibleSize {
    // Frame width (DIP).
    pub width: i32,
    // Frame height (DIP).
    pub height: i32,
}
pub struct SetVisibleSizeReturnObject {}
// Allows overriding user agent with the given string.
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
pub struct SetUserAgentOverrideReturnObject {}
