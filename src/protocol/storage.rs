// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Enum of possible storage types.
pub type StorageType = String;
// Usage for a storage type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsageForType {
    // Name of storage type.
    pub storage_type: StorageType,
    // Storage usage (bytes).
    pub usage: f64,
}

// Clears storage for origin.
#[derive(Serialize, Debug)]
pub struct ClearDataForOrigin {
    // Security origin.
    pub origin: String,
    // Comma separated list of StorageType to clear.
    pub storage_types: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearDataForOriginReturnObject {}
impl super::Command for ClearDataForOrigin {
    const NAME: &'static str = "Storage.clearDataForOrigin";

    type ReturnObject = ClearDataForOriginReturnObject;
}
// Returns all browser cookies.
#[derive(Serialize, Debug)]
pub struct GetCookies {
    // Browser context to use when called on the browser endpoint.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<super::network::Cookie>,
}
impl super::Command for GetCookies {
    const NAME: &'static str = "Storage.getCookies";

    type ReturnObject = GetCookiesReturnObject;
}
// Sets given cookies.
#[derive(Serialize, Debug)]
pub struct SetCookies {
    // Cookies to be set.
    pub cookies: Vec<super::network::CookieParam>,
    // Browser context to use when called on the browser endpoint.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetCookiesReturnObject {}
impl super::Command for SetCookies {
    const NAME: &'static str = "Storage.setCookies";

    type ReturnObject = SetCookiesReturnObject;
}
// Clears cookies.
#[derive(Serialize, Debug)]
pub struct ClearCookies {
    // Browser context to use when called on the browser endpoint.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ClearCookiesReturnObject {}
impl super::Command for ClearCookies {
    const NAME: &'static str = "Storage.clearCookies";

    type ReturnObject = ClearCookiesReturnObject;
}
// Returns usage and quota in bytes.
#[derive(Serialize, Debug)]
pub struct GetUsageAndQuota {
    // Security origin.
    pub origin: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetUsageAndQuotaReturnObject {
    // Storage usage (bytes).
    pub usage: f64,
    // Storage quota (bytes).
    pub quota: f64,
    // Whether or not the origin has an active storage quota override
    pub override_active: bool,
    // Storage usage per type (bytes).
    pub usage_breakdown: Vec<UsageForType>,
}
impl super::Command for GetUsageAndQuota {
    const NAME: &'static str = "Storage.getUsageAndQuota";

    type ReturnObject = GetUsageAndQuotaReturnObject;
}
// Override quota for the specified origin
#[derive(Serialize, Debug)]
pub struct OverrideQuotaForOrigin {
    // Security origin.
    pub origin: String,
    // The quota size (in bytes) to override the original quota with.
    // If this is called multiple times, the overriden quota will be equal to
    // the quotaSize provided in the final call. If this is called without
    // specifying a quotaSize, the quota will be reset to the default value for
    // the specified origin. If this is called multiple times with different
    // origins, the override will be maintained for each origin until it is
    // disabled (called without a quotaSize).
    pub quota_size: Option<f64>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct OverrideQuotaForOriginReturnObject {}
impl super::Command for OverrideQuotaForOrigin {
    const NAME: &'static str = "Storage.overrideQuotaForOrigin";

    type ReturnObject = OverrideQuotaForOriginReturnObject;
}
// Registers origin to be notified when an update occurs to its cache storage list.
#[derive(Serialize, Debug)]
pub struct TrackCacheStorageForOrigin {
    // Security origin.
    pub origin: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct TrackCacheStorageForOriginReturnObject {}
impl super::Command for TrackCacheStorageForOrigin {
    const NAME: &'static str = "Storage.trackCacheStorageForOrigin";

    type ReturnObject = TrackCacheStorageForOriginReturnObject;
}
// Registers origin to be notified when an update occurs to its IndexedDB.
#[derive(Serialize, Debug)]
pub struct TrackIndexedDBForOrigin {
    // Security origin.
    pub origin: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct TrackIndexedDBForOriginReturnObject {}
impl super::Command for TrackIndexedDBForOrigin {
    const NAME: &'static str = "Storage.trackIndexedDBForOrigin";

    type ReturnObject = TrackIndexedDBForOriginReturnObject;
}
// Unregisters origin from receiving notifications for cache storage.
#[derive(Serialize, Debug)]
pub struct UntrackCacheStorageForOrigin {
    // Security origin.
    pub origin: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UntrackCacheStorageForOriginReturnObject {}
impl super::Command for UntrackCacheStorageForOrigin {
    const NAME: &'static str = "Storage.untrackCacheStorageForOrigin";

    type ReturnObject = UntrackCacheStorageForOriginReturnObject;
}
// Unregisters origin from receiving notifications for IndexedDB.
#[derive(Serialize, Debug)]
pub struct UntrackIndexedDBForOrigin {
    // Security origin.
    pub origin: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UntrackIndexedDBForOriginReturnObject {}
impl super::Command for UntrackIndexedDBForOrigin {
    const NAME: &'static str = "Storage.untrackIndexedDBForOrigin";

    type ReturnObject = UntrackIndexedDBForOriginReturnObject;
}

// A cache's contents have been modified.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CacheStorageContentUpdatedEvent {
    pub params: CacheStorageContentUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CacheStorageContentUpdatedParams {
    // Origin to update.
    pub origin: String,
    // Name of cache in origin.
    pub cache_name: String,
}
// A cache has been added/deleted.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CacheStorageListUpdatedEvent {
    pub params: CacheStorageListUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CacheStorageListUpdatedParams {
    // Origin to update.
    pub origin: String,
}
// The origin's IndexedDB object store has been modified.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IndexedDBContentUpdatedEvent {
    pub params: IndexedDBContentUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexedDBContentUpdatedParams {
    // Origin to update.
    pub origin: String,
    // Database to update.
    pub database_name: String,
    // ObjectStore to update.
    pub object_store_name: String,
}
// The origin's IndexedDB database list has been modified.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IndexedDBListUpdatedEvent {
    pub params: IndexedDBListUpdatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexedDBListUpdatedParams {
    // Origin to update.
    pub origin: String,
}
