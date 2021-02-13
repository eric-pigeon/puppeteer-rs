// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Unique DOM node identifier.
pub type NodeId = i32;
// Unique DOM node identifier used to reference a node that may not have been pushed to the
// front-end.
pub type BackendNodeId = i32;
// Backend node with a friendly name.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackendNode {
    // `Node`'s nodeType.
    pub node_type: i32,
    // `Node`'s nodeName.
    pub node_name: String,
    pub backend_node_id: BackendNodeId,
}
// Pseudo element type.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PseudoType {
    FirstLine,
    FirstLetter,
    Before,
    After,
    Marker,
    Backdrop,
    Selection,
    FirstLineInherited,
    Scrollbar,
    ScrollbarThumb,
    ScrollbarButton,
    ScrollbarTrack,
    ScrollbarTrackPiece,
    ScrollbarCorner,
    Resizer,
    InputListButton,
}
// Shadow root type.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ShadowRootType {
    UserAgent,
    Open,
    Closed,
}
// DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
// DOMNode is a base node mirror type.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    // Node identifier that is passed into the rest of the DOM messages as the `nodeId`. Backend
    // will only push node with given `id` once. It is aware of all requested nodes and will only
    // fire DOM events for nodes known to the client.
    pub node_id: NodeId,
    // The id of the parent node if any.
    pub parent_id: Option<NodeId>,
    // The BackendNodeId for this node.
    pub backend_node_id: BackendNodeId,
    // `Node`'s nodeType.
    pub node_type: i32,
    // `Node`'s nodeName.
    pub node_name: String,
    // `Node`'s localName.
    pub local_name: String,
    // `Node`'s nodeValue.
    pub node_value: String,
    // Child count for `Container` nodes.
    pub child_node_count: Option<i32>,
    // Child nodes of this node when requested with children.
    pub children: Option<Vec<Node>>,
    // Attributes of the `Element` node in the form of flat array `[name1, value1, name2, value2]`.
    pub attributes: Option<Vec<String>>,
    // Document URL that `Document` or `FrameOwner` node points to.
    pub document_url: Option<String>,
    // Base URL that `Document` or `FrameOwner` node uses for URL completion.
    pub base_url: Option<String>,
    // `DocumentType`'s publicId.
    pub public_id: Option<String>,
    // `DocumentType`'s systemId.
    pub system_id: Option<String>,
    // `DocumentType`'s internalSubset.
    pub internal_subset: Option<String>,
    // `Document`'s XML version in case of XML documents.
    pub xml_version: Option<String>,
    // `Attr`'s name.
    pub name: Option<String>,
    // `Attr`'s value.
    pub value: Option<String>,
    // Pseudo element type for this node.
    pub pseudo_type: Option<PseudoType>,
    // Shadow root type.
    pub shadow_root_type: Option<ShadowRootType>,
    // Frame ID for frame owner elements.
    pub frame_id: Option<super::page::FrameId>,
    // Content document for frame owner elements.
    pub content_document: Option<Box<Node>>,
    // Shadow root list for given element host.
    pub shadow_roots: Option<Vec<Node>>,
    // Content document fragment for template elements.
    pub template_content: Option<Box<Node>>,
    // Pseudo elements associated with this node.
    pub pseudo_elements: Option<Vec<Node>>,
    // Import document for the HTMLImport links.
    pub imported_document: Option<Box<Node>>,
    // Distributed nodes for given insertion point.
    pub distributed_nodes: Option<Vec<BackendNode>>,
    // Whether the node is SVG.
    pub is_svg: Option<bool>,
}
// A structure holding an RGBA color.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RGBA {
    // The red component, in the [0-255] range.
    pub r: i32,
    // The green component, in the [0-255] range.
    pub g: i32,
    // The blue component, in the [0-255] range.
    pub b: i32,
    // The alpha component, in the [0-1] range (default: 1).
    pub a: Option<f64>,
}
// An array of quad vertices, x immediately followed by y for each point, points clock-wise.
// TODO figure out how to handle other array types
// only working for DOM now
pub type Quad = [f64; 8];
// Box model.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoxModel {
    // Content box
    pub content: Quad,
    // Padding box
    pub padding: Quad,
    // Border box
    pub border: Quad,
    // Margin box
    pub margin: Quad,
    // Node width
    pub width: i32,
    // Node height
    pub height: i32,
    // Shape outside coordinates
    pub shape_outside: Option<ShapeOutsideInfo>,
}
// CSS Shape Outside details.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShapeOutsideInfo {
    // Shape bounds
    pub bounds: Quad,
    // Shape coordinate details
    pub shape: Vec<serde_json::Value>,
    // Margin shape bounds
    pub margin_shape: Vec<serde_json::Value>,
}
// Rectangle.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rect {
    // X coordinate
    pub x: f64,
    // Y coordinate
    pub y: f64,
    // Rectangle width
    pub width: f64,
    // Rectangle height
    pub height: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CSSComputedStyleProperty {
    // Computed style property name.
    pub name: String,
    // Computed style property value.
    pub value: String,
}

