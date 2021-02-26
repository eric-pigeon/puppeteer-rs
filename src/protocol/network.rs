// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Resource type as it was perceived by the rendering engine.
pub type ResourceType = String;
// Unique loader identifier.
pub type LoaderId = String;
// Unique request identifier.
pub type RequestId = String;
// Unique intercepted request identifier.
pub type InterceptionId = String;
// Network level fetch failure reason.
pub type ErrorReason = String;
// UTC time in seconds, counted from January 1, 1970.
pub type TimeSinceEpoch = f64;
// Monotonically increasing time in seconds since an arbitrary point in the past.
pub type MonotonicTime = f64;
// Request / response headers as keys / values of JSON object.
pub type Headers = std::collections::HashMap<String, String>;
// The underlying connection technology that the browser is supposedly using.
pub type ConnectionType = String;
// Represents the cookie's 'SameSite' status:
// https://tools.ietf.org/html/draft-west-first-party-cookies
pub type CookieSameSite = String;
// Represents the cookie's 'Priority' status:
// https://tools.ietf.org/html/draft-west-cookie-priority-00
pub type CookiePriority = String;
// Timing information for the request.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    // Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    // milliseconds relatively to this requestTime.
    pub request_time: f64,
    // Started resolving proxy.
    pub proxy_start: f64,
    // Finished resolving proxy.
    pub proxy_end: f64,
    // Started DNS address resolve.
    pub dns_start: f64,
    // Finished DNS address resolve.
    pub dns_end: f64,
    // Started connecting to the remote host.
    pub connect_start: f64,
    // Connected to the remote host.
    pub connect_end: f64,
    // Started SSL handshake.
    pub ssl_start: f64,
    // Finished SSL handshake.
    pub ssl_end: f64,
    // Started running ServiceWorker.
    pub worker_start: f64,
    // Finished Starting ServiceWorker.
    pub worker_ready: f64,
    // Started fetch event.
    pub worker_fetch_start: f64,
    // Settled fetch event respondWith promise.
    pub worker_respond_with_settled: f64,
    // Started sending request.
    pub send_start: f64,
    // Finished sending request.
    pub send_end: f64,
    // Time the server started pushing request.
    pub push_start: f64,
    // Time the server finished pushing request.
    pub push_end: f64,
    // Finished receiving response headers.
    pub receive_headers_end: f64,
}
// Loading priority of a resource request.
pub type ResourcePriority = String;
// Post data entry for HTTP request
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostDataEntry {
    pub bytes: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RequestReferrerPolicy {
    UnsafeUrl,
    NoReferrerWhenDowngrade,
    NoReferrer,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
}
// HTTP request data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    // Request URL (without fragment).
    pub url: String,
    // Fragment of the requested URL starting with hash, if present.
    pub url_fragment: Option<String>,
    // HTTP request method.
    pub method: String,
    // HTTP request headers.
    pub headers: Headers,
    // HTTP POST request data.
    pub post_data: Option<String>,
    // True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.
    pub has_post_data: Option<bool>,
    // Request body elements. This will be converted from base64 to binary
    pub post_data_entries: Option<Vec<PostDataEntry>>,
    // The mixed content type of the request.
    pub mixed_content_type: Option<super::security::MixedContentType>,
    // Priority of the resource request at the time request is sent.
    pub initial_priority: ResourcePriority,
    // The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/
    pub referrer_policy: String,
    // Whether is loaded via link preload.
    pub is_link_preload: Option<bool>,
    // Set for requests when the TrustToken API is used. Contains the parameters
    // passed by the developer (e.g. via "fetch") as understood by the backend.
    pub trust_token_params: Option<TrustTokenParams>,
}
// Details of a signed certificate timestamp (SCT).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedCertificateTimestamp {
    // Validation status.
    pub status: String,
    // Origin.
    pub origin: String,
    // Log name / description.
    pub log_description: String,
    // Log ID.
    pub log_id: String,
    // Issuance date.
    pub timestamp: TimeSinceEpoch,
    // Hash algorithm.
    pub hash_algorithm: String,
    // Signature algorithm.
    pub signature_algorithm: String,
    // Signature data.
    pub signature_data: String,
}
// Security details about a request.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDetails {
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
    // Certificate ID value.
    pub certificate_id: super::security::CertificateId,
    // Certificate subject name.
    pub subject_name: String,
    // Subject Alternative Name (SAN) DNS names and IP addresses.
    pub san_list: Vec<String>,
    // Name of the issuing CA.
    pub issuer: String,
    // Certificate valid from date.
    pub valid_from: TimeSinceEpoch,
    // Certificate valid to (expiration) date
    pub valid_to: TimeSinceEpoch,
    // List of signed certificate timestamps (SCTs).
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
    // Whether the request complied with Certificate Transparency policy
    pub certificate_transparency_compliance: CertificateTransparencyCompliance,
}
// Whether the request complied with Certificate Transparency policy.
pub type CertificateTransparencyCompliance = String;
// The reason why request was blocked.
pub type BlockedReason = String;
// Source of serviceworker response.
pub type ServiceWorkerResponseSource = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TrustTokenParamsRefreshPolicy {
    UseCached,
    Refresh,
}
// Determines what type of Trust Token operation is executed and
// depending on the type, some additional parameters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokenParams {
    pub r#type: TrustTokenOperationType,
    // Only set for "srr-token-redemption" type and determine whether
    // to request a fresh SRR or use a still valid cached SRR.
    pub refresh_policy: String,
    // Origins of issuers from whom to request tokens or redemption
    // records.
    pub issuers: Option<Vec<String>>,
}
pub type TrustTokenOperationType = String;
// HTTP response data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    // Response URL. This URL can be different from CachedResource.url in case of redirect.
    pub url: String,
    // HTTP response status code.
    pub status: i32,
    // HTTP response status text.
    pub status_text: String,
    // HTTP response headers.
    pub headers: Headers,
    // HTTP response headers text.
    pub headers_text: Option<String>,
    // Resource mimeType as determined by the browser.
    pub mime_type: String,
    // Refined HTTP request headers that were actually transmitted over the network.
    pub request_headers: Option<Headers>,
    // HTTP request headers text.
    pub request_headers_text: Option<String>,
    // Specifies whether physical connection was actually reused for this request.
    pub connection_reused: bool,
    // Physical connection id that was actually used for this request.
    pub connection_id: f64,
    // Remote IP address.
    pub remote_ip_address: Option<String>,
    // Remote port.
    pub remote_port: Option<i32>,
    // Specifies that the request was served from the disk cache.
    pub from_disk_cache: Option<bool>,
    // Specifies that the request was served from the ServiceWorker.
    pub from_service_worker: Option<bool>,
    // Specifies that the request was served from the prefetch cache.
    pub from_prefetch_cache: Option<bool>,
    // Total number of bytes received for this request so far.
    pub encoded_data_length: f64,
    // Timing information for the given request.
    pub timing: Option<ResourceTiming>,
    // Response source of response from ServiceWorker.
    pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
    // The time at which the returned response was generated.
    pub response_time: Option<TimeSinceEpoch>,
    // Cache Storage Cache Name.
    pub cache_storage_cache_name: Option<String>,
    // Protocol used to fetch this request.
    pub protocol: Option<String>,
    // Security state of the request resource.
    pub security_state: super::security::SecurityState,
    // Security details for the request.
    pub security_details: Option<SecurityDetails>,
}
// WebSocket request data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketRequest {
    // HTTP request headers.
    pub headers: Headers,
}
// WebSocket response data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketResponse {
    // HTTP response status code.
    pub status: i32,
    // HTTP response status text.
    pub status_text: String,
    // HTTP response headers.
    pub headers: Headers,
    // HTTP response headers text.
    pub headers_text: Option<String>,
    // HTTP request headers.
    pub request_headers: Option<Headers>,
    // HTTP request headers text.
    pub request_headers_text: Option<String>,
}
// WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrame {
    // WebSocket message opcode.
    pub opcode: f64,
    // WebSocket message mask.
    pub mask: bool,
    // WebSocket message payload data.
    // If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    // If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.
    pub payload_data: String,
}
// Information about the cached resource.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CachedResource {
    // Resource URL. This is the url of the original network request.
    pub url: String,
    // Type of this resource.
    pub r#type: ResourceType,
    // Cached response data.
    pub response: Option<Response>,
    // Cached response body size.
    pub body_size: f64,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InitiatorType {
    Parser,
    Script,
    Preload,
    SignedExchange,
    Other,
}
// Information about the request initiator.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Initiator {
    // Type of this initiator.
    pub r#type: String,
    // Initiator JavaScript stack trace, set for Script only.
    pub stack: Option<super::runtime::StackTrace>,
    // Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.
    pub url: Option<String>,
    // Initiator line number, set for Parser type or for Script type (when script is importing
    // module) (0-based).
    pub line_number: Option<f64>,
    // Initiator column number, set for Parser type or for Script type (when script is importing
    // module) (0-based).
    pub column_number: Option<f64>,
}
// Cookie object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    // Cookie name.
    pub name: String,
    // Cookie value.
    pub value: String,
    // Cookie domain.
    pub domain: String,
    // Cookie path.
    pub path: String,
    // Cookie expiration date as the number of seconds since the UNIX epoch.
    pub expires: f64,
    // Cookie size.
    pub size: i32,
    // True if cookie is http-only.
    pub http_only: bool,
    // True if cookie is secure.
    pub secure: bool,
    // True in case of session cookie.
    pub session: bool,
    // Cookie SameSite type.
    pub same_site: Option<CookieSameSite>,
    // Cookie Priority
    pub priority: CookiePriority,
}
// Types of reasons why a cookie may not be stored from a response.
pub type SetCookieBlockedReason = String;
// Types of reasons why a cookie may not be sent with a request.
pub type CookieBlockedReason = String;
// A cookie which was not stored from a response with the corresponding reason.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlockedSetCookieWithReason {
    // The reason(s) this cookie was blocked.
    pub blocked_reasons: Vec<SetCookieBlockedReason>,
    // The string representing this individual cookie as it would appear in the header.
    // This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.
    pub cookie_line: String,
    // The cookie object which represents the cookie which was not stored. It is optional because
    // sometimes complete cookie information is not available, such as in the case of parsing
    // errors.
    pub cookie: Option<Cookie>,
}
// A cookie with was not sent with a request with the corresponding reason.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlockedCookieWithReason {
    // The reason(s) the cookie was blocked.
    pub blocked_reasons: Vec<CookieBlockedReason>,
    // The cookie object representing the cookie which was not sent.
    pub cookie: Cookie,
}
// Cookie parameter object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CookieParam {
    // Cookie name.
    pub name: String,
    // Cookie value.
    pub value: String,
    // The request-URI to associate with the setting of the cookie. This value can affect the
    // default domain and path values of the created cookie.
    pub url: Option<String>,
    // Cookie domain.
    pub domain: Option<String>,
    // Cookie path.
    pub path: Option<String>,
    // True if cookie is secure.
    pub secure: Option<bool>,
    // True if cookie is http-only.
    pub http_only: Option<bool>,
    // Cookie SameSite type.
    pub same_site: Option<CookieSameSite>,
    // Cookie expiration date, session cookie if not set
    pub expires: Option<TimeSinceEpoch>,
    // Cookie Priority.
    pub priority: Option<CookiePriority>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AuthChallengeSource {
    Server,
    Proxy,
}
// Authorization challenge for HTTP status code 401 or 407.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallenge {
    // Source of the authentication challenge.
    pub source: Option<String>,
    // Origin of the challenger.
    pub origin: String,
    // The authentication scheme used, such as basic or digest
    pub scheme: String,
    // The realm of the challenge. May be empty.
    pub realm: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AuthChallengeResponseResponse {
    Default,
    CancelAuth,
    ProvideCredentials,
}
// Response to an AuthChallenge.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthChallengeResponse {
    // The decision on what to do in response to the authorization challenge.  Default means
    // deferring to the default behavior of the net stack, which will likely either the Cancel
    // authentication or display a popup dialog box.
    pub response: String,
    // The username to provide, possibly empty. Should only be set if response is
    // ProvideCredentials.
    pub username: Option<String>,
    // The password to provide, possibly empty. Should only be set if response is
    // ProvideCredentials.
    pub password: Option<String>,
}
// Stages of the interception to begin intercepting. Request will intercept before the request is
// sent. Response will intercept after the response is received.
pub type InterceptionStage = String;
// Request pattern for interception.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern {
    // Wildcards ('*' -> zero or more, '?' -> exactly one) are allowed. Escape character is
    // backslash. Omitting is equivalent to "*".
    pub url_pattern: Option<String>,
    // If set, only requests for matching resource types will be intercepted.
    pub resource_type: Option<ResourceType>,
    // Stage at wich to begin intercepting requests. Default is Request.
    pub interception_stage: Option<InterceptionStage>,
}
// Information about a signed exchange signature.
// https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeSignature {
    // Signed exchange signature label.
    pub label: String,
    // The hex string of signed exchange signature.
    pub signature: String,
    // Signed exchange signature integrity.
    pub integrity: String,
    // Signed exchange signature cert Url.
    pub cert_url: Option<String>,
    // The hex string of signed exchange signature cert sha256.
    pub cert_sha256: Option<String>,
    // Signed exchange signature validity Url.
    pub validity_url: String,
    // Signed exchange signature date.
    pub date: i32,
    // Signed exchange signature expires.
    pub expires: i32,
    // The encoded certificates.
    pub certificates: Option<Vec<String>>,
}
// Information about a signed exchange header.
// https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeHeader {
    // Signed exchange request URL.
    pub request_url: String,
    // Signed exchange response code.
    pub response_code: i32,
    // Signed exchange response headers.
    pub response_headers: Headers,
    // Signed exchange response signature.
    pub signatures: Vec<SignedExchangeSignature>,
    // Signed exchange header integrity hash in the form of "sha256-<base64-hash-value>".
    pub header_integrity: String,
}
// Field type for a signed exchange related error.
pub type SignedExchangeErrorField = String;
// Information about a signed exchange response.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeError {
    // Error message.
    pub message: String,
    // The index of the signature which caused the error.
    pub signature_index: Option<i32>,
    // The field which caused the error.
    pub error_field: Option<SignedExchangeErrorField>,
}
// Information about a signed exchange response.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeInfo {
    // The outer response of signed HTTP exchange which was received from network.
    pub outer_response: Response,
    // Information about the signed exchange header.
    pub header: Option<SignedExchangeHeader>,
    // Security details for the signed exchange header.
    pub security_details: Option<SecurityDetails>,
    // Errors occurred while handling the signed exchagne.
    pub errors: Option<Vec<SignedExchangeError>>,
}
pub type CrossOriginOpenerPolicyValue = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginOpenerPolicyStatus {
    pub value: CrossOriginOpenerPolicyValue,
    pub report_only_value: CrossOriginOpenerPolicyValue,
    pub reporting_endpoint: Option<String>,
    pub report_only_reporting_endpoint: Option<String>,
}
pub type CrossOriginEmbedderPolicyValue = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CrossOriginEmbedderPolicyStatus {
    pub value: CrossOriginEmbedderPolicyValue,
    pub report_only_value: CrossOriginEmbedderPolicyValue,
    pub reporting_endpoint: Option<String>,
    pub report_only_reporting_endpoint: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityIsolationStatus {
    pub coop: CrossOriginOpenerPolicyStatus,
    pub coep: CrossOriginEmbedderPolicyStatus,
}
// An object providing the result of a network resource load.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourcePageResult {
    pub success: bool,
    // Optional values used for error reporting.
    pub net_error: Option<f64>,
    pub net_error_name: Option<String>,
    pub http_status_code: Option<f64>,
    // If successful, one of the following two fields holds the result.
    pub stream: Option<super::io::StreamHandle>,
    // Response headers.
    pub headers: Option<super::network::Headers>,
}
// An options object that may be extended later to better support CORS,
// CORB and streaming.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceOptions {
    pub disable_cache: bool,
    pub include_credentials: bool,
}

