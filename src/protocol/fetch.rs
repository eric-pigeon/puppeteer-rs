// This file is auto-generated do not edit manually.

// Unique request identifier.
pub type RequestId = String;
// Stages of the request to handle. Request will intercept before the request is
// sent. Response will intercept after the response is received (but before response
// body is received.
pub enum RequestStage {
    Request,
    Response,
}
pub struct RequestPattern {
    // Wildcards ('*' -> zero or more, '?' -> exactly one) are allowed. Escape character is
    // backslash. Omitting is equivalent to "*".
    pub url_pattern: Option<String>,
    // If set, only requests for matching resource types will be intercepted.
    pub resource_type: Option<super::network::ResourceType>,
    // Stage at wich to begin intercepting requests. Default is Request.
    pub request_stage: Option<RequestStage>,
}
// Response HTTP header entry
pub struct HeaderEntry {
    pub name: String,
    pub value: String,
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

// Disables the fetch domain.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables issuing of requestPaused events. A request will be paused until client
// calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.
pub struct Enable {
    // If specified, only requests matching any of these patterns will produce
    // fetchRequested event and will be paused until clients response. If not set,
    // all requests will be affected.
    pub patterns: Option<Vec<RequestPattern>>,
    // If true, authRequired events will be issued and requests will be paused
    // expecting a call to continueWithAuth.
    pub handle_auth_requests: Option<bool>,
}
pub struct EnableReturnObject {}
// Causes the request to fail with specified reason.
pub struct FailRequest {
    // An id the client received in requestPaused event.
    pub request_id: RequestId,
    // Causes the request to fail with the given reason.
    pub error_reason: super::network::ErrorReason,
}
pub struct FailRequestReturnObject {}
// Provides response to the request.
pub struct FulfillRequest {
    // An id the client received in requestPaused event.
    pub request_id: RequestId,
    // An HTTP response code.
    pub response_code: i32,
    // Response headers.
    pub response_headers: Option<Vec<HeaderEntry>>,
    // Alternative way of specifying response headers as a \0-separated
    // series of name: value pairs. Prefer the above method unless you
    // need to represent some non-UTF8 values that can't be transmitted
    // over the protocol as text.
    pub binary_response_headers: Option<String>,
    // A response body.
    pub body: Option<String>,
    // A textual representation of responseCode.
    // If absent, a standard phrase matching responseCode is used.
    pub response_phrase: Option<String>,
}
pub struct FulfillRequestReturnObject {}
// Continues the request, optionally modifying some of its parameters.
pub struct ContinueRequest {
    // An id the client received in requestPaused event.
    pub request_id: RequestId,
    // If set, the request url will be modified in a way that's not observable by page.
    pub url: Option<String>,
    // If set, the request method is overridden.
    pub method: Option<String>,
    // If set, overrides the post data in the request.
    pub post_data: Option<String>,
    // If set, overrides the request headers.
    pub headers: Option<Vec<HeaderEntry>>,
}
pub struct ContinueRequestReturnObject {}
// Continues a request supplying authChallengeResponse following authRequired event.
pub struct ContinueWithAuth {
    // An id the client received in authRequired event.
    pub request_id: RequestId,
    // Response to  with an authChallenge.
    pub auth_challenge_response: AuthChallengeResponse,
}
pub struct ContinueWithAuthReturnObject {}
// Causes the body of the response to be received from the server and
// returned as a single string. May only be issued for a request that
// is paused in the Response stage and is mutually exclusive with
// takeResponseBodyForInterceptionAsStream. Calling other methods that
// affect the request or disabling fetch domain before body is received
// results in an undefined behavior.
pub struct GetResponseBody {
    // Identifier for the intercepted request to get body for.
    pub request_id: RequestId,
}
pub struct GetResponseBodyReturnObject {
    // Response body.
    pub body: String,
    // True, if content was sent as base64.
    pub base64_encoded: bool,
}
// Returns a handle to the stream representing the response body.
// The request must be paused in the HeadersReceived stage.
// Note that after this command the request can't be continued
// as is -- client either needs to cancel it or to provide the
// response body.
// The stream only supports sequential read, IO.read will fail if the position
// is specified.
// This method is mutually exclusive with getResponseBody.
// Calling other methods that affect the request or disabling fetch
// domain before body is received results in an undefined behavior.
pub struct TakeResponseBodyAsStream {
    pub request_id: RequestId,
}
pub struct TakeResponseBodyAsStreamReturnObject {
    pub stream: super::io::StreamHandle,
}
