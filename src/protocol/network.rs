// This file is auto-generated do not edit manually.

// Resource type as it was perceived by the rendering engine.
pub enum ResourceType {
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    TextTrack,
    XHR,
    Fetch,
    EventSource,
    WebSocket,
    Manifest,
    SignedExchange,
    Ping,
    CSPViolationReport,
    Other,
}
// Unique loader identifier.
pub type LoaderId = String;
// Unique request identifier.
pub type RequestId = String;
// Unique intercepted request identifier.
pub type InterceptionId = String;
// Network level fetch failure reason.
pub enum ErrorReason {
    Failed,
    Aborted,
    TimedOut,
    AccessDenied,
    ConnectionClosed,
    ConnectionReset,
    ConnectionRefused,
    ConnectionAborted,
    ConnectionFailed,
    NameNotResolved,
    InternetDisconnected,
    AddressUnreachable,
    BlockedByClient,
    BlockedByResponse,
}
// UTC time in seconds, counted from January 1, 1970.
pub type TimeSinceEpoch = f64;
// Monotonically increasing time in seconds since an arbitrary point in the past.
pub type MonotonicTime = f64;
// Request / response headers as keys / values of JSON object.
pub type Headers = std::collections::HashMap<String, String>;
// The underlying connection technology that the browser is supposedly using.
pub enum ConnectionType {
    None,
    Cellular2g,
    Cellular3g,
    Cellular4g,
    Bluetooth,
    Ethernet,
    Wifi,
    Wimax,
    Other,
}
// Represents the cookie's 'SameSite' status:
// https://tools.ietf.org/html/draft-west-first-party-cookies
pub enum CookieSameSite {
    Strict,
    Lax,
    None,
}
// Represents the cookie's 'Priority' status:
// https://tools.ietf.org/html/draft-west-cookie-priority-00
pub enum CookiePriority {
    Low,
    Medium,
    High,
}
// Timing information for the request.
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
pub enum ResourcePriority {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}
// Post data entry for HTTP request
pub struct PostDataEntry {
    pub bytes: Option<String>,
}
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
    pub referrer_policy: RequestReferrerPolicy,
    // Whether is loaded via link preload.
    pub is_link_preload: Option<bool>,
    // Set for requests when the TrustToken API is used. Contains the parameters
    // passed by the developer (e.g. via "fetch") as understood by the backend.
    pub trust_token_params: Option<TrustTokenParams>,
}
// Details of a signed certificate timestamp (SCT).
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
pub enum CertificateTransparencyCompliance {
    Unknown,
    NotCompliant,
    Compliant,
}
// The reason why request was blocked.
pub enum BlockedReason {
    Other,
    Csp,
    MixedContent,
    Origin,
    Inspector,
    SubresourceFilter,
    ContentType,
    CollapsedByClient,
    CoepFrameResourceNeedsCoepHeader,
    CoopSandboxedIframeCannotNavigateToCoopPage,
    CorpNotSameOrigin,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    CorpNotSameSite,
}
// Source of serviceworker response.
pub enum ServiceWorkerResponseSource {
    CacheStorage,
    HttpCache,
    FallbackCode,
    Network,
}
pub enum TrustTokenParamsRefreshPolicy {
    UseCached,
    Refresh,
}
// Determines what type of Trust Token operation is executed and
// depending on the type, some additional parameters.
pub struct TrustTokenParams {
    pub r#type: TrustTokenOperationType,
    // Only set for "srr-token-redemption" type and determine whether
    // to request a fresh SRR or use a still valid cached SRR.
    pub refresh_policy: TrustTokenParamsRefreshPolicy,
    // Origins of issuers from whom to request tokens or redemption
    // records.
    pub issuers: Option<Vec<String>>,
}
pub enum TrustTokenOperationType {
    Issuance,
    Redemption,
    Signing,
}
// HTTP response data.
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
pub struct WebSocketRequest {
    // HTTP request headers.
    pub headers: Headers,
}
// WebSocket response data.
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
pub enum InitiatorType {
    Parser,
    Script,
    Preload,
    SignedExchange,
    Other,
}
// Information about the request initiator.
pub struct Initiator {
    // Type of this initiator.
    pub r#type: InitiatorType,
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
pub enum SetCookieBlockedReason {
    SecureOnly,
    SameSiteStrict,
    SameSiteLax,
    SameSiteUnspecifiedTreatedAsLax,
    SameSiteNoneInsecure,
    UserPreferences,
    SyntaxError,
    SchemeNotSupported,
    OverwriteSecure,
    InvalidDomain,
    InvalidPrefix,
    UnknownError,
    SchemefulSameSiteStrict,
    SchemefulSameSiteLax,
    SchemefulSameSiteUnspecifiedTreatedAsLax,
}
// Types of reasons why a cookie may not be sent with a request.
pub enum CookieBlockedReason {
    SecureOnly,
    NotOnPath,
    DomainMismatch,
    SameSiteStrict,
    SameSiteLax,
    SameSiteUnspecifiedTreatedAsLax,
    SameSiteNoneInsecure,
    UserPreferences,
    UnknownError,
    SchemefulSameSiteStrict,
    SchemefulSameSiteLax,
    SchemefulSameSiteUnspecifiedTreatedAsLax,
}
// A cookie which was not stored from a response with the corresponding reason.
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
pub struct BlockedCookieWithReason {
    // The reason(s) the cookie was blocked.
    pub blocked_reasons: Vec<CookieBlockedReason>,
    // The cookie object representing the cookie which was not sent.
    pub cookie: Cookie,
}
// Cookie parameter object
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
pub enum AuthChallengeSource {
    Server,
    Proxy,
}
// Authorization challenge for HTTP status code 401 or 407.
pub struct AuthChallenge {
    // Source of the authentication challenge.
    pub source: Option<AuthChallengeSource>,
    // Origin of the challenger.
    pub origin: String,
    // The authentication scheme used, such as basic or digest
    pub scheme: String,
    // The realm of the challenge. May be empty.
    pub realm: String,
}
pub enum AuthChallengeResponseResponse {
    Default,
    CancelAuth,
    ProvideCredentials,
}
// Response to an AuthChallenge.
pub struct AuthChallengeResponse {
    // The decision on what to do in response to the authorization challenge.  Default means
    // deferring to the default behavior of the net stack, which will likely either the Cancel
    // authentication or display a popup dialog box.
    pub response: AuthChallengeResponseResponse,
    // The username to provide, possibly empty. Should only be set if response is
    // ProvideCredentials.
    pub username: Option<String>,
    // The password to provide, possibly empty. Should only be set if response is
    // ProvideCredentials.
    pub password: Option<String>,
}
// Stages of the interception to begin intercepting. Request will intercept before the request is
// sent. Response will intercept after the response is received.
pub enum InterceptionStage {
    Request,
    HeadersReceived,
}
// Request pattern for interception.
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
pub enum SignedExchangeErrorField {
    SignatureSig,
    SignatureIntegrity,
    SignatureCertUrl,
    SignatureCertSha256,
    SignatureValidityUrl,
    SignatureTimestamps,
}
// Information about a signed exchange response.
pub struct SignedExchangeError {
    // Error message.
    pub message: String,
    // The index of the signature which caused the error.
    pub signature_index: Option<i32>,
    // The field which caused the error.
    pub error_field: Option<SignedExchangeErrorField>,
}
// Information about a signed exchange response.
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
pub enum CrossOriginOpenerPolicyValue {
    SameOrigin,
    SameOriginAllowPopups,
    UnsafeNone,
    SameOriginPlusCoep,
}
pub struct CrossOriginOpenerPolicyStatus {
    pub value: CrossOriginOpenerPolicyValue,
    pub report_only_value: CrossOriginOpenerPolicyValue,
    pub reporting_endpoint: Option<String>,
    pub report_only_reporting_endpoint: Option<String>,
}
pub enum CrossOriginEmbedderPolicyValue {
    None,
    RequireCorp,
}
pub struct CrossOriginEmbedderPolicyStatus {
    pub value: CrossOriginEmbedderPolicyValue,
    pub report_only_value: CrossOriginEmbedderPolicyValue,
    pub reporting_endpoint: Option<String>,
    pub report_only_reporting_endpoint: Option<String>,
}
pub struct SecurityIsolationStatus {
    pub coop: CrossOriginOpenerPolicyStatus,
    pub coep: CrossOriginEmbedderPolicyStatus,
}
// An object providing the result of a network resource load.
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
pub struct LoadNetworkResourceOptions {
    pub disable_cache: bool,
    pub include_credentials: bool,
}

// Tells whether clearing browser cache is supported.
pub struct CanClearBrowserCache {}
pub struct CanClearBrowserCacheReturnObject {
    // True if browser cache can be cleared.
    pub result: bool,
}
// Tells whether clearing browser cookies is supported.
pub struct CanClearBrowserCookies {}
pub struct CanClearBrowserCookiesReturnObject {
    // True if browser cookies can be cleared.
    pub result: bool,
}
// Tells whether emulation of network conditions is supported.
pub struct CanEmulateNetworkConditions {}
pub struct CanEmulateNetworkConditionsReturnObject {
    // True if emulation of network conditions is supported.
    pub result: bool,
}
// Clears browser cache.
pub struct ClearBrowserCache {}
pub struct ClearBrowserCacheReturnObject {}
// Clears browser cookies.
pub struct ClearBrowserCookies {}
pub struct ClearBrowserCookiesReturnObject {}
// Response to Network.requestIntercepted which either modifies the request to continue with any
// modifications, or blocks it, or completes it with the provided response bytes. If a network
// fetch occurs as a result which encounters a redirect an additional Network.requestIntercepted
// event will be sent with the same InterceptionId.
// Deprecated, use Fetch.continueRequest, Fetch.fulfillRequest and Fetch.failRequest instead.
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
pub struct ContinueInterceptedRequestReturnObject {}
// Deletes browser cookies with matching name and url or domain/path pair.
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
pub struct DeleteCookiesReturnObject {}
// Disables network tracking, prevents network events from being sent to the client.
pub struct Disable {}
pub struct DisableReturnObject {}
// Activates emulation of network conditions.
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
pub struct EmulateNetworkConditionsReturnObject {}
// Enables network tracking, network events will now be delivered to the client.
pub struct Enable {
    // Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub max_total_buffer_size: Option<i32>,
    // Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).
    pub max_resource_buffer_size: Option<i32>,
    // Longest post body size (in bytes) that would be included in requestWillBeSent notification
    pub max_post_data_size: Option<i32>,
}
pub struct EnableReturnObject {}
// Returns all browser cookies. Depending on the backend support, will return detailed cookie
// information in the `cookies` field.
pub struct GetAllCookies {}
pub struct GetAllCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<Cookie>,
}
// Returns the DER-encoded certificate.
pub struct GetCertificate {
    // Origin to get certificate for.
    pub origin: String,
}
pub struct GetCertificateReturnObject {
    pub table_names: Vec<String>,
}
// Returns all browser cookies for the current URL. Depending on the backend support, will return
// detailed cookie information in the `cookies` field.
pub struct GetCookies {
    // The list of URLs for which applicable cookies will be fetched.
    // If not specified, it's assumed to be set to the list containing
    // the URLs of the page and all of its subframes.
    pub urls: Option<Vec<String>>,
}
pub struct GetCookiesReturnObject {
    // Array of cookie objects.
    pub cookies: Vec<Cookie>,
}
// Returns content served for the given request.
pub struct GetResponseBody {
    // Identifier of the network request to get content for.
    pub request_id: RequestId,
}
pub struct GetResponseBodyReturnObject {
    // Response body.
    pub body: String,
    // True, if content was sent as base64.
    pub base64_encoded: bool,
}
// Returns post data sent with the request. Returns an error when no data was sent with the request.
pub struct GetRequestPostData {
    // Identifier of the network request to get content for.
    pub request_id: RequestId,
}
pub struct GetRequestPostDataReturnObject {
    // Request body string, omitting files from multipart requests
    pub post_data: String,
}
// Returns content served for the given currently intercepted request.
pub struct GetResponseBodyForInterception {
    // Identifier for the intercepted request to get body for.
    pub interception_id: InterceptionId,
}
pub struct GetResponseBodyForInterceptionReturnObject {
    // Response body.
    pub body: String,
    // True, if content was sent as base64.
    pub base64_encoded: bool,
}
// Returns a handle to the stream representing the response body. Note that after this command,
// the intercepted request can't be continued as is -- you either need to cancel it or to provide
// the response body. The stream only supports sequential read, IO.read will fail if the position
// is specified.
pub struct TakeResponseBodyForInterceptionAsStream {
    pub interception_id: InterceptionId,
}
pub struct TakeResponseBodyForInterceptionAsStreamReturnObject {
    pub stream: super::io::StreamHandle,
}
// This method sends a new XMLHttpRequest which is identical to the original one. The following
// parameters should be identical: method, url, async, request body, extra headers, withCredentials
// attribute, user, password.
pub struct ReplayXHR {
    // Identifier of XHR to replay.
    pub request_id: RequestId,
}
pub struct ReplayXHRReturnObject {}
// Searches for given string in response content.
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
pub struct SearchInResponseBodyReturnObject {
    // List of search matches.
    pub result: Vec<super::debugger::SearchMatch>,
}
// Blocks URLs from loading.
pub struct SetBlockedURLs {
    // URL patterns to block. Wildcards ('*') are allowed.
    pub urls: Vec<String>,
}
pub struct SetBlockedURLsReturnObject {}
// Toggles ignoring of service worker for each request.
pub struct SetBypassServiceWorker {
    // Bypass service worker and load from network.
    pub bypass: bool,
}
pub struct SetBypassServiceWorkerReturnObject {}
// Toggles ignoring cache for each request. If `true`, cache will not be used.
pub struct SetCacheDisabled {
    // Cache disabled state.
    pub cache_disabled: bool,
}
pub struct SetCacheDisabledReturnObject {}
// Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
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
pub struct SetCookieReturnObject {
    // Always set to true. If an error occurs, the response indicates protocol error.
    pub success: bool,
}
// Sets given cookies.
pub struct SetCookies {
    // Cookies to be set.
    pub cookies: Vec<CookieParam>,
}
pub struct SetCookiesReturnObject {}
// For testing.
pub struct SetDataSizeLimitsForTest {
    // Maximum total buffer size.
    pub max_total_size: i32,
    // Maximum per-resource size.
    pub max_resource_size: i32,
}
pub struct SetDataSizeLimitsForTestReturnObject {}
// Specifies whether to always send extra HTTP headers with the requests from this page.
pub struct SetExtraHTTPHeaders {
    // Map with extra HTTP headers.
    pub headers: Headers,
}
pub struct SetExtraHTTPHeadersReturnObject {}
// Specifies whether to sned a debug header to all outgoing requests.
pub struct SetAttachDebugHeader {
    // Whether to send a debug header.
    pub enabled: bool,
}
pub struct SetAttachDebugHeaderReturnObject {}
// Sets the requests to intercept that match the provided patterns and optionally resource types.
// Deprecated, please use Fetch.enable instead.
pub struct SetRequestInterception {
    // Requests matching any of these patterns will be forwarded and wait for the corresponding
    // continueInterceptedRequest call.
    pub patterns: Vec<RequestPattern>,
}
pub struct SetRequestInterceptionReturnObject {}
// Allows overriding user agent with the given string.
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
pub struct SetUserAgentOverrideReturnObject {}
// Returns information about the COEP/COOP isolation status.
pub struct GetSecurityIsolationStatus {
    // If no frameId is provided, the status of the target is provided.
    pub frame_id: Option<super::page::FrameId>,
}
pub struct GetSecurityIsolationStatusReturnObject {
    pub status: SecurityIsolationStatus,
}
// Fetches the resource and returns the content.
pub struct LoadNetworkResource {
    // Frame id to get the resource for.
    pub frame_id: super::page::FrameId,
    // URL of the resource to get content for.
    pub url: String,
    // Options for the request.
    pub options: LoadNetworkResourceOptions,
}
pub struct LoadNetworkResourceReturnObject {
    pub resource: LoadNetworkResourcePageResult,
}
