use thiserror::Error;

use crate::Span;

/// Describes a possible parse error (i.e. a "warning") and its location.
///
/// In `asciidoc-parser`, all documents are parseable, so this mechanism is used
/// to convey conditions where the parse result might be unexpected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Warning<'src> {
    /// Location where the warning was detected.
    pub source: Span<'src>,

    /// Type of warning detected.
    pub warning: WarningType,
}

/// Type of possible parse error that was detected.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[allow(dead_code)] // TEMPORARY while building
pub enum WarningType {
    #[error(
        "A shorthand element attribute marker ('.', '#', or '%') was found with no subsequent text"
    )]
    EmptyShorthandItem,
}

/// Return type used to signal one possible parse error.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)] // TEMPORARY while building
pub(crate) struct MatchAndMaybeWarning<'src, T> {
    /// Matched item. Typically either `MatchedItem<X>` or
    /// `Option<MatchedItem<X>>`.
    pub(crate) item: T,

    /// Possible parse error.
    pub(crate) warning: Option<Warning<'src>>,
}

/// Return type used to signal one or more possible parse error.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)] // TEMPORARY while building
pub(crate) struct MatchAndWarnings<'src, T> {
    /// Matched item. Typically either `MatchedItem<X>` or
    /// `Option<MatchedItem<X>>`.
    pub(crate) item: T,

    /// Possible parse errors.
    pub(crate) warnings: Vec<Warning<'src>>,
}
