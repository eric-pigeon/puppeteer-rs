// This file is auto-generated do not edit manually.

// Enum of possible storage types.
pub enum StorageType {
    Appcache,
    Cookies,
    Filesystems,
    Indexeddb,
    Localstorage,
    Shadercache,
    Websql,
    Serviceworkers,
    Cachestorage,
    All,
    Other,
}
// Usage for a storage type.
pub struct UsageForType {
    // Name of storage type.
    pub storage_type: StorageType,
    // Storage usage (bytes).
    pub usage: f64,
}

// Clears storage for origin.
pub struct ClearDataForOrigin {
    // Security origin.
    pub origin: String,
    // Comma separated list of StorageType to clear.
    pub storage_types: String,
}
pub struct ClearDataForOriginReturnObject {}
// Returns all browser cookies.
pub struct GetCookies {
    // Browser context to use when called on the browser endpoint.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
pub struct GetCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<super::network::Cookie>,
}
// Sets given cookies.
pub struct SetCookies {
    // Cookies to be set.
    pub cookies: Vec<super::network::CookieParam>,
    // Browser context to use when called on the browser endpoint.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
pub struct SetCookiesReturnObject {}
// Clears cookies.
pub struct ClearCookies {
    // Browser context to use when called on the browser endpoint.
    pub browser_context_id: Option<super::browser::BrowserContextID>,
}
pub struct ClearCookiesReturnObject {}
// Returns usage and quota in bytes.
pub struct GetUsageAndQuota {
    // Security origin.
    pub origin: String,
}
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
// Override quota for the specified origin
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
pub struct OverrideQuotaForOriginReturnObject {}
// Registers origin to be notified when an update occurs to its cache storage list.
pub struct TrackCacheStorageForOrigin {
    // Security origin.
    pub origin: String,
}
pub struct TrackCacheStorageForOriginReturnObject {}
// Registers origin to be notified when an update occurs to its IndexedDB.
pub struct TrackIndexedDBForOrigin {
    // Security origin.
    pub origin: String,
}
pub struct TrackIndexedDBForOriginReturnObject {}
// Unregisters origin from receiving notifications for cache storage.
pub struct UntrackCacheStorageForOrigin {
    // Security origin.
    pub origin: String,
}
pub struct UntrackCacheStorageForOriginReturnObject {}
// Unregisters origin from receiving notifications for IndexedDB.
pub struct UntrackIndexedDBForOrigin {
    // Security origin.
    pub origin: String,
}
pub struct UntrackIndexedDBForOriginReturnObject {}
