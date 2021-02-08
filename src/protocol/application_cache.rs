// This file is auto-generated do not edit manually.

// Detailed application cache resource information.
pub struct ApplicationCacheResource {
    // Resource url.
    pub url: String,
    // Resource size.
    pub size: i32,
    // Resource type.
    pub r#type: String,
}
// Detailed application cache information.
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
pub struct FrameWithManifest {
    // Frame identifier.
    pub frame_id: super::page::FrameId,
    // Manifest URL.
    pub manifest_url: String,
    // Application cache status.
    pub status: i32,
}

// Enables application cache domain notifications.
pub struct Enable {}
pub struct EnableReturnObject {}
// Returns relevant application cache data for the document in given frame.
pub struct GetApplicationCacheForFrame {
    // Identifier of the frame containing document whose application cache is retrieved.
    pub frame_id: super::page::FrameId,
}
pub struct GetApplicationCacheForFrameReturnObject {
    // Relevant application cache data for the document in given frame.
    pub application_cache: ApplicationCache,
}
// Returns array of frame identifiers with manifest urls for each frame containing a document
// associated with some application cache.
pub struct GetFramesWithManifests {}
pub struct GetFramesWithManifestsReturnObject {
    // Array of frame identifiers with manifest urls for each frame containing a document
    // associated with some application cache.
    pub frame_ids: Vec<FrameWithManifest>,
}
// Returns manifest URL for document in the given frame.
pub struct GetManifestForFrame {
    // Identifier of the frame containing document whose manifest is retrieved.
    pub frame_id: super::page::FrameId,
}
pub struct GetManifestForFrameReturnObject {
    // Manifest URL for document in the given frame.
    pub manifest_url: String,
}
