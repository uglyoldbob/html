/// The HTML `<small>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
#[doc(alias = "small")]
#[non_exhaustive]
pub struct SideComment {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for SideComment {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<small")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</small>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for SideComment {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for SideComment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}