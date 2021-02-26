// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Database with an array of object stores.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum KeyType {
    Number,
    String,
    Date,
    Array,
}
// Key.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    // Key type.
    pub r#type: String,
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry {
    // Key object.
    pub key: super::runtime::RemoteObject,
    // Primary key object.
    pub primary_key: super::runtime::RemoteObject,
    // Value object.
    pub value: super::runtime::RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum KeyPathType {
    Null,
    String,
    Array,
}
// Key path.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyPath {
    // Key path type.
    pub r#type: String,
    // String value.
    pub string: Option<String>,
    // Array value.
    pub array: Option<Vec<String>>,
}

// Clears all entries from an object store.
#[derive(Serialize, Debug)]
pub struct ClearObjectStore {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
    // Object store name.
    pub object_store_name: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearObjectStoreReturnObject {}
impl super::Command for ClearObjectStore {
    const NAME: &'static str = "IndexedDB.clearObjectStore";

    type ReturnObject = ClearObjectStoreReturnObject;
}
// Deletes a database.
#[derive(Serialize, Debug)]
pub struct DeleteDatabase {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatabaseReturnObject {}
impl super::Command for DeleteDatabase {
    const NAME: &'static str = "IndexedDB.deleteDatabase";

    type ReturnObject = DeleteDatabaseReturnObject;
}
// Delete a range of entries from an object store
#[derive(Serialize, Debug)]
pub struct DeleteObjectStoreEntries {
    pub security_origin: String,
    pub database_name: String,
    pub object_store_name: String,
    // Range of entry keys to delete
    pub key_range: KeyRange,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObjectStoreEntriesReturnObject {}
impl super::Command for DeleteObjectStoreEntries {
    const NAME: &'static str = "IndexedDB.deleteObjectStoreEntries";

    type ReturnObject = DeleteObjectStoreEntriesReturnObject;
}
// Disables events from backend.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "IndexedDB.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables events from backend.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "IndexedDB.enable";

    type ReturnObject = EnableReturnObject;
}
// Requests data from object store or index.
#[derive(Serialize, Debug)]
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
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataReturnObject {
    // Array of object store data entries.
    pub object_store_data_entries: Vec<DataEntry>,
    // If true, there are more entries to fetch in the given range.
    pub has_more: bool,
}
impl super::Command for RequestData {
    const NAME: &'static str = "IndexedDB.requestData";

    type ReturnObject = RequestDataReturnObject;
}
// Gets metadata of an object store
#[derive(Serialize, Debug)]
pub struct GetMetadata {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
    // Object store name.
    pub object_store_name: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataReturnObject {
    // the entries count
    pub entries_count: f64,
    // the current value of key generator, to become the next inserted
    // key into the object store. Valid if objectStore.autoIncrement
    // is true.
    pub key_generator_value: f64,
}
impl super::Command for GetMetadata {
    const NAME: &'static str = "IndexedDB.getMetadata";

    type ReturnObject = GetMetadataReturnObject;
}
// Requests database with given name in given frame.
#[derive(Serialize, Debug)]
pub struct RequestDatabase {
    // Security origin.
    pub security_origin: String,
    // Database name.
    pub database_name: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseReturnObject {
    // Database with an array of object stores.
    pub database_with_object_stores: DatabaseWithObjectStores,
}
impl super::Command for RequestDatabase {
    const NAME: &'static str = "IndexedDB.requestDatabase";

    type ReturnObject = RequestDatabaseReturnObject;
}
// Requests database names for given security origin.
#[derive(Serialize, Debug)]
pub struct RequestDatabaseNames {
    // Security origin.
    pub security_origin: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesReturnObject {
    // Database names for origin.
    pub database_names: Vec<String>,
}
impl super::Command for RequestDatabaseNames {
    const NAME: &'static str = "IndexedDB.requestDatabaseNames";

    type ReturnObject = RequestDatabaseNamesReturnObject;
}
