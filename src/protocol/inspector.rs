// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Disables inspector domain notifications.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Inspector.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables inspector domain notifications.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Inspector.enable";

    type ReturnObject = EnableReturnObject;
}
