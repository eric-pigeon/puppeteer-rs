// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScreenshotParamsFormat {
    Jpeg,
    Png,
}
// Encoding options for a screenshot.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotParams {
    // Image compression format (defaults to png).
    pub format: Option<String>,
    // Compression quality from range [0..100] (jpeg only).
    pub quality: Option<i32>,
}

// Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
// screenshot from the resulting frame. Requires that the target was created with enabled
// BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
// https://goo.gl/3zHXhB for more background.
#[derive(Serialize, Debug)]
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
#[derive(Deserialize, Debug, Clone)]
pub struct BeginFrameReturnObject {
    // Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    // display. Reported for diagnostic uses, may be removed in the future.
    pub has_damage: bool,
    // Base64-encoded image data of the screenshot, if one was requested and successfully taken.
    pub screenshot_data: Option<String>,
}
impl super::Command for BeginFrame {
    const NAME: &'static str = "HeadlessExperimental.beginFrame";

    type ReturnObject = BeginFrameReturnObject;
}
// Disables headless events for the target.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "HeadlessExperimental.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables headless events for the target.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "HeadlessExperimental.enable";

    type ReturnObject = EnableReturnObject;
}

// Issued when the target starts or stops needing BeginFrames.
// Deprecated. Issue beginFrame unconditionally instead and use result from
// beginFrame to detect whether the frames were suppressed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NeedsBeginFramesChangedEvent {
    pub params: NeedsBeginFramesChangedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NeedsBeginFramesChangedParams {
    // True if BeginFrames are needed, false otherwise.
    pub needs_begin_frames: bool,
}
