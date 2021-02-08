// This file is auto-generated do not edit manually.

// Players will get an ID that is unique within the agent context.
pub type PlayerId = String;
pub type Timestamp = f64;
pub enum PlayerMessageLevel {
    Error,
    Warning,
    Info,
    Debug,
}
// Have one type per entry in MediaLogRecord::Type
// Corresponds to kMessage
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
    pub level: PlayerMessageLevel,
    pub message: String,
}
// Corresponds to kMediaPropertyChange
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
}
// Corresponds to kMediaEventTriggered
pub struct PlayerEvent {
    pub timestamp: Timestamp,
    pub value: String,
}
pub enum PlayerErrorType {
    Pipelineerror,
    Mediaerror,
}
// Corresponds to kMediaError
pub struct PlayerError {
    pub r#type: PlayerErrorType,
    // When this switches to using media::Status instead of PipelineStatus
    // we can remove "errorCode" and replace it with the fields from
    // a Status instance. This also seems like a duplicate of the error
    // level enum - there is a todo bug to have that level removed and
    // use this instead. (crbug.com/1068454)
    pub error_code: String,
}

// Enables the Media domain
pub struct Enable {}
pub struct EnableReturnObject {}
// Disables the Media domain.
pub struct Disable {}
pub struct DisableReturnObject {}