// Collects class names for the node with given id and all of it's child nodes.
#[derive(Serialize, Debug)]
pub struct CollectClassNamesFromSubtree {
    // Id of the node to collect class names.
    pub node_id: NodeId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CollectClassNamesFromSubtreeReturnObject {
    // Class name list.
    pub class_names: Vec<String>,
}
impl super::Command for CollectClassNamesFromSubtree {
    const NAME: &'static str = "DOM.collectClassNamesFromSubtree";

    type ReturnObject = CollectClassNamesFromSubtreeReturnObject;
}
// Creates a deep copy of the specified node and places it into the target container before the
// given anchor.
#[derive(Serialize, Debug)]
pub struct CopyTo {
    // Id of the node to copy.
    pub node_id: NodeId,
    // Id of the element to drop the copy into.
    pub target_node_id: NodeId,
    // Drop the copy before this node (if absent, the copy becomes the last child of
    // `targetNodeId`).
    pub insert_before_node_id: Option<NodeId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CopyToReturnObject {
    // Id of the node clone.
    pub node_id: NodeId,
}
impl super::Command for CopyTo {
    const NAME: &'static str = "DOM.copyTo";

    type ReturnObject = CopyToReturnObject;
}
// Describes node given its id, does not require domain to be enabled. Does not start tracking any
// objects, can be used for automation.
#[derive(Serialize, Debug)]
pub struct DescribeNode {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
    // The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the subtree
    // (default is false).
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DescribeNodeReturnObject {
    // Node description.
    pub node: Node,
}
impl super::Command for DescribeNode {
    const NAME: &'static str = "DOM.describeNode";

    type ReturnObject = DescribeNodeReturnObject;
}
// Scrolls the specified rect of the given node into view if not already visible.
// Note: exactly one between nodeId, backendNodeId and objectId should be passed
// to identify the node.
#[derive(Serialize, Debug)]
pub struct ScrollIntoViewIfNeeded {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
    // The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    // When omitted, center of the node will be used, similar to Element.scrollIntoView.
    pub rect: Option<Rect>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ScrollIntoViewIfNeededReturnObject {}
impl super::Command for ScrollIntoViewIfNeeded {
    const NAME: &'static str = "DOM.scrollIntoViewIfNeeded";

    type ReturnObject = ScrollIntoViewIfNeededReturnObject;
}
// Disables DOM agent for the given page.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "DOM.disable";

    type ReturnObject = DisableReturnObject;
}
// Discards search results from the session with the given id. `getSearchResults` should no longer
// be called for that search.
#[derive(Serialize, Debug)]
pub struct DiscardSearchResults {
    // Unique search session identifier.
    pub search_id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DiscardSearchResultsReturnObject {}
impl super::Command for DiscardSearchResults {
    const NAME: &'static str = "DOM.discardSearchResults";

