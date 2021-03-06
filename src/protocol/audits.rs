// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Information about a cookie that is affected by an inspector issue.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AffectedCookie {
    // The following three properties uniquely identify a cookie
    pub name: String,
    pub path: String,
    pub domain: String,
}
// Information about a request that is affected by an inspector issue.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AffectedRequest {
    // The unique request id.
    pub request_id: super::network::RequestId,
    pub url: Option<String>,
}
// Information about the frame affected by an inspector issue.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AffectedFrame {
    pub frame_id: super::page::FrameId,
}
pub type SameSiteCookieExclusionReason = String;
pub type SameSiteCookieWarningReason = String;
pub type SameSiteCookieOperation = String;
// This information is currently necessary, as the front-end has a difficult
// time finding a specific cookie. With this, we can convey specific error
// information without the cookie.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SameSiteCookieIssueDetails {
    pub cookie: AffectedCookie,
    pub cookie_warning_reasons: Vec<SameSiteCookieWarningReason>,
    pub cookie_exclusion_reasons: Vec<SameSiteCookieExclusionReason>,
    // Optionally identifies the site-for-cookies and the cookie url, which
    // may be used by the front-end as additional context.
    pub operation: SameSiteCookieOperation,
    pub site_for_cookies: Option<String>,
    pub cookie_url: Option<String>,
    pub request: Option<AffectedRequest>,
}
pub type MixedContentResolutionStatus = String;
pub type MixedContentResourceType = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MixedContentIssueDetails {
    // The type of resource causing the mixed content issue (css, js, iframe,
    // form,...). Marked as optional because it is mapped to from
    // blink::mojom::RequestContextType, which will be replaced
    // by network::mojom::RequestDestination
    pub resource_type: Option<MixedContentResourceType>,
    // The way the mixed content issue is being resolved.
    pub resolution_status: MixedContentResolutionStatus,
    // The unsafe http url causing the mixed content issue.
    pub insecure_url: String,
    // The url responsible for the call to an unsafe url.
    pub main_resource_url: String,
    // The mixed content request.
    // Does not always exist (e.g. for unsafe form submission urls).
    pub request: Option<AffectedRequest>,
    // Optional because not every mixed content issue is necessarily linked to a frame.
    pub frame: Option<AffectedFrame>,
}
// Enum indicating the reason a response has been blocked. These reasons are
// refinements of the net error BLOCKED_BY_RESPONSE.
pub type BlockedByResponseReason = String;
// Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
// code. Currently only used for COEP/COOP, but may be extended to include
// some CSP errors in the future.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlockedByResponseIssueDetails {
    pub request: AffectedRequest,
    pub parent_frame: Option<AffectedFrame>,
    pub blocked_frame: Option<AffectedFrame>,
    pub reason: BlockedByResponseReason,
}
pub type HeavyAdResolutionStatus = String;
pub type HeavyAdReason = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeavyAdIssueDetails {
    // The resolution status, either blocking the content or warning.
    pub resolution: HeavyAdResolutionStatus,
    // The reason the ad was blocked, total network or cpu or peak cpu.
    pub reason: HeavyAdReason,
    // The frame that was blocked.
    pub frame: AffectedFrame,
}
pub type ContentSecurityPolicyViolationType = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceCodeLocation {
    pub url: String,
    pub line_number: i32,
    pub column_number: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentSecurityPolicyIssueDetails {
    // The url not included in allowed sources.
    pub blocked_url: Option<String>,
    // Specific directive that is violated, causing the CSP issue.
    pub violated_directive: String,
    pub is_report_only: bool,
    pub content_security_policy_violation_type: ContentSecurityPolicyViolationType,
    pub frame_ancestor: Option<AffectedFrame>,
    pub source_code_location: Option<SourceCodeLocation>,
    pub violating_node_id: Option<super::dom::BackendNodeId>,
}
// A unique identifier for the type of issue. Each type may use one of the
// optional fields in InspectorIssueDetails to convey more specific
// information about the kind of issue.
pub type InspectorIssueCode = String;
// This struct holds a list of optional fields with additional information
// specific to the kind of issue. When adding a new issue code, please also
// add a new optional field to this type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssueDetails {
    pub same_site_cookie_issue_details: Option<SameSiteCookieIssueDetails>,
    pub mixed_content_issue_details: Option<MixedContentIssueDetails>,
    pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
    pub heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
    pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
}
// An inspector issue reported from the back-end.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InspectorIssue {
    pub code: InspectorIssueCode,
    pub details: InspectorIssueDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Encoding {
    Webp,
    Jpeg,
    Png,
}
// Returns the response body and size if it were re-encoded with the specified settings. Only
// applies to images.
#[derive(Serialize, Debug)]
pub struct GetEncodedResponse {
    // Identifier of the network request to get content for.
    pub request_id: super::network::RequestId,
    // The encoding to use.
    pub encoding: String,
    // The quality of the encoding (0-1). (defaults to 1)
    pub quality: Option<f64>,
    // Whether to only return the size information (defaults to false).
    pub size_only: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetEncodedResponseReturnObject {
    // The encoded body as a base64 string. Omitted if sizeOnly is true.
    pub body: Option<String>,
    // Size before re-encoding.
    pub original_size: i32,
    // Size after re-encoding.
    pub encoded_size: i32,
}
impl super::Command for GetEncodedResponse {
    const NAME: &'static str = "Audits.getEncodedResponse";

    type ReturnObject = GetEncodedResponseReturnObject;
}
// Disables issues domain, prevents further issues from being reported to the client.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Audits.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables issues domain, sends the issues collected so far to the client by means of the
// `issueAdded` event.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Audits.enable";

    type ReturnObject = EnableReturnObject;
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IssueAddedEvent {
    pub params: IssueAddedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueAddedParams {
    pub issue: InspectorIssue,
}
