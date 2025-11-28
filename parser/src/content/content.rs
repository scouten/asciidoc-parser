//! Describes the content of a non-compound block after any relevant
//! [substitutions] have been performed.
//!
//! [substitutions]: https://docs.asciidoctor.org/asciidoc/latest/subs/

use crate::{Span, strings::CowStr};

/// Describes the annotated content of a block after any relevant
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
    /// Constructs a `Content` from a source `Span` and a potentially-filtered
    /// view of that source text.
    pub(crate) fn from_filtered<T: AsRef<str>>(span: Span<'src>, filtered: T) -> Self {
        Self {
            original: span,
            rendered: filtered.as_ref().to_string().into(),
        }
    }

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
}

impl<'src> From<Span<'src>> for Content<'src> {
    fn from(span: Span<'src>) -> Self {
        Self {
            original: span,
            rendered: CowStr::from(span.data()),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    mod is_empty {
        #[test]
        fn basic_empty_span() {
            let content = crate::content::Content::from(crate::Span::default());
            assert!(content.is_empty());
        }

        #[test]
        fn basic_non_empty_span() {
            let content = crate::content::Content::from(crate::Span::new("blah"));
            assert!(!content.is_empty());
        }
    }
}
