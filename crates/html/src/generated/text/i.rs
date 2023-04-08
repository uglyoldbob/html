/// The HTML `<i>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i)
#[doc(alias = "i")]
#[non_exhaustive]
pub struct Italic {
    _sys: html_sys::text::Italic,
}
impl crate::categories::FlowContent for Italic {}
impl crate::categories::PhrasingContent for Italic {}
impl crate::categories::PalpableContent for Italic {}
