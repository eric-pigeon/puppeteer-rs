// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Configuration data for drawing the source order of an elements children.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceOrderConfig {
    // the color to outline the givent element in.
    pub parent_outline_color: super::dom::RGBA,
    // the color to outline the child elements in.
    pub child_outline_color: super::dom::RGBA,
}
// Configuration data for the highlighting of Grid elements.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GridHighlightConfig {
    // Whether the extension lines from grid cells to the rulers should be shown (default: false).
    pub show_grid_extension_lines: Option<bool>,
    // Show Positive line number labels (default: false).
    pub show_positive_line_numbers: Option<bool>,
    // Show Negative line number labels (default: false).
    pub show_negative_line_numbers: Option<bool>,
    // Show area name labels (default: false).
    pub show_area_names: Option<bool>,
    // Show line name labels (default: false).
    pub show_line_names: Option<bool>,
    // Show track size labels (default: false).
    pub show_track_sizes: Option<bool>,
    // The grid container border highlight color (default: transparent).
    pub grid_border_color: Option<super::dom::RGBA>,
    // The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.
    pub cell_border_color: Option<super::dom::RGBA>,
    // The row line color (default: transparent).
    pub row_line_color: Option<super::dom::RGBA>,
    // The column line color (default: transparent).
    pub column_line_color: Option<super::dom::RGBA>,
    // Whether the grid border is dashed (default: false).
    pub grid_border_dash: Option<bool>,
    // Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.
    pub cell_border_dash: Option<bool>,
    // Whether row lines are dashed (default: false).
    pub row_line_dash: Option<bool>,
    // Whether column lines are dashed (default: false).
    pub column_line_dash: Option<bool>,
    // The row gap highlight fill color (default: transparent).
    pub row_gap_color: Option<super::dom::RGBA>,
    // The row gap hatching fill color (default: transparent).
    pub row_hatch_color: Option<super::dom::RGBA>,
    // The column gap highlight fill color (default: transparent).
    pub column_gap_color: Option<super::dom::RGBA>,
    // The column gap hatching fill color (default: transparent).
    pub column_hatch_color: Option<super::dom::RGBA>,
    // The named grid areas border color (Default: transparent).
    pub area_border_color: Option<super::dom::RGBA>,
    // The grid container background color (Default: transparent).
    pub grid_background_color: Option<super::dom::RGBA>,
}
// Configuration data for the highlighting of page elements.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightConfig {
    // Whether the node info tooltip should be shown (default: false).
    pub show_info: Option<bool>,
    // Whether the node styles in the tooltip (default: false).
    pub show_styles: Option<bool>,
    // Whether the rulers should be shown (default: false).
    pub show_rulers: Option<bool>,
    // Whether the a11y info should be shown (default: true).
    pub show_accessibility_info: Option<bool>,
    // Whether the extension lines from node to the rulers should be shown (default: false).
    pub show_extension_lines: Option<bool>,
    // The content box highlight fill color (default: transparent).
    pub content_color: Option<super::dom::RGBA>,
    // The padding highlight fill color (default: transparent).
    pub padding_color: Option<super::dom::RGBA>,
    // The border highlight fill color (default: transparent).
    pub border_color: Option<super::dom::RGBA>,
    // The margin highlight fill color (default: transparent).
    pub margin_color: Option<super::dom::RGBA>,
    // The event target element highlight fill color (default: transparent).
    pub event_target_color: Option<super::dom::RGBA>,
    // The shape outside fill color (default: transparent).
    pub shape_color: Option<super::dom::RGBA>,
    // The shape margin fill color (default: transparent).
    pub shape_margin_color: Option<super::dom::RGBA>,
    // The grid layout color (default: transparent).
    pub css_grid_color: Option<super::dom::RGBA>,
    // The color format used to format color styles (default: hex).
    pub color_format: Option<ColorFormat>,
    // The grid layout highlight configuration (default: all transparent).
    pub grid_highlight_config: Option<GridHighlightConfig>,
}
pub type ColorFormat = String;
// Configurations for Persistent Grid Highlight
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GridNodeHighlightConfig {
    // A descriptor for the highlight appearance.
    pub grid_highlight_config: GridHighlightConfig,
    // Identifier of the node to highlight.
    pub node_id: super::dom::NodeId,
}
// Configuration for dual screen hinge
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HingeConfig {
    // A rectangle represent hinge
    pub rect: super::dom::Rect,
    // The content box highlight fill color (default: a dark color).
    pub content_color: Option<super::dom::RGBA>,
    // The content box highlight outline color (default: transparent).
    pub outline_color: Option<super::dom::RGBA>,
}
pub type InspectMode = String;

