#![allow(unused)] // TEMPORARY while building
use crate::Span;

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
    rendered: Option<String>,

    /// Ordered list of substitutions applied to the original span.
    substitutions: Vec<Substitution<'src>>,
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
        match self.rendered.as_ref() {
            Some(r) => r,
            None => self.original.data(),
        }
    }

    /// Returns the final text after substitions, but only if substitions were
    /// applied.
    ///
    /// Returns `None` otherwise.
    pub(crate) fn rendered_if_changed(&'src self) -> Option<&'src str> {
        match self.rendered.as_ref() {
            Some(r) => Some(r),
            None => None,
        }
    }

    /// Returns `true` if `self` contains no text.
    pub fn is_empty(&self) -> bool {
        if let Some(ref rendered) = self.rendered {
            rendered.is_empty()
        } else {
            self.original.is_empty()
        }
    }

    /// Returns an iterator that can be used to identify regions of unaltered
    /// text vs those where substitutions occurred.
    pub fn spans_and_substitions(&self) -> SpansAndSubstitutions<'src> {
        if self.rendered.is_some() {
            todo!("Implement iterator when substitions have occurred");
        }

        SpansAndSubstitutions {
            original: self.original,
        }
    }
}

impl<'src> From<Span<'src>> for Content<'src> {
    fn from(span: Span<'src>) -> Self {
        Self {
            original: span,
            rendered: None,
            substitutions: vec![],
        }
    }
}

/// The [`Content::spans_and_substitions()`] function returns an iterator that
/// yields this type, which contains unaltered text from the original source
/// file interspersed with regions of substituted text ("substitutions").
pub enum SpanOrSubstitution<'src> {
    /// A region of unaltered text from the original source file.
    Span(Span<'src>),

    /// A region of text where a substitition occurred.
    Substitution(Substitution<'src>),
}

/// A [`Substitition`] describes a single substitution made to the original
/// source text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Substitution<'src> {
    /// The original text before substitution.
    pub original: Span<'src>,

    /// The replacement value.
    pub replacement: &'src str,
}

/// The [`Content::spans_and_substitions()`] function returns an iterator of
/// this type, which can be used to identify regions of unaltered text vs those
/// where substitutions occurred.
#[derive(Debug)]
pub struct SpansAndSubstitutions<'src> {
    original: Span<'src>,
}

impl<'src> Iterator for SpansAndSubstitutions<'src> {
    type Item = SpanOrSubstitution<'src>;

    fn next(&mut self) -> Option<SpanOrSubstitution<'src>> {
        if self.original.is_empty() {
            None
        } else {
            // TO DO: Naive implementation before we implement substititions.
            let result = self.original;
            self.original = self.original.discard_all();
            Some(SpanOrSubstitution::Span(result))
        }
    }
}
