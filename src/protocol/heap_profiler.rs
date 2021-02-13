// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Heap snapshot object id.
pub type HeapSnapshotObjectId = String;
// Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileNode {
    // Function location.
    pub call_frame: super::runtime::CallFrame,
    // Allocations size in bytes for the node excluding children.
    pub self_size: f64,
    // Node id. Ids are unique across all profiles collected between startSampling and stopSampling.
    pub id: i32,
    // Child nodes.
    pub children: Vec<SamplingHeapProfileNode>,
}
// A single sample from a sampling profile.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileSample {
    // Allocation size in bytes attributed to the sample.
    pub size: f64,
    // Id of the corresponding profile tree node.
    pub node_id: i32,
    // Time-ordered sample ordinal number. It is unique across all profiles retrieved
    // between startSampling and stopSampling.
    pub ordinal: f64,
}
// Sampling profile.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfile {
    pub head: SamplingHeapProfileNode,
    pub samples: Vec<SamplingHeapProfileSample>,
}

// Enables console to refer to the node with given id via $x (see Command Line API for more details
// $x functions).
#[derive(Serialize, Debug)]
pub struct AddInspectedHeapObject {
    // Heap snapshot object id to be accessible by means of $x command line API.
    pub heap_object_id: HeapSnapshotObjectId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct AddInspectedHeapObjectReturnObject {}
impl super::Command for AddInspectedHeapObject {
    const NAME: &'static str = "HeapProfiler.addInspectedHeapObject";

    type ReturnObject = AddInspectedHeapObjectReturnObject;
}
#[derive(Serialize, Debug)]
pub struct CollectGarbage {}
#[derive(Deserialize, Debug, Clone)]
pub struct CollectGarbageReturnObject {}
impl super::Command for CollectGarbage {
    const NAME: &'static str = "HeapProfiler.collectGarbage";

    type ReturnObject = CollectGarbageReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "HeapProfiler.disable";

    type ReturnObject = DisableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "HeapProfiler.enable";

    type ReturnObject = EnableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetHeapObjectId {
    // Identifier of the object to get heap object id for.
    pub object_id: super::runtime::RemoteObjectId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetHeapObjectIdReturnObject {
    // Id of the heap snapshot object corresponding to the passed remote object id.
    pub heap_snapshot_object_id: HeapSnapshotObjectId,
}
impl super::Command for GetHeapObjectId {
    const NAME: &'static str = "HeapProfiler.getHeapObjectId";

    type ReturnObject = GetHeapObjectIdReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetObjectByHeapObjectId {
    pub object_id: HeapSnapshotObjectId,
    // Symbolic group name that can be used to release multiple objects.
    pub object_group: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetObjectByHeapObjectIdReturnObject {
    // Evaluation result.
    pub result: super::runtime::RemoteObject,
}
impl super::Command for GetObjectByHeapObjectId {
    const NAME: &'static str = "HeapProfiler.getObjectByHeapObjectId";

    type ReturnObject = GetObjectByHeapObjectIdReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetSamplingProfile {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetSamplingProfileReturnObject {
    // Return the sampling profile being collected.
    pub profile: SamplingHeapProfile,
}
impl super::Command for GetSamplingProfile {
    const NAME: &'static str = "HeapProfiler.getSamplingProfile";

    type ReturnObject = GetSamplingProfileReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StartSampling {
    // Average sample interval in bytes. Poisson distribution is used for the intervals. The
    // default value is 32768 bytes.
    pub sampling_interval: Option<f64>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartSamplingReturnObject {}
impl super::Command for StartSampling {
    const NAME: &'static str = "HeapProfiler.startSampling";

    type ReturnObject = StartSamplingReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StartTrackingHeapObjects {
    pub track_allocations: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StartTrackingHeapObjectsReturnObject {}
impl super::Command for StartTrackingHeapObjects {
    const NAME: &'static str = "HeapProfiler.startTrackingHeapObjects";

    type ReturnObject = StartTrackingHeapObjectsReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StopSampling {}
#[derive(Deserialize, Debug, Clone)]
pub struct StopSamplingReturnObject {
    // Recorded sampling heap profile.
    pub profile: SamplingHeapProfile,
}
impl super::Command for StopSampling {
    const NAME: &'static str = "HeapProfiler.stopSampling";

    type ReturnObject = StopSamplingReturnObject;
}
#[derive(Serialize, Debug)]
pub struct StopTrackingHeapObjects {
    // If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    // when the tracking is stopped.
    pub report_progress: Option<bool>,
    pub treat_global_objects_as_roots: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct StopTrackingHeapObjectsReturnObject {}
impl super::Command for StopTrackingHeapObjects {
    const NAME: &'static str = "HeapProfiler.stopTrackingHeapObjects";

    type ReturnObject = StopTrackingHeapObjectsReturnObject;
}
#[derive(Serialize, Debug)]
pub struct TakeHeapSnapshot {
    // If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.
    pub report_progress: Option<bool>,
    // If true, a raw snapshot without artifical roots will be generated
    pub treat_global_objects_as_roots: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct TakeHeapSnapshotReturnObject {}
impl super::Command for TakeHeapSnapshot {
    const NAME: &'static str = "HeapProfiler.takeHeapSnapshot";

    type ReturnObject = TakeHeapSnapshotReturnObject;
}