    type ReturnObject = DiscardSearchResultsReturnObject;
}
// Enables DOM agent for the given page.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "DOM.enable";

    type ReturnObject = EnableReturnObject;
}
// Focuses the given element.
#[derive(Serialize, Debug)]
pub struct Focus {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct FocusReturnObject {}
impl super::Command for Focus {
    const NAME: &'static str = "DOM.focus";

    type ReturnObject = FocusReturnObject;
}
// Returns attributes for the specified node.
#[derive(Serialize, Debug)]
pub struct GetAttributes {
    // Id of the node to retrieve attibutes for.
    pub node_id: NodeId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetAttributesReturnObject {
    // An interleaved array of node attribute names and values.
    pub attributes: Vec<String>,
}
impl super::Command for GetAttributes {
    const NAME: &'static str = "DOM.getAttributes";

    type ReturnObject = GetAttributesReturnObject;
}
// Returns boxes for the given node.
#[derive(Serialize, Debug)]
pub struct GetBoxModel {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetBoxModelReturnObject {
    // Box model for the node.
    pub model: BoxModel,
}
impl super::Command for GetBoxModel {
    const NAME: &'static str = "DOM.getBoxModel";

    type ReturnObject = GetBoxModelReturnObject;
}
// Returns quads that describe node position on the page. This method
// might return multiple quads for inline nodes.
#[derive(Serialize, Debug)]
pub struct GetContentQuads {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetContentQuadsReturnObject {
    // Quads that describe node layout relative to viewport.
    pub quads: Vec<Quad>,
}
impl super::Command for GetContentQuads {
    const NAME: &'static str = "DOM.getContentQuads";

    type ReturnObject = GetContentQuadsReturnObject;
}
// Returns the root DOM node (and optionally the subtree) to the caller.
#[derive(Serialize, Debug)]
pub struct GetDocument {
    // The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the subtree
    // (default is false).
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetDocumentReturnObject {
    // Resulting node.
    pub root: Node,
}
impl super::Command for GetDocument {
    const NAME: &'static str = "DOM.getDocument";

    type ReturnObject = GetDocumentReturnObject;
}
// Returns the root DOM node (and optionally the subtree) to the caller.
// Deprecated, as it is not designed to work well with the rest of the DOM agent.
// Use DOMSnapshot.captureSnapshot instead.
#[derive(Serialize, Debug)]
pub struct GetFlattenedDocument {
    // The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the subtree
    // (default is false).
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetFlattenedDocumentReturnObject {
    // Resulting node.
    pub nodes: Vec<Node>,
}
impl super::Command for GetFlattenedDocument {
    const NAME: &'static str = "DOM.getFlattenedDocument";

    type ReturnObject = GetFlattenedDocumentReturnObject;
}
// Finds nodes with a given computed style in a subtree.
#[derive(Serialize, Debug)]
pub struct GetNodesForSubtreeByStyle {
    // Node ID pointing to the root of a subtree.
    pub node_id: NodeId,
    // The style to filter nodes by (includes nodes if any of properties matches).
    pub computed_styles: Vec<CSSComputedStyleProperty>,
    // Whether or not iframes and shadow roots in the same target should be traversed when returning the
    // results (default is false).
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetNodesForSubtreeByStyleReturnObject {
    // Resulting nodes.
    pub node_ids: Vec<NodeId>,
}
impl super::Command for GetNodesForSubtreeByStyle {
    const NAME: &'static str = "DOM.getNodesForSubtreeByStyle";

