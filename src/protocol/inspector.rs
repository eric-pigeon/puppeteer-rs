// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Disables inspector domain notifications.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Inspector.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables inspector domain notifications.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Inspector.enable";

    type ReturnObject = EnableReturnObject;
}

// Fired when remote debugging connection is about to be terminated. Contains detach reason.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DetachedEvent {
    pub params: DetachedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DetachedParams {
    // The reason why connection has been terminated.
    pub reason: String,
}
// Fired when debugging target has crashed
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TargetCrashedEvent {
    pub params: TargetCrashedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TargetCrashedParams {}
// Fired when debugging target has reloaded after crash
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TargetReloadedAfterCrashEvent {
    pub params: TargetReloadedAfterCrashParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TargetReloadedAfterCrashParams {}
