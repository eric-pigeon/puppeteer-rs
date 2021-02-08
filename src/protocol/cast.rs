// This file is auto-generated do not edit manually.

pub struct Sink {
    pub name: String,
    pub id: String,
    // Text describing the current session. Present only if there is an active
    // session on the sink.
    pub session: Option<String>,
}

// Starts observing for sinks that can be used for tab mirroring, and if set,
// sinks compatible with |presentationUrl| as well. When sinks are found, a
// |sinksUpdated| event is fired.
// Also starts observing for issue messages. When an issue is added or removed,
// an |issueUpdated| event is fired.
pub struct Enable {
    pub presentation_url: Option<String>,
}
pub struct EnableReturnObject {}
// Stops observing for sinks and issues.
pub struct Disable {}
pub struct DisableReturnObject {}
// Sets a sink to be used when the web page requests the browser to choose a
// sink via Presentation API, Remote Playback API, or Cast SDK.
pub struct SetSinkToUse {
    pub sink_name: String,
}
pub struct SetSinkToUseReturnObject {}
// Starts mirroring the tab to the sink.
pub struct StartTabMirroring {
    pub sink_name: String,
}
pub struct StartTabMirroringReturnObject {}
// Stops the active Cast session on the sink.
pub struct StopCasting {
    pub sink_name: String,
}
pub struct StopCastingReturnObject {}
