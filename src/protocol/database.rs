// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Unique identifier of Database object.
pub type DatabaseId = String;
// Database object.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    // Error message.
    pub message: String,
    // Error code.
    pub code: i32,
}

// Disables database tracking, prevents database events from being sent to the client.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Database.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables database tracking, database events will now be delivered to the client.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Database.enable";

    type ReturnObject = EnableReturnObject;
}
#[derive(Serialize, Debug)]
pub struct ExecuteSQL {
    pub database_id: DatabaseId,
    pub query: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ExecuteSQLReturnObject {
    pub column_names: Option<Vec<String>>,
    pub values: Option<Vec<serde_json::Value>>,
    pub sql_error: Option<Error>,
}
impl super::Command for ExecuteSQL {
    const NAME: &'static str = "Database.executeSQL";

    type ReturnObject = ExecuteSQLReturnObject;
}
#[derive(Serialize, Debug)]
pub struct GetDatabaseTableNames {
    pub database_id: DatabaseId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetDatabaseTableNamesReturnObject {
    pub table_names: Vec<String>,
}
impl super::Command for GetDatabaseTableNames {
    const NAME: &'static str = "Database.getDatabaseTableNames";

    type ReturnObject = GetDatabaseTableNamesReturnObject;
}
