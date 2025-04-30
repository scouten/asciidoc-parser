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
    rendered: CowStr<'src>,

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
        self.rendered.as_ref()
    }

    /// Returns `true` if `self` contains no text.
    pub fn is_empty(&self) -> bool {
        self.rendered.as_ref().is_empty()
    }

    /// Returns an iterator that can be used to identify regions of unaltered
    /// text vs those where substitutions occurred.
    pub fn spans_and_substitutions<'c>(&'c self) -> SpansAndSubstitutions<'src, 'c> {
        SpansAndSubstitutions::new(self)
    }

    /// Replaces an exact string with another exact string.
    ///
    /// Like [`str::replace`], but does the change in place and records
    /// annotations about the character offsets of the replacement for use in
    /// future calls to [`Content::spans_and_substitutions()`].
    ///
    /// [`str::replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
    pub(crate) fn replace_str(&mut self, from: &str, to: &'static str) {
        self.apply_substitutions(
            self.rendered
                .as_ref()
                .match_indices(from)
                .map(|(index, from)| (index, from.len(), CowStr::<'src>::from(to)))
                .collect(),
        );
    }

    fn apply_substitutions(&mut self, substitutions: Vec<(usize, usize, CowStr<'src>)>) {
        // Shortcut: If no substitutions, avoid the complex calculations ahead.
        if substitutions.is_empty() {
            return;
        }

        if !self.substitutions.is_empty() {
            todo!("My head asplode!");
        }

        // Pre-allocate string buffer for result size.
        let haystack = self.rendered.as_ref();
        let capacity = substitutions
            .iter()
            .fold(haystack.len(), |acc, (_, from_len, to)| {
                acc - from_len + to.len()
            });

        let mut result = String::with_capacity(capacity);
        let mut last_end = 0;

        let mut new_substitutions: Vec<Substitution<'src>> =
            Vec::with_capacity(self.substitutions.len() + substitutions.len());

        for (haystack_index, from_len, replacement) in substitutions {
            if let Some(unchanged) = haystack.get(last_end..haystack_index) {
                result.push_str(unchanged);
            }

            result.push_str(&replacement);

            new_substitutions.push(Substitution {
                original: self
                    .original
                    .slice(haystack_index..haystack_index + from_len),
                replacement: replacement.to_string(),
            });

            last_end = haystack_index + from_len;
        }

        if let Some(last_unchanged) = haystack.get(last_end..haystack.len()) {
            result.push_str(last_unchanged);
        }

        self.rendered = result.into();
        self.substitutions = new_substitutions;
    }
}

impl<'src> From<Span<'src>> for Content<'src> {
    fn from(span: Span<'src>) -> Self {
        Self {
            original: span,
            rendered: CowStr::from(span.data()),
            substitutions: vec![],
        }
    }
}

/// The [`Content::spans_and_substitutions()`] function returns an iterator that
/// yields this type, which contains unaltered text from the original source
/// file interspersed with regions of substituted text ("substitutions").
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpanOrSubstitution<'src> {
    /// A region of unaltered text from the original source file.
    Span(Span<'src>),

    /// A region of text where a substitition occurred.
    Substitution(Substitution<'src>),
}

/// A [`Substitution`] describes a single substitution made to the original
/// source text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Substitution<'src> {
    /// The original text before substitution.
    pub original: Span<'src>,

    /// The replacement value.
    pub replacement: String,
}

/// The [`Content::spans_and_substitutions()`] function returns an iterator of
/// this type, which can be used to identify regions of unaltered text vs those
/// where substitutions occurred.
#[derive(Debug)]
pub struct SpansAndSubstitutions<'src, 'c: 'src> {
    content: &'c Content<'src>,
    substitutions_index: usize,
    last_original_end: usize,
}

impl<'c> SpansAndSubstitutions<'_, 'c> {
    fn new(content: &'c Content) -> Self {
        Self {
            content,
            substitutions_index: 0,
            last_original_end: 0,
        }
    }
}

impl<'src> Iterator for SpansAndSubstitutions<'src, '_> {
    type Item = SpanOrSubstitution<'src>;

    fn next(&mut self) -> Option<SpanOrSubstitution<'src>> {
        // TO DO: What if span original has an offset that's non-zero?

        let last_original_end = self.last_original_end;

        let Some(next_substitution) = self.content.substitutions.get(self.substitutions_index)
        else {
            let original = self.content.original;
            let original_end = original.byte_offset() + original.len();

            if last_original_end < original_end {
                self.last_original_end = original_end;
                return Some(SpanOrSubstitution::Span(
                    self.content
                        .original
                        .slice_from(last_original_end - original.byte_offset()..),
                ));
            } else {
                return None;
            }
        };

        let next_substitution_start = next_substitution.original.byte_offset();

        if last_original_end < next_substitution_start {
            let original = self.content.original;
            self.last_original_end = next_substitution_start;

            return Some(SpanOrSubstitution::Span(self.content.original.slice(
                last_original_end - original.byte_offset()..next_substitution_start,
            )));
        } else if next_substitution.replacement.is_empty() {
            self.substitutions_index += 1;
            self.next()
        } else {
            self.substitutions_index += 1;
            self.last_original_end += next_substitution.original.len();
            Some(SpanOrSubstitution::Substitution(next_substitution.clone()))
        }
    }
}
