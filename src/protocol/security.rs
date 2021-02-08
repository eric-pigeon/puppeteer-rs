// This file is auto-generated do not edit manually.

// An internal certificate ID value.
pub type CertificateId = i32;
// A description of mixed content (HTTP resources on HTTPS pages), as defined by
// https://www.w3.org/TR/mixed-content/#categories
pub enum MixedContentType {
    Blockable,
    OptionallyBlockable,
    None,
}
// The security level of a page or resource.
pub enum SecurityState {
    Unknown,
    Neutral,
    Insecure,
    Secure,
    Info,
    InsecureBroken,
}
// Details about the security state of the page certificate.
pub struct CertificateSecurityState {
    // Protocol name (e.g. "TLS 1.2" or "QUIC").
    pub protocol: String,
    // Key Exchange used by the connection, or the empty string if not applicable.
    pub key_exchange: String,
    // (EC)DH group used by the connection, if applicable.
    pub key_exchange_group: Option<String>,
    // Cipher name.
    pub cipher: String,
    // TLS MAC. Note that AEAD ciphers do not have separate MACs.
    pub mac: Option<String>,
    // Page certificate.
    pub certificate: Vec<String>,
    // Certificate subject name.
    pub subject_name: String,
    // Name of the issuing CA.
    pub issuer: String,
    // Certificate valid from date.
    pub valid_from: super::network::TimeSinceEpoch,
    // Certificate valid to (expiration) date
    pub valid_to: super::network::TimeSinceEpoch,
    // The highest priority network error code, if the certificate has an error.
    pub certificate_network_error: Option<String>,
    // True if the certificate uses a weak signature aglorithm.
    pub certificate_has_weak_signature: bool,
    // True if the certificate has a SHA1 signature in the chain.
    pub certificate_has_sha1_signature: bool,
    // True if modern SSL
    pub modern_ssl: bool,
    // True if the connection is using an obsolete SSL protocol.
    pub obsolete_ssl_protocol: bool,
    // True if the connection is using an obsolete SSL key exchange.
    pub obsolete_ssl_key_exchange: bool,
    // True if the connection is using an obsolete SSL cipher.
    pub obsolete_ssl_cipher: bool,
    // True if the connection is using an obsolete SSL signature.
    pub obsolete_ssl_signature: bool,
}
pub enum SafetyTipStatus {
    BadReputation,
    Lookalike,
}
pub struct SafetyTipInfo {
    // Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    pub safety_tip_status: SafetyTipStatus,
    // The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    pub safe_url: Option<String>,
}
// Security state information about the page.
pub struct VisibleSecurityState {
    // The security level of the page.
    pub security_state: SecurityState,
    // Security state details about the page certificate.
    pub certificate_security_state: Option<CertificateSecurityState>,
    // The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.
    pub safety_tip_info: Option<SafetyTipInfo>,
    // Array of security state issues ids.
    pub security_state_issue_ids: Vec<String>,
}
// An explanation of an factor contributing to the security state.
pub struct SecurityStateExplanation {
    // Security state representing the severity of the factor being explained.
    pub security_state: SecurityState,
    // Title describing the type of factor.
    pub title: String,
    // Short phrase describing the type of factor.
    pub summary: String,
    // Full text explanation of the factor.
    pub description: String,
    // The type of mixed content described by the explanation.
    pub mixed_content_type: MixedContentType,
    // Page certificate.
    pub certificate: Vec<String>,
    // Recommendations to fix any issues.
    pub recommendations: Option<Vec<String>>,
}
// Information about insecure content on the page.
pub struct InsecureContentStatus {
    // Always false.
    pub ran_mixed_content: bool,
    // Always false.
    pub displayed_mixed_content: bool,
    // Always false.
    pub contained_mixed_form: bool,
    // Always false.
    pub ran_content_with_cert_errors: bool,
    // Always false.
    pub displayed_content_with_cert_errors: bool,
    // Always set to unknown.
    pub ran_insecure_content_style: SecurityState,
    // Always set to unknown.
    pub displayed_insecure_content_style: SecurityState,
}
// The action to take when a certificate error occurs. continue will continue processing the
// request and cancel will cancel the request.
pub enum CertificateErrorAction {
    Continue,
    Cancel,
}

// Disables tracking security state changes.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables tracking security state changes.
pub struct Enable {}
pub struct EnableReturnObject {}
// Enable/disable whether all certificate errors should be ignored.
pub struct SetIgnoreCertificateErrors {
    // If true, all certificate errors will be ignored.
    pub ignore: bool,
}
pub struct SetIgnoreCertificateErrorsReturnObject {}
// Handles a certificate error that fired a certificateError event.
pub struct HandleCertificateError {
    // The ID of the event.
    pub event_id: i32,
    // The action to take on the certificate error.
    pub action: CertificateErrorAction,
}
pub struct HandleCertificateErrorReturnObject {}
// Enable/disable overriding certificate errors. If enabled, all certificate error events need to
// be handled by the DevTools client and should be answered with `handleCertificateError` commands.
pub struct SetOverrideCertificateErrors {
    // If true, certificate errors will be overridden.
    pub r#override: bool,
}
pub struct SetOverrideCertificateErrorsReturnObject {}
