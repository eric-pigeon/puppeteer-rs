// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// An internal certificate ID value.
pub type CertificateId = i32;
// A description of mixed content (HTTP resources on HTTPS pages), as defined by
// https://www.w3.org/TR/mixed-content/#categories
pub type MixedContentType = String;
// The security level of a page or resource.
pub type SecurityState = String;
// Details about the security state of the page certificate.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
pub type SafetyTipStatus = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafetyTipInfo {
    // Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.
    pub safety_tip_status: SafetyTipStatus,
    // The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.
    pub safe_url: Option<String>,
}
// Security state information about the page.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
pub type CertificateErrorAction = String;

// Disables tracking security state changes.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Security.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables tracking security state changes.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Security.enable";

    type ReturnObject = EnableReturnObject;
}
// Enable/disable whether all certificate errors should be ignored.
#[derive(Serialize, Debug)]
pub struct SetIgnoreCertificateErrors {
    // If true, all certificate errors will be ignored.
    pub ignore: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreCertificateErrorsReturnObject {}
impl super::Command for SetIgnoreCertificateErrors {
    const NAME: &'static str = "Security.setIgnoreCertificateErrors";

    type ReturnObject = SetIgnoreCertificateErrorsReturnObject;
}
// Handles a certificate error that fired a certificateError event.
#[derive(Serialize, Debug)]
pub struct HandleCertificateError {
    // The ID of the event.
    pub event_id: i32,
    // The action to take on the certificate error.
    pub action: CertificateErrorAction,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HandleCertificateErrorReturnObject {}
impl super::Command for HandleCertificateError {
    const NAME: &'static str = "Security.handleCertificateError";

    type ReturnObject = HandleCertificateErrorReturnObject;
}
// Enable/disable overriding certificate errors. If enabled, all certificate error events need to
// be handled by the DevTools client and should be answered with `handleCertificateError` commands.
#[derive(Serialize, Debug)]
pub struct SetOverrideCertificateErrors {
    // If true, certificate errors will be overridden.
    pub r#override: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetOverrideCertificateErrorsReturnObject {}
impl super::Command for SetOverrideCertificateErrors {
    const NAME: &'static str = "Security.setOverrideCertificateErrors";

    type ReturnObject = SetOverrideCertificateErrorsReturnObject;
}

// There is a certificate error. If overriding certificate errors is enabled, then it should be
// handled with the `handleCertificateError` command. Note: this event does not fire if the
// certificate error has been allowed internally. Only one client per target should override
// certificate errors at the same time.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CertificateErrorEvent {
    pub params: CertificateErrorParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateErrorParams {
    // The ID of the event.
    pub event_id: i32,
    // The type of the error.
    pub error_type: String,
    // The url that was requested.
    pub request_url: String,
}
// The security state of the page changed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct VisibleSecurityStateChangedEvent {
    pub params: VisibleSecurityStateChangedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VisibleSecurityStateChangedParams {
    // Security state information about the page.
    pub visible_security_state: VisibleSecurityState,
}
// The security state of the page changed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SecurityStateChangedEvent {
    pub params: SecurityStateChangedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStateChangedParams {
    // Security state.
    pub security_state: SecurityState,
    // True if the page was loaded over cryptographic transport such as HTTPS.
    pub scheme_is_cryptographic: bool,
    // List of explanations for the security state. If the overall security state is `insecure` or
    // `warning`, at least one corresponding explanation should be included.
    pub explanations: Vec<SecurityStateExplanation>,
    // Information about insecure content on the page.
    pub insecure_content_status: InsecureContentStatus,
    // Overrides user-visible description of the state.
    pub summary: Option<String>,
}
