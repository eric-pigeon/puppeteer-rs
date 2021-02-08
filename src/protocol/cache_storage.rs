// This file is auto-generated do not edit manually.

// Unique identifier of the Cache object.
pub type CacheId = String;
// type of HTTP response cached
pub enum CachedResponseType {
    Basic,
    Cors,
    Default,
    Error,
    OpaqueResponse,
    OpaqueRedirect,
}
// Data entry.
pub struct DataEntry {
    // Request URL.
    pub request_url: String,
    // Request method.
    pub request_method: String,
    // Request headers
    pub request_headers: Vec<Header>,
    // Number of seconds since epoch.
    pub response_time: f64,
    // HTTP response status code.
    pub response_status: i32,
    // HTTP response status text.
    pub response_status_text: String,
    // HTTP response type
    pub response_type: CachedResponseType,
    // Response headers
    pub response_headers: Vec<Header>,
}
// Cache identifier.
pub struct Cache {
    // An opaque unique id of the cache.
    pub cache_id: CacheId,
    // Security origin of the cache.
    pub security_origin: String,
    // The name of the cache.
    pub cache_name: String,
}
pub struct Header {
    pub name: String,
    pub value: String,
}
// Cached response
pub struct CachedResponse {
    // Entry content, base64-encoded.
    pub body: String,
}

// Deletes a cache.
pub struct DeleteCache {
    // Id of cache for deletion.
    pub cache_id: CacheId,
}
pub struct DeleteCacheReturnObject {}
// Deletes a cache entry.
pub struct DeleteEntry {
    // Id of cache where the entry will be deleted.
    pub cache_id: CacheId,
    // URL spec of the request.
    pub request: String,
}
pub struct DeleteEntryReturnObject {}
// Requests cache names.
pub struct RequestCacheNames {
    // Security origin.
    pub security_origin: String,
}
pub struct RequestCacheNamesReturnObject {
    // Caches for the security origin.
    pub caches: Vec<Cache>,
}
// Fetches cache entry.
pub struct RequestCachedResponse {
    // Id of cache that contains the entry.
    pub cache_id: CacheId,
    // URL spec of the request.
    pub request_url: String,
    // headers of the request.
    pub request_headers: Vec<Header>,
}
pub struct RequestCachedResponseReturnObject {
    // Response read from the cache.
    pub response: CachedResponse,
}
// Requests data from cache.
pub struct RequestEntries {
    // ID of cache to get entries from.
    pub cache_id: CacheId,
    // Number of records to skip.
    pub skip_count: Option<i32>,
    // Number of records to fetch.
    pub page_size: Option<i32>,
    // If present, only return the entries containing this substring in the path
    pub path_filter: Option<String>,
}
pub struct RequestEntriesReturnObject {
    // Array of object store data entries.
    pub cache_data_entries: Vec<DataEntry>,
    // Count of returned entries from this storage. If pathFilter is empty, it
    // is the count of all entries from this storage.
    pub return_count: f64,
}
