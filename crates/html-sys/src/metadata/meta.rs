/// The HTML `<meta>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
#[doc(alias = "meta")]
#[non_exhaustive]
pub struct Meta {
    global_attributes: crate::GlobalAttributes,
    /// Metadata name
pub name: std::option::Option<String>,
/// Pragma directive
pub http_equiv: std::option::Option<String>,
/// Value of the element
pub content: std::option::Option<String>,
/// Character encoding declaration
pub charset: std::option::Option<String>,
/// Applicable media
pub media: std::option::Option<String>,

}

impl crate::RenderElement for Meta {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<meta")?;
if let Some(field) = self.name.as_ref() {
    write!(writer, r#""name="{}""#, field)?;
}
if let Some(field) = self.http_equiv.as_ref() {
    write!(writer, r#""http-equiv="{}""#, field)?;
}
if let Some(field) = self.content.as_ref() {
    write!(writer, r#""content="{}""#, field)?;
}
if let Some(field) = self.charset.as_ref() {
    write!(writer, r#""charset="{}""#, field)?;
}
if let Some(field) = self.media.as_ref() {
    write!(writer, r#""media="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        
        Ok(())
    }
}
impl std::ops::Deref for Meta {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}