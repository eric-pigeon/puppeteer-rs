// This file is auto-generated do not edit manually.

pub enum AnimationType {
    CSSTransition,
    CSSAnimation,
    WebAnimation,
}
// Animation instance.
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
    pub r#type: AnimationType,
    // `Animation`'s source animation node.
    pub source: Option<AnimationEffect>,
    // A unique ID for `Animation` representing the sources that triggered this CSS
    // animation/transition.
    pub css_id: Option<String>,
}
// AnimationEffect instance
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
pub struct KeyframesRule {
    // CSS keyframed animation's name.
    pub name: Option<String>,
    // List of animation keyframes.
    pub keyframes: Vec<KeyframeStyle>,
}
// Keyframe Style
pub struct KeyframeStyle {
    // Keyframe's time offset.
    pub offset: String,
    // `AnimationEffect`'s timing function.
    pub easing: String,
}

// Disables animation domain notifications.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables animation domain notifications.
pub struct Enable {}
pub struct EnableReturnObject {}
// Returns the current time of the an animation.
pub struct GetCurrentTime {
    // Id of animation.
    pub id: String,
}
pub struct GetCurrentTimeReturnObject {
    // Current time of the page.
    pub current_time: f64,
}
// Gets the playback rate of the document timeline.
pub struct GetPlaybackRate {}
pub struct GetPlaybackRateReturnObject {
    // Playback rate for animations on page.
    pub playback_rate: f64,
}
// Releases a set of animations to no longer be manipulated.
pub struct ReleaseAnimations {
    // List of animation ids to seek.
    pub animations: Vec<String>,
}
pub struct ReleaseAnimationsReturnObject {}
// Gets the remote object of the Animation.
pub struct ResolveAnimation {
    // Animation id.
    pub animation_id: String,
}
pub struct ResolveAnimationReturnObject {
    // Corresponding remote object.
    pub remote_object: super::runtime::RemoteObject,
}
// Seek a set of animations to a particular time within each animation.
pub struct SeekAnimations {
    // List of animation ids to seek.
    pub animations: Vec<String>,
    // Set the current time of each animation.
    pub current_time: f64,
}
pub struct SeekAnimationsReturnObject {}
// Sets the paused state of a set of animations.
pub struct SetPaused {
    // Animations to set the pause state of.
    pub animations: Vec<String>,
    // Paused state to set to.
    pub paused: bool,
}
pub struct SetPausedReturnObject {}
// Sets the playback rate of the document timeline.
pub struct SetPlaybackRate {
    // Playback rate for animations on page
    pub playback_rate: f64,
}
pub struct SetPlaybackRateReturnObject {}
// Sets the timing of an animation node.
pub struct SetTiming {
    // Animation id.
    pub animation_id: String,
    // Duration of the animation.
    pub duration: f64,
    // Delay of the animation.
    pub delay: f64,
}
pub struct SetTimingReturnObject {}
