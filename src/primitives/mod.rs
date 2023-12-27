mod line;
#[allow(unused_imports)]
pub(crate) use line::{consume_empty_lines, empty_line, line, non_empty_line, normalized_line};

/// Represents a subset of the overall input stream.
///
/// Annotated with 1-based line and column numbers relative to the
/// beginning of the overall input stream.
///
/// Called `Span` because its `data` member can be consumed
/// to yield another `Span` with annotations for the end of the
/// syntactic element in question.
pub type Span<'a> = nom_span::Spanned<&'a str>;
