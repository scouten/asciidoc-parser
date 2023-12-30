use crate::Span;

/// Any syntactic element can describe its location
/// within the source material using this trait.
pub trait HasSpan<'a> {
    /// Return a [`Span`] describing the syntactic element's
    /// location within the source string/file.
    fn span(&'a self) -> &'a Span<'a>;
}
