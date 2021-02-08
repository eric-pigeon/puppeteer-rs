// This file is auto-generated do not edit manually.

pub type AuthenticatorId = String;
pub enum AuthenticatorProtocol {
    U2f,
    Ctap2,
}
pub enum AuthenticatorTransport {
    Usb,
    Nfc,
    Ble,
    Cable,
    Internal,
}
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
pub struct Enable {}
pub struct EnableReturnObject {}
// Disable the WebAuthn domain.
pub struct Disable {}
pub struct DisableReturnObject {}
// Creates and adds a virtual authenticator.
pub struct AddVirtualAuthenticator {
    pub options: VirtualAuthenticatorOptions,
}
pub struct AddVirtualAuthenticatorReturnObject {
    pub authenticator_id: AuthenticatorId,
}
// Removes the given authenticator.
pub struct RemoveVirtualAuthenticator {
    pub authenticator_id: AuthenticatorId,
}
pub struct RemoveVirtualAuthenticatorReturnObject {}
// Adds the credential to the specified authenticator.
pub struct AddCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential: Credential,
}
pub struct AddCredentialReturnObject {}
// Returns a single credential stored in the given virtual authenticator that
// matches the credential ID.
pub struct GetCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: String,
}
pub struct GetCredentialReturnObject {
    pub credential: Credential,
}
// Returns all the credentials stored in the given virtual authenticator.
pub struct GetCredentials {
    pub authenticator_id: AuthenticatorId,
}
pub struct GetCredentialsReturnObject {
    pub credentials: Vec<Credential>,
}
// Removes a credential from the authenticator.
pub struct RemoveCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: String,
}
pub struct RemoveCredentialReturnObject {}
// Clears all the credentials from the specified device.
pub struct ClearCredentials {
    pub authenticator_id: AuthenticatorId,
}
pub struct ClearCredentialsReturnObject {}
// Sets whether User Verification succeeds or fails for an authenticator.
// The default is true.
pub struct SetUserVerified {
    pub authenticator_id: AuthenticatorId,
    pub is_user_verified: bool,
}
pub struct SetUserVerifiedReturnObject {}
// Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
// The default is true.
pub struct SetAutomaticPresenceSimulation {
    pub authenticator_id: AuthenticatorId,
    pub enabled: bool,
}
pub struct SetAutomaticPresenceSimulationReturnObject {}
