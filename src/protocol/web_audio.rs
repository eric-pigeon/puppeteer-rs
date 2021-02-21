// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API
pub type GraphObjectId = String;
// Enum of BaseAudioContext types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ContextType {
    Realtime,
    Offline,
}
// Enum of AudioContextState from the spec
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ContextState {
    Suspended,
    Running,
    Closed,
}
// Enum of AudioNode types
pub type NodeType = String;
// Enum of AudioNode::ChannelCountMode from the spec
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChannelCountMode {
    ClampedMax,
    Explicit,
    Max,
}
// Enum of AudioNode::ChannelInterpretation from the spec
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChannelInterpretation {
    Discrete,
    Speakers,
}
// Enum of AudioParam types
pub type ParamType = String;
// Enum of AudioParam::AutomationRate from the spec
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AutomationRate {
    ARate,
    KRate,
}
// Fields in AudioContext that change in real-time.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContextRealtimeData {
    // The current context time in second in BaseAudioContext.
    pub current_time: f64,
    // The time spent on rendering graph divided by render qunatum duration,
    // and multiplied by 100. 100 means the audio renderer reached the full
    // capacity and glitch may occur.
    pub render_capacity: f64,
    // A running mean of callback interval.
    pub callback_interval_mean: f64,
    // A running variance of callback interval.
    pub callback_interval_variance: f64,
}
// Protocol object for BaseAudioContext
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseAudioContext {
    pub context_id: GraphObjectId,
    pub context_type: ContextType,
    pub context_state: ContextState,
    pub realtime_data: Option<ContextRealtimeData>,
    // Platform-dependent callback buffer size.
    pub callback_buffer_size: f64,
    // Number of output channels supported by audio hardware in use.
    pub max_output_channel_count: f64,
    // Context sample rate.
    pub sample_rate: f64,
}
// Protocol object for AudioListener
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioListener {
    pub listener_id: GraphObjectId,
    pub context_id: GraphObjectId,
}
// Protocol object for AudioNode
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioNode {
    pub node_id: GraphObjectId,
    pub context_id: GraphObjectId,
    pub node_type: NodeType,
    pub number_of_inputs: f64,
    pub number_of_outputs: f64,
    pub channel_count: f64,
    pub channel_count_mode: ChannelCountMode,
    pub channel_interpretation: ChannelInterpretation,
}
// Protocol object for AudioParam
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioParam {
    pub param_id: GraphObjectId,
    pub node_id: GraphObjectId,
    pub context_id: GraphObjectId,
    pub param_type: ParamType,
    pub rate: AutomationRate,
    pub default_value: f64,
    pub min_value: f64,
    pub max_value: f64,
}

// Enables the WebAudio domain and starts sending context lifetime events.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "WebAudio.enable";

    type ReturnObject = EnableReturnObject;
}
// Disables the WebAudio domain.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "WebAudio.disable";

    type ReturnObject = DisableReturnObject;
}
// Fetch the realtime data from the registered contexts.
#[derive(Serialize, Debug)]
pub struct GetRealtimeData {
    pub context_id: GraphObjectId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetRealtimeDataReturnObject {
    pub realtime_data: ContextRealtimeData,
}
impl super::Command for GetRealtimeData {
    const NAME: &'static str = "WebAudio.getRealtimeData";

    type ReturnObject = GetRealtimeDataReturnObject;
}

// Notifies that a new BaseAudioContext has been created.
#[derive(Deserialize, Debug, Clone)]
pub struct ContextCreated {
    pub context: BaseAudioContext,
}
// Notifies that an existing BaseAudioContext will be destroyed.
#[derive(Deserialize, Debug, Clone)]
pub struct ContextWillBeDestroyed {
    pub context_id: GraphObjectId,
}
// Notifies that existing BaseAudioContext has changed some properties (id stays the same)..
#[derive(Deserialize, Debug, Clone)]
pub struct ContextChanged {
    pub context: BaseAudioContext,
}
// Notifies that the construction of an AudioListener has finished.
#[derive(Deserialize, Debug, Clone)]
pub struct AudioListenerCreated {
    pub listener: AudioListener,
}
// Notifies that a new AudioListener has been created.
#[derive(Deserialize, Debug, Clone)]
pub struct AudioListenerWillBeDestroyed {
    pub context_id: GraphObjectId,
    pub listener_id: GraphObjectId,
}
// Notifies that a new AudioNode has been created.
#[derive(Deserialize, Debug, Clone)]
pub struct AudioNodeCreated {
    pub node: AudioNode,
}
// Notifies that an existing AudioNode has been destroyed.
#[derive(Deserialize, Debug, Clone)]
pub struct AudioNodeWillBeDestroyed {
    pub context_id: GraphObjectId,
    pub node_id: GraphObjectId,
}
// Notifies that a new AudioParam has been created.
#[derive(Deserialize, Debug, Clone)]
pub struct AudioParamCreated {
    pub param: AudioParam,
}
// Notifies that an existing AudioParam has been destroyed.
#[derive(Deserialize, Debug, Clone)]
pub struct AudioParamWillBeDestroyed {
    pub context_id: GraphObjectId,
    pub node_id: GraphObjectId,
    pub param_id: GraphObjectId,
}
// Notifies that two AudioNodes are connected.
#[derive(Deserialize, Debug, Clone)]
pub struct NodesConnected {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
    pub destination_input_index: Option<f64>,
}
// Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
#[derive(Deserialize, Debug, Clone)]
pub struct NodesDisconnected {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
    pub destination_input_index: Option<f64>,
}
// Notifies that an AudioNode is connected to an AudioParam.
#[derive(Deserialize, Debug, Clone)]
pub struct NodeParamConnected {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
}
// Notifies that an AudioNode is disconnected to an AudioParam.
#[derive(Deserialize, Debug, Clone)]
pub struct NodeParamDisconnected {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
}
