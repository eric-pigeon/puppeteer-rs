// This file is auto-generated do not edit manually.

// Database with an array of object stores.
pub struct DatabaseWithObjectStores {
    // Database name.
    pub name: String,
    // Database version (type is not 'integer', as the standard
    // requires the version number to be 'unsigned long long')
    pub version: f64,
    // Object stores in this database.
    pub object_stores: Vec<ObjectStore>,
}
// Object store.
pub struct ObjectStore {
    // Object store name.
    pub name: String,
    // Object store key path.
    pub key_path: KeyPath,
    // If true, object store has auto increment flag set.
    pub auto_increment: bool,
    // Indexes in this object store.
    pub indexes: Vec<ObjectStoreIndex>,
}
// Object store index.
pub struct ObjectStoreIndex {
    // Index name.
    pub name: String,
    // Index key path.
    pub key_path: KeyPath,
    // If true, index is unique.
    pub unique: bool,
    // If true, index allows multiple entries for a key.
    pub multi_entry: bool,
}
pub enum KeyType {
    Number,
    String,
    Date,
    Array,
}
// Key.
pub struct Key {
    // Key type.
    pub r#type: KeyType,
    // Number value.
    pub number: Option<f64>,
    // String value.
    pub string: Option<String>,
    // Date value.
    pub date: Option<f64>,
    // Array value.
    pub array: Option<Vec<Key>>,
}
// Key range.
pub struct KeyRange {
    // Lower bound.
    pub lower: Option<Key>,
    // Upper bound.
    pub upper: Option<Key>,
    // If true lower bound is open.
    pub lower_open: bool,
    // If true upper bound is open.
    pub upper_open: bool,
}
// Data entry.
pub struct DataEntry {
    // Key object.
    pub key: super::runtime::RemoteObject,
    // Primary key object.
    pub primary_key: super::runtime::RemoteObject,
    // Value object.
    pub value: super::runtime::RemoteObject,
}
pub enum KeyPathType {
    Null,
    String,
    Array,
}
// Key path.
pub struct KeyPath {
    // Key path type.
    pub r#type: KeyPathType,
    // String value.
    pub string: Option<String>,
    // Array value.
    pub array: Option<Vec<String>>,
}

// Clears all entries from an object store.
pub struct ClearObjectStore {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
    // Object store name.
    pub object_store_name: String,
}
pub struct ClearObjectStoreReturnObject {}
// Deletes a database.
pub struct DeleteDatabase {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
}
pub struct DeleteDatabaseReturnObject {}
// Delete a range of entries from an object store
pub struct DeleteObjectStoreEntries {
    pub security_origin: String,
    pub database_name: String,
    pub object_store_name: String,
    // Range of entry keys to delete
    pub key_range: KeyRange,
}
pub struct DeleteObjectStoreEntriesReturnObject {}
// Disables events from backend.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables events from backend.
pub struct Enable {}
pub struct EnableReturnObject {}
// Requests data from object store or index.
pub struct RequestData {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
    // Object store name.
    pub object_store_name: String,
    // Index name, empty string for object store data requests.
    pub index_name: String,
    // Number of records to skip.
    pub skip_count: i32,
    // Number of records to fetch.
    pub page_size: i32,
    // Key range.
    pub key_range: Option<KeyRange>,
}
pub struct RequestDataReturnObject {
    // Array of object store data entries.
    pub object_store_data_entries: Vec<DataEntry>,
    // If true, there are more entries to fetch in the given range.
    pub has_more: bool,
}
// Gets metadata of an object store
pub struct GetMetadata {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
    // Object store name.
    pub object_store_name: String,
}
pub struct GetMetadataReturnObject {
    // the entries count
    pub entries_count: f64,
    // the current value of key generator, to become the next inserted
    // key into the object store. Valid if objectStore.autoIncrement
    // is true.
    pub key_generator_value: f64,
}
// Requests database with given name in given frame.
pub struct RequestDatabase {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
}
pub struct RequestDatabaseReturnObject {
    // Database with an array of object stores.
    pub database_with_object_stores: DatabaseWithObjectStores,
}
// Requests database names for given security origin.
pub struct RequestDatabaseNames {
    // Security origin.
    pub security_origin: String,
}
pub struct RequestDatabaseNamesReturnObject {
    // Database names for origin.
    pub database_names: Vec<String>,
}
