// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

pub type AuthenticatorId = String;
pub type AuthenticatorProtocol = String;
pub type AuthenticatorTransport = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAuthenticatorOptions {
    pub protocol: AuthenticatorProtocol,
    pub transport: AuthenticatorTransport,
    // Defaults to false.
    pub has_resident_key: Option<bool>,
    // Defaults to false.
    pub has_user_verification: Option<bool>,
    // If set to true, the authenticator will support the largeBlob extension.
    // https://w3c.github.io/webauthn#largeBlob
    // Defaults to false.
    pub has_large_blob: Option<bool>,
    // If set to true, tests of user presence will succeed immediately.
    // Otherwise, they will not be resolved. Defaults to true.
    pub automatic_presence_simulation: Option<bool>,
    // Sets whether User Verification succeeds or fails for an authenticator.
    // Defaults to false.
    pub is_user_verified: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub credential_id: String,
    pub is_resident_credential: bool,
    // Relying Party ID the credential is scoped to. Must be set when adding a
    // credential.
    pub rp_id: Option<String>,
    // The ECDSA P-256 private key in PKCS#8 format.
    pub private_key: String,
    // An opaque byte sequence with a maximum size of 64 bytes mapping the
    // credential to a specific user.
    pub user_handle: Option<String>,
    // Signature counter. This is incremented by one for each successful
    // assertion.
    // See https://w3c.github.io/webauthn/#signature-counter
    pub sign_count: i32,
}

// Enable the WebAuthn domain and start intercepting credential storage and
// retrieval with a virtual authenticator.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "WebAuthn.enable";

    type ReturnObject = EnableReturnObject;
}
// Disable the WebAuthn domain.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "WebAuthn.disable";

    type ReturnObject = DisableReturnObject;
}
// Creates and adds a virtual authenticator.
#[derive(Serialize, Debug)]
pub struct AddVirtualAuthenticator {
    pub options: VirtualAuthenticatorOptions,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddVirtualAuthenticatorReturnObject {
    pub authenticator_id: AuthenticatorId,
}
impl super::Command for AddVirtualAuthenticator {
    const NAME: &'static str = "WebAuthn.addVirtualAuthenticator";

    type ReturnObject = AddVirtualAuthenticatorReturnObject;
}
// Removes the given authenticator.
#[derive(Serialize, Debug)]
pub struct RemoveVirtualAuthenticator {
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveVirtualAuthenticatorReturnObject {}
impl super::Command for RemoveVirtualAuthenticator {
    const NAME: &'static str = "WebAuthn.removeVirtualAuthenticator";

    type ReturnObject = RemoveVirtualAuthenticatorReturnObject;
}
// Adds the credential to the specified authenticator.
#[derive(Serialize, Debug)]
pub struct AddCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential: Credential,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddCredentialReturnObject {}
impl super::Command for AddCredential {
    const NAME: &'static str = "WebAuthn.addCredential";

    type ReturnObject = AddCredentialReturnObject;
}
// Returns a single credential stored in the given virtual authenticator that
// matches the credential ID.
#[derive(Serialize, Debug)]
pub struct GetCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialReturnObject {
    pub credential: Credential,
}
impl super::Command for GetCredential {
    const NAME: &'static str = "WebAuthn.getCredential";

    type ReturnObject = GetCredentialReturnObject;
}
// Returns all the credentials stored in the given virtual authenticator.
#[derive(Serialize, Debug)]
pub struct GetCredentials {
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCredentialsReturnObject {
    pub credentials: Vec<Credential>,
}
impl super::Command for GetCredentials {
    const NAME: &'static str = "WebAuthn.getCredentials";

    type ReturnObject = GetCredentialsReturnObject;
}
// Removes a credential from the authenticator.
#[derive(Serialize, Debug)]
pub struct RemoveCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCredentialReturnObject {}
impl super::Command for RemoveCredential {
    const NAME: &'static str = "WebAuthn.removeCredential";

    type ReturnObject = RemoveCredentialReturnObject;
}
// Clears all the credentials from the specified device.
#[derive(Serialize, Debug)]
pub struct ClearCredentials {
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearCredentialsReturnObject {}
impl super::Command for ClearCredentials {
    const NAME: &'static str = "WebAuthn.clearCredentials";

    type ReturnObject = ClearCredentialsReturnObject;
}
// Sets whether User Verification succeeds or fails for an authenticator.
// The default is true.
#[derive(Serialize, Debug)]
pub struct SetUserVerified {
    pub authenticator_id: AuthenticatorId,
    pub is_user_verified: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetUserVerifiedReturnObject {}
impl super::Command for SetUserVerified {
    const NAME: &'static str = "WebAuthn.setUserVerified";

    type ReturnObject = SetUserVerifiedReturnObject;
}
// Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
// The default is true.
#[derive(Serialize, Debug)]
pub struct SetAutomaticPresenceSimulation {
    pub authenticator_id: AuthenticatorId,
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomaticPresenceSimulationReturnObject {}
impl super::Command for SetAutomaticPresenceSimulation {
    const NAME: &'static str = "WebAuthn.setAutomaticPresenceSimulation";

    type ReturnObject = SetAutomaticPresenceSimulationReturnObject;
}
