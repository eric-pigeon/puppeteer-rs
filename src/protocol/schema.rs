// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Description of the protocol domain.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    // Domain name.
    pub name: String,
    // Domain version.
    pub version: String,
}

// Returns supported domains.
#[derive(Serialize, Debug)]
pub struct GetDomains {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsReturnObject {
    // List of supported domains.
    pub domains: Vec<Domain>,
}
impl super::Command for GetDomains {
    const NAME: &'static str = "Schema.getDomains";

    type ReturnObject = GetDomainsReturnObject;
}
