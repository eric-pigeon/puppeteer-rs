// This file is auto-generated do not edit manually.

pub type StyleSheetId = String;
// Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
// stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
// inspector" rules), "regular" for regular stylesheets.
pub enum StyleSheetOrigin {
    Injected,
    UserAgent,
    Inspector,
    Regular,
}
// CSS rule collection for a single pseudo style.
pub struct PseudoElementMatches {
    // Pseudo element type.
    pub pseudo_type: super::dom::PseudoType,
    // Matches of CSS rules applicable to the pseudo style.
    pub matches: Vec<RuleMatch>,
}
// Inherited CSS rule collection from ancestor node.
pub struct InheritedStyleEntry {
    // The ancestor node's inline style, if any, in the style inheritance chain.
    pub inline_style: Option<CSSStyle>,
    // Matches of CSS rules matching the ancestor node in the style inheritance chain.
    pub matched_css_rules: Vec<RuleMatch>,
}
// Match data for a CSS rule.
pub struct RuleMatch {
    // CSS rule in the match.
    pub rule: CSSRule,
    // Matching selector indices in the rule's selectorList selectors (0-based).
    pub matching_selectors: Vec<i32>,
}
// Data for a simple selector (these are delimited by commas in a selector list).
pub struct Value {
    // Value text.
    pub text: String,
    // Value range in the underlying resource (if available).
    pub range: Option<SourceRange>,
}
// Selector list data.
pub struct SelectorList {
    // Selectors in the list.
    pub selectors: Vec<Value>,
    // Rule selector text.
    pub text: String,
}
// CSS stylesheet metainformation.
pub struct CSSStyleSheetHeader {
    // The stylesheet identifier.
    pub style_sheet_id: StyleSheetId,
    // Owner frame identifier.
    pub frame_id: super::page::FrameId,
    // Stylesheet resource URL.
    pub source_url: String,
    // URL of source map associated with the stylesheet (if any).
    pub source_map_url: Option<String>,
    // Stylesheet origin.
    pub origin: StyleSheetOrigin,
    // Stylesheet title.
    pub title: String,
    // The backend id for the owner node of the stylesheet.
    pub owner_node: Option<super::dom::BackendNodeId>,
    // Denotes whether the stylesheet is disabled.
    pub disabled: bool,
    // Whether the sourceURL field value comes from the sourceURL comment.
    pub has_source_url: Option<bool>,
    // Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    // document.written STYLE tags.
    pub is_inline: bool,
    // Whether this stylesheet is mutable. Inline stylesheets become mutable
    // after they have been modified via CSSOM API.
    // <link> element's stylesheets become mutable only if DevTools modifies them.
    // Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.
    pub is_mutable: bool,
    // Whether this stylesheet is a constructed stylesheet (created using new CSSStyleSheet()).
    pub is_constructed: bool,
    // Line offset of the stylesheet within the resource (zero based).
    pub start_line: f64,
    // Column offset of the stylesheet within the resource (zero based).
    pub start_column: f64,
    // Size of the content (in characters).
    pub length: f64,
    // Line offset of the end of the stylesheet within the resource (zero based).
    pub end_line: f64,
    // Column offset of the end of the stylesheet within the resource (zero based).
    pub end_column: f64,
}
// CSS rule representation.
pub struct CSSRule {
    // The css style sheet identifier (absent for user agent stylesheet and user-specified
    // stylesheet rules) this rule came from.
    pub style_sheet_id: Option<StyleSheetId>,
    // Rule selector data.
    pub selector_list: SelectorList,
    // Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    // Associated style declaration.
    pub style: CSSStyle,
    // Media list array (for rules involving media queries). The array enumerates media queries
    // starting with the innermost one, going outwards.
    pub media: Option<Vec<CSSMedia>>,
}
// CSS coverage information.
pub struct RuleUsage {
    // The css style sheet identifier (absent for user agent stylesheet and user-specified
    // stylesheet rules) this rule came from.
    pub style_sheet_id: StyleSheetId,
    // Offset of the start of the rule (including selector) from the beginning of the stylesheet.
    pub start_offset: f64,
    // Offset of the end of the rule body from the beginning of the stylesheet.
    pub end_offset: f64,
    // Indicates whether the rule was actually used by some element in the page.
    pub used: bool,
}
// Text range within a resource. All numbers are zero-based.
pub struct SourceRange {
    // Start line of range.
    pub start_line: i32,
    // Start column of range (inclusive).
    pub start_column: i32,
    // End line of range
    pub end_line: i32,
    // End column of range (exclusive).
    pub end_column: i32,
}
pub struct ShorthandEntry {
    // Shorthand name.
    pub name: String,
    // Shorthand value.
    pub value: String,
    // Whether the property has "!important" annotation (implies `false` if absent).
    pub important: Option<bool>,
}
pub struct CSSComputedStyleProperty {
    // Computed style property name.
    pub name: String,
    // Computed style property value.
    pub value: String,
}
// CSS style representation.
pub struct CSSStyle {
    // The css style sheet identifier (absent for user agent stylesheet and user-specified
    // stylesheet rules) this rule came from.
    pub style_sheet_id: Option<StyleSheetId>,
    // CSS properties in the style.
    pub css_properties: Vec<CSSProperty>,
    // Computed values for all shorthands found in the style.
    pub shorthand_entries: Vec<ShorthandEntry>,
    // Style declaration text (if available).
    pub css_text: Option<String>,
    // Style declaration range in the enclosing stylesheet (if available).
    pub range: Option<SourceRange>,
}
// CSS property declaration data.
pub struct CSSProperty {
    // The property name.
    pub name: String,
    // The property value.
    pub value: String,
    // Whether the property has "!important" annotation (implies `false` if absent).
    pub important: Option<bool>,
    // Whether the property is implicit (implies `false` if absent).
    pub implicit: Option<bool>,
    // The full property text as specified in the style.
    pub text: Option<String>,
    // Whether the property is understood by the browser (implies `true` if absent).
    pub parsed_ok: Option<bool>,
    // Whether the property is disabled by the user (present for source-based properties only).
    pub disabled: Option<bool>,
    // The entire property range in the enclosing style declaration (if available).
    pub range: Option<SourceRange>,
}
pub enum CSSMediaSource {
    MediaRule,
    ImportRule,
    LinkedSheet,
    InlineSheet,
}
// CSS media rule descriptor.
pub struct CSSMedia {
    // Media query text.
    pub text: String,
    // Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    // specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    // stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    // stylesheet's STYLE tag.
    pub source: CSSMediaSource,
    // URL of the document containing the media query description.
    pub source_url: Option<String>,
    // The associated rule (@media or @import) header range in the enclosing stylesheet (if
    // available).
    pub range: Option<SourceRange>,
    // Identifier of the stylesheet containing this object (if exists).
    pub style_sheet_id: Option<StyleSheetId>,
    // Array of media queries.
    pub media_list: Option<Vec<MediaQuery>>,
}
// Media query descriptor.
pub struct MediaQuery {
    // Array of media query expressions.
    pub expressions: Vec<MediaQueryExpression>,
    // Whether the media query condition is satisfied.
    pub active: bool,
}
// Media query expression descriptor.
pub struct MediaQueryExpression {
    // Media query expression value.
    pub value: f64,
    // Media query expression units.
    pub unit: String,
    // Media query expression feature.
    pub feature: String,
    // The associated range of the value text in the enclosing stylesheet (if available).
    pub value_range: Option<SourceRange>,
    // Computed length of media query expression (if applicable).
    pub computed_length: Option<f64>,
}
// Information about amount of glyphs that were rendered with given font.
pub struct PlatformFontUsage {
    // Font's family name reported by platform.
    pub family_name: String,
    // Indicates if the font was downloaded or resolved locally.
    pub is_custom_font: bool,
    // Amount of glyphs that were rendered with this font.
    pub glyph_count: f64,
}
// Information about font variation axes for variable fonts
pub struct FontVariationAxis {
    // The font-variation-setting tag (a.k.a. "axis tag").
    pub tag: String,
    // Human-readable variation name in the default language (normally, "en").
    pub name: String,
    // The minimum value (inclusive) the font supports for this tag.
    pub min_value: f64,
    // The maximum value (inclusive) the font supports for this tag.
    pub max_value: f64,
    // The default value.
    pub default_value: f64,
}
// Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
// and additional information such as platformFontFamily and fontVariationAxes.
pub struct FontFace {
    // The font-family.
    pub font_family: String,
    // The font-style.
    pub font_style: String,
    // The font-variant.
    pub font_variant: String,
    // The font-weight.
    pub font_weight: String,
    // The font-stretch.
    pub font_stretch: String,
    // The unicode-range.
    pub unicode_range: String,
    // The src.
    pub src: String,
    // The resolved platform font family
    pub platform_font_family: String,
    // Available variation settings (a.k.a. "axes").
    pub font_variation_axes: Option<Vec<FontVariationAxis>>,
}
// CSS keyframes rule representation.
pub struct CSSKeyframesRule {
    // Animation name.
    pub animation_name: Value,
    // List of keyframes.
    pub keyframes: Vec<CSSKeyframeRule>,
}
// CSS keyframe rule representation.
pub struct CSSKeyframeRule {
    // The css style sheet identifier (absent for user agent stylesheet and user-specified
    // stylesheet rules) this rule came from.
    pub style_sheet_id: Option<StyleSheetId>,
    // Parent stylesheet's origin.
    pub origin: StyleSheetOrigin,
    // Associated key text.
    pub key_text: Value,
    // Associated style declaration.
    pub style: CSSStyle,
}
// A descriptor of operation to mutate style declaration text.
pub struct StyleDeclarationEdit {
    // The css style sheet identifier.
    pub style_sheet_id: StyleSheetId,
    // The range of the style text in the enclosing stylesheet.
    pub range: SourceRange,
    // New style text.
    pub text: String,
}

// Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the
// position specified by `location`.
pub struct AddRule {
    // The css style sheet identifier where a new rule should be inserted.
    pub style_sheet_id: StyleSheetId,
    // The text of a new rule.
    pub rule_text: String,
    // Text position of a new rule in the target style sheet.
    pub location: SourceRange,
}
pub struct AddRuleReturnObject {
    // The newly created rule.
    pub rule: CSSRule,
}
// Returns all class names from specified stylesheet.
pub struct CollectClassNames {
    pub style_sheet_id: StyleSheetId,
}
pub struct CollectClassNamesReturnObject {
    // Class name list.
    pub class_names: Vec<String>,
}
// Creates a new special "via-inspector" stylesheet in the frame with given `frameId`.
pub struct CreateStyleSheet {
    // Identifier of the frame where "via-inspector" stylesheet should be created.
    pub frame_id: super::page::FrameId,
}
pub struct CreateStyleSheetReturnObject {
    // Identifier of the created "via-inspector" stylesheet.
    pub style_sheet_id: StyleSheetId,
}
// Disables the CSS agent for the given page.
pub struct Disable {}
pub struct DisableReturnObject {}
// Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been
// enabled until the result of this command is received.
pub struct Enable {}
pub struct EnableReturnObject {}
// Ensures that the given node will have specified pseudo-classes whenever its style is computed by
// the browser.
pub struct ForcePseudoState {
    // The element id for which to force the pseudo state.
    pub node_id: super::dom::NodeId,
    // Element pseudo classes to force when computing the element's style.
    pub forced_pseudo_classes: Vec<String>,
}
pub struct ForcePseudoStateReturnObject {}
pub struct GetBackgroundColors {
    // Id of the node to get background colors for.
    pub node_id: super::dom::NodeId,
}
pub struct GetBackgroundColorsReturnObject {
    // The range of background colors behind this element, if it contains any visible text. If no
    // visible text is present, this will be undefined. In the case of a flat background color,
    // this will consist of simply that color. In the case of a gradient, this will consist of each
    // of the color stops. For anything more complicated, this will be an empty array. Images will
    // be ignored (as if the image had failed to load).
    pub background_colors: Option<Vec<String>>,
    // The computed font size for this node, as a CSS computed value string (e.g. '12px').
    pub computed_font_size: Option<String>,
    // The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    // '100').
    pub computed_font_weight: Option<String>,
}
// Returns the computed style for a DOM node identified by `nodeId`.
pub struct GetComputedStyleForNode {
    pub node_id: super::dom::NodeId,
}
pub struct GetComputedStyleForNodeReturnObject {
    // Computed style for the specified DOM node.
    pub computed_style: Vec<CSSComputedStyleProperty>,
}
// Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
// attributes) for a DOM node identified by `nodeId`.
pub struct GetInlineStylesForNode {
    pub node_id: super::dom::NodeId,
}
pub struct GetInlineStylesForNodeReturnObject {
    // Inline style for the specified DOM node.
    pub inline_style: Option<CSSStyle>,
    // Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub attributes_style: Option<CSSStyle>,
}
// Returns requested styles for a DOM node identified by `nodeId`.
pub struct GetMatchedStylesForNode {
    pub node_id: super::dom::NodeId,
}
pub struct GetMatchedStylesForNodeReturnObject {
    // Inline style for the specified DOM node.
    pub inline_style: Option<CSSStyle>,
    // Attribute-defined element style (e.g. resulting from "width=20 height=100%").
    pub attributes_style: Option<CSSStyle>,
    // CSS rules matching this node, from all applicable stylesheets.
    pub matched_css_rules: Option<Vec<RuleMatch>>,
    // Pseudo style matches for this node.
    pub pseudo_elements: Option<Vec<PseudoElementMatches>>,
    // A chain of inherited styles (from the immediate node parent up to the DOM tree root).
    pub inherited: Option<Vec<InheritedStyleEntry>>,
    // A list of CSS keyframed animations matching this node.
    pub css_keyframes_rules: Option<Vec<CSSKeyframesRule>>,
}
// Returns all media queries parsed by the rendering engine.
pub struct GetMediaQueries {}
pub struct GetMediaQueriesReturnObject {
    pub medias: Vec<CSSMedia>,
}
// Requests information about platform fonts which we used to render child TextNodes in the given
// node.
pub struct GetPlatformFontsForNode {
    pub node_id: super::dom::NodeId,
}
pub struct GetPlatformFontsForNodeReturnObject {
    // Usage statistics for every employed platform font.
    pub fonts: Vec<PlatformFontUsage>,
}
// Returns the current textual content for a stylesheet.
pub struct GetStyleSheetText {
    pub style_sheet_id: StyleSheetId,
}
pub struct GetStyleSheetTextReturnObject {
    // The stylesheet text.
    pub text: String,
}
// Starts tracking the given computed styles for updates. The specified array of properties
// replaces the one previously specified. Pass empty array to disable tracking.
// Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
// The changes to computed style properties are only tracked for nodes pushed to the front-end
// by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
// to the front-end, no updates will be issued for the node.
pub struct TrackComputedStyleUpdates {
    pub properties_to_track: Vec<CSSComputedStyleProperty>,
}
pub struct TrackComputedStyleUpdatesReturnObject {}
// Polls the next batch of computed style updates.
pub struct TakeComputedStyleUpdates {}
pub struct TakeComputedStyleUpdatesReturnObject {
    // The list of node Ids that have their tracked computed styles updated
    pub node_ids: Vec<super::dom::NodeId>,
}
// Find a rule with the given active property for the given node and set the new value for this
// property
pub struct SetEffectivePropertyValueForNode {
    // The element id for which to set property.
    pub node_id: super::dom::NodeId,
    pub property_name: String,
    pub value: String,
}
pub struct SetEffectivePropertyValueForNodeReturnObject {}
// Modifies the keyframe rule key text.
pub struct SetKeyframeKey {
    pub style_sheet_id: StyleSheetId,
    pub range: SourceRange,
    pub key_text: String,
}
pub struct SetKeyframeKeyReturnObject {
    // The resulting key text after modification.
    pub key_text: Value,
}
// Modifies the rule selector.
pub struct SetMediaText {
    pub style_sheet_id: StyleSheetId,
    pub range: SourceRange,
    pub text: String,
}
pub struct SetMediaTextReturnObject {
    // The resulting CSS media rule after modification.
    pub media: CSSMedia,
}
// Modifies the rule selector.
pub struct SetRuleSelector {
    pub style_sheet_id: StyleSheetId,
    pub range: SourceRange,
    pub selector: String,
}
pub struct SetRuleSelectorReturnObject {
    // The resulting selector list after modification.
    pub selector_list: SelectorList,
}
// Sets the new stylesheet text.
pub struct SetStyleSheetText {
    pub style_sheet_id: StyleSheetId,
    pub text: String,
}
pub struct SetStyleSheetTextReturnObject {
    // URL of source map associated with script (if any).
    pub source_map_url: Option<String>,
}
// Applies specified style edits one after another in the given order.
pub struct SetStyleTexts {
    pub edits: Vec<StyleDeclarationEdit>,
}
pub struct SetStyleTextsReturnObject {
    // The resulting styles after modification.
    pub styles: Vec<CSSStyle>,
}
// Enables the selector recording.
pub struct StartRuleUsageTracking {}
pub struct StartRuleUsageTrackingReturnObject {}
// Stop tracking rule usage and return the list of rules that were used since last call to
// `takeCoverageDelta` (or since start of coverage instrumentation)
pub struct StopRuleUsageTracking {}
pub struct StopRuleUsageTrackingReturnObject {
    pub rule_usage: Vec<RuleUsage>,
}
// Obtain list of rules that became used since last call to this method (or since start of coverage
// instrumentation)
pub struct TakeCoverageDelta {}
pub struct TakeCoverageDeltaReturnObject {
    pub coverage: Vec<RuleUsage>,
    // Monotonically increasing time, in seconds.
    pub timestamp: f64,
}
// Enables/disables rendering of local CSS fonts (enabled by default).
pub struct SetLocalFontsEnabled {
    // Whether rendering of local fonts is enabled.
    pub enabled: bool,
}
pub struct SetLocalFontsEnabledReturnObject {}