// Tells whether clearing browser cache is supported.
#[derive(Serialize, Debug)]
pub struct CanClearBrowserCache {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCacheReturnObject {
    // True if browser cache can be cleared.
    pub result: bool,
}
impl super::Command for CanClearBrowserCache {
    const NAME: &'static str = "Network.canClearBrowserCache";

    type ReturnObject = CanClearBrowserCacheReturnObject;
}
// Tells whether clearing browser cookies is supported.
#[derive(Serialize, Debug)]
pub struct CanClearBrowserCookies {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanClearBrowserCookiesReturnObject {
    // True if browser cookies can be cleared.
    pub result: bool,
}
impl super::Command for CanClearBrowserCookies {
    const NAME: &'static str = "Network.canClearBrowserCookies";

    type ReturnObject = CanClearBrowserCookiesReturnObject;
}
// Tells whether emulation of network conditions is supported.
#[derive(Serialize, Debug)]
pub struct CanEmulateNetworkConditions {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulateNetworkConditionsReturnObject {
    // True if emulation of network conditions is supported.
    pub result: bool,
}
impl super::Command for CanEmulateNetworkConditions {
    const NAME: &'static str = "Network.canEmulateNetworkConditions";

    type ReturnObject = CanEmulateNetworkConditionsReturnObject;
}
// Clears browser cache.
#[derive(Serialize, Debug)]
pub struct ClearBrowserCache {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearBrowserCacheReturnObject {}
impl super::Command for ClearBrowserCache {
    const NAME: &'static str = "Network.clearBrowserCache";

