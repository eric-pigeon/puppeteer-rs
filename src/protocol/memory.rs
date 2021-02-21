// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Memory pressure level.
pub type PressureLevel = String;
// Heap profile sample.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfileNode {
    // Size of the sampled allocation.
    pub size: f64,
    // Total bytes attributed to this sample.
    pub total: f64,
    // Execution stack at the point of allocation.
    pub stack: Vec<String>,
}
// Array of heap profile samples.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SamplingProfile {
    pub samples: Vec<SamplingProfileNode>,
    pub modules: Vec<Module>,
}
// Executable module information
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    // Name of the module.
    pub name: String,
    // UUID of the module.
    pub uuid: String,
    // Base address where the module is loaded into memory. Encoded as a decimal
    // or hexadecimal (0x prefixed) string.
    pub base_address: String,
    // Size of the module in bytes.
    pub size: f64,
}

#[derive(Serialize, Debug)]
pub struct GetDOMCounters {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetDOMCountersReturnObject {
    pub documents: i32,
    pub nodes: i32,
    pub js_event_listeners: i32,
}
impl super::Command for GetDOMCounters {
    const NAME: &'static str = "Memory.getDOMCounters";

    type ReturnObject = GetDOMCountersReturnObject;
}
#[derive(Serialize, Debug)]
pub struct PrepareForLeakDetection {}
#[derive(Deserialize, Debug, Clone)]
pub struct PrepareForLeakDetectionReturnObject {}
impl super::Command for PrepareForLeakDetection {
    const NAME: &'static str = "Memory.prepareForLeakDetection";

    type ReturnObject = PrepareForLeakDetectionReturnObject;
}
// Simulate OomIntervention by purging V8 memory.
#[derive(Serialize, Debug)]
pub struct ForciblyPurgeJavaScriptMemory {}
#[derive(Deserialize, Debug, Clone)]
pub struct ForciblyPurgeJavaScriptMemoryReturnObject {}
impl super::Command for ForciblyPurgeJavaScriptMemory {
    const NAME: &'static str = "Memory.forciblyPurgeJavaScriptMemory";

    type ReturnObject = ForciblyPurgeJavaScriptMemoryReturnObject;
}
// Enable/disable suppressing memory pressure notifications in all processes.
#[derive(Serialize, Debug)]
pub struct SetPressureNotificationsSuppressed {
    // If true, memory pressure notifications will be suppressed.
    pub suppressed: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetPressureNotificationsSuppressedReturnObject {}
impl super::Command for SetPressureNotificationsSuppressed {
    const NAME: &'static str = "Memory.setPressureNotificationsSuppressed";

    type ReturnObject = SetPressureNotificationsSuppressedReturnObject;
}
// Simulate a memory pressure notification in all processes.
#[derive(Serialize, Debug)]
pub struct SimulatePressureNotification {
    // Memory pressure level of the notification.
    pub level: PressureLevel,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SimulatePressureNotificationReturnObject {}
impl super::Command for SimulatePressureNotification {
    const NAME: &'static str = "Memory.simulatePressureNotification";

    type ReturnObject = SimulatePressureNotificationReturnObject;
}
// Start collecting native memory profile.
#[derive(Serialize, Debug)]
pub struct StartSampling {
    // Average number of bytes between samples.
    pub sampling_interval: Option<i32>,
    // Do not randomize intervals between samples.
    pub suppress_randomness: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartSamplingReturnObject {}
impl super::Command for StartSampling {
    const NAME: &'static str = "Memory.startSampling";

    type ReturnObject = StartSamplingReturnObject;
}
// Stop collecting native memory profile.
#[derive(Serialize, Debug)]
pub struct StopSampling {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopSamplingReturnObject {}
impl super::Command for StopSampling {
    const NAME: &'static str = "Memory.stopSampling";

    type ReturnObject = StopSamplingReturnObject;
}
// Retrieve native memory allocations profile
// collected since renderer process startup.
#[derive(Serialize, Debug)]
pub struct GetAllTimeSamplingProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetAllTimeSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
impl super::Command for GetAllTimeSamplingProfile {
    const NAME: &'static str = "Memory.getAllTimeSamplingProfile";

    type ReturnObject = GetAllTimeSamplingProfileReturnObject;
}
// Retrieve native memory allocations profile
// collected since browser process startup.
#[derive(Serialize, Debug)]
pub struct GetBrowserSamplingProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetBrowserSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
impl super::Command for GetBrowserSamplingProfile {
    const NAME: &'static str = "Memory.getBrowserSamplingProfile";

    type ReturnObject = GetBrowserSamplingProfileReturnObject;
}
// Retrieve native memory allocations profile collected since last
// `startSampling` call.
#[derive(Serialize, Debug)]
pub struct GetSamplingProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
impl super::Command for GetSamplingProfile {
    const NAME: &'static str = "Memory.getSamplingProfile";

    type ReturnObject = GetSamplingProfileReturnObject;
}
