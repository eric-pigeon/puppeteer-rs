// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Players will get an ID that is unique within the agent context.
pub type PlayerId = String;
pub type Timestamp = f64;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlayerMessageLevel {
    Error,
    Warning,
    Info,
    Debug,
}
// Have one type per entry in MediaLogRecord::Type
// Corresponds to kMessage
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMessage {
    // Keep in sync with MediaLogMessageLevel
    // We are currently keeping the message level 'error' separate from the
    // PlayerError type because right now they represent different things,
    // this one being a DVLOG(ERROR) style log message that gets printed
    // based on what log level is selected in the UI, and the other is a
    // representation of a media::PipelineStatus object. Soon however we're
    // going to be moving away from using PipelineStatus for errors and
    // introducing a new error type which should hopefully let us integrate
    // the error log level into the PlayerError type.
    pub level: String,
    pub message: String,
}
// Corresponds to kMediaPropertyChange
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
}
// Corresponds to kMediaEventTriggered
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerEvent {
    pub timestamp: Timestamp,
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlayerErrorType {
    Pipelineerror,
    Mediaerror,
}
// Corresponds to kMediaError
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerError {
    pub r#type: String,
    // When this switches to using media::Status instead of PipelineStatus
    // we can remove "errorCode" and replace it with the fields from
    // a Status instance. This also seems like a duplicate of the error
    // level enum - there is a todo bug to have that level removed and
    // use this instead. (crbug.com/1068454)
    pub error_code: String,
}

// Enables the Media domain
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Media.enable";

    type ReturnObject = EnableReturnObject;
}
// Disables the Media domain.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Media.disable";

    type ReturnObject = DisableReturnObject;
}

// This can be called multiple times, and can be used to set / override /
// remove player properties. A null propValue indicates removal.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerPropertiesChangedEvent {
    pub params: PlayerPropertiesChangedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPropertiesChangedParams {
    pub player_id: PlayerId,
    pub properties: Vec<PlayerProperty>,
}
// Send events as a list, allowing them to be batched on the browser for less
// congestion. If batched, events must ALWAYS be in chronological order.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerEventsAddedEvent {
    pub params: PlayerEventsAddedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerEventsAddedParams {
    pub player_id: PlayerId,
    pub events: Vec<PlayerEvent>,
}
// Send a list of any messages that need to be delivered.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerMessagesLoggedEvent {
    pub params: PlayerMessagesLoggedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMessagesLoggedParams {
    pub player_id: PlayerId,
    pub messages: Vec<PlayerMessage>,
}
// Send a list of any errors that need to be delivered.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerErrorsRaisedEvent {
    pub params: PlayerErrorsRaisedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerErrorsRaisedParams {
    pub player_id: PlayerId,
    pub errors: Vec<PlayerError>,
}
// Called whenever a player is created, or when a new agent joins and recieves
// a list of active players. If an agent is restored, it will recieve the full
// list of player ids and all events again.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayersCreatedEvent {
    pub params: PlayersCreatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayersCreatedParams {
    pub players: Vec<PlayerId>,
}