    type ReturnObject = ClearBrowserCacheReturnObject;
}
// Clears browser cookies.
#[derive(Serialize, Debug)]
pub struct ClearBrowserCookies {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClearBrowserCookiesReturnObject {}
impl super::Command for ClearBrowserCookies {
    const NAME: &'static str = "Network.clearBrowserCookies";

    type ReturnObject = ClearBrowserCookiesReturnObject;
}
// Response to Network.requestIntercepted which either modifies the request to continue with any
// modifications, or blocks it, or completes it with the provided response bytes. If a network
// fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted
// event will be sent with the same InterceptionId.
// Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead.
#[derive(Serialize, Debug)]
pub struct ContinueInterceptedRequest {
    pub interception_id: InterceptionId,
    // If set this causes the request to fail with the given reason. Passing `Aborted` for requests
    // marked with `isNavigationRequest` also cancels the navigation. Must not be set in response
    // to an authChallenge.
    pub error_reason: Option<ErrorReason>,
    // If set the requests completes using with the provided base64 encoded raw response, including
    // HTTP status line and headers etc... Must not be set in response to an authChallenge.
    pub raw_response: Option<String>,
    // If set the request url will be modified in a way that's not observable by page. Must not be
    // set in response to an authChallenge.
    pub url: Option<String>,
    // If set this allows the request method to be overridden. Must not be set in response to an
    // authChallenge.
    pub method: Option<String>,
    // If set this allows postData to be set. Must not be set in response to an authChallenge.
    pub post_data: Option<String>,
    // If set this allows the request headers to be changed. Must not be set in response to an
    // authChallenge.
    pub headers: Option<Headers>,
    // Response to a requestIntercepted with an authChallenge. Must not be set otherwise.
    pub auth_challenge_response: Option<AuthChallengeResponse>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinueInterceptedRequestReturnObject {}
impl super::Command for ContinueInterceptedRequest {
    const NAME: &'static str = "Network.continueInterceptedRequest";

