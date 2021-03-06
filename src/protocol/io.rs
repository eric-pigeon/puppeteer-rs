// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// This is either obtained from another method or specifed as `blob:&lt;uuid&gt;` where
// `&lt;uuid&gt` is an UUID of a Blob.
pub type StreamHandle = String;

// Close the stream, discard any temporary backing storage.
#[derive(Serialize, Debug)]
pub struct Close {
    // Handle of the stream to close.
    pub handle: StreamHandle,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CloseReturnObject {}
impl super::Command for Close {
    const NAME: &'static str = "IO.close";

    type ReturnObject = CloseReturnObject;
}
// Read a chunk of the stream
#[derive(Serialize, Debug)]
pub struct Read {
    // Handle of the stream to read.
    pub handle: StreamHandle,
    // Seek to the specified offset before reading (if not specificed, proceed with offset
    // following the last read). Some types of streams may only support sequential reads.
    pub offset: Option<i32>,
    // Maximum number of bytes to read (left upon the agent discretion if not specified).
    pub size: Option<i32>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadReturnObject {
    // Set if the data is base64-encoded
    pub base64_encoded: Option<bool>,
    // Data that were read.
    pub data: String,
    // Set if the end-of-file condition occured while reading.
    pub eof: bool,
}
impl super::Command for Read {
    const NAME: &'static str = "IO.read";

    type ReturnObject = ReadReturnObject;
}
// Return UUID of Blob object specified by a remote object id.
#[derive(Serialize, Debug)]
pub struct ResolveBlob {
    // Object id of a Blob object wrapper.
    pub object_id: super::runtime::RemoteObjectId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResolveBlobReturnObject {
    // UUID of the specified Blob.
    pub uuid: String,
}
impl super::Command for ResolveBlob {
    const NAME: &'static str = "IO.resolveBlob";

    type ReturnObject = ResolveBlobReturnObject;
}
