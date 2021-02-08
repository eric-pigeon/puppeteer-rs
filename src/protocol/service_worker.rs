// This file is auto-generated do not edit manually.

pub type RegistrationID = String;
// ServiceWorker registration.
pub struct ServiceWorkerRegistration {
    pub registration_id: RegistrationID,
    pub scope_url: String,
    pub is_deleted: bool,
}
pub enum ServiceWorkerVersionRunningStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
}
pub enum ServiceWorkerVersionStatus {
    New,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
}
// ServiceWorker version.
pub struct ServiceWorkerVersion {
    pub version_id: String,
    pub registration_id: RegistrationID,
    pub script_url: String,
    pub running_status: ServiceWorkerVersionRunningStatus,
    pub status: ServiceWorkerVersionStatus,
    // The Last-Modified header value of the main script.
    pub script_last_modified: Option<f64>,
    // The time at which the response headers of the main script were received from the server.
    // For cached script it is the last time the cache entry was validated.
    pub script_response_time: Option<f64>,
    pub controlled_clients: Option<Vec<super::target::TargetID>>,
    pub target_id: Option<super::target::TargetID>,
}
// ServiceWorker error message.
pub struct ServiceWorkerErrorMessage {
    pub error_message: String,
    pub registration_id: RegistrationID,
    pub version_id: String,
    pub source_url: String,
    pub line_number: i32,
    pub column_number: i32,
}

pub struct DeliverPushMessage {
    pub origin: String,
    pub registration_id: RegistrationID,
    pub data: String,
}
pub struct DeliverPushMessageReturnObject {}
pub struct Disable {}
pub struct DisableReturnObject {}
pub struct DispatchSyncEvent {
    pub origin: String,
    pub registration_id: RegistrationID,
    pub tag: String,
    pub last_chance: bool,
}
pub struct DispatchSyncEventReturnObject {}
pub struct DispatchPeriodicSyncEvent {
    pub origin: String,
    pub registration_id: RegistrationID,
    pub tag: String,
}
pub struct DispatchPeriodicSyncEventReturnObject {}
pub struct Enable {}
pub struct EnableReturnObject {}
pub struct InspectWorker {
    pub version_id: String,
}
pub struct InspectWorkerReturnObject {}
pub struct SetForceUpdateOnPageLoad {
    pub force_update_on_page_load: bool,
}
pub struct SetForceUpdateOnPageLoadReturnObject {}
pub struct SkipWaiting {
    pub scope_url: String,
}
pub struct SkipWaitingReturnObject {}
pub struct StartWorker {
    pub scope_url: String,
}
pub struct StartWorkerReturnObject {}
pub struct StopAllWorkers {}
pub struct StopAllWorkersReturnObject {}
pub struct StopWorker {
    pub version_id: String,
}
pub struct StopWorkerReturnObject {}
pub struct Unregister {
    pub scope_url: String,
}
pub struct UnregisterReturnObject {}
pub struct UpdateRegistration {
    pub scope_url: String,
}
pub struct UpdateRegistrationReturnObject {}