    type ReturnObject = ContinueInterceptedRequestReturnObject;
}
// Deletes browser cookies with matching name and url or domain/path pair.
#[derive(Serialize, Debug)]
pub struct DeleteCookies {
    // Name of the cookies to remove.
    pub name: String,
    // If specified, deletes all the cookies with the given name where domain and path match
    // provided URL.
    pub url: Option<String>,
    // If specified, deletes only cookies with the exact domain.
    pub domain: Option<String>,
    // If specified, deletes only cookies with the exact path.
    pub path: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCookiesReturnObject {}
impl super::Command for DeleteCookies {
    const NAME: &'static str = "Network.deleteCookies";

    type ReturnObject = DeleteCookiesReturnObject;
}
// Disables network tracking, prevents network events from being sent to the client.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Network.disable";

    type ReturnObject = DisableReturnObject;
}
// Activates emulation of network conditions.
#[derive(Serialize, Debug)]
pub struct EmulateNetworkConditions {
    // True to emulate internet disconnection.
    pub offline: bool,
    // Minimum latency from request sent to response headers received (ms).
    pub latency: f64,
    // Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.
    pub download_throughput: f64,
    // Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.
    pub upload_throughput: f64,
    // Connection type if known.
    pub connection_type: Option<ConnectionType>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmulateNetworkConditionsReturnObject {}
impl super::Command for EmulateNetworkConditions {
    const NAME: &'static str = "Network.emulateNetworkConditions";

