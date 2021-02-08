// This file is auto-generated do not edit manually.

// The Background Service that will be associated with the commands/events.
// Every Background Service operates independently, but they share the same
// API.
pub enum ServiceName {
    BackgroundFetch,
    BackgroundSync,
    PushMessaging,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
}
// A key-value pair for additional event information to pass along.
pub struct EventMetadata {
    pub key: String,
    pub value: String,
}
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
pub struct StartObserving {
    pub service: ServiceName,
}
pub struct StartObservingReturnObject {}
// Disables event updates for the service.
pub struct StopObserving {
    pub service: ServiceName,
}
pub struct StopObservingReturnObject {}
// Set the recording state for the service.
pub struct SetRecording {
    pub should_record: bool,
    pub service: ServiceName,
}
pub struct SetRecordingReturnObject {}
// Clears all stored data for the service.
pub struct ClearEvents {
    pub service: ServiceName,
}
pub struct ClearEventsReturnObject {}
