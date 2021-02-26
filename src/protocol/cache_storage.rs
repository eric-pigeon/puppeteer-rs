// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Unique identifier of the Cache object.
pub type CacheId = String;
// type of HTTP response cached
pub type CachedResponseType = String;
// Data entry.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    // An opaque unique id of the cache.
    pub cache_id: CacheId,
    // Security origin of the cache.
    pub security_origin: String,
    // The name of the cache.
    pub cache_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}
// Cached response
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CachedResponse {
    // Entry content, base64-encoded.
    pub body: String,
}

// Deletes a cache.
#[derive(Serialize, Debug)]
pub struct DeleteCache {
    // Id of cache for deletion.
    pub cache_id: CacheId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCacheReturnObject {}
impl super::Command for DeleteCache {
    const NAME: &'static str = "CacheStorage.deleteCache";

    type ReturnObject = DeleteCacheReturnObject;
}
// Deletes a cache entry.
#[derive(Serialize, Debug)]
pub struct DeleteEntry {
    // Id of cache where the entry will be deleted.
    pub cache_id: CacheId,
    // URL spec of the request.
    pub request: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntryReturnObject {}
impl super::Command for DeleteEntry {
    const NAME: &'static str = "CacheStorage.deleteEntry";

    type ReturnObject = DeleteEntryReturnObject;
}
// Requests cache names.
#[derive(Serialize, Debug)]
pub struct RequestCacheNames {
    // Security origin.
    pub security_origin: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestCacheNamesReturnObject {
    // Caches for the security origin.
    pub caches: Vec<Cache>,
}
impl super::Command for RequestCacheNames {
    const NAME: &'static str = "CacheStorage.requestCacheNames";

    type ReturnObject = RequestCacheNamesReturnObject;
}
// Fetches cache entry.
#[derive(Serialize, Debug)]
pub struct RequestCachedResponse {
    // Id of cache that contains the entry.
    pub cache_id: CacheId,
    // URL spec of the request.
    pub request_url: String,
    // headers of the request.
    pub request_headers: Vec<Header>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestCachedResponseReturnObject {
    // Response read from the cache.
    pub response: CachedResponse,
}
impl super::Command for RequestCachedResponse {
    const NAME: &'static str = "CacheStorage.requestCachedResponse";

    type ReturnObject = RequestCachedResponseReturnObject;
}
// Requests data from cache.
#[derive(Serialize, Debug)]
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
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesReturnObject {
    // Array of object store data entries.
    pub cache_data_entries: Vec<DataEntry>,
    // Count of returned entries from this storage. If pathFilter is empty, it
    // is the count of all entries from this storage.
    pub return_count: f64,
}
impl super::Command for RequestEntries {
    const NAME: &'static str = "CacheStorage.requestEntries";

    type ReturnObject = RequestEntriesReturnObject;
}