    type ReturnObject = EmulateNetworkConditionsReturnObject;
}
// Enables network tracking, network events will now be delivered to the client.
#[derive(Serialize, Debug)]
pub struct Enable {
    // Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub max_total_buffer_size: Option<i32>,
    // Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub max_resource_buffer_size: Option<i32>,
    // Longest post body size (in bytes) that would be included in requestWillBeSent notification
    pub max_post_data_size: Option<i32>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Network.enable";

    type ReturnObject = EnableReturnObject;
}
// Returns all browser cookies. Depending on the backend support, will return detailed cookie
// information in the `cookies` field.
#[derive(Serialize, Debug)]
pub struct GetAllCookies {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<Cookie>,
}
impl super::Command for GetAllCookies {
    const NAME: &'static str = "Network.getAllCookies";

    type ReturnObject = GetAllCookiesReturnObject;
}
// Returns the DER-encoded certificate.
#[derive(Serialize, Debug)]
pub struct GetCertificate {
    // Origin to get certificate for.
    pub origin: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateReturnObject {
    pub table_names: Vec<String>,
}
impl super::Command for GetCertificate {
    const NAME: &'static str = "Network.getCertificate";

    type ReturnObject = GetCertificateReturnObject;
}
// Returns all browser cookies for the current URL. Depending on the backend support, will return
// detailed cookie information in the `cookies` field.
#[derive(Serialize, Debug)]
pub struct GetCookies {
    // The list of URLs for which applicable cookies will be fetched.
    // If not specified, it's assumed to be set to the list containing
    // the URLs of the page and all of its subframes.
    pub urls: Option<Vec<String>>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<Cookie>,
}
impl super::Command for GetCookies {
    const NAME: &'static str = "Network.getCookies";

    type ReturnObject = GetCookiesReturnObject;
}
// Returns content served for the given request.
#[derive(Serialize, Debug)]
pub struct GetResponseBody {
    // Identifier of the network request to get content for.
    pub request_id: RequestId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyReturnObject {
    // Response body.
    pub body: String,
    // True, if content was sent as base64.
    pub base64_encoded: bool,
}
impl super::Command for GetResponseBody {
    const NAME: &'static str = "Network.getResponseBody";

    type ReturnObject = GetResponseBodyReturnObject;
}
// Returns post data sent with the request. Returns an error when no data was sent with the request.
#[derive(Serialize, Debug)]
pub struct GetRequestPostData {
    // Identifier of the network request to get content for.
    pub request_id: RequestId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetRequestPostDataReturnObject {
    // Request body string, omitting files from multipart requests
    pub post_data: String,
}
impl super::Command for GetRequestPostData {
    const NAME: &'static str = "Network.getRequestPostData";

    type ReturnObject = GetRequestPostDataReturnObject;
}
// Returns content served for the given currently intercepted request.
#[derive(Serialize, Debug)]
pub struct GetResponseBodyForInterception {
    // Identifier for the intercepted request to get body for.
    pub interception_id: InterceptionId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetResponseBodyForInterceptionReturnObject {
    // Response body.
    pub body: String,
    // True, if content was sent as base64.
    pub base64_encoded: bool,
}
impl super::Command for GetResponseBodyForInterception {
    const NAME: &'static str = "Network.getResponseBodyForInterception";

    type ReturnObject = GetResponseBodyForInterceptionReturnObject;
}
// Returns a handle to the stream representing the response body. Note that after this command,
// the intercepted request can't be continued as is -- you either need to cancel it or to provide
// the response body. The stream only supports sequential read, IO.read will fail if the position
// is specified.
#[derive(Serialize, Debug)]
pub struct TakeResponseBodyForInterceptionAsStream {
    pub interception_id: InterceptionId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TakeResponseBodyForInterceptionAsStreamReturnObject {
    pub stream: super::io::StreamHandle,
}
impl super::Command for TakeResponseBodyForInterceptionAsStream {
    const NAME: &'static str = "Network.takeResponseBodyForInterceptionAsStream";

    type ReturnObject = TakeResponseBodyForInterceptionAsStreamReturnObject;
}
// This method sends a new XMLHttpRequest which is identical to the original one. The following
// parameters should be identical: method, url, async, request body, extra headers, withCredentials
// attribute, user, password.
#[derive(Serialize, Debug)]
pub struct ReplayXHR {
    // Identifier of XHR to replay.
    pub request_id: RequestId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplayXHRReturnObject {}
impl super::Command for ReplayXHR {
    const NAME: &'static str = "Network.replayXHR";

