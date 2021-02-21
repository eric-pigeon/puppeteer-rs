// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnimationType {
    CSSTransition,
    CSSAnimation,
    WebAnimation,
}
// Animation instance.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    // `Animation`'s id.
    pub id: String,
    // `Animation`'s name.
    pub name: String,
    // `Animation`'s internal paused state.
    pub paused_state: bool,
    // `Animation`'s play state.
    pub play_state: String,
    // `Animation`'s playback rate.
    pub playback_rate: f64,
    // `Animation`'s start time.
    pub start_time: f64,
    // `Animation`'s current time.
    pub current_time: f64,
    // Animation type of `Animation`.
    pub r#type: String,
    // `Animation`'s source animation node.
    pub source: Option<AnimationEffect>,
    // A unique ID for `Animation` representing the sources that triggered this CSS
    // animation/transition.
    pub css_id: Option<String>,
}
// AnimationEffect instance
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationEffect {
    // `AnimationEffect`'s delay.
    pub delay: f64,
    // `AnimationEffect`'s end delay.
    pub end_delay: f64,
    // `AnimationEffect`'s iteration start.
    pub iteration_start: f64,
    // `AnimationEffect`'s iterations.
    pub iterations: f64,
    // `AnimationEffect`'s iteration duration.
    pub duration: f64,
    // `AnimationEffect`'s playback direction.
    pub direction: String,
    // `AnimationEffect`'s fill mode.
    pub fill: String,
    // `AnimationEffect`'s target node.
    pub backend_node_id: Option<super::dom::BackendNodeId>,
    // `AnimationEffect`'s keyframes.
    pub keyframes_rule: Option<KeyframesRule>,
    // `AnimationEffect`'s timing function.
    pub easing: String,
}
// Keyframes Rule
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyframesRule {
    // CSS keyframed animation's name.
    pub name: Option<String>,
    // List of animation keyframes.
    pub keyframes: Vec<KeyframeStyle>,
}
// Keyframe Style
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyframeStyle {
    // Keyframe's time offset.
    pub offset: String,
    // `AnimationEffect`'s timing function.
    pub easing: String,
}

// Disables animation domain notifications.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Animation.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables animation domain notifications.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Animation.enable";

    type ReturnObject = EnableReturnObject;
}
// Returns the current time of the an animation.
#[derive(Serialize, Debug)]
pub struct GetCurrentTime {
    // Id of animation.
    pub id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetCurrentTimeReturnObject {
    // Current time of the page.
    pub current_time: f64,
}
impl super::Command for GetCurrentTime {
    const NAME: &'static str = "Animation.getCurrentTime";

    type ReturnObject = GetCurrentTimeReturnObject;
}
// Gets the playback rate of the document timeline.
#[derive(Serialize, Debug)]
pub struct GetPlaybackRate {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetPlaybackRateReturnObject {
    // Playback rate for animations on page.
    pub playback_rate: f64,
}
impl super::Command for GetPlaybackRate {
    const NAME: &'static str = "Animation.getPlaybackRate";

    type ReturnObject = GetPlaybackRateReturnObject;
}
// Releases a set of animations to no longer be manipulated.
#[derive(Serialize, Debug)]
pub struct ReleaseAnimations {
    // List of animation ids to seek.
    pub animations: Vec<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ReleaseAnimationsReturnObject {}
impl super::Command for ReleaseAnimations {
    const NAME: &'static str = "Animation.releaseAnimations";

    type ReturnObject = ReleaseAnimationsReturnObject;
}
// Gets the remote object of the Animation.
#[derive(Serialize, Debug)]
pub struct ResolveAnimation {
    // Animation id.
    pub animation_id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ResolveAnimationReturnObject {
    // Corresponding remote object.
    pub remote_object: super::runtime::RemoteObject,
}
impl super::Command for ResolveAnimation {
    const NAME: &'static str = "Animation.resolveAnimation";

    type ReturnObject = ResolveAnimationReturnObject;
}
// Seek a set of animations to a particular time within each animation.
#[derive(Serialize, Debug)]
pub struct SeekAnimations {
    // List of animation ids to seek.
    pub animations: Vec<String>,
    // Set the current time of each animation.
    pub current_time: f64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SeekAnimationsReturnObject {}
impl super::Command for SeekAnimations {
    const NAME: &'static str = "Animation.seekAnimations";

    type ReturnObject = SeekAnimationsReturnObject;
}
// Sets the paused state of a set of animations.
#[derive(Serialize, Debug)]
pub struct SetPaused {
    // Animations to set the pause state of.
    pub animations: Vec<String>,
    // Paused state to set to.
    pub paused: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetPausedReturnObject {}
impl super::Command for SetPaused {
    const NAME: &'static str = "Animation.setPaused";

    type ReturnObject = SetPausedReturnObject;
}
// Sets the playback rate of the document timeline.
#[derive(Serialize, Debug)]
pub struct SetPlaybackRate {
    // Playback rate for animations on page
    pub playback_rate: f64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetPlaybackRateReturnObject {}
impl super::Command for SetPlaybackRate {
    const NAME: &'static str = "Animation.setPlaybackRate";

    type ReturnObject = SetPlaybackRateReturnObject;
}
// Sets the timing of an animation node.
#[derive(Serialize, Debug)]
pub struct SetTiming {
    // Animation id.
    pub animation_id: String,
    // Duration of the animation.
    pub duration: f64,
    // Delay of the animation.
    pub delay: f64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetTimingReturnObject {}
impl super::Command for SetTiming {
    const NAME: &'static str = "Animation.setTiming";

    type ReturnObject = SetTimingReturnObject;
}

// Event for when an animation has been cancelled.
#[derive(Deserialize, Debug, Clone)]
pub struct AnimationCanceled {
    // Id of the animation that was cancelled.
    pub id: String,
}
// Event for each animation that has been created.
#[derive(Deserialize, Debug, Clone)]
pub struct AnimationCreated {
    // Id of the animation that was created.
    pub id: String,
}
// Event for animation that has been started.
#[derive(Deserialize, Debug, Clone)]
pub struct AnimationStarted {
    // Animation that was started.
    pub animation: Animation,
}
