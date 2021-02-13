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
