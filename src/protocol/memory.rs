// This file is auto-generated do not edit manually.

// Memory pressure level.
pub enum PressureLevel {
    Moderate,
    Critical,
}
// Heap profile sample.
pub struct SamplingProfileNode {
    // Size of the sampled allocation.
    pub size: f64,
    // Total bytes attributed to this sample.
    pub total: f64,
    // Execution stack at the point of allocation.
    pub stack: Vec<String>,
}
// Array of heap profile samples.
pub struct SamplingProfile {
    pub samples: Vec<SamplingProfileNode>,
    pub modules: Vec<Module>,
}
// Executable module information
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

pub struct GetDOMCounters {}
pub struct GetDOMCountersReturnObject {
    pub documents: i32,
    pub nodes: i32,
    pub js_event_listeners: i32,
}
pub struct PrepareForLeakDetection {}
pub struct PrepareForLeakDetectionReturnObject {}
// Simulate OomIntervention by purging V8 memory.
pub struct ForciblyPurgeJavaScriptMemory {}
pub struct ForciblyPurgeJavaScriptMemoryReturnObject {}
// Enable/disable suppressing memory pressure notifications in all processes.
pub struct SetPressureNotificationsSuppressed {
    // If true, memory pressure notifications will be suppressed.
    pub suppressed: bool,
}
pub struct SetPressureNotificationsSuppressedReturnObject {}
// Simulate a memory pressure notification in all processes.
pub struct SimulatePressureNotification {
    // Memory pressure level of the notification.
    pub level: PressureLevel,
}
pub struct SimulatePressureNotificationReturnObject {}
// Start collecting native memory profile.
pub struct StartSampling {
    // Average number of bytes between samples.
    pub sampling_interval: Option<i32>,
    // Do not randomize intervals between samples.
    pub suppress_randomness: Option<bool>,
}
pub struct StartSamplingReturnObject {}
// Stop collecting native memory profile.
pub struct StopSampling {}
pub struct StopSamplingReturnObject {}
// Retrieve native memory allocations profile
// collected since renderer process startup.
pub struct GetAllTimeSamplingProfile {}
pub struct GetAllTimeSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
// Retrieve native memory allocations profile
// collected since browser process startup.
pub struct GetBrowserSamplingProfile {}
pub struct GetBrowserSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
// Retrieve native memory allocations profile collected since last
// `startSampling` call.
pub struct GetSamplingProfile {}
pub struct GetSamplingProfileReturnObject {
    pub profile: SamplingProfile,
}
