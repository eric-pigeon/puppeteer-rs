// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Detailed application cache resource information.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationCacheResource {
    // Resource url.
    pub url: String,
    // Resource size.
    pub size: i32,
    // Resource type.
    pub r#type: String,
}
// Detailed application cache information.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationCache {
    // Manifest URL.
    pub manifest_url: String,
    // Application cache size.
    pub size: f64,
    // Application cache creation time.
    pub creation_time: f64,
    // Application cache update time.
    pub update_time: f64,
    // Application cache resources.
    pub resources: Vec<ApplicationCacheResource>,
}
// Frame identifier - manifest URL pair.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameWithManifest {
    // Frame identifier.
    pub frame_id: super::page::FrameId,
    // Manifest URL.
    pub manifest_url: String,
    // Application cache status.
    pub status: i32,
}

// Enables application cache domain notifications.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "ApplicationCache.enable";

    type ReturnObject = EnableReturnObject;
}
// Returns relevant application cache data for the document in given frame.
#[derive(Serialize, Debug)]
pub struct GetApplicationCacheForFrame {
    // Identifier of the frame containing document whose application cache is retrieved.
    pub frame_id: super::page::FrameId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetApplicationCacheForFrameReturnObject {
    // Relevant application cache data for the document in given frame.
    pub application_cache: ApplicationCache,
}
impl super::Command for GetApplicationCacheForFrame {
    const NAME: &'static str = "ApplicationCache.getApplicationCacheForFrame";

    type ReturnObject = GetApplicationCacheForFrameReturnObject;
}
// Returns array of frame identifiers with manifest urls for each frame containing a document
// associated with some application cache.
#[derive(Serialize, Debug)]
pub struct GetFramesWithManifests {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetFramesWithManifestsReturnObject {
    // Array of frame identifiers with manifest urls for each frame containing a document
    // associated with some application cache.
    pub frame_ids: Vec<FrameWithManifest>,
}
impl super::Command for GetFramesWithManifests {
    const NAME: &'static str = "ApplicationCache.getFramesWithManifests";

    type ReturnObject = GetFramesWithManifestsReturnObject;
}
// Returns manifest URL for document in the given frame.
#[derive(Serialize, Debug)]
pub struct GetManifestForFrame {
    // Identifier of the frame containing document whose manifest is retrieved.
    pub frame_id: super::page::FrameId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetManifestForFrameReturnObject {
    // Manifest URL for document in the given frame.
    pub manifest_url: String,
}
impl super::Command for GetManifestForFrame {
    const NAME: &'static str = "ApplicationCache.getManifestForFrame";

    type ReturnObject = GetManifestForFrameReturnObject;
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationCacheStatusUpdated {
    // Identifier of the frame containing document whose application cache updated status.
    pub frame_id: super::page::FrameId,
    // Manifest URL.
    pub manifest_url: String,
    // Updated application cache status.
    pub status: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct NetworkStateUpdated {
    pub is_now_online: bool,
}
