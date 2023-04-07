/// The HTML `<details>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
#[doc(alias = "details")]
#[non_exhaustive]
pub struct Details {
    global_attributes: crate::GlobalAttributes,
    /// Whether the details are visible
pub open: std::option::Option<String>,

}

impl crate::RenderElement for Details {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<details")?;
if let Some(field) = self.open.as_ref() {
    write!(writer, r#""open="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</details>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for Details {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Details {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}