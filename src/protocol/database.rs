// This file is auto-generated do not edit manually.

// Unique identifier of Database object.
pub type DatabaseId = String;
// Database object.
pub struct Database {
    // Database ID.
    pub id: DatabaseId,
    // Database domain.
    pub domain: String,
    // Database name.
    pub name: String,
    // Database version.
    pub version: String,
}
// Database error.
pub struct Error {
    // Error message.
    pub message: String,
    // Error code.
    pub code: i32,
}

// Disables database tracking, prevents database events from being sent to the client.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables database tracking, database events will now be delivered to the client.
pub struct Enable {}
pub struct EnableReturnObject {}
pub struct ExecuteSQL {
    pub database_id: DatabaseId,
    pub query: String,
}
pub struct ExecuteSQLReturnObject {
    pub column_names: Option<Vec<String>>,
    pub values: Option<Vec<serde_json::Value>>,
    pub sql_error: Option<Error>,
}
pub struct GetDatabaseTableNames {
    pub database_id: DatabaseId,
}
pub struct GetDatabaseTableNamesReturnObject {
    pub table_names: Vec<String>,
}
