// This file is auto-generated do not edit manually.

pub enum ScreenshotParamsFormat {
    Jpeg,
    Png,
}
// Encoding options for a screenshot.
pub struct ScreenshotParams {
    // Image compression format (defaults to png).
    pub format: Option<ScreenshotParamsFormat>,
    // Compression quality from range [0..100] (jpeg only).
    pub quality: Option<i32>,
}

// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
// screenshot from the resulting frame. Requires that the target was created with enabled
// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
// https://goo.gl/3zHXhB for more background.
pub struct BeginFrame {
    // Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    // the current time will be used.
    pub frame_time_ticks: Option<f64>,
    // The interval between BeginFrames that is reported to the compositor, in milliseconds.
    // Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.
    pub interval: Option<f64>,
    // Whether updates should not be committed and drawn onto the display. False by default. If
    // true, only side effects of the BeginFrame will be run, such as layout and animations, but
    // any visual updates may not be visible on the display or in screenshots.
    pub no_display_updates: Option<bool>,
    // If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    // no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    // during renderer initialization. In such a case, no screenshot data will be returned.
    pub screenshot: Option<ScreenshotParams>,
}
pub struct BeginFrameReturnObject {
    // Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    // display. Reported for diagnostic uses, may be removed in the future.
    pub has_damage: bool,
    // Base64-encoded image data of the screenshot, if one was requested and successfully taken.
    pub screenshot_data: Option<String>,
}
// Disables headless events for the target.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables headless events for the target.
pub struct Enable {}
pub struct EnableReturnObject {}