    type ReturnObject = GetNodesForSubtreeByStyleReturnObject;
}
// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
// either returned or not.
#[derive(Serialize, Debug)]
pub struct GetNodeForLocation {
    // X coordinate.
    pub x: i32,
    // Y coordinate.
    pub y: i32,
    // False to skip to the nearest non-UA shadow root ancestor (default: false).
    pub include_user_agent_shadow_dom: Option<bool>,
    // Whether to ignore pointer-events: none on elements and hit test them.
    pub ignore_pointer_events_none: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetNodeForLocationReturnObject {
    // Resulting node.
    pub backend_node_id: BackendNodeId,
    // Frame this node belongs to.
    pub frame_id: super::page::FrameId,
    // Id of the node at given coordinates, only when enabled and requested document.
    pub node_id: Option<NodeId>,
}
impl super::Command for GetNodeForLocation {
    const NAME: &'static str = "DOM.getNodeForLocation";

    type ReturnObject = GetNodeForLocationReturnObject;
}
// Returns node's HTML markup.
#[derive(Serialize, Debug)]
pub struct GetOuterHTML {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetOuterHTMLReturnObject {
    // Outer HTML markup.
    pub outer_html: String,
}
impl super::Command for GetOuterHTML {
    const NAME: &'static str = "DOM.getOuterHTML";

    type ReturnObject = GetOuterHTMLReturnObject;
}
// Returns the id of the nearest ancestor that is a relayout boundary.
#[derive(Serialize, Debug)]
pub struct GetRelayoutBoundary {
    // Id of the node.
    pub node_id: NodeId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetRelayoutBoundaryReturnObject {
    // Relayout boundary node id for the given node.
    pub node_id: NodeId,
}
impl super::Command for GetRelayoutBoundary {
    const NAME: &'static str = "DOM.getRelayoutBoundary";

    type ReturnObject = GetRelayoutBoundaryReturnObject;
}
// Returns search results from given `fromIndex` to given `toIndex` from the search with the given
// identifier.
#[derive(Serialize, Debug)]
pub struct GetSearchResults {
    // Unique search session identifier.
    pub search_id: String,
    // Start index of the search result to be returned.
    pub from_index: i32,
    // End index of the search result to be returned.
    pub to_index: i32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetSearchResultsReturnObject {
    // Ids of the search result nodes.
    pub node_ids: Vec<NodeId>,
}
impl super::Command for GetSearchResults {
    const NAME: &'static str = "DOM.getSearchResults";

    type ReturnObject = GetSearchResultsReturnObject;
}
// Hides any highlight.
#[derive(Serialize, Debug)]
pub struct HideHighlight {}
#[derive(Deserialize, Debug, Clone)]
pub struct HideHighlightReturnObject {}
impl super::Command for HideHighlight {
    const NAME: &'static str = "DOM.hideHighlight";

    type ReturnObject = HideHighlightReturnObject;
}
// Highlights DOM node.
#[derive(Serialize, Debug)]
pub struct HighlightNode {}
#[derive(Deserialize, Debug, Clone)]
pub struct HighlightNodeReturnObject {}
impl super::Command for HighlightNode {
    const NAME: &'static str = "DOM.highlightNode";

    type ReturnObject = HighlightNodeReturnObject;
}
// Highlights given rectangle.
#[derive(Serialize, Debug)]
pub struct HighlightRect {}
#[derive(Deserialize, Debug, Clone)]
pub struct HighlightRectReturnObject {}
impl super::Command for HighlightRect {
    const NAME: &'static str = "DOM.highlightRect";

    type ReturnObject = HighlightRectReturnObject;
}
// Marks last undoable state.
#[derive(Serialize, Debug)]
pub struct MarkUndoableState {}
#[derive(Deserialize, Debug, Clone)]
pub struct MarkUndoableStateReturnObject {}
impl super::Command for MarkUndoableState {
    const NAME: &'static str = "DOM.markUndoableState";

    type ReturnObject = MarkUndoableStateReturnObject;
}
// Moves node into the new container, places it before the given anchor.
#[derive(Serialize, Debug)]
pub struct MoveTo {
    // Id of the node to move.
    pub node_id: NodeId,
    // Id of the element to drop the moved node into.
    pub target_node_id: NodeId,
    // Drop node before this one (if absent, the moved node becomes the last child of
    // `targetNodeId`).
    pub insert_before_node_id: Option<NodeId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct MoveToReturnObject {
    // New id of the moved node.
    pub node_id: NodeId,
}
impl super::Command for MoveTo {
    const NAME: &'static str = "DOM.moveTo";

