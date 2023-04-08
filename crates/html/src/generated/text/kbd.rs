/// The HTML `<kbd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
#[doc(alias = "kbd")]
#[non_exhaustive]
pub struct KeyboardInput {
    _sys: html_sys::text::KeyboardInput,
}
impl crate::categories::FlowContent for KeyboardInput {}
impl crate::categories::PhrasingContent for KeyboardInput {}
impl crate::categories::PalpableContent for KeyboardInput {}
