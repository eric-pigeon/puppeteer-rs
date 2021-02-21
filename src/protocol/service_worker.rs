// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

pub type RegistrationID = String;
// ServiceWorker registration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRegistration {
    pub registration_id: RegistrationID,
    pub scope_url: String,
    pub is_deleted: bool,
}
pub type ServiceWorkerVersionRunningStatus = String;
pub type ServiceWorkerVersionStatus = String;
// ServiceWorker version.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerErrorMessage {
    pub error_message: String,
    pub registration_id: RegistrationID,
    pub version_id: String,
    pub source_url: String,
    pub line_number: i32,
    pub column_number: i32,
}

#[derive(Serialize, Debug)]
pub struct DeliverPushMessage {
    pub origin: String,
    pub registration_id: RegistrationID,
    pub data: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DeliverPushMessageReturnObject {}
impl super::Command for DeliverPushMessage {
    const NAME: &'static str = "ServiceWorker.deliverPushMessage";

    type ReturnObject = DeliverPushMessageReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "ServiceWorker.disable";

    type ReturnObject = DisableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct DispatchSyncEvent {
    pub origin: String,
    pub registration_id: RegistrationID,
    pub tag: String,
    pub last_chance: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DispatchSyncEventReturnObject {}
impl super::Command for DispatchSyncEvent {
    const NAME: &'static str = "ServiceWorker.dispatchSyncEvent";

    type ReturnObject = DispatchSyncEventReturnObject;
}
#[derive(Serialize, Debug)]
pub struct DispatchPeriodicSyncEvent {
    pub origin: String,
    pub registration_id: RegistrationID,
    pub tag: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DispatchPeriodicSyncEventReturnObject {}
impl super::Command for DispatchPeriodicSyncEvent {
    const NAME: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";

    type ReturnObject = DispatchPeriodicSyncEventReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "ServiceWorker.enable";

    type ReturnObject = EnableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct InspectWorker {
    pub version_id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct InspectWorkerReturnObject {}
impl super::Command for InspectWorker {
    const NAME: &'static str = "ServiceWorker.inspectWorker";

    type ReturnObject = InspectWorkerReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SetForceUpdateOnPageLoad {
    pub force_update_on_page_load: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetForceUpdateOnPageLoadReturnObject {}
impl super::Command for SetForceUpdateOnPageLoad {
    const NAME: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";

    type ReturnObject = SetForceUpdateOnPageLoadReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SkipWaiting {
    pub scope_url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SkipWaitingReturnObject {}
impl super::Command for SkipWaiting {
    const NAME: &'static str = "ServiceWorker.skipWaiting";

    type ReturnObject = SkipWaitingReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StartWorker {
    pub scope_url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartWorkerReturnObject {}
impl super::Command for StartWorker {
    const NAME: &'static str = "ServiceWorker.startWorker";

    type ReturnObject = StartWorkerReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StopAllWorkers {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopAllWorkersReturnObject {}
impl super::Command for StopAllWorkers {
    const NAME: &'static str = "ServiceWorker.stopAllWorkers";

    type ReturnObject = StopAllWorkersReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StopWorker {
    pub version_id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StopWorkerReturnObject {}
impl super::Command for StopWorker {
    const NAME: &'static str = "ServiceWorker.stopWorker";

    type ReturnObject = StopWorkerReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Unregister {
    pub scope_url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UnregisterReturnObject {}
impl super::Command for Unregister {
    const NAME: &'static str = "ServiceWorker.unregister";

    type ReturnObject = UnregisterReturnObject;
}
#[derive(Serialize, Debug)]
pub struct UpdateRegistration {
    pub scope_url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateRegistrationReturnObject {}
impl super::Command for UpdateRegistration {
    const NAME: &'static str = "ServiceWorker.updateRegistration";

    type ReturnObject = UpdateRegistrationReturnObject;
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WorkerErrorReportedEvent {
    pub params: WorkerErrorReportedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WorkerErrorReportedParams {
    pub error_message: ServiceWorkerErrorMessage,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WorkerRegistrationUpdatedEvent {
    pub params: WorkerRegistrationUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRegistrationUpdatedParams {
    pub registrations: Vec<ServiceWorkerRegistration>,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WorkerVersionUpdatedEvent {
    pub params: WorkerVersionUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WorkerVersionUpdatedParams {
    pub versions: Vec<ServiceWorkerVersion>,
}
