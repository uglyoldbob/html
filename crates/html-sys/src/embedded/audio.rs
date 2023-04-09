/// The HTML `<audio>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
#[doc(alias = "audio")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Audio {
    global_attrs: crate::GlobalAttributes,
    /// Address of the resource
    pub src: std::option::Option<String>,
    /// How the element handles crossorigin requests
    pub crossorigin: std::option::Option<String>,
    /// Hints how much buffering the media resource will likely need
    pub preload: std::option::Option<String>,
    /// Hint that the media resource can be started automatically when the page is loaded
    pub autoplay: std::option::Option<String>,
    /// Whether to loop the media resource
    pub loop_: std::option::Option<String>,
    /// Whether to mute the media resource by default
    pub muted: std::option::Option<String>,
    /// Show user agent controls
    pub controls: std::option::Option<String>,
}
impl crate::RenderElement for Audio {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<audio")?;
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            write!(writer, r#" crossorigin="{field}""#)?;
        }
        if let Some(field) = self.preload.as_ref() {
            write!(writer, r#" preload="{field}""#)?;
        }
        if let Some(field) = self.autoplay.as_ref() {
            write!(writer, r#" autoplay="{field}""#)?;
        }
        if let Some(field) = self.loop_.as_ref() {
            write!(writer, r#" loop="{field}""#)?;
        }
        if let Some(field) = self.muted.as_ref() {
            write!(writer, r#" muted="{field}""#)?;
        }
        if let Some(field) = self.controls.as_ref() {
            write!(writer, r#" controls="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</audio>")?;
        Ok(())
    }
}
impl std::fmt::Display for Audio {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Audio {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