    type ReturnObject = MoveToReturnObject;
}
// Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
// `cancelSearch` to end this search session.
#[derive(Serialize, Debug)]
pub struct PerformSearch {
    // Plain text or query selector or XPath search query.
    pub query: String,
    // True to search in user agent shadow DOM.
    pub include_user_agent_shadow_dom: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PerformSearchReturnObject {
    // Unique search session identifier.
    pub search_id: String,
    // Number of search results.
    pub result_count: i32,
}
impl super::Command for PerformSearch {
    const NAME: &'static str = "DOM.performSearch";

    type ReturnObject = PerformSearchReturnObject;
}
// Requests that the node is sent to the caller given its path. // FIXME, use XPath
#[derive(Serialize, Debug)]
pub struct PushNodeByPathToFrontend {
    // Path to node in the proprietary format.
    pub path: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PushNodeByPathToFrontendReturnObject {
    // Id of the node for given path.
    pub node_id: NodeId,
}
impl super::Command for PushNodeByPathToFrontend {
    const NAME: &'static str = "DOM.pushNodeByPathToFrontend";

    type ReturnObject = PushNodeByPathToFrontendReturnObject;
}
// Requests that a batch of nodes is sent to the caller given their backend node ids.
#[derive(Serialize, Debug)]
pub struct PushNodesByBackendIdsToFrontend {
    // The array of backend node ids.
    pub backend_node_ids: Vec<BackendNodeId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PushNodesByBackendIdsToFrontendReturnObject {
    // The array of ids of pushed nodes that correspond to the backend ids specified in
    // backendNodeIds.
    pub node_ids: Vec<NodeId>,
}
impl super::Command for PushNodesByBackendIdsToFrontend {
    const NAME: &'static str = "DOM.pushNodesByBackendIdsToFrontend";

    type ReturnObject = PushNodesByBackendIdsToFrontendReturnObject;
}
// Executes `querySelector` on a given node.
#[derive(Serialize, Debug)]
pub struct QuerySelector {
    // Id of the node to query upon.
    pub node_id: NodeId,
    // Selector string.
    pub selector: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct QuerySelectorReturnObject {
    // Query selector result.
    pub node_id: NodeId,
}
impl super::Command for QuerySelector {
    const NAME: &'static str = "DOM.querySelector";

    type ReturnObject = QuerySelectorReturnObject;
}
// Executes `querySelectorAll` on a given node.
#[derive(Serialize, Debug)]
pub struct QuerySelectorAll {
    // Id of the node to query upon.
    pub node_id: NodeId,
    // Selector string.
    pub selector: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct QuerySelectorAllReturnObject {
    // Query selector result.
    pub node_ids: Vec<NodeId>,
}
impl super::Command for QuerySelectorAll {
    const NAME: &'static str = "DOM.querySelectorAll";

    type ReturnObject = QuerySelectorAllReturnObject;
}
// Re-does the last undone action.
#[derive(Serialize, Debug)]
pub struct Redo {}
#[derive(Deserialize, Debug, Clone)]
pub struct RedoReturnObject {}
impl super::Command for Redo {
    const NAME: &'static str = "DOM.redo";

    type ReturnObject = RedoReturnObject;
}
// Removes attribute with given name from an element with given id.
#[derive(Serialize, Debug)]
pub struct RemoveAttribute {
    // Id of the element to remove attribute from.
    pub node_id: NodeId,
    // Name of the attribute to remove.
    pub name: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RemoveAttributeReturnObject {}
impl super::Command for RemoveAttribute {
    const NAME: &'static str = "DOM.removeAttribute";

