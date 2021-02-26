// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TouchPoint {
    // X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub x: f64,
    // Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    // the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub y: f64,
    // X radius of the touch area (default: 1.0).
    pub radius_x: Option<f64>,
    // Y radius of the touch area (default: 1.0).
    pub radius_y: Option<f64>,
    // Rotation angle (default: 0.0).
    pub rotation_angle: Option<f64>,
    // Force (default: 1.0).
    pub force: Option<f64>,
    // Identifier used to track touch sources between events, must be unique within an event.
    pub id: Option<f64>,
}
pub type GestureSourceType = String;
pub type MouseButton = String;
// UTC time in seconds, counted from January 1, 1970.
pub type TimeSinceEpoch = f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    KeyDown,
    KeyUp,
    RawKeyDown,
    Char,
}
// Dispatches a key event to the page.
#[derive(Serialize, Debug)]
pub struct DispatchKeyEvent {
    // Type of the key event.
    pub r#type: String,
    // Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    // (default: 0).
    pub modifiers: Option<i32>,
    // Time at which the event occurred.
    pub timestamp: Option<TimeSinceEpoch>,
    // Text as generated by processing a virtual key code with a keyboard layout. Not needed for
    // for `keyUp` and `rawKeyDown` events (default: "")
    pub text: Option<String>,
    // Text that would have been generated by the keyboard if no modifiers were pressed (except for
    // shift). Useful for shortcut (accelerator) key handling (default: "").
    pub unmodified_text: Option<String>,
    // Unique key identifier (e.g., 'U+0041') (default: "").
    pub key_identifier: Option<String>,
    // Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").
    pub code: Option<String>,
    // Unique DOM defined string value describing the meaning of the key in the context of active
    // modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").
    pub key: Option<String>,
    // Windows virtual key code (default: 0).
    pub windows_virtual_key_code: Option<i32>,
    // Native virtual key code (default: 0).
    pub native_virtual_key_code: Option<i32>,
    // Whether the event was generated from auto repeat (default: false).
    pub auto_repeat: Option<bool>,
    // Whether the event was generated from the keypad (default: false).
    pub is_keypad: Option<bool>,
    // Whether the event was a system key event (default: false).
    pub is_system_key: Option<bool>,
    // Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    // 0).
    pub location: Option<i32>,
    // Editing commands to send with the key event (e.g., 'selectAll') (default: []).
    // These are related to but not equal the command names used in `document.execCommand` and NSStandardKeyBindingResponding.
    // See https://source.chromium.org/chromium/chromium/src/+/master:third_party/blink/renderer/core/editing/commands/editor_command_names.h for valid command names.
    pub commands: Option<Vec<String>>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DispatchKeyEventReturnObject {}
impl super::Command for DispatchKeyEvent {
    const NAME: &'static str = "Input.dispatchKeyEvent";

    type ReturnObject = DispatchKeyEventReturnObject;
}
// This method emulates inserting text that doesn't come from a key press,
// for example an emoji keyboard or an IME.
#[derive(Serialize, Debug)]
pub struct InsertText {
    // The text to insert.
    pub text: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextReturnObject {}
impl super::Command for InsertText {
    const NAME: &'static str = "Input.insertText";

    type ReturnObject = InsertTextReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PointerType {
    Mouse,
    Pen,
}
// Dispatches a mouse event to the page.
#[derive(Serialize, Debug)]
pub struct DispatchMouseEvent {
    // Type of the mouse event.
    pub r#type: String,
    // X coordinate of the event relative to the main frame's viewport in CSS pixels.
    pub x: f64,
    // Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    // the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.
    pub y: f64,
    // Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    // (default: 0).
    pub modifiers: Option<i32>,
    // Time at which the event occurred.
    pub timestamp: Option<TimeSinceEpoch>,
    // Mouse button (default: "none").
    pub button: Option<MouseButton>,
    // A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    // Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.
    pub buttons: Option<i32>,
    // Number of times the mouse button was clicked (default: 0).
    pub click_count: Option<i32>,
    // X delta in CSS pixels for mouse wheel event (default: 0).
    pub delta_x: Option<f64>,
    // Y delta in CSS pixels for mouse wheel event (default: 0).
    pub delta_y: Option<f64>,
    // Pointer type (default: "mouse").
    pub pointer_type: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DispatchMouseEventReturnObject {}
impl super::Command for DispatchMouseEvent {
    const NAME: &'static str = "Input.dispatchMouseEvent";

    type ReturnObject = DispatchMouseEventReturnObject;
}
// Dispatches a touch event to the page.
#[derive(Serialize, Debug)]
pub struct DispatchTouchEvent {
    // Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
    // TouchStart and TouchMove must contains at least one.
    pub r#type: String,
    // Active touch points on the touch device. One event per any changed point (compared to
    // previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
    // one by one.
    pub touch_points: Vec<TouchPoint>,
    // Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    // (default: 0).
    pub modifiers: Option<i32>,
    // Time at which the event occurred.
    pub timestamp: Option<TimeSinceEpoch>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DispatchTouchEventReturnObject {}
impl super::Command for DispatchTouchEvent {
    const NAME: &'static str = "Input.dispatchTouchEvent";