// Disables domain notifications.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Overlay.disable";

    type ReturnObject = DisableReturnObject;
}
// Enables domain notifications.
#[derive(Serialize, Debug)]
pub struct Enable {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Overlay.enable";

    type ReturnObject = EnableReturnObject;
}
// For testing.
#[derive(Serialize, Debug)]
pub struct GetHighlightObjectForTest {
    // Id of the node to get highlight object for.
    pub node_id: super::dom::NodeId,
    // Whether to include distance info.
    pub include_distance: Option<bool>,
    // Whether to include style info.
    pub include_style: Option<bool>,
    // The color format to get config with (default: hex).
    pub color_format: Option<ColorFormat>,
    // Whether to show accessibility info (default: true).
    pub show_accessibility_info: Option<bool>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetHighlightObjectForTestReturnObject {
    // Highlight data for the node.
// TODO objectProperty
}
impl super::Command for GetHighlightObjectForTest {
    const NAME: &'static str = "Overlay.getHighlightObjectForTest";

    type ReturnObject = GetHighlightObjectForTestReturnObject;
}
// For Persistent Grid testing.
#[derive(Serialize, Debug)]
pub struct GetGridHighlightObjectsForTest {
    // Ids of the node to get highlight object for.
    pub node_ids: Vec<super::dom::NodeId>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetGridHighlightObjectsForTestReturnObject {
    // Grid Highlight data for the node ids provided.
// TODO objectProperty
}
impl super::Command for GetGridHighlightObjectsForTest {
    const NAME: &'static str = "Overlay.getGridHighlightObjectsForTest";

    type ReturnObject = GetGridHighlightObjectsForTestReturnObject;
}
// For Source Order Viewer testing.
#[derive(Serialize, Debug)]
pub struct GetSourceOrderHighlightObjectForTest {
    // Id of the node to highlight.
    pub node_id: super::dom::NodeId,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceOrderHighlightObjectForTestReturnObject {
    // Source order highlight data for the node id provided.
// TODO objectProperty
}
impl super::Command for GetSourceOrderHighlightObjectForTest {
    const NAME: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";

    type ReturnObject = GetSourceOrderHighlightObjectForTestReturnObject;
}
// Hides any highlight.
#[derive(Serialize, Debug)]
pub struct HideHighlight {}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HideHighlightReturnObject {}
impl super::Command for HideHighlight {
    const NAME: &'static str = "Overlay.hideHighlight";

    type ReturnObject = HideHighlightReturnObject;
}
// Highlights owner element of the frame with given id.
#[derive(Serialize, Debug)]
pub struct HighlightFrame {
    // Identifier of the frame to highlight.
    pub frame_id: super::page::FrameId,
    // The content box highlight fill color (default: transparent).
    pub content_color: Option<super::dom::RGBA>,
    // The content box highlight outline color (default: transparent).
    pub content_outline_color: Option<super::dom::RGBA>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HighlightFrameReturnObject {}
impl super::Command for HighlightFrame {
    const NAME: &'static str = "Overlay.highlightFrame";

    type ReturnObject = HighlightFrameReturnObject;
}
// Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
// objectId must be specified.
#[derive(Serialize, Debug)]
pub struct HighlightNode {
    // A descriptor for the highlight appearance.
    pub highlight_config: HighlightConfig,
    // Identifier of the node to highlight.
    pub node_id: Option<super::dom::NodeId>,
    // Identifier of the backend node to highlight.
    pub backend_node_id: Option<super::dom::BackendNodeId>,
    // JavaScript object id of the node to be highlighted.
    pub object_id: Option<super::runtime::RemoteObjectId>,
    // Selectors to highlight relevant nodes.
    pub selector: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HighlightNodeReturnObject {}
impl super::Command for HighlightNode {
    const NAME: &'static str = "Overlay.highlightNode";

