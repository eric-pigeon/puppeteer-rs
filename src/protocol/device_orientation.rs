// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Clears the overridden Device Orientation.
#[derive(Serialize, Debug)]
pub struct ClearDeviceOrientationOverride {}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearDeviceOrientationOverrideReturnObject {}
impl super::Command for ClearDeviceOrientationOverride {
    const NAME: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";

    type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
}
// Overrides the Device Orientation.
#[derive(Serialize, Debug)]
pub struct SetDeviceOrientationOverride {
    // Mock alpha
    pub alpha: f64,
    // Mock beta
    pub beta: f64,
    // Mock gamma
    pub gamma: f64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetDeviceOrientationOverrideReturnObject {}
impl super::Command for SetDeviceOrientationOverride {
    const NAME: &'static str = "DeviceOrientation.setDeviceOrientationOverride";

    type ReturnObject = SetDeviceOrientationOverrideReturnObject;
}
