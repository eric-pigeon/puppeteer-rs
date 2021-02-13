// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// The Background Service that will be associated with the commands/events.
// Every Background Service operates independently, but they share the same
// API.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ServiceName {
    BackgroundFetch,
    BackgroundSync,
    PushMessaging,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
}
// A key-value pair for additional event information to pass along.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventMetadata {
    pub key: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundServiceEvent {
    // Timestamp of the event (in seconds).
    pub timestamp: super::network::TimeSinceEpoch,
    // The origin this event belongs to.
    pub origin: String,
    // The Service Worker ID that initiated the event.
    pub service_worker_registration_id: super::service_worker::RegistrationID,
    // The Background Service this event belongs to.
    pub service: ServiceName,
    // A description of the event.
    pub event_name: String,
    // An identifier that groups related events together.
    pub instance_id: String,
    // A list of event-specific information.
    pub event_metadata: Vec<EventMetadata>,
}

// Enables event updates for the service.
#[derive(Serialize, Debug)]
pub struct StartObserving {
    pub service: ServiceName,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartObservingReturnObject {}
impl super::Command for StartObserving {
    const NAME: &'static str = "BackgroundService.startObserving";

    type ReturnObject = StartObservingReturnObject;
}
// Disables event updates for the service.
#[derive(Serialize, Debug)]
pub struct StopObserving {
    pub service: ServiceName,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StopObservingReturnObject {}
impl super::Command for StopObserving {
    const NAME: &'static str = "BackgroundService.stopObserving";

    type ReturnObject = StopObservingReturnObject;
}
// Set the recording state for the service.
#[derive(Serialize, Debug)]
pub struct SetRecording {
    pub should_record: bool,
    pub service: ServiceName,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetRecordingReturnObject {}
impl super::Command for SetRecording {
    const NAME: &'static str = "BackgroundService.setRecording";

    type ReturnObject = SetRecordingReturnObject;
}
// Clears all stored data for the service.
#[derive(Serialize, Debug)]
pub struct ClearEvents {
    pub service: ServiceName,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearEventsReturnObject {}
impl super::Command for ClearEvents {
    const NAME: &'static str = "BackgroundService.clearEvents";

    type ReturnObject = ClearEventsReturnObject;
}
