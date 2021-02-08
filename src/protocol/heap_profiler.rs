// This file is auto-generated do not edit manually.

// Heap snapshot object id.
pub type HeapSnapshotObjectId = String;
// Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
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
pub struct SamplingHeapProfile {
    pub head: SamplingHeapProfileNode,
    pub samples: Vec<SamplingHeapProfileSample>,
}

// Enables console to refer to the node with given id via $x (see Command Line API for more details
// $x functions).
pub struct AddInspectedHeapObject {
    // Heap snapshot object id to be accessible by means of $x command line API.
    pub heap_object_id: HeapSnapshotObjectId,
}
pub struct AddInspectedHeapObjectReturnObject {}
pub struct CollectGarbage {}
pub struct CollectGarbageReturnObject {}
pub struct Disable {}
pub struct DisableReturnObject {}
pub struct Enable {}
pub struct EnableReturnObject {}
pub struct GetHeapObjectId {
    // Identifier of the object to get heap object id for.
    pub object_id: super::runtime::RemoteObjectId,
}
pub struct GetHeapObjectIdReturnObject {
    // Id of the heap snapshot object corresponding to the passed remote object id.
    pub heap_snapshot_object_id: HeapSnapshotObjectId,
}
pub struct GetObjectByHeapObjectId {
    pub object_id: HeapSnapshotObjectId,
    // Symbolic group name that can be used to release multiple objects.
    pub object_group: Option<String>,
}
pub struct GetObjectByHeapObjectIdReturnObject {
    // Evaluation result.
    pub result: super::runtime::RemoteObject,
}
pub struct GetSamplingProfile {}
pub struct GetSamplingProfileReturnObject {
    // Return the sampling profile being collected.
    pub profile: SamplingHeapProfile,
}
pub struct StartSampling {
    // Average sample interval in bytes. Poisson distribution is used for the intervals. The
    // default value is 32768 bytes.
    pub sampling_interval: Option<f64>,
}
pub struct StartSamplingReturnObject {}
pub struct StartTrackingHeapObjects {
    pub track_allocations: Option<bool>,
}
pub struct StartTrackingHeapObjectsReturnObject {}
pub struct StopSampling {}
pub struct StopSamplingReturnObject {
    // Recorded sampling heap profile.
    pub profile: SamplingHeapProfile,
}
pub struct StopTrackingHeapObjects {
    // If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    // when the tracking is stopped.
    pub report_progress: Option<bool>,
    pub treat_global_objects_as_roots: Option<bool>,
}
pub struct StopTrackingHeapObjectsReturnObject {}
pub struct TakeHeapSnapshot {
    // If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.
    pub report_progress: Option<bool>,
    // If true, a raw snapshot without artifical roots will be generated
    pub treat_global_objects_as_roots: Option<bool>,
}
pub struct TakeHeapSnapshotReturnObject {}