    type ReturnObject = HighlightNodeReturnObject;
}
// Highlights given quad. Coordinates are absolute with respect to the main frame viewport.
#[derive(Serialize, Debug)]
pub struct HighlightQuad {
    // Quad to highlight
    pub quad: super::dom::Quad,
    // The highlight fill color (default: transparent).
    pub color: Option<super::dom::RGBA>,
    // The highlight outline color (default: transparent).
    pub outline_color: Option<super::dom::RGBA>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HighlightQuadReturnObject {}
impl super::Command for HighlightQuad {
    const NAME: &'static str = "Overlay.highlightQuad";

    type ReturnObject = HighlightQuadReturnObject;
}
// Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.
#[derive(Serialize, Debug)]
pub struct HighlightRect {
    // X coordinate
    pub x: i32,
    // Y coordinate
    pub y: i32,
    // Rectangle width
    pub width: i32,
    // Rectangle height
    pub height: i32,
    // The highlight fill color (default: transparent).
    pub color: Option<super::dom::RGBA>,
    // The highlight outline color (default: transparent).
    pub outline_color: Option<super::dom::RGBA>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HighlightRectReturnObject {}
impl super::Command for HighlightRect {
    const NAME: &'static str = "Overlay.highlightRect";

    type ReturnObject = HighlightRectReturnObject;
}
// Highlights the source order of the children of the DOM node with given id or with the given
// JavaScript object wrapper. Either nodeId or objectId must be specified.
#[derive(Serialize, Debug)]
pub struct HighlightSourceOrder {
    // A descriptor for the appearance of the overlay drawing.
    pub source_order_config: SourceOrderConfig,
    // Identifier of the node to highlight.
    pub node_id: Option<super::dom::NodeId>,
    // Identifier of the backend node to highlight.
    pub backend_node_id: Option<super::dom::BackendNodeId>,
    // JavaScript object id of the node to be highlighted.
    pub object_id: Option<super::runtime::RemoteObjectId>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HighlightSourceOrderReturnObject {}
impl super::Command for HighlightSourceOrder {
    const NAME: &'static str = "Overlay.highlightSourceOrder";

    type ReturnObject = HighlightSourceOrderReturnObject;
}
// Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
// Backend then generates 'inspectNodeRequested' event upon element selection.
#[derive(Serialize, Debug)]
pub struct SetInspectMode {
    // Set an inspection mode.
    pub mode: InspectMode,
    // A descriptor for the highlight appearance of hovered-over nodes. May be omitted if `enabled
    // == false`.
    pub highlight_config: Option<HighlightConfig>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetInspectModeReturnObject {}
impl super::Command for SetInspectMode {
    const NAME: &'static str = "Overlay.setInspectMode";

    type ReturnObject = SetInspectModeReturnObject;
}
// Highlights owner element of all frames detected to be ads.
#[derive(Serialize, Debug)]
pub struct SetShowAdHighlights {
    // True for showing ad highlights
    pub show: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowAdHighlightsReturnObject {}
impl super::Command for SetShowAdHighlights {
    const NAME: &'static str = "Overlay.setShowAdHighlights";

    type ReturnObject = SetShowAdHighlightsReturnObject;
}
#[derive(Serialize, Debug)]
pub struct SetPausedInDebuggerMessage {
    // The message to display, also triggers resume and step over controls.
    pub message: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedInDebuggerMessageReturnObject {}
impl super::Command for SetPausedInDebuggerMessage {
    const NAME: &'static str = "Overlay.setPausedInDebuggerMessage";

    type ReturnObject = SetPausedInDebuggerMessageReturnObject;
}
// Requests that backend shows debug borders on layers
#[derive(Serialize, Debug)]
pub struct SetShowDebugBorders {
    // True for showing debug borders
    pub show: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowDebugBordersReturnObject {}
impl super::Command for SetShowDebugBorders {
    const NAME: &'static str = "Overlay.setShowDebugBorders";

    type ReturnObject = SetShowDebugBordersReturnObject;
}
// Requests that backend shows the FPS counter
#[derive(Serialize, Debug)]
pub struct SetShowFPSCounter {
    // True for showing the FPS counter
    pub show: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowFPSCounterReturnObject {}
impl super::Command for SetShowFPSCounter {
    const NAME: &'static str = "Overlay.setShowFPSCounter";

