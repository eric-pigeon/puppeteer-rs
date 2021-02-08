// This file is auto-generated do not edit manually.

// Unique DOM node identifier.
pub type NodeId = i32;
// Unique DOM node identifier used to reference a node that may not have been pushed to the
// front-end.
pub type BackendNodeId = i32;
// Backend node with a friendly name.
pub struct BackendNode {
    // `Node`'s nodeType.
    pub node_type: i32,
    // `Node`'s nodeName.
    pub node_name: String,
    pub backend_node_id: BackendNodeId,
}
// Pseudo element type.
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
pub enum ShadowRootType {
    UserAgent,
    Open,
    Closed,
}
// DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
// DOMNode is a base node mirror type.
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
pub struct ShapeOutsideInfo {
    // Shape bounds
    pub bounds: Quad,
    // Shape coordinate details
    pub shape: Vec<serde_json::Value>,
    // Margin shape bounds
    pub margin_shape: Vec<serde_json::Value>,
}
// Rectangle.
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
pub struct CSSComputedStyleProperty {
    // Computed style property name.
    pub name: String,
    // Computed style property value.
    pub value: String,
}

// Collects class names for the node with given id and all of it's child nodes.
pub struct CollectClassNamesFromSubtree {
    // Id of the node to collect class names.
    pub node_id: NodeId,
}
pub struct CollectClassNamesFromSubtreeReturnObject {
    // Class name list.
    pub class_names: Vec<String>,
}
// Creates a deep copy of the specified node and places it into the target container before the
// given anchor.
pub struct CopyTo {
    // Id of the node to copy.
    pub node_id: NodeId,
    // Id of the element to drop the copy into.
    pub target_node_id: NodeId,
    // Drop the copy before this node (if absent, the copy becomes the last child of
    // `targetNodeId`).
    pub insert_before_node_id: Option<NodeId>,
}
pub struct CopyToReturnObject {
    // Id of the node clone.
    pub node_id: NodeId,
}
// Describes node given its id, does not require domain to be enabled. Does not start tracking any
// objects, can be used for automation.
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
pub struct DescribeNodeReturnObject {
    // Node description.
    pub node: Node,
}
// Scrolls the specified rect of the given node into view if not already visible.
// Note: exactly one between nodeId, backendNodeId and objectId should be passed
// to identify the node.
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
pub struct ScrollIntoViewIfNeededReturnObject {}
// Disables DOM agent for the given page.
pub struct Disable {}
pub struct DisableReturnObject {}
// Discards search results from the session with the given id. `getSearchResults` should no longer
// be called for that search.
pub struct DiscardSearchResults {
    // Unique search session identifier.
    pub search_id: String,
}
pub struct DiscardSearchResultsReturnObject {}
// Enables DOM agent for the given page.
pub struct Enable {}
pub struct EnableReturnObject {}
// Focuses the given element.
pub struct Focus {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
pub struct FocusReturnObject {}
// Returns attributes for the specified node.
pub struct GetAttributes {
    // Id of the node to retrieve attibutes for.
    pub node_id: NodeId,
}
pub struct GetAttributesReturnObject {
    // An interleaved array of node attribute names and values.
    pub attributes: Vec<String>,
}
// Returns boxes for the given node.
pub struct GetBoxModel {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
pub struct GetBoxModelReturnObject {
    // Box model for the node.
    pub model: BoxModel,
}
// Returns quads that describe node position on the page. This method
// might return multiple quads for inline nodes.
pub struct GetContentQuads {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
pub struct GetContentQuadsReturnObject {
    // Quads that describe node layout relative to viewport.
    pub quads: Vec<Quad>,
}
// Returns the root DOM node (and optionally the subtree) to the caller.
pub struct GetDocument {
    // The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the subtree
    // (default is false).
    pub pierce: Option<bool>,
}
pub struct GetDocumentReturnObject {
    // Resulting node.
    pub root: Node,
}
// Returns the root DOM node (and optionally the subtree) to the caller.
// Deprecated, as it is not designed to work well with the rest of the DOM agent.
// Use DOMSnapshot.captureSnapshot instead.
pub struct GetFlattenedDocument {
    // The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
    // entire subtree or provide an integer larger than 0.
    pub depth: Option<i32>,
    // Whether or not iframes and shadow roots should be traversed when returning the subtree
    // (default is false).
    pub pierce: Option<bool>,
}
pub struct GetFlattenedDocumentReturnObject {
    // Resulting node.
    pub nodes: Vec<Node>,
}
// Finds nodes with a given computed style in a subtree.
pub struct GetNodesForSubtreeByStyle {
    // Node ID pointing to the root of a subtree.
    pub node_id: NodeId,
    // The style to filter nodes by (includes nodes if any of properties matches).
    pub computed_styles: Vec<CSSComputedStyleProperty>,
    // Whether or not iframes and shadow roots in the same target should be traversed when returning the
    // results (default is false).
    pub pierce: Option<bool>,
}
pub struct GetNodesForSubtreeByStyleReturnObject {
    // Resulting nodes.
    pub node_ids: Vec<NodeId>,
}
// Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
// either returned or not.
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
pub struct GetNodeForLocationReturnObject {
    // Resulting node.
    pub backend_node_id: BackendNodeId,
    // Frame this node belongs to.
    pub frame_id: super::page::FrameId,
    // Id of the node at given coordinates, only when enabled and requested document.
    pub node_id: Option<NodeId>,
}
// Returns node's HTML markup.
pub struct GetOuterHTML {
    // Identifier of the node.
    pub node_id: Option<NodeId>,
    // Identifier of the backend node.
    pub backend_node_id: Option<BackendNodeId>,
    // JavaScript object id of the node wrapper.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
pub struct GetOuterHTMLReturnObject {
    // Outer HTML markup.
    pub outer_html: String,
}
// Returns the id of the nearest ancestor that is a relayout boundary.
pub struct GetRelayoutBoundary {
    // Id of the node.
    pub node_id: NodeId,
}
pub struct GetRelayoutBoundaryReturnObject {
    // Relayout boundary node id for the given node.
    pub node_id: NodeId,
}
// Returns search results from given `fromIndex` to given `toIndex` from the search with the given
// identifier.
pub struct GetSearchResults {
    // Unique search session identifier.
    pub search_id: String,
    // Start index of the search result to be returned.
    pub from_index: i32,
    // End index of the search result to be returned.
    pub to_index: i32,
}
pub struct GetSearchResultsReturnObject {
    // Ids of the search result nodes.
    pub node_ids: Vec<NodeId>,
}
// Hides any highlight.
pub struct HideHighlight {}
pub struct HideHighlightReturnObject {}
// Highlights DOM node.
pub struct HighlightNode {}
pub struct HighlightNodeReturnObject {}
// Highlights given rectangle.
pub struct HighlightRect {}
pub struct HighlightRectReturnObject {}
// Marks last undoable state.
pub struct MarkUndoableState {}
pub struct MarkUndoableStateReturnObject {}
// Moves node into the new container, places it before the given anchor.
pub struct MoveTo {
    // Id of the node to move.
    pub node_id: NodeId,
    // Id of the element to drop the moved node into.
    pub target_node_id: NodeId,
    // Drop node before this one (if absent, the moved node becomes the last child of
    // `targetNodeId`).
    pub insert_before_node_id: Option<NodeId>,
}
pub struct MoveToReturnObject {
    // New id of the moved node.
    pub node_id: NodeId,
}
// Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
// `cancelSearch` to end this search session.
pub struct PerformSearch {
    // Plain text or query selector or XPath search query.
    pub query: String,
    // True to search in user agent shadow DOM.
    pub include_user_agent_shadow_dom: Option<bool>,
}
pub struct PerformSearchReturnObject {
    // Unique search session identifier.
    pub search_id: String,
    // Number of search results.
    pub result_count: i32,
}
// Requests that the node is sent to the caller given its path. // FIXME, use XPath
pub struct PushNodeByPathToFrontend {
    // Path to node in the proprietary format.
    pub path: String,
}
pub struct PushNodeByPathToFrontendReturnObject {
    // Id of the node for given path.
    pub node_id: NodeId,
}
// Requests that a batch of nodes is sent to the caller given their backend node ids.
pub struct PushNodesByBackendIdsToFrontend {
    // The array of backend node ids.
    pub backend_node_ids: Vec<BackendNodeId>,
}
pub struct PushNodesByBackendIdsToFrontendReturnObject {
    // The array of ids of pushed nodes that correspond to the backend ids specified in
    // backendNodeIds.
    pub node_ids: Vec<NodeId>,
}
// Executes `querySelector` on a given node.
pub struct QuerySelector {
    // Id of the node to query upon.
    pub node_id: NodeId,
    // Selector string.
    pub selector: String,
}
pub struct QuerySelectorReturnObject {
    // Query selector result.
    pub node_id: NodeId,
}
// Executes `querySelectorAll` on a given node.
pub struct QuerySelectorAll {
    // Id of the node to query upon.
    pub node_id: NodeId,
    // Selector string.
    pub selector: String,
}
pub struct QuerySelectorAllReturnObject {
    // Query selector result.
    pub node_ids: Vec<NodeId>,
}
// Re-does the last undone action.
pub struct Redo {}
pub struct RedoReturnObject {}
// Removes attribute with given name from an element with given id.
pub struct RemoveAttribute {
    // Id of the element to remove attribute from.
    pub node_id: NodeId,
    // Name of the attribute to remove.
    pub name: String,
}
pub struct RemoveAttributeReturnObject {}
// Removes node with given id.
pub struct RemoveNode {
    // Id of the node to remove.
    pub node_id: NodeId,
}
pub struct RemoveNodeReturnObject {}
// Requests that children of the node with given id are returned to the caller in form of
// `setChildNodes` events where not only immediate children are retrieved, but all children down to
// the specified depth.
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
pub struct RequestChildNodesReturnObject {}
// Requests that the node is sent to the caller given the JavaScript node object reference. All
// nodes that form the path from the node to the root are also sent to the client as a series of
// `setChildNodes` notifications.
pub struct RequestNode {
    // JavaScript object id to convert into node.
    pub object_id: super::runtime::RemoteObjectId,
}
pub struct RequestNodeReturnObject {
    // Node id for given object.
    pub node_id: NodeId,
}
// Resolves the JavaScript node object for a given NodeId or BackendNodeId.
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
pub struct ResolveNodeReturnObject {
    // JavaScript object wrapper for given node.
    pub object: super::runtime::RemoteObject,
}
// Sets attribute for an element with given id.
pub struct SetAttributeValue {
    // Id of the element to set attribute for.
    pub node_id: NodeId,
    // Attribute name.
    pub name: String,
    // Attribute value.
    pub value: String,
}
pub struct SetAttributeValueReturnObject {}
// Sets attributes on element with given id. This method is useful when user edits some existing
// attribute value and types in several attribute name/value pairs.
pub struct SetAttributesAsText {
    // Id of the element to set attributes for.
    pub node_id: NodeId,
    // Text with a number of attributes. Will parse this text using HTML parser.
    pub text: String,
    // Attribute name to replace with new attributes derived from text in case text parsed
    // successfully.
    pub name: Option<String>,
}
pub struct SetAttributesAsTextReturnObject {}
// Sets files for the given file input element.
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
pub struct SetFileInputFilesReturnObject {}
// Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.
pub struct SetNodeStackTracesEnabled {
    // Enable or disable.
    pub enable: bool,
}
pub struct SetNodeStackTracesEnabledReturnObject {}
// Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.
pub struct GetNodeStackTraces {
    // Id of the node to get stack traces for.
    pub node_id: NodeId,
}
pub struct GetNodeStackTracesReturnObject {
    // Creation stack trace, if available.
    pub creation: Option<super::runtime::StackTrace>,
}
// Returns file information for the given
// File wrapper.
pub struct GetFileInfo {
    // JavaScript object id of the node wrapper.
    pub object_id: super::runtime::RemoteObjectId,
}
pub struct GetFileInfoReturnObject {
    pub path: String,
}
// Enables console to refer to the node with given id via $x (see Command Line API for more details
// $x functions).
pub struct SetInspectedNode {
    // DOM node id to be accessible by means of $x command line API.
    pub node_id: NodeId,
}
pub struct SetInspectedNodeReturnObject {}
// Sets node name for a node with given id.
pub struct SetNodeName {
    // Id of the node to set name for.
    pub node_id: NodeId,
    // New node's name.
    pub name: String,
}
pub struct SetNodeNameReturnObject {
    // New node's id.
    pub node_id: NodeId,
}
// Sets node value for a node with given id.
pub struct SetNodeValue {
    // Id of the node to set value for.
    pub node_id: NodeId,
    // New node's value.
    pub value: String,
}
pub struct SetNodeValueReturnObject {}
// Sets node HTML markup, returns new node id.
pub struct SetOuterHTML {
    // Id of the node to set markup for.
    pub node_id: NodeId,
    // Outer HTML markup to set.
    pub outer_html: String,
}
pub struct SetOuterHTMLReturnObject {}
// Undoes the last performed action.
pub struct Undo {}
pub struct UndoReturnObject {}
// Returns iframe node that owns iframe with the given domain.
pub struct GetFrameOwner {
    pub frame_id: super::page::FrameId,
}
pub struct GetFrameOwnerReturnObject {
    // Resulting node.
    pub backend_node_id: BackendNodeId,
    // Id of the node at given coordinates, only when enabled and requested document.
    pub node_id: Option<NodeId>,
}
