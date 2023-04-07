/// The HTML `<div>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
#[doc(alias = "div")]
#[non_exhaustive]
pub struct Division {
    global_attributes: crate::GlobalAttributes,
    
}

impl crate::RenderElement for Division {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<div")?;
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</div>")?;
        
        Ok(())
    }
}
impl std::ops::Deref for Division {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Division {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}