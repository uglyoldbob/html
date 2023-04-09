//! Demarcating edits elements
//!
//! These elements let you provide indications that specific parts of the text
//! have been altered.

pub use crate::generated::edits::*;

/// Child elements
pub mod children {
    pub use crate::generated::edits::children::*;
}

pub use crate::generated::embedded::{Embed, Iframe, MediaSource, Object, Picture};