    type ReturnObject = RemoveAttributeReturnObject;
}
// Removes node with given id.
#[derive(Serialize, Debug)]
pub struct RemoveNode {
    // Id of the node to remove.
    pub node_id: NodeId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RemoveNodeReturnObject {}
impl super::Command for RemoveNode {
    const NAME: &'static str = "DOM.removeNode";

    type ReturnObject = RemoveNodeReturnObject;
}
// Requests that children of the node with given id are returned to the caller in form of
// `setChildNodes` events where not only immediate children are retrieved, but all children down to
// the specified depth.
#[derive(Serialize, Debug)]
pub struct RequestChildNodes {
    // Id of the node to get children for.
    pub node_id: NodeId,
    // The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    // (default is false).
    pub pierce: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestChildNodesReturnObject {}
impl super::Command for RequestChildNodes {
    const NAME: &'static str = "DOM.requestChildNodes";

    type ReturnObject = RequestChildNodesReturnObject;
}
// Requests that the node is sent to the caller given the JavaScript node object reference. All
// nodes that form the path from the node to the root are also sent to the client as a series of
// `setChildNodes` notifications.
#[derive(Serialize, Debug)]
pub struct RequestNode {
    // JavaScript object id to convert into node.
    pub object_id: super::runtime::RemoteObjectId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestNodeReturnObject {
    // Node id for given object.
    pub node_id: NodeId,
}
impl super::Command for RequestNode {
    const NAME: &'static str = "DOM.requestNode";

    type ReturnObject = RequestNodeReturnObject;
}
// Resolves the JavaScript node object for a given NodeId or BackendNodeId.
#[derive(Serialize, Debug)]
pub struct ResolveNode {
    // Id of the node to resolve.
    pub node_id: Option<NodeId>,
    // Backend identifier of the node to resolve.
    pub backend_node_id: Option<super::dom::BackendNodeId>,
    // Symbolic group name that can be used to release multiple objects.
    pub object_group: Option<String>,
    // Execution context in which to resolve the node.
    pub execution_context_id: Option<super::runtime::ExecutionContextId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ResolveNodeReturnObject {
    // JavaScript object wrapper for given node.
    pub object: super::runtime::RemoteObject,
}
impl super::Command for ResolveNode {
    const NAME: &'static str = "DOM.resolveNode";

    type ReturnObject = ResolveNodeReturnObject;
}
// Sets attribute for an element with given id.
#[derive(Serialize, Debug)]
pub struct SetAttributeValue {
    // Id of the element to set attribute for.
    pub node_id: NodeId,
    // Attribute name.
    pub name: String,
    // Attribute value.
    pub value: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetAttributeValueReturnObject {}
impl super::Command for SetAttributeValue {
    const NAME: &'static str = "DOM.setAttributeValue";

    type ReturnObject = SetAttributeValueReturnObject;
}
// Sets attributes on element with given id. This method is useful when user edits some existing
// attribute value and types in several attribute name/value pairs.
#[derive(Serialize, Debug)]
pub struct SetAttributesAsText {
    // Id of the element to set attributes for.
    pub node_id: NodeId,
    // Text with a number of attributes. Will parse this text using HTML parser.
    pub text: String,
    // Attribute name to replace with new attributes derived from text in case text parsed
    // successfully.
    pub name: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetAttributesAsTextReturnObject {}
impl super::Command for SetAttributesAsText {
    const NAME: &'static str = "DOM.setAttributesAsText";

    type ReturnObject = SetAttributesAsTextReturnObject;
}
// Sets files for the given file input element.
#[derive(Serialize, Debug)]
pub struct SetFileInputFiles {
    // Array of file paths to set.
    pub files: Vec<String>,
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetFileInputFilesReturnObject {}
impl super::Command for SetFileInputFiles {
    const NAME: &'static str = "DOM.setFileInputFiles";

