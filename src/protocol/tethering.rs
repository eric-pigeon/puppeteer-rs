// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Request browser port binding.
#[derive(Serialize, Debug)]
pub struct Bind {
    // Port number to bind.
    pub port: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct BindReturnObject {}
impl super::Command for Bind {
    const NAME: &'static str = "Tethering.bind";

    type ReturnObject = BindReturnObject;
}
// Request browser port unbinding.
#[derive(Serialize, Debug)]
pub struct Unbind {
    // Port number to unbind.
    pub port: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UnbindReturnObject {}
impl super::Command for Unbind {
    const NAME: &'static str = "Tethering.unbind";

    type ReturnObject = UnbindReturnObject;
}

// Informs that port was successfully bound and got a specified connection id.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AcceptedEvent {
    pub params: AcceptedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AcceptedParams {
    // Port number that was successfully bound.
    pub port: i32,
    // Connection id to be used.
    pub connection_id: String,
}