    type ReturnObject = ReplayXHRReturnObject;
}
// Searches for given string in response content.
#[derive(Serialize, Debug)]
pub struct SearchInResponseBody {
    // Identifier of the network response to search.
    pub request_id: RequestId,
    // String to search for.
    pub query: String,
    // If true, search is case sensitive.
    pub case_sensitive: Option<bool>,
    // If true, treats string parameter as regex.
    pub is_regex: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchInResponseBodyReturnObject {
    // List of search matches.
    pub result: Vec<super::debugger::SearchMatch>,
}
impl super::Command for SearchInResponseBody {
    const NAME: &'static str = "Network.searchInResponseBody";

    type ReturnObject = SearchInResponseBodyReturnObject;
}
// Blocks URLs from loading.
#[derive(Serialize, Debug)]
pub struct SetBlockedURLs {
    // URL patterns to block. Wildcards ('*') are allowed.
    pub urls: Vec<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBlockedURLsReturnObject {}
impl super::Command for SetBlockedURLs {
    const NAME: &'static str = "Network.setBlockedURLs";

    type ReturnObject = SetBlockedURLsReturnObject;
}
// Toggles ignoring of service worker for each request.
#[derive(Serialize, Debug)]
pub struct SetBypassServiceWorker {
    // Bypass service worker and load from network.
    pub bypass: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetBypassServiceWorkerReturnObject {}
impl super::Command for SetBypassServiceWorker {
    const NAME: &'static str = "Network.setBypassServiceWorker";

    type ReturnObject = SetBypassServiceWorkerReturnObject;
}
// Toggles ignoring cache for each request. If `true`, cache will not be used.
#[derive(Serialize, Debug)]
pub struct SetCacheDisabled {
    // Cache disabled state.
    pub cache_disabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCacheDisabledReturnObject {}
impl super::Command for SetCacheDisabled {
    const NAME: &'static str = "Network.setCacheDisabled";

    type ReturnObject = SetCacheDisabledReturnObject;
}
// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
#[derive(Serialize, Debug)]
pub struct SetCookie {
    // Cookie name.
    pub name: String,
    // Cookie value.
    pub value: String,
    // The request-URI to associate with the setting of the cookie. This value can affect the
    // default domain and path values of the created cookie.
    pub url: Option<String>,
    // Cookie domain.
    pub domain: Option<String>,
    // Cookie path.
    pub path: Option<String>,
    // True if cookie is secure.
    pub secure: Option<bool>,
    // True if cookie is http-only.
    pub http_only: Option<bool>,
    // Cookie SameSite type.
    pub same_site: Option<CookieSameSite>,
    // Cookie expiration date, session cookie if not set
    pub expires: Option<TimeSinceEpoch>,
    // Cookie Priority type.
    pub priority: Option<CookiePriority>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCookieReturnObject {
    // Always set to true. If an error occurs, the response indicates protocol error.
    pub success: bool,
}
impl super::Command for SetCookie {
    const NAME: &'static str = "Network.setCookie";

    type ReturnObject = SetCookieReturnObject;
}
// Sets given cookies.
#[derive(Serialize, Debug)]
pub struct SetCookies {
    // Cookies to be set.
    pub cookies: Vec<CookieParam>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesReturnObject {}
impl super::Command for SetCookies {
    const NAME: &'static str = "Network.setCookies";

    type ReturnObject = SetCookiesReturnObject;
}
// For testing.
#[derive(Serialize, Debug)]
pub struct SetDataSizeLimitsForTest {
    // Maximum total buffer size.
    pub max_total_size: i32,
    // Maximum per-resource size.
    pub max_resource_size: i32,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetDataSizeLimitsForTestReturnObject {}
impl super::Command for SetDataSizeLimitsForTest {
    const NAME: &'static str = "Network.setDataSizeLimitsForTest";

    type ReturnObject = SetDataSizeLimitsForTestReturnObject;
}
// Specifies whether to always send extra HTTP headers with the requests from this page.
#[derive(Serialize, Debug)]
pub struct SetExtraHTTPHeaders {
    // Map with extra HTTP headers.
    pub headers: Headers,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetExtraHTTPHeadersReturnObject {}
impl super::Command for SetExtraHTTPHeaders {
    const NAME: &'static str = "Network.setExtraHTTPHeaders";

    type ReturnObject = SetExtraHTTPHeadersReturnObject;
}
// Specifies whether to sned a debug header to all outgoing requests.
#[derive(Serialize, Debug)]
pub struct SetAttachDebugHeader {
    // Whether to send a debug header.
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetAttachDebugHeaderReturnObject {}
impl super::Command for SetAttachDebugHeader {
    const NAME: &'static str = "Network.setAttachDebugHeader";

    type ReturnObject = SetAttachDebugHeaderReturnObject;
}
// Sets the requests to intercept that match the provided patterns and optionally resource types.
// Deprecated, please use Fetch.enable instead.
#[derive(Serialize, Debug)]
pub struct SetRequestInterception {
    // Requests matching any of these patterns will be forwarded and wait for the corresponding
    // continueInterceptedRequest call.
    pub patterns: Vec<RequestPattern>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetRequestInterceptionReturnObject {}
impl super::Command for SetRequestInterception {
    const NAME: &'static str = "Network.setRequestInterception";

