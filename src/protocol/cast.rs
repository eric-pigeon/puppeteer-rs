// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Debug)]
pub struct Enable {
    pub presentation_url: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Cast.enable";

    type ReturnObject = EnableReturnObject;
}
// Stops observing for sinks and issues.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Cast.disable";

    type ReturnObject = DisableReturnObject;
}
// Sets a sink to be used when the web page requests the browser to choose a
// sink via Presentation API, Remote Playback API, or Cast SDK.
#[derive(Serialize, Debug)]
pub struct SetSinkToUse {
    pub sink_name: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetSinkToUseReturnObject {}
impl super::Command for SetSinkToUse {
    const NAME: &'static str = "Cast.setSinkToUse";

    type ReturnObject = SetSinkToUseReturnObject;
}
// Starts mirroring the tab to the sink.
#[derive(Serialize, Debug)]
pub struct StartTabMirroring {
    pub sink_name: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartTabMirroringReturnObject {}
impl super::Command for StartTabMirroring {
    const NAME: &'static str = "Cast.startTabMirroring";

    type ReturnObject = StartTabMirroringReturnObject;
}
// Stops the active Cast session on the sink.
#[derive(Serialize, Debug)]
pub struct StopCasting {
    pub sink_name: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StopCastingReturnObject {}
impl super::Command for StopCasting {
    const NAME: &'static str = "Cast.stopCasting";

    type ReturnObject = StopCastingReturnObject;
}

// This is fired whenever the list of available sinks changes. A sink is a
// device or a software surface that you can cast to.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SinksUpdatedEvent {
    pub params: SinksUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SinksUpdatedParams {
    pub sinks: Vec<Sink>,
}
// This is fired whenever the outstanding issue/error message changes.
// |issueMessage| is empty if there is no issue.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IssueUpdatedEvent {
    pub params: IssueUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueUpdatedParams {
    pub issue_message: String,
}