    type ReturnObject = SetFileInputFilesReturnObject;
}
// Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.
#[derive(Serialize, Debug)]
pub struct SetNodeStackTracesEnabled {
    // Enable or disable.
    pub enable: bool,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetNodeStackTracesEnabledReturnObject {}
impl super::Command for SetNodeStackTracesEnabled {
    const NAME: &'static str = "DOM.setNodeStackTracesEnabled";

    type ReturnObject = SetNodeStackTracesEnabledReturnObject;
}
// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.
#[derive(Serialize, Debug)]
pub struct GetNodeStackTraces {
    // Id of the node to get stack traces for.
    pub node_id: NodeId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetNodeStackTracesReturnObject {
    // Creation stack trace, if available.
    pub creation: Option<super::runtime::StackTrace>,
}
impl super::Command for GetNodeStackTraces {
    const NAME: &'static str = "DOM.getNodeStackTraces";

    type ReturnObject = GetNodeStackTracesReturnObject;
}
// Returns file information for the given
// File wrapper.
#[derive(Serialize, Debug)]
pub struct GetFileInfo {
    // JavaScript object id of the node wrapper.
    pub object_id: super::runtime::RemoteObjectId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetFileInfoReturnObject {
    pub path: String,
}
impl super::Command for GetFileInfo {
    const NAME: &'static str = "DOM.getFileInfo";

    type ReturnObject = GetFileInfoReturnObject;
}
// Enables console to refer to the node with given id via $x (see Command Line API for more details
// $x functions).
#[derive(Serialize, Debug)]
pub struct SetInspectedNode {
    // DOM node id to be accessible by means of $x command line API.
    pub node_id: NodeId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetInspectedNodeReturnObject {}
impl super::Command for SetInspectedNode {
    const NAME: &'static str = "DOM.setInspectedNode";

    type ReturnObject = SetInspectedNodeReturnObject;
}
// Sets node name for a node with given id.
#[derive(Serialize, Debug)]
pub struct SetNodeName {
    // Id of the node to set name for.
    pub node_id: NodeId,
    // New node's name.
    pub name: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetNodeNameReturnObject {
    // New node's id.
    pub node_id: NodeId,
}
impl super::Command for SetNodeName {
    const NAME: &'static str = "DOM.setNodeName";

    type ReturnObject = SetNodeNameReturnObject;
}
// Sets node value for a node with given id.
#[derive(Serialize, Debug)]
pub struct SetNodeValue {
    // Id of the node to set value for.
    pub node_id: NodeId,
    // New node's value.
    pub value: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetNodeValueReturnObject {}
impl super::Command for SetNodeValue {
    const NAME: &'static str = "DOM.setNodeValue";

    type ReturnObject = SetNodeValueReturnObject;
}
// Sets node HTML markup, returns new node id.
#[derive(Serialize, Debug)]
pub struct SetOuterHTML {
    // Id of the node to set markup for.
    pub node_id: NodeId,
    // Outer HTML markup to set.
    pub outer_html: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetOuterHTMLReturnObject {}
impl super::Command for SetOuterHTML {
    const NAME: &'static str = "DOM.setOuterHTML";

    type ReturnObject = SetOuterHTMLReturnObject;
}
// Undoes the last performed action.
#[derive(Serialize, Debug)]
pub struct Undo {}
#[derive(Deserialize, Debug, Clone)]
pub struct UndoReturnObject {}
impl super::Command for Undo {
    const NAME: &'static str = "DOM.undo";

    type ReturnObject = UndoReturnObject;
}
// Returns iframe node that owns iframe with the given domain.
#[derive(Serialize, Debug)]
pub struct GetFrameOwner {
    pub frame_id: super::page::FrameId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GetFrameOwnerReturnObject {
    // Resulting node.
    pub backend_node_id: BackendNodeId,
    // Id of the node at given coordinates, only when enabled and requested document.
    pub node_id: Option<NodeId>,
}
impl super::Command for GetFrameOwner {
    const NAME: &'static str = "DOM.getFrameOwner";

    type ReturnObject = GetFrameOwnerReturnObject;
}