    type ReturnObject = SetRequestInterceptionReturnObject;
}
// Allows overriding user agent with the given string.
#[derive(Serialize, Debug)]
pub struct SetUserAgentOverride {
    // User agent to use.
    pub user_agent: String,
    // Browser langugage to emulate.
    pub accept_language: Option<String>,
    // The platform navigator.platform should return.
    pub platform: Option<String>,
    // To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData
    pub user_agent_metadata: Option<super::emulation::UserAgentMetadata>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideReturnObject {}
impl super::Command for SetUserAgentOverride {
    const NAME: &'static str = "Network.setUserAgentOverride";

    type ReturnObject = SetUserAgentOverrideReturnObject;
}
// Returns information about the COEP/COOP isolation status.
#[derive(Serialize, Debug)]
pub struct GetSecurityIsolationStatus {
    // If no frameId is provided, the status of the target is provided.
    pub frame_id: Option<super::page::FrameId>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSecurityIsolationStatusReturnObject {
    pub status: SecurityIsolationStatus,
}
impl super::Command for GetSecurityIsolationStatus {
    const NAME: &'static str = "Network.getSecurityIsolationStatus";

    type ReturnObject = GetSecurityIsolationStatusReturnObject;
}
// Fetches the resource and returns the content.
#[derive(Serialize, Debug)]
pub struct LoadNetworkResource {
    // Frame id to get the resource for.
    pub frame_id: super::page::FrameId,
    // URL of the resource to get content for.
    pub url: String,
    // Options for the request.
    pub options: LoadNetworkResourceOptions,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoadNetworkResourceReturnObject {
    pub resource: LoadNetworkResourcePageResult,
}
impl super::Command for LoadNetworkResource {
    const NAME: &'static str = "Network.loadNetworkResource";

