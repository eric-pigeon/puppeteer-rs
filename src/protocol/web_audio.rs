// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API
pub type GraphObjectId = String;
// Enum of BaseAudioContext types
pub type ContextType = String;
// Enum of AudioContextState from the spec
pub type ContextState = String;
// Enum of AudioNode types
pub type NodeType = String;
// Enum of AudioNode::ChannelCountMode from the spec
pub type ChannelCountMode = String;
// Enum of AudioNode::ChannelInterpretation from the spec
pub type ChannelInterpretation = String;
// Enum of AudioParam types
pub type ParamType = String;
// Enum of AudioParam::AutomationRate from the spec
pub type AutomationRate = String;
// Fields in AudioContext that change in real-time.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioListener {
    pub listener_id: GraphObjectId,
    pub context_id: GraphObjectId,
}
// Protocol object for AudioNode
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "WebAudio.enable";

    type ReturnObject = EnableReturnObject;
}
// Disables the WebAudio domain.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct GetRealtimeDataReturnObject {
    pub realtime_data: ContextRealtimeData,
}
impl super::Command for GetRealtimeData {
    const NAME: &'static str = "WebAudio.getRealtimeData";

    type ReturnObject = GetRealtimeDataReturnObject;
}

// Notifies that a new BaseAudioContext has been created.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ContextCreatedEvent {
    pub params: ContextCreatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContextCreatedParams {
    pub context: BaseAudioContext,
}
// Notifies that an existing BaseAudioContext will be destroyed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ContextWillBeDestroyedEvent {
    pub params: ContextWillBeDestroyedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContextWillBeDestroyedParams {
    pub context_id: GraphObjectId,
}
// Notifies that existing BaseAudioContext has changed some properties (id stays the same)..
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ContextChangedEvent {
    pub params: ContextChangedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContextChangedParams {
    pub context: BaseAudioContext,
}
// Notifies that the construction of an AudioListener has finished.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AudioListenerCreatedEvent {
    pub params: AudioListenerCreatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioListenerCreatedParams {
    pub listener: AudioListener,
}
// Notifies that a new AudioListener has been created.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AudioListenerWillBeDestroyedEvent {
    pub params: AudioListenerWillBeDestroyedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioListenerWillBeDestroyedParams {
    pub context_id: GraphObjectId,
    pub listener_id: GraphObjectId,
}
// Notifies that a new AudioNode has been created.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AudioNodeCreatedEvent {
    pub params: AudioNodeCreatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioNodeCreatedParams {
    pub node: AudioNode,
}
// Notifies that an existing AudioNode has been destroyed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AudioNodeWillBeDestroyedEvent {
    pub params: AudioNodeWillBeDestroyedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioNodeWillBeDestroyedParams {
    pub context_id: GraphObjectId,
    pub node_id: GraphObjectId,
}
// Notifies that a new AudioParam has been created.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AudioParamCreatedEvent {
    pub params: AudioParamCreatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioParamCreatedParams {
    pub param: AudioParam,
}
// Notifies that an existing AudioParam has been destroyed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AudioParamWillBeDestroyedEvent {
    pub params: AudioParamWillBeDestroyedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioParamWillBeDestroyedParams {
    pub context_id: GraphObjectId,
    pub node_id: GraphObjectId,
    pub param_id: GraphObjectId,
}
// Notifies that two AudioNodes are connected.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NodesConnectedEvent {
    pub params: NodesConnectedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodesConnectedParams {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
    pub destination_input_index: Option<f64>,
}
// Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NodesDisconnectedEvent {
    pub params: NodesDisconnectedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodesDisconnectedParams {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
    pub destination_input_index: Option<f64>,
}
// Notifies that an AudioNode is connected to an AudioParam.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NodeParamConnectedEvent {
    pub params: NodeParamConnectedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeParamConnectedParams {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
}
// Notifies that an AudioNode is disconnected to an AudioParam.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NodeParamDisconnectedEvent {
    pub params: NodeParamDisconnectedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeParamDisconnectedParams {
    pub context_id: GraphObjectId,
    pub source_id: GraphObjectId,
    pub destination_id: GraphObjectId,
    pub source_output_index: Option<f64>,
}
