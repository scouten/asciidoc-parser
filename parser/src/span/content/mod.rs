#![allow(unused)] // TEMPORARY while building

use crate::{strings::CowStr, Span};

/// Describes the annotated content of a [`Span`] after any relevant
/// [substitutions] have been performed.
///
/// This is typically used to represent the main body of block types that don't
/// contain other blocks, such as [`SimpleBlock`] or [`RawDelimitedBlock`].
///
/// [substitutions]: https://docs.asciidoctor.org/asciidoc/latest/subs/
/// [`SimpleBlock`]: crate::blocks::SimpleBlock
/// [`RawDelimitedBlock`]: crate::blocks::RawDelimitedBlock
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Content<'src> {
    /// The original [`Span`] from which this content was derived.
    original: Span<'src>,

    /// The possibly-modified text after substititions have been performed.
    pub(crate) rendered: CowStr<'src>,
}

impl<'src> Content<'src> {
    /// Returns the original span from which this [`Content`] was derived.
    ///
    /// This is the source text before any substitions have been applied.
    pub fn original(&self) -> Span<'src> {
        self.original
    }

    /// Returns the final text after all substitutions have been applied.
    pub fn rendered(&'src self) -> &'src str {
        self.rendered.as_ref()
    }

    /// Returns `true` if `self` contains no text.
    pub fn is_empty(&self) -> bool {
        self.rendered.as_ref().is_empty()
    }

    /// Replaces an exact string with another exact string.
    pub(crate) fn replace_str(&mut self, from: &str, to: &'static str) {
        if self.rendered.contains(from) {
            self.rendered = self.rendered.replace(from, to).into();
        }
    }
}

impl<'src> From<Span<'src>> for Content<'src> {
    fn from(span: Span<'src>) -> Self {
        Self {
            original: span,
            rendered: CowStr::from(span.data()),
        }
    }
}

mod passthroughs;
use passthroughs::Passthroughs;

mod substitution_group;
pub(crate) use substitution_group::SubstitutionGroup;

mod substitution_step;
pub(crate) use substitution_step::SubstitutionStep;