    type ReturnObject = LoadNetworkResourceReturnObject;
}

// Fired when data chunk was received over the network.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DataReceivedEvent {
    pub params: DataReceivedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataReceivedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // Data chunk length.
    pub data_length: i32,
    // Actual bytes received (might be less than dataLength for compressed encodings).
    pub encoded_data_length: i32,
}
// Fired when EventSource message is received.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct EventSourceMessageReceivedEvent {
    pub params: EventSourceMessageReceivedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventSourceMessageReceivedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // Message type.
    pub event_name: String,
    // Message identifier.
    pub event_id: String,
    // Message content.
    pub data: String,
}
// Fired when HTTP request has failed to load.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoadingFailedEvent {
    pub params: LoadingFailedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoadingFailedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // Resource type.
    pub r#type: ResourceType,
    // User friendly error message.
    pub error_text: String,
    // True if loading was canceled.
    pub canceled: Option<bool>,
    // The reason why loading was blocked, if any.
    pub blocked_reason: Option<BlockedReason>,
}
// Fired when HTTP request has finished loading.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoadingFinishedEvent {
    pub params: LoadingFinishedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoadingFinishedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // Total number of bytes received for this request.
    pub encoded_data_length: f64,
    // Set when 1) response was blocked by Cross-Origin Read Blocking and also
    // 2) this needs to be reported to the DevTools console.
    pub should_report_corb_blocking: Option<bool>,
}
// Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
// mocked.
// Deprecated, use Fetch.requestPaused instead.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RequestInterceptedEvent {
    pub params: RequestInterceptedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestInterceptedParams {
    // Each request the page makes will have a unique id, however if any redirects are encountered
    // while processing that fetch, they will be reported with the same id as the original fetch.
    // Likewise if HTTP authentication is needed then the same fetch id will be used.
    pub interception_id: InterceptionId,
    pub request: Request,
    // The id of the frame that initiated the request.
    pub frame_id: super::page::FrameId,
    // How the requested resource will be used.
    pub resource_type: ResourceType,
    // Whether this is a navigation request, which can abort the navigation completely.
    pub is_navigation_request: bool,
    // Set if the request is a navigation that will result in a download.
    // Only present after response is received from the server (i.e. HeadersReceived stage).
    pub is_download: Option<bool>,
    // Redirect location, only sent if a redirect was intercepted.
    pub redirect_url: Option<String>,
    // Details of the Authorization Challenge encountered. If this is set then
    // continueInterceptedRequest must contain an authChallengeResponse.
    pub auth_challenge: Option<AuthChallenge>,
    // Response error if intercepted at response stage or if redirect occurred while intercepting
    // request.
    pub response_error_reason: Option<ErrorReason>,
    // Response code if intercepted at response stage or if redirect occurred while intercepting
    // request or auth retry occurred.
    pub response_status_code: Option<i32>,
    // Response headers if intercepted at the response stage or if redirect occurred while
    // intercepting request or auth retry occurred.
    pub response_headers: Option<Headers>,
    // If the intercepted request had a corresponding requestWillBeSent event fired for it, then
    // this requestId will be the same as the requestId present in the requestWillBeSent event.
    pub request_id: Option<RequestId>,
}
// Fired if request ended up loading from cache.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RequestServedFromCacheEvent {
    pub params: RequestServedFromCacheParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestServedFromCacheParams {
    // Request identifier.
    pub request_id: RequestId,
}
// Fired when page is about to send HTTP request.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RequestWillBeSentEvent {
    pub params: RequestWillBeSentParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestWillBeSentParams {
    // Request identifier.
    pub request_id: RequestId,
    // Loader identifier. Empty string if the request is fetched from worker.
    pub loader_id: LoaderId,
    // URL of the document this request is loaded for.
    pub document_url: String,
    // Request data.
    pub request: Request,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // Timestamp.
    pub wall_time: TimeSinceEpoch,
    // Request initiator.
    pub initiator: Initiator,
    // Redirect response data.
    pub redirect_response: Option<Response>,
    // Type of this resource.
    pub r#type: Option<ResourceType>,
    // Frame identifier.
    pub frame_id: Option<super::page::FrameId>,
    // Whether the request is initiated by a user gesture. Defaults to false.
    pub has_user_gesture: Option<bool>,
}
// Fired when resource loading priority is changed
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ResourceChangedPriorityEvent {
    pub params: ResourceChangedPriorityParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceChangedPriorityParams {
    // Request identifier.
    pub request_id: RequestId,
    // New priority
    pub new_priority: ResourcePriority,
    // Timestamp.
    pub timestamp: MonotonicTime,
}
// Fired when a signed exchange was received over the network
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SignedExchangeReceivedEvent {
    pub params: SignedExchangeReceivedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedExchangeReceivedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Information about the signed exchange response.
    pub info: SignedExchangeInfo,
}
// Fired when HTTP response is available.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseReceivedEvent {
    pub params: ResponseReceivedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseReceivedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Loader identifier. Empty string if the request is fetched from worker.
    pub loader_id: LoaderId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // Resource type.
    pub r#type: ResourceType,
    // Response data.
    pub response: Response,
    // Frame identifier.
    pub frame_id: Option<super::page::FrameId>,
}
// Fired when WebSocket is closed.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketClosedEvent {
    pub params: WebSocketClosedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketClosedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
}
// Fired upon WebSocket creation.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketCreatedEvent {
    pub params: WebSocketCreatedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketCreatedParams {
    // Request identifier.
    pub request_id: RequestId,
    // WebSocket request URL.
    pub url: String,
    // Request initiator.
    pub initiator: Option<Initiator>,
}
// Fired when WebSocket message error occurs.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketFrameErrorEvent {
    pub params: WebSocketFrameErrorParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrameErrorParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // WebSocket error message.
    pub error_message: String,
}
// Fired when WebSocket message is received.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketFrameReceivedEvent {
    pub params: WebSocketFrameReceivedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrameReceivedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // WebSocket response data.
    pub response: WebSocketFrame,
}
// Fired when WebSocket message is sent.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketFrameSentEvent {
    pub params: WebSocketFrameSentParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrameSentParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // WebSocket response data.
    pub response: WebSocketFrame,
}
// Fired when WebSocket handshake response becomes available.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketHandshakeResponseReceivedEvent {
    pub params: WebSocketHandshakeResponseReceivedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketHandshakeResponseReceivedParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // WebSocket response data.
    pub response: WebSocketResponse,
}
// Fired when WebSocket is about to initiate handshake.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct WebSocketWillSendHandshakeRequestEvent {
    pub params: WebSocketWillSendHandshakeRequestParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketWillSendHandshakeRequestParams {
    // Request identifier.
    pub request_id: RequestId,
    // Timestamp.
    pub timestamp: MonotonicTime,
    // UTC Timestamp.
    pub wall_time: TimeSinceEpoch,
    // WebSocket request data.
    pub request: WebSocketRequest,
}
// Fired when additional information about a requestWillBeSent event is available from the
// network stack. Not every requestWillBeSent event will have an additional
// requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
// or requestWillBeSentExtraInfo will be fired first for the same request.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RequestWillBeSentExtraInfoEvent {
    pub params: RequestWillBeSentExtraInfoParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestWillBeSentExtraInfoParams {
    // Request identifier. Used to match this information to an existing requestWillBeSent event.
    pub request_id: RequestId,
    // A list of cookies potentially associated to the requested URL. This includes both cookies sent with
    // the request and the ones not sent; the latter are distinguished by having blockedReason field set.
    pub associated_cookies: Vec<BlockedCookieWithReason>,
    // Raw request headers as they will be sent over the wire.
    pub headers: Headers,
}
// Fired when additional information about a responseReceived event is available from the network
// stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
// it, and responseReceivedExtraInfo may be fired before or after responseReceived.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseReceivedExtraInfoEvent {
    pub params: ResponseReceivedExtraInfoParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseReceivedExtraInfoParams {
    // Request identifier. Used to match this information to another responseReceived event.
    pub request_id: RequestId,
    // A list of cookies which were not stored from the response along with the corresponding
    // reasons for blocking. The cookies here may not be valid due to syntax errors, which
    // are represented by the invalid cookie line string instead of a proper cookie.
    pub blocked_cookies: Vec<BlockedSetCookieWithReason>,
    // Raw response headers as they were received over the wire.
    pub headers: Headers,
    // Raw response header text as it was received over the wire. The raw text may not always be
    // available, such as in the case of HTTP/2 or QUIC.
    pub headers_text: Option<String>,
}
