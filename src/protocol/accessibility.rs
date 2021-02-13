// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Unique accessibility node identifier.
pub type AXNodeId = String;
// Enum of possible property types.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AXValueType {
    Boolean,
    Tristate,
    BooleanOrUndefined,
    Idref,
    IdrefList,
    Integer,
    Node,
    NodeList,
    Number,
    String,
    ComputedString,
    Token,
    TokenList,
    DomRelation,
    Role,
    InternalRole,
    ValueUndefined,
}
// Enum of possible property sources.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AXValueSourceType {
    Attribute,
    Implicit,
    Style,
    Contents,
    Placeholder,
    RelatedElement,
}
// Enum of possible native property sources (as a subtype of a particular AXValueSourceType).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AXValueNativeSourceType {
    Figcaption,
    Label,
    Labelfor,
    Labelwrapped,
    Legend,
    Tablecaption,
    Title,
    Other,
}
// A single source for a computed AX property.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AXValueSource {
    // What type of source this is.
    pub r#type: AXValueSourceType,
    // The value of this property source.
    pub value: Option<AXValue>,
    // The name of the relevant attribute, if any.
    pub attribute: Option<String>,
    // The value of the relevant attribute, if any.
    pub attribute_value: Option<AXValue>,
    // Whether this source is superseded by a higher priority source.
    pub superseded: Option<bool>,
    // The native markup source for this value, e.g. a <label> element.
    pub native_source: Option<AXValueNativeSourceType>,
    // The value, such as a node or node list, of the native source.
    pub native_source_value: Option<AXValue>,
    // Whether the value for this property is invalid.
    pub invalid: Option<bool>,
    // Reason for the value being invalid, if it is.
    pub invalid_reason: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AXRelatedNode {
    // The BackendNodeId of the related DOM node.
    pub backend_dom_node_id: super::dom::BackendNodeId,
    // The IDRef value provided, if any.
    pub idref: Option<String>,
    // The text alternative of this node in the current context.
    pub text: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AXProperty {
    // The name of this property.
    pub name: AXPropertyName,
    // The value of this property.
    pub value: AXValue,
}
// A single computed AX property.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AXValue {
    // The type of this value.
    pub r#type: AXValueType,
    // The computed value of this property.
    pub value: Option<serde_json::Value>,
    // One or more related nodes, if applicable.
    pub related_nodes: Option<Vec<AXRelatedNode>>,
    // The sources which contributed to the computation of this property.
    pub sources: Option<Vec<AXValueSource>>,
}
// Values of AXProperty name:
// - from 'busy' to 'roledescription': states which apply to every AX node
// - from 'live' to 'root': attributes which apply to nodes in live regions
// - from 'autocomplete' to 'valuetext': attributes which apply to widgets
// - from 'checked' to 'selected': states which apply to widgets
// - from 'activedescendant' to 'owns' - relationships between elements other than parent/child/sibling.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AXPropertyName {
    Busy,
    Disabled,
    Editable,
    Focusable,
    Focused,
    Hidden,
    HiddenRoot,
    Invalid,
    Keyshortcuts,
    Settable,
    Roledescription,
    Live,
    Atomic,
    Relevant,
    Root,
    Autocomplete,
    HasPopup,
    Level,
    Multiselectable,
    Orientation,
    Multiline,
    Readonly,
    Required,
    Valuemin,
    Valuemax,
    Valuetext,
    Checked,
    Expanded,
    Modal,
    Pressed,
    Selected,
    Activedescendant,
    Controls,
    Describedby,
    Details,
    Errormessage,
    Flowto,
    Labelledby,
    Owns,
}
// A node in the accessibility tree.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AXNode {
    // Unique identifier for this node.
    pub node_id: AXNodeId,
    // Whether this node is ignored for accessibility
    pub ignored: bool,
    // Collection of reasons why this node is hidden.
    pub ignored_reasons: Option<Vec<AXProperty>>,
    // This `Node`'s role, whether explicit or implicit.
    pub role: Option<AXValue>,
    // The accessible name for this `Node`.
    pub name: Option<AXValue>,
    // The accessible description for this `Node`.
    pub description: Option<AXValue>,
    // The value for this `Node`.
    pub value: Option<AXValue>,
    // All other properties
    pub properties: Option<Vec<AXProperty>>,
    // IDs for each of this node's child nodes.
    pub child_ids: Option<Vec<AXNodeId>>,
    // The backend ID for the associated DOM node, if any.
    pub backend_dom_node_id: Option<super::dom::BackendNodeId>,
}

// Disables the accessibility domain.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Accessibility.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.
// This turns on accessibility for the page, which can impact performance until accessibility is disabled.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Accessibility.enable";

    type ReturnObject = EnableReturnObject;
}
// Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.
#[derive(Serialize, Debug)]
pub struct GetPartialAXTree {
    // Identifier of the node to get the partial accessibility tree for.
    pub node_id: Option<super::dom::NodeId>,
    // Identifier of the backend node to get the partial accessibility tree for.
    pub backend_node_id: Option<super::dom::BackendNodeId>,
    // JavaScript object id of the node wrapper to get the partial accessibility tree for.
    pub object_id: Option<super::runtime::RemoteObjectId>,
    // Whether to fetch this nodes ancestors, siblings and children. Defaults to true.
    pub fetch_relatives: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetPartialAXTreeReturnObject {
    // The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and
    // children, if requested.
    pub nodes: Vec<AXNode>,
}
impl super::Command for GetPartialAXTree {
    const NAME: &'static str = "Accessibility.getPartialAXTree";

    type ReturnObject = GetPartialAXTreeReturnObject;
}
// Fetches the entire accessibility tree
#[derive(Serialize, Debug)]
pub struct GetFullAXTree {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetFullAXTreeReturnObject {
    pub nodes: Vec<AXNode>,
}
impl super::Command for GetFullAXTree {
    const NAME: &'static str = "Accessibility.getFullAXTree";

    type ReturnObject = GetFullAXTreeReturnObject;
}
// Query a DOM node's accessibility subtree for accessible name and role.
// This command computes the name and role for all nodes in the subtree, including those that are
// ignored for accessibility, and returns those that mactch the specified name and role. If no DOM
// node is specified, or the DOM node does not exist, the command returns an error. If neither
// `accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.
#[derive(Serialize, Debug)]
pub struct QueryAXTree {
    // Identifier of the node for the root to query.
    pub node_id: Option<super::dom::NodeId>,
    // Identifier of the backend node for the root to query.
    pub backend_node_id: Option<super::dom::BackendNodeId>,
    // JavaScript object id of the node wrapper for the root to query.
    pub object_id: Option<super::runtime::RemoteObjectId>,
    // Find nodes with this computed name.
    pub accessible_name: Option<String>,
    // Find nodes with this computed role.
    pub role: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct QueryAXTreeReturnObject {
    // A list of `Accessibility.AXNode` matching the specified attributes,
    // including nodes that are ignored for accessibility.
    pub nodes: Vec<AXNode>,
}
impl super::Command for QueryAXTree {
    const NAME: &'static str = "Accessibility.queryAXTree";

    type ReturnObject = QueryAXTreeReturnObject;
}