    type ReturnObject = DispatchTouchEventReturnObject;
}
// Emulates touch event from the mouse event parameters.
#[derive(Serialize, Debug)]
pub struct EmulateTouchFromMouseEvent {
    // Type of the mouse event.
    pub r#type: String,
    // X coordinate of the mouse pointer in DIP.
    pub x: i32,
    // Y coordinate of the mouse pointer in DIP.
    pub y: i32,
    // Mouse button. Only "none", "left", "right" are supported.
    pub button: MouseButton,
    // Time at which the event occurred (default: current time).
    pub timestamp: Option<TimeSinceEpoch>,
    // X delta in DIP for mouse wheel event (default: 0).
    pub delta_x: Option<f64>,
    // Y delta in DIP for mouse wheel event (default: 0).
    pub delta_y: Option<f64>,
    // Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    // (default: 0).
    pub modifiers: Option<i32>,
    // Number of times the mouse button was clicked (default: 0).
    pub click_count: Option<i32>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmulateTouchFromMouseEventReturnObject {}
impl super::Command for EmulateTouchFromMouseEvent {
    const NAME: &'static str = "Input.emulateTouchFromMouseEvent";

    type ReturnObject = EmulateTouchFromMouseEventReturnObject;
}
// Ignores input events (useful while auditing page).
#[derive(Serialize, Debug)]
pub struct SetIgnoreInputEvents {
    // Ignores input events processing when set to true.
    pub ignore: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreInputEventsReturnObject {}
impl super::Command for SetIgnoreInputEvents {
    const NAME: &'static str = "Input.setIgnoreInputEvents";

    type ReturnObject = SetIgnoreInputEventsReturnObject;
}
// Synthesizes a pinch gesture over a time period by issuing appropriate touch events.
#[derive(Serialize, Debug)]
pub struct SynthesizePinchGesture {
    // X coordinate of the start of the gesture in CSS pixels.
    pub x: f64,
    // Y coordinate of the start of the gesture in CSS pixels.
    pub y: f64,
    // Relative scale factor after zooming (>1.0 zooms in, <1.0 zooms out).
    pub scale_factor: f64,
    // Relative pointer speed in pixels per second (default: 800).
    pub relative_speed: Option<i32>,
    // Which type of input events to be generated (default: 'default', which queries the platform
    // for the preferred input type).
    pub gesture_source_type: Option<GestureSourceType>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizePinchGestureReturnObject {}
impl super::Command for SynthesizePinchGesture {
    const NAME: &'static str = "Input.synthesizePinchGesture";

    type ReturnObject = SynthesizePinchGestureReturnObject;
}
// Synthesizes a scroll gesture over a time period by issuing appropriate touch events.
#[derive(Serialize, Debug)]
pub struct SynthesizeScrollGesture {
    // X coordinate of the start of the gesture in CSS pixels.
    pub x: f64,
    // Y coordinate of the start of the gesture in CSS pixels.
    pub y: f64,
    // The distance to scroll along the X axis (positive to scroll left).
    pub x_distance: Option<f64>,
    // The distance to scroll along the Y axis (positive to scroll up).
    pub y_distance: Option<f64>,
    // The number of additional pixels to scroll back along the X axis, in addition to the given
    // distance.
    pub x_overscroll: Option<f64>,
    // The number of additional pixels to scroll back along the Y axis, in addition to the given
    // distance.
    pub y_overscroll: Option<f64>,
    // Prevent fling (default: true).
    pub prevent_fling: Option<bool>,
    // Swipe speed in pixels per second (default: 800).
    pub speed: Option<i32>,
    // Which type of input events to be generated (default: 'default', which queries the platform
    // for the preferred input type).
    pub gesture_source_type: Option<GestureSourceType>,
    // The number of times to repeat the gesture (default: 0).
    pub repeat_count: Option<i32>,
    // The number of milliseconds delay between each repeat. (default: 250).
    pub repeat_delay_ms: Option<i32>,
    // The name of the interaction markers to generate, if not empty (default: "").
    pub interaction_marker_name: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeScrollGestureReturnObject {}
impl super::Command for SynthesizeScrollGesture {
    const NAME: &'static str = "Input.synthesizeScrollGesture";

    type ReturnObject = SynthesizeScrollGestureReturnObject;
}
// Synthesizes a tap gesture over a time period by issuing appropriate touch events.
#[derive(Serialize, Debug)]
pub struct SynthesizeTapGesture {
    // X coordinate of the start of the gesture in CSS pixels.
    pub x: f64,
    // Y coordinate of the start of the gesture in CSS pixels.
    pub y: f64,
    // Duration between touchdown and touchup events in ms (default: 50).
    pub duration: Option<i32>,
    // Number of times to perform the tap (e.g. 2 for double tap, default: 1).
    pub tap_count: Option<i32>,
    // Which type of input events to be generated (default: 'default', which queries the platform
    // for the preferred input type).
    pub gesture_source_type: Option<GestureSourceType>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeTapGestureReturnObject {}
impl super::Command for SynthesizeTapGesture {
    const NAME: &'static str = "Input.synthesizeTapGesture";

    type ReturnObject = SynthesizeTapGestureReturnObject;
}