    type ReturnObject = SetShowFPSCounterReturnObject;
}
// Highlight multiple elements with the CSS Grid overlay.
#[derive(Serialize, Debug)]
pub struct SetShowGridOverlays {
    // An array of node identifiers and descriptors for the highlight appearance.
    pub grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowGridOverlaysReturnObject {}
impl super::Command for SetShowGridOverlays {
    const NAME: &'static str = "Overlay.setShowGridOverlays";

    type ReturnObject = SetShowGridOverlaysReturnObject;
}
// Requests that backend shows paint rectangles
#[derive(Serialize, Debug)]
pub struct SetShowPaintRects {
    // True for showing paint rectangles
    pub result: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowPaintRectsReturnObject {}
impl super::Command for SetShowPaintRects {
    const NAME: &'static str = "Overlay.setShowPaintRects";

    type ReturnObject = SetShowPaintRectsReturnObject;
}
// Requests that backend shows layout shift regions
#[derive(Serialize, Debug)]
pub struct SetShowLayoutShiftRegions {
    // True for showing layout shift regions
    pub result: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowLayoutShiftRegionsReturnObject {}
impl super::Command for SetShowLayoutShiftRegions {
    const NAME: &'static str = "Overlay.setShowLayoutShiftRegions";

    type ReturnObject = SetShowLayoutShiftRegionsReturnObject;
}
// Requests that backend shows scroll bottleneck rects
#[derive(Serialize, Debug)]
pub struct SetShowScrollBottleneckRects {
    // True for showing scroll bottleneck rects
    pub show: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowScrollBottleneckRectsReturnObject {}
impl super::Command for SetShowScrollBottleneckRects {
    const NAME: &'static str = "Overlay.setShowScrollBottleneckRects";

    type ReturnObject = SetShowScrollBottleneckRectsReturnObject;
}
// Requests that backend shows hit-test borders on layers
#[derive(Serialize, Debug)]
pub struct SetShowHitTestBorders {
    // True for showing hit-test borders
    pub show: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHitTestBordersReturnObject {}
impl super::Command for SetShowHitTestBorders {
    const NAME: &'static str = "Overlay.setShowHitTestBorders";

    type ReturnObject = SetShowHitTestBordersReturnObject;
}
// Paints viewport size upon main frame resize.
#[derive(Serialize, Debug)]
pub struct SetShowViewportSizeOnResize {
    // Whether to paint size or not.
    pub show: bool,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowViewportSizeOnResizeReturnObject {}
impl super::Command for SetShowViewportSizeOnResize {
    const NAME: &'static str = "Overlay.setShowViewportSizeOnResize";

    type ReturnObject = SetShowViewportSizeOnResizeReturnObject;
}
// Add a dual screen device hinge
#[derive(Serialize, Debug)]
pub struct SetShowHinge {
    // hinge data, null means hideHinge
    pub hinge_config: Option<HingeConfig>,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetShowHingeReturnObject {}
impl super::Command for SetShowHinge {
    const NAME: &'static str = "Overlay.setShowHinge";

    type ReturnObject = SetShowHingeReturnObject;
}

// Fired when the node should be inspected. This happens after call to `setInspectMode` or when
// user manually inspects an element.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InspectNodeRequestedEvent {
    pub params: InspectNodeRequestedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InspectNodeRequestedParams {
    // Id of the node to inspect.
    pub backend_node_id: super::dom::BackendNodeId,
}
// Fired when the node should be highlighted. This happens after call to `setInspectMode`.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct NodeHighlightRequestedEvent {
    pub params: NodeHighlightRequestedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeHighlightRequestedParams {
    pub node_id: super::dom::NodeId,
}
// Fired when user asks to capture screenshot of some area on the page.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScreenshotRequestedEvent {
    pub params: ScreenshotRequestedParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotRequestedParams {
    // Viewport to capture, in device independent pixels (dip).
    pub viewport: super::page::Viewport,
}
// Fired when user cancels the inspect mode.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InspectModeCanceledEvent {
    pub params: InspectModeCanceledParams,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InspectModeCanceledParams {}
