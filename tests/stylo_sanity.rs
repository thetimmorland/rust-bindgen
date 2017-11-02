// Don't want to copy that nasty `cfg` below...
#[allow(unused_extern_crates)]
extern crate bindgen;

/// A sanity test that we can generate bindings for Stylo.
///
/// We don't assert on expected output because its just too big. The output will
/// change too often, and it won't be clear what is going on at a glance, unlike
/// the other tests with smaller input headers.
///
/// This test is relatively slow, so we also only run it in release mode.
///
/// Finally, uncomment the `panic!` at the bottom of the test to get logs timing
/// how long bindings generation takes for Stylo. Stylo bindings generation
/// takes too long to be a proper `#[bench]`.
#[test]
#[cfg(not(any(debug_assertions,
              feature = "testing_only_extra_assertions",
              feature = "testing_only_libclang_3_8")))]
#[cfg(any(feature = "testing_only_libclang_3_9",
          feature = "testing_only_libclang_4"))]
fn sanity_check_can_generate_stylo_bindings() {
    use std::time::Instant;

    let then = Instant::now();

    bindgen::builder()
        .time_phases(true)
        .header(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/stylo.hpp"))
        .whitelist_function("Servo_.*")
        .whitelist_function("Gecko_.*")
        .blacklist_type("nsACString_internal")
        .blacklist_type("nsAString_internal")
        .blacklist_type("mozilla::css::URLValue")
        .blacklist_type("RawGeckoAnimationPropertySegment")
        .blacklist_type("RawGeckoComputedTiming")
        .blacklist_type("RawGeckoDocument")
        .blacklist_type("RawGeckoElement")
        .blacklist_type("RawGeckoKeyframeList")
        .blacklist_type("RawGeckoComputedKeyframeValuesList")
        .blacklist_type("RawGeckoFontFaceRuleList")
        .blacklist_type("RawGeckoNode")
        .blacklist_type("RawGeckoAnimationValueList")
        .blacklist_type("RawServoAnimationValue")
        .blacklist_type("RawServoAnimationValueMap")
        .blacklist_type("RawServoDeclarationBlock")
        .blacklist_type("RawGeckoPresContext")
        .blacklist_type("RawGeckoPresContextOwned")
        .blacklist_type("RawGeckoStyleAnimationList")
        .blacklist_type("RawGeckoURLExtraData")
        .blacklist_type("RefPtr")
        .blacklist_type("CSSPseudoClassType")
        .blacklist_type("TraversalRootBehavior")
        .blacklist_type("ComputedTimingFunction_BeforeFlag")
        .blacklist_type("FontFamilyList")
        .blacklist_type("FontFamilyType")
        .blacklist_type("Keyframe")
        .blacklist_type("ServoBundledURI")
        .blacklist_type("ServoElementSnapshot")
        .blacklist_type("SheetParsingMode")
        .blacklist_type("StyleBasicShape")
        .blacklist_type("StyleBasicShapeType")
        .blacklist_type("StyleShapeSource")
        .blacklist_type("nsCSSFontFaceRule")
        .blacklist_type("nsCSSKeyword")
        .blacklist_type("nsCSSPropertyID")
        .blacklist_type("nsCSSShadowArray")
        .blacklist_type("nsCSSUnit")
        .blacklist_type("nsCSSValue")
        .blacklist_type("nsCSSValueSharedList")
        .blacklist_type("nsChangeHint")
        .blacklist_type("nsCursorImage")
        .blacklist_type("nsFont")
        .blacklist_type("nsIAtom")
        .blacklist_type("nsMediaFeature")
        .blacklist_type("nsRestyleHint")
        .blacklist_type("nsStyleBackground")
        .blacklist_type("nsStyleBorder")
        .blacklist_type("nsStyleColor")
        .blacklist_type("nsStyleColumn")
        .blacklist_type("nsStyleContent")
        .blacklist_type("nsStyleContentData")
        .blacklist_type("nsStyleContentType")
        .blacklist_type("nsStyleContext")
        .blacklist_type("nsStyleCoord")
        .blacklist_type("nsStyleCoord_Calc")
        .blacklist_type("nsStyleCoord_CalcValue")
        .blacklist_type("nsStyleDisplay")
        .blacklist_type("nsStyleEffects")
        .blacklist_type("nsStyleFilter")
        .blacklist_type("nsStyleFont")
        .blacklist_type("nsStyleGradient")
        .blacklist_type("nsStyleGradientStop")
        .blacklist_type("nsStyleImage")
        .blacklist_type("nsStyleImageLayers")
        .blacklist_type("nsStyleImageLayers_Layer")
        .blacklist_type("nsStyleImageLayers_LayerType")
        .blacklist_type("nsStyleImageRequest")
        .blacklist_type("nsStyleList")
        .blacklist_type("nsStyleMargin")
        .blacklist_type("nsStyleOutline")
        .blacklist_type("nsStylePadding")
        .blacklist_type("nsStylePosition")
        .blacklist_type("nsStyleQuoteValues")
        .blacklist_type("nsStyleSVG")
        .blacklist_type("nsStyleSVGPaint")
        .blacklist_type("nsStyleSVGReset")
        .blacklist_type("nsStyleTable")
        .blacklist_type("nsStyleTableBorder")
        .blacklist_type("nsStyleText")
        .blacklist_type("nsStyleTextReset")
        .blacklist_type("nsStyleUIReset")
        .blacklist_type("nsStyleUnion")
        .blacklist_type("nsStyleUnit")
        .blacklist_type("nsStyleUserInterface")
        .blacklist_type("nsStyleVariables")
        .blacklist_type("nsStyleVisibility")
        .blacklist_type("nsStyleXUL")
        .blacklist_type("nsTimingFunction")
        .blacklist_type("nscolor")
        .blacklist_type("nscoord")
        .blacklist_type("nsresult")
        .blacklist_type("Loader")
        .blacklist_type("ServoStyleSheet")
        .blacklist_type("EffectCompositor_CascadeLevel")
        .blacklist_type("UpdateAnimationsTasks")
        .blacklist_type("nsTArrayBorrowed_uintptr_t")
        .blacklist_type("ServoCssRulesStrong")
        .blacklist_type("ServoCssRulesBorrowed")
        .blacklist_type("ServoCssRulesBorrowedOrNull")
        .blacklist_type("ServoCssRules")
        .blacklist_type("RawServoStyleSheetStrong")
        .blacklist_type("RawServoStyleSheetBorrowed")
        .blacklist_type("RawServoStyleSheetBorrowedOrNull")
        .blacklist_type("RawServoStyleSheet")
        .blacklist_type("ServoComputedValuesStrong")
        .blacklist_type("ServoComputedValuesBorrowed")
        .blacklist_type("ServoComputedValuesBorrowedOrNull")
        .blacklist_type("ServoComputedValues")
        .blacklist_type("RawServoDeclarationBlockStrong")
        .blacklist_type("RawServoDeclarationBlockBorrowed")
        .blacklist_type("RawServoDeclarationBlockBorrowedOrNull")
        .blacklist_type("RawServoStyleRuleStrong")
        .blacklist_type("RawServoStyleRuleBorrowed")
        .blacklist_type("RawServoStyleRuleBorrowedOrNull")
        .blacklist_type("RawServoStyleRule")
        .blacklist_type("RawServoImportRuleStrong")
        .blacklist_type("RawServoImportRuleBorrowed")
        .blacklist_type("RawServoImportRuleBorrowedOrNull")
        .blacklist_type("RawServoImportRule")
        .blacklist_type("RawServoAnimationValueStrong")
        .blacklist_type("RawServoAnimationValueBorrowed")
        .blacklist_type("RawServoAnimationValueBorrowedOrNull")
        .blacklist_type("RawServoAnimationValueMapStrong")
        .blacklist_type("RawServoAnimationValueMapBorrowed")
        .blacklist_type("RawServoAnimationValueMapBorrowedOrNull")
        .blacklist_type("RawServoMediaListStrong")
        .blacklist_type("RawServoMediaListBorrowed")
        .blacklist_type("RawServoMediaListBorrowedOrNull")
        .blacklist_type("RawServoMediaList")
        .blacklist_type("RawServoMediaRuleStrong")
        .blacklist_type("RawServoMediaRuleBorrowed")
        .blacklist_type("RawServoMediaRuleBorrowedOrNull")
        .blacklist_type("RawServoMediaRule")
        .blacklist_type("RawServoNamespaceRuleStrong")
        .blacklist_type("RawServoNamespaceRuleBorrowed")
        .blacklist_type("RawServoNamespaceRuleBorrowedOrNull")
        .blacklist_type("RawServoNamespaceRule")
        .blacklist_type("RawServoStyleSetOwned")
        .blacklist_type("RawServoStyleSetOwnedOrNull")
        .blacklist_type("RawServoStyleSetBorrowed")
        .blacklist_type("RawServoStyleSetBorrowedOrNull")
        .blacklist_type("RawServoStyleSetBorrowedMut")
        .blacklist_type("RawServoStyleSetBorrowedMutOrNull")
        .blacklist_type("RawServoStyleSet")
        .blacklist_type("StyleChildrenIteratorOwned")
        .blacklist_type("StyleChildrenIteratorOwnedOrNull")
        .blacklist_type("StyleChildrenIteratorBorrowed")
        .blacklist_type("StyleChildrenIteratorBorrowedOrNull")
        .blacklist_type("StyleChildrenIteratorBorrowedMut")
        .blacklist_type("StyleChildrenIteratorBorrowedMutOrNull")
        .blacklist_type("StyleChildrenIterator")
        .blacklist_type("ServoElementSnapshotOwned")
        .blacklist_type("ServoElementSnapshotOwnedOrNull")
        .blacklist_type("ServoElementSnapshotBorrowed")
        .blacklist_type("ServoElementSnapshotBorrowedOrNull")
        .blacklist_type("ServoElementSnapshotBorrowedMut")
        .blacklist_type("ServoElementSnapshotBorrowedMutOrNull")
        .blacklist_type("RawGeckoNodeBorrowed")
        .blacklist_type("RawGeckoNodeBorrowedOrNull")
        .blacklist_type("RawGeckoElementBorrowed")
        .blacklist_type("RawGeckoElementBorrowedOrNull")
        .blacklist_type("RawGeckoDocumentBorrowed")
        .blacklist_type("RawGeckoDocumentBorrowedOrNull")
        .blacklist_type("RawServoDeclarationBlockStrongBorrowed")
        .blacklist_type("RawServoDeclarationBlockStrongBorrowedOrNull")
        .blacklist_type("RawGeckoPresContextBorrowed")
        .blacklist_type("RawGeckoPresContextBorrowedOrNull")
        .blacklist_type("RawGeckoStyleAnimationListBorrowed")
        .blacklist_type("RawGeckoStyleAnimationListBorrowedOrNull")
        .blacklist_type("nsCSSValueBorrowed")
        .blacklist_type("nsCSSValueBorrowedOrNull")
        .blacklist_type("nsCSSValueBorrowedMut")
        .blacklist_type("nsCSSValueBorrowedMutOrNull")
        .blacklist_type("nsTimingFunctionBorrowed")
        .blacklist_type("nsTimingFunctionBorrowedOrNull")
        .blacklist_type("nsTimingFunctionBorrowedMut")
        .blacklist_type("nsTimingFunctionBorrowedMutOrNull")
        .blacklist_type("RawGeckoAnimationPropertySegmentBorrowed")
        .blacklist_type("RawGeckoAnimationPropertySegmentBorrowedOrNull")
        .blacklist_type("RawGeckoAnimationPropertySegmentBorrowedMut")
        .blacklist_type("RawGeckoAnimationPropertySegmentBorrowedMutOrNull")
        .blacklist_type("RawGeckoAnimationValueListBorrowed")
        .blacklist_type("RawGeckoAnimationValueListBorrowedOrNull")
        .blacklist_type("RawGeckoAnimationValueListBorrowedMut")
        .blacklist_type("RawGeckoAnimationValueListBorrowedMutOrNull")
        .blacklist_type("RawGeckoComputedTimingBorrowed")
        .blacklist_type("RawGeckoComputedTimingBorrowedOrNull")
        .blacklist_type("RawGeckoComputedTimingBorrowedMut")
        .blacklist_type("RawGeckoComputedTimingBorrowedMutOrNull")
        .blacklist_type("RawGeckoKeyframeListBorrowed")
        .blacklist_type("RawGeckoKeyframeListBorrowedOrNull")
        .blacklist_type("RawGeckoKeyframeListBorrowedMut")
        .blacklist_type("RawGeckoKeyframeListBorrowedMutOrNull")
        .blacklist_type("RawGeckoComputedKeyframeValuesListBorrowed")
        .blacklist_type("RawGeckoComputedKeyframeValuesListBorrowedOrNull")
        .blacklist_type("RawGeckoComputedKeyframeValuesListBorrowedMut")
        .blacklist_type("RawGeckoComputedKeyframeValuesListBorrowedMutOrNull")
        .blacklist_type("RawGeckoFontFaceRuleListBorrowed")
        .blacklist_type("RawGeckoFontFaceRuleListBorrowedOrNull")
        .blacklist_type("RawGeckoFontFaceRuleListBorrowedMut")
        .blacklist_type("RawGeckoFontFaceRuleListBorrowedMutOrNull")
        .raw_line(r#"pub use nsstring::{nsACString, nsAString, nsString};"#)
        .raw_line(r#"type nsACString_internal = nsACString;"#)
        .raw_line(r#"type nsAString_internal = nsAString;"#)
        .raw_line(r#"use gecko_bindings::structs::mozilla::css::URLValue;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoAnimationPropertySegment;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoComputedTiming;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoDocument;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoElement;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoKeyframeList;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoComputedKeyframeValuesList;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoFontFaceRuleList;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoNode;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoAnimationValueList;"#)
        .raw_line(r#"use gecko_bindings::structs::RawServoAnimationValue;"#)
        .raw_line(r#"use gecko_bindings::structs::RawServoAnimationValueMap;"#)
        .raw_line(r#"use gecko_bindings::structs::RawServoDeclarationBlock;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoPresContext;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoPresContextOwned;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoStyleAnimationList;"#)
        .raw_line(r#"use gecko_bindings::structs::RawGeckoURLExtraData;"#)
        .raw_line(r#"use gecko_bindings::structs::RefPtr;"#)
        .raw_line(r#"use gecko_bindings::structs::CSSPseudoClassType;"#)
        .raw_line(r#"use gecko_bindings::structs::TraversalRootBehavior;"#)
        .raw_line(r#"use gecko_bindings::structs::ComputedTimingFunction_BeforeFlag;"#)
        .raw_line(r#"use gecko_bindings::structs::FontFamilyList;"#)
        .raw_line(r#"use gecko_bindings::structs::FontFamilyType;"#)
        .raw_line(r#"use gecko_bindings::structs::Keyframe;"#)
        .raw_line(r#"use gecko_bindings::structs::ServoBundledURI;"#)
        .raw_line(r#"use gecko_bindings::structs::ServoElementSnapshot;"#)
        .raw_line(r#"use gecko_bindings::structs::SheetParsingMode;"#)
        .raw_line(r#"use gecko_bindings::structs::StyleBasicShape;"#)
        .raw_line(r#"use gecko_bindings::structs::StyleBasicShapeType;"#)
        .raw_line(r#"use gecko_bindings::structs::StyleShapeSource;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSFontFaceRule;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSKeyword;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSPropertyID;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSShadowArray;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSUnit;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSValue;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCSSValueSharedList;"#)
        .raw_line(r#"use gecko_bindings::structs::nsChangeHint;"#)
        .raw_line(r#"use gecko_bindings::structs::nsCursorImage;"#)
        .raw_line(r#"use gecko_bindings::structs::nsFont;"#)
        .raw_line(r#"use gecko_bindings::structs::nsIAtom;"#)
        .raw_line(r#"use gecko_bindings::structs::nsMediaFeature;"#)
        .raw_line(r#"use gecko_bindings::structs::nsRestyleHint;"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleBackground;"#)
        .raw_line(r#"unsafe impl Send for nsStyleBackground {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleBackground {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleBorder;"#)
        .raw_line(r#"unsafe impl Send for nsStyleBorder {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleBorder {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleColor;"#)
        .raw_line(r#"unsafe impl Send for nsStyleColor {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleColor {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleColumn;"#)
        .raw_line(r#"unsafe impl Send for nsStyleColumn {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleColumn {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleContent;"#)
        .raw_line(r#"unsafe impl Send for nsStyleContent {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleContent {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleContentData;"#)
        .raw_line(r#"unsafe impl Send for nsStyleContentData {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleContentData {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleContentType;"#)
        .raw_line(r#"unsafe impl Send for nsStyleContentType {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleContentType {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleContext;"#)
        .raw_line(r#"unsafe impl Send for nsStyleContext {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleContext {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleCoord;"#)
        .raw_line(r#"unsafe impl Send for nsStyleCoord {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleCoord {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleCoord_Calc;"#)
        .raw_line(r#"unsafe impl Send for nsStyleCoord_Calc {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleCoord_Calc {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleCoord_CalcValue;"#)
        .raw_line(r#"unsafe impl Send for nsStyleCoord_CalcValue {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleCoord_CalcValue {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleDisplay;"#)
        .raw_line(r#"unsafe impl Send for nsStyleDisplay {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleDisplay {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleEffects;"#)
        .raw_line(r#"unsafe impl Send for nsStyleEffects {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleEffects {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleFilter;"#)
        .raw_line(r#"unsafe impl Send for nsStyleFilter {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleFilter {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleFont;"#)
        .raw_line(r#"unsafe impl Send for nsStyleFont {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleFont {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleGradient;"#)
        .raw_line(r#"unsafe impl Send for nsStyleGradient {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleGradient {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleGradientStop;"#)
        .raw_line(r#"unsafe impl Send for nsStyleGradientStop {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleGradientStop {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleImage;"#)
        .raw_line(r#"unsafe impl Send for nsStyleImage {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleImage {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleImageLayers;"#)
        .raw_line(r#"unsafe impl Send for nsStyleImageLayers {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleImageLayers {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleImageLayers_Layer;"#)
        .raw_line(r#"unsafe impl Send for nsStyleImageLayers_Layer {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleImageLayers_Layer {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleImageLayers_LayerType;"#)
        .raw_line(r#"unsafe impl Send for nsStyleImageLayers_LayerType {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleImageLayers_LayerType {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleImageRequest;"#)
        .raw_line(r#"unsafe impl Send for nsStyleImageRequest {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleImageRequest {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleList;"#)
        .raw_line(r#"unsafe impl Send for nsStyleList {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleList {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleMargin;"#)
        .raw_line(r#"unsafe impl Send for nsStyleMargin {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleMargin {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleOutline;"#)
        .raw_line(r#"unsafe impl Send for nsStyleOutline {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleOutline {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStylePadding;"#)
        .raw_line(r#"unsafe impl Send for nsStylePadding {}"#)
        .raw_line(r#"unsafe impl Sync for nsStylePadding {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStylePosition;"#)
        .raw_line(r#"unsafe impl Send for nsStylePosition {}"#)
        .raw_line(r#"unsafe impl Sync for nsStylePosition {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleQuoteValues;"#)
        .raw_line(r#"unsafe impl Send for nsStyleQuoteValues {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleQuoteValues {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleSVG;"#)
        .raw_line(r#"unsafe impl Send for nsStyleSVG {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleSVG {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleSVGPaint;"#)
        .raw_line(r#"unsafe impl Send for nsStyleSVGPaint {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleSVGPaint {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleSVGReset;"#)
        .raw_line(r#"unsafe impl Send for nsStyleSVGReset {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleSVGReset {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleTable;"#)
        .raw_line(r#"unsafe impl Send for nsStyleTable {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleTable {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleTableBorder;"#)
        .raw_line(r#"unsafe impl Send for nsStyleTableBorder {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleTableBorder {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleText;"#)
        .raw_line(r#"unsafe impl Send for nsStyleText {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleText {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleTextReset;"#)
        .raw_line(r#"unsafe impl Send for nsStyleTextReset {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleTextReset {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleUIReset;"#)
        .raw_line(r#"unsafe impl Send for nsStyleUIReset {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleUIReset {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleUnion;"#)
        .raw_line(r#"unsafe impl Send for nsStyleUnion {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleUnion {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleUnit;"#)
        .raw_line(r#"unsafe impl Send for nsStyleUnit {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleUnit {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleUserInterface;"#)
        .raw_line(r#"unsafe impl Send for nsStyleUserInterface {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleUserInterface {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleVariables;"#)
        .raw_line(r#"unsafe impl Send for nsStyleVariables {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleVariables {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleVisibility;"#)
        .raw_line(r#"unsafe impl Send for nsStyleVisibility {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleVisibility {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsStyleXUL;"#)
        .raw_line(r#"unsafe impl Send for nsStyleXUL {}"#)
        .raw_line(r#"unsafe impl Sync for nsStyleXUL {}"#)
        .raw_line(r#"use gecko_bindings::structs::nsTimingFunction;"#)
        .raw_line(r#"use gecko_bindings::structs::nscolor;"#)
        .raw_line(r#"use gecko_bindings::structs::nscoord;"#)
        .raw_line(r#"use gecko_bindings::structs::nsresult;"#)
        .raw_line(r#"use gecko_bindings::structs::Loader;"#)
        .raw_line(r#"use gecko_bindings::structs::ServoStyleSheet;"#)
        .raw_line(r#"use gecko_bindings::structs::EffectCompositor_CascadeLevel;"#)
        .raw_line(r#"use gecko_bindings::structs::UpdateAnimationsTasks;"#)
        .raw_line(r#"pub type nsTArrayBorrowed_uintptr_t<'a> = &'a mut ::gecko_bindings::structs::nsTArray<usize>;"#)
        .raw_line(r#"pub type ServoCssRulesStrong = ::gecko_bindings::sugar::ownership::Strong<ServoCssRules>;"#)
        .raw_line(r#"pub type ServoCssRulesBorrowed<'a> = &'a ServoCssRules;"#)
        .raw_line(r#"pub type ServoCssRulesBorrowedOrNull<'a> = Option<&'a ServoCssRules>;"#)
        .raw_line(r#"enum ServoCssRulesVoid { }"#)
        .raw_line(r#"pub struct ServoCssRules(ServoCssRulesVoid);"#)
        .raw_line(r#"pub type RawServoStyleSheetStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoStyleSheet>;"#)
        .raw_line(r#"pub type RawServoStyleSheetBorrowed<'a> = &'a RawServoStyleSheet;"#)
        .raw_line(r#"pub type RawServoStyleSheetBorrowedOrNull<'a> = Option<&'a RawServoStyleSheet>;"#)
        .raw_line(r#"enum RawServoStyleSheetVoid { }"#)
        .raw_line(r#"pub struct RawServoStyleSheet(RawServoStyleSheetVoid);"#)
        .raw_line(r#"pub type ServoComputedValuesStrong = ::gecko_bindings::sugar::ownership::Strong<ServoComputedValues>;"#)
        .raw_line(r#"pub type ServoComputedValuesBorrowed<'a> = &'a ServoComputedValues;"#)
        .raw_line(r#"pub type ServoComputedValuesBorrowedOrNull<'a> = Option<&'a ServoComputedValues>;"#)
        .raw_line(r#"enum ServoComputedValuesVoid { }"#)
        .raw_line(r#"pub struct ServoComputedValues(ServoComputedValuesVoid);"#)
        .raw_line(r#"pub type RawServoDeclarationBlockStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoDeclarationBlock>;"#)
        .raw_line(r#"pub type RawServoDeclarationBlockBorrowed<'a> = &'a RawServoDeclarationBlock;"#)
        .raw_line(r#"pub type RawServoDeclarationBlockBorrowedOrNull<'a> = Option<&'a RawServoDeclarationBlock>;"#)
        .raw_line(r#"pub type RawServoStyleRuleStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoStyleRule>;"#)
        .raw_line(r#"pub type RawServoStyleRuleBorrowed<'a> = &'a RawServoStyleRule;"#)
        .raw_line(r#"pub type RawServoStyleRuleBorrowedOrNull<'a> = Option<&'a RawServoStyleRule>;"#)
        .raw_line(r#"enum RawServoStyleRuleVoid { }"#)
        .raw_line(r#"pub struct RawServoStyleRule(RawServoStyleRuleVoid);"#)
        .raw_line(r#"pub type RawServoImportRuleStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoImportRule>;"#)
        .raw_line(r#"pub type RawServoImportRuleBorrowed<'a> = &'a RawServoImportRule;"#)
        .raw_line(r#"pub type RawServoImportRuleBorrowedOrNull<'a> = Option<&'a RawServoImportRule>;"#)
        .raw_line(r#"enum RawServoImportRuleVoid { }"#)
        .raw_line(r#"pub struct RawServoImportRule(RawServoImportRuleVoid);"#)
        .raw_line(r#"pub type RawServoAnimationValueStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoAnimationValue>;"#)
        .raw_line(r#"pub type RawServoAnimationValueBorrowed<'a> = &'a RawServoAnimationValue;"#)
        .raw_line(r#"pub type RawServoAnimationValueBorrowedOrNull<'a> = Option<&'a RawServoAnimationValue>;"#)
        .raw_line(r#"pub type RawServoAnimationValueMapStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoAnimationValueMap>;"#)
        .raw_line(r#"pub type RawServoAnimationValueMapBorrowed<'a> = &'a RawServoAnimationValueMap;"#)
        .raw_line(r#"pub type RawServoAnimationValueMapBorrowedOrNull<'a> = Option<&'a RawServoAnimationValueMap>;"#)
        .raw_line(r#"pub type RawServoMediaListStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoMediaList>;"#)
        .raw_line(r#"pub type RawServoMediaListBorrowed<'a> = &'a RawServoMediaList;"#)
        .raw_line(r#"pub type RawServoMediaListBorrowedOrNull<'a> = Option<&'a RawServoMediaList>;"#)
        .raw_line(r#"enum RawServoMediaListVoid { }"#)
        .raw_line(r#"pub struct RawServoMediaList(RawServoMediaListVoid);"#)
        .raw_line(r#"pub type RawServoMediaRuleStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoMediaRule>;"#)
        .raw_line(r#"pub type RawServoMediaRuleBorrowed<'a> = &'a RawServoMediaRule;"#)
        .raw_line(r#"pub type RawServoMediaRuleBorrowedOrNull<'a> = Option<&'a RawServoMediaRule>;"#)
        .raw_line(r#"enum RawServoMediaRuleVoid { }"#)
        .raw_line(r#"pub struct RawServoMediaRule(RawServoMediaRuleVoid);"#)
        .raw_line(r#"pub type RawServoNamespaceRuleStrong = ::gecko_bindings::sugar::ownership::Strong<RawServoNamespaceRule>;"#)
        .raw_line(r#"pub type RawServoNamespaceRuleBorrowed<'a> = &'a RawServoNamespaceRule;"#)
        .raw_line(r#"pub type RawServoNamespaceRuleBorrowedOrNull<'a> = Option<&'a RawServoNamespaceRule>;"#)
        .raw_line(r#"enum RawServoNamespaceRuleVoid { }"#)
        .raw_line(r#"pub struct RawServoNamespaceRule(RawServoNamespaceRuleVoid);"#)
        .raw_line(r#"pub type RawServoStyleSetOwned = ::gecko_bindings::sugar::ownership::Owned<RawServoStyleSet>;"#)
        .raw_line(r#"pub type RawServoStyleSetOwnedOrNull = ::gecko_bindings::sugar::ownership::OwnedOrNull<RawServoStyleSet>;"#)
        .raw_line(r#"pub type RawServoStyleSetBorrowed<'a> = &'a RawServoStyleSet;"#)
        .raw_line(r#"pub type RawServoStyleSetBorrowedOrNull<'a> = Option<&'a RawServoStyleSet>;"#)
        .raw_line(r#"pub type RawServoStyleSetBorrowedMut<'a> = &'a mut RawServoStyleSet;"#)
        .raw_line(r#"pub type RawServoStyleSetBorrowedMutOrNull<'a> = Option<&'a mut RawServoStyleSet>;"#)
        .raw_line(r#"enum RawServoStyleSetVoid { }"#)
        .raw_line(r#"pub struct RawServoStyleSet(RawServoStyleSetVoid);"#)
        .raw_line(r#"pub type StyleChildrenIteratorOwned = ::gecko_bindings::sugar::ownership::Owned<StyleChildrenIterator>;"#)
        .raw_line(r#"pub type StyleChildrenIteratorOwnedOrNull = ::gecko_bindings::sugar::ownership::OwnedOrNull<StyleChildrenIterator>;"#)
        .raw_line(r#"pub type StyleChildrenIteratorBorrowed<'a> = &'a StyleChildrenIterator;"#)
        .raw_line(r#"pub type StyleChildrenIteratorBorrowedOrNull<'a> = Option<&'a StyleChildrenIterator>;"#)
        .raw_line(r#"pub type StyleChildrenIteratorBorrowedMut<'a> = &'a mut StyleChildrenIterator;"#)
        .raw_line(r#"pub type StyleChildrenIteratorBorrowedMutOrNull<'a> = Option<&'a mut StyleChildrenIterator>;"#)
        .raw_line(r#"enum StyleChildrenIteratorVoid { }"#)
        .raw_line(r#"pub struct StyleChildrenIterator(StyleChildrenIteratorVoid);"#)
        .raw_line(r#"pub type ServoElementSnapshotOwned = ::gecko_bindings::sugar::ownership::Owned<ServoElementSnapshot>;"#)
        .raw_line(r#"pub type ServoElementSnapshotOwnedOrNull = ::gecko_bindings::sugar::ownership::OwnedOrNull<ServoElementSnapshot>;"#)
        .raw_line(r#"pub type ServoElementSnapshotBorrowed<'a> = &'a ServoElementSnapshot;"#)
        .raw_line(r#"pub type ServoElementSnapshotBorrowedOrNull<'a> = Option<&'a ServoElementSnapshot>;"#)
        .raw_line(r#"pub type ServoElementSnapshotBorrowedMut<'a> = &'a mut ServoElementSnapshot;"#)
        .raw_line(r#"pub type ServoElementSnapshotBorrowedMutOrNull<'a> = Option<&'a mut ServoElementSnapshot>;"#)
        .raw_line(r#"pub type RawGeckoNodeBorrowed<'a> = &'a RawGeckoNode;"#)
        .raw_line(r#"pub type RawGeckoNodeBorrowedOrNull<'a> = Option<&'a RawGeckoNode>;"#)
        .raw_line(r#"pub type RawGeckoElementBorrowed<'a> = &'a RawGeckoElement;"#)
        .raw_line(r#"pub type RawGeckoElementBorrowedOrNull<'a> = Option<&'a RawGeckoElement>;"#)
        .raw_line(r#"pub type RawGeckoDocumentBorrowed<'a> = &'a RawGeckoDocument;"#)
        .raw_line(r#"pub type RawGeckoDocumentBorrowedOrNull<'a> = Option<&'a RawGeckoDocument>;"#)
        .raw_line(r#"pub type RawServoDeclarationBlockStrongBorrowed<'a> = &'a RawServoDeclarationBlockStrong;"#)
        .raw_line(r#"pub type RawServoDeclarationBlockStrongBorrowedOrNull<'a> = Option<&'a RawServoDeclarationBlockStrong>;"#)
        .raw_line(r#"pub type RawGeckoPresContextBorrowed<'a> = &'a RawGeckoPresContext;"#)
        .raw_line(r#"pub type RawGeckoPresContextBorrowedOrNull<'a> = Option<&'a RawGeckoPresContext>;"#)
        .raw_line(r#"pub type RawGeckoStyleAnimationListBorrowed<'a> = &'a RawGeckoStyleAnimationList;"#)
        .raw_line(r#"pub type RawGeckoStyleAnimationListBorrowedOrNull<'a> = Option<&'a RawGeckoStyleAnimationList>;"#)
        .raw_line(r#"pub type nsCSSValueBorrowed<'a> = &'a nsCSSValue;"#)
        .raw_line(r#"pub type nsCSSValueBorrowedOrNull<'a> = Option<&'a nsCSSValue>;"#)
        .raw_line(r#"pub type nsCSSValueBorrowedMut<'a> = &'a mut nsCSSValue;"#)
        .raw_line(r#"pub type nsCSSValueBorrowedMutOrNull<'a> = Option<&'a mut nsCSSValue>;"#)
        .raw_line(r#"pub type nsTimingFunctionBorrowed<'a> = &'a nsTimingFunction;"#)
        .raw_line(r#"pub type nsTimingFunctionBorrowedOrNull<'a> = Option<&'a nsTimingFunction>;"#)
        .raw_line(r#"pub type nsTimingFunctionBorrowedMut<'a> = &'a mut nsTimingFunction;"#)
        .raw_line(r#"pub type nsTimingFunctionBorrowedMutOrNull<'a> = Option<&'a mut nsTimingFunction>;"#)
        .raw_line(r#"pub type RawGeckoAnimationPropertySegmentBorrowed<'a> = &'a RawGeckoAnimationPropertySegment;"#)
        .raw_line(r#"pub type RawGeckoAnimationPropertySegmentBorrowedOrNull<'a> = Option<&'a RawGeckoAnimationPropertySegment>;"#)
        .raw_line(r#"pub type RawGeckoAnimationPropertySegmentBorrowedMut<'a> = &'a mut RawGeckoAnimationPropertySegment;"#)
        .raw_line(r#"pub type RawGeckoAnimationPropertySegmentBorrowedMutOrNull<'a> = Option<&'a mut RawGeckoAnimationPropertySegment>;"#)
        .raw_line(r#"pub type RawGeckoAnimationValueListBorrowed<'a> = &'a RawGeckoAnimationValueList;"#)
        .raw_line(r#"pub type RawGeckoAnimationValueListBorrowedOrNull<'a> = Option<&'a RawGeckoAnimationValueList>;"#)
        .raw_line(r#"pub type RawGeckoAnimationValueListBorrowedMut<'a> = &'a mut RawGeckoAnimationValueList;"#)
        .raw_line(r#"pub type RawGeckoAnimationValueListBorrowedMutOrNull<'a> = Option<&'a mut RawGeckoAnimationValueList>;"#)
        .raw_line(r#"pub type RawGeckoComputedTimingBorrowed<'a> = &'a RawGeckoComputedTiming;"#)
        .raw_line(r#"pub type RawGeckoComputedTimingBorrowedOrNull<'a> = Option<&'a RawGeckoComputedTiming>;"#)
        .raw_line(r#"pub type RawGeckoComputedTimingBorrowedMut<'a> = &'a mut RawGeckoComputedTiming;"#)
        .raw_line(r#"pub type RawGeckoComputedTimingBorrowedMutOrNull<'a> = Option<&'a mut RawGeckoComputedTiming>;"#)
        .raw_line(r#"pub type RawGeckoKeyframeListBorrowed<'a> = &'a RawGeckoKeyframeList;"#)
        .raw_line(r#"pub type RawGeckoKeyframeListBorrowedOrNull<'a> = Option<&'a RawGeckoKeyframeList>;"#)
        .raw_line(r#"pub type RawGeckoKeyframeListBorrowedMut<'a> = &'a mut RawGeckoKeyframeList;"#)
        .raw_line(r#"pub type RawGeckoKeyframeListBorrowedMutOrNull<'a> = Option<&'a mut RawGeckoKeyframeList>;"#)
        .raw_line(r#"pub type RawGeckoComputedKeyframeValuesListBorrowed<'a> = &'a RawGeckoComputedKeyframeValuesList;"#)
        .raw_line(r#"pub type RawGeckoComputedKeyframeValuesListBorrowedOrNull<'a> = Option<&'a RawGeckoComputedKeyframeValuesList>;"#)
        .raw_line(r#"pub type RawGeckoComputedKeyframeValuesListBorrowedMut<'a> = &'a mut RawGeckoComputedKeyframeValuesList;"#)
        .raw_line(r#"pub type RawGeckoComputedKeyframeValuesListBorrowedMutOrNull<'a> = Option<&'a mut RawGeckoComputedKeyframeValuesList>;"#)
        .raw_line(r#"pub type RawGeckoFontFaceRuleListBorrowed<'a> = &'a RawGeckoFontFaceRuleList;"#)
        .raw_line(r#"pub type RawGeckoFontFaceRuleListBorrowedOrNull<'a> = Option<&'a RawGeckoFontFaceRuleList>;"#)
        .raw_line(r#"pub type RawGeckoFontFaceRuleListBorrowedMut<'a> = &'a mut RawGeckoFontFaceRuleList;"#)
        .raw_line(r#"pub type RawGeckoFontFaceRuleListBorrowedMutOrNull<'a> = Option<&'a mut RawGeckoFontFaceRuleList>;"#)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .clang_arg("-DTRACING=1")
        .clang_arg("-DIMPL_LIBXUL")
        .clang_arg("-DMOZ_STYLO_BINDINGS=1")
        .clang_arg("-DMOZILLA_INTERNAL_API")
        .clang_arg("-DRUST_BINDGEN")
        .clang_arg("-DMOZ_STYLO")
        .clang_arg("-DOS_POSIX=1")
        .clang_arg("-DOS_LINUX=1")
        .generate()
        .expect("Should generate stylo bindings");

    let now = Instant::now();

    println!("");
    println!("");
    println!(
        "Generated Stylo bindings in: {:?}",
        now.duration_since(then)
    );
    println!("");
    println!("");

    // panic!("Uncomment this line to get timing logs");
}
