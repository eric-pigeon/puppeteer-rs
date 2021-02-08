// This file is auto-generated do not edit manually.

// Description of the protocol domain.
pub struct Domain {
    // Domain name.
    pub name: String,
    // Domain version.
    pub version: String,
}

// Returns supported domains.
pub struct GetDomains {}
pub struct GetDomainsReturnObject {
    // List of supported domains.
    pub domains: Vec<Domain>,
}
