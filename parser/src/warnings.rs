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
#[derive(Clone, Eq, Error, PartialEq)]
pub enum WarningType {
    #[error("An attribute value is missing its terminating quote")]
    AttributeValueMissingTerminatingQuote,

    #[error(
        "Document header wasn't terminated by a blank line (this line can't be parsed as part of a document header)"
    )]
    DocumentHeaderNotTerminated,

    #[error("An empty attribute value was detected")]
    EmptyAttributeValue,

    #[error(
        "A shorthand element attribute marker ('.', '#', or '%') was found with no subsequent text"
    )]
    EmptyShorthandItem,

    // TO DO BEFORE CHECKING IN TO MAIN: Review these error names and descriptions.
    #[error("Macro name is not a valid identifier")]
    InvalidMacroName,

    #[error("Media macro missing target")]
    MediaMacroMissingTarget,

    #[error("Macro missing attribute list")]
    MacroMissingAttributeList,

    #[error("Macro missing :: separator")]
    MacroMissingDoubleColon,

    #[error("Missing comma after quoted attribute value")]
    MissingCommaAfterQuotedAttributeValue,

    #[error("Closing marker for delimited block not found")]
    UnterminatedDelimitedBlock,

    #[error("A block title or attribute list was found without a subsequent block")]
    MissingBlockAfterTitleOrAttributeList,

    #[error("Block anchor name is empty")]
    EmptyBlockAnchorName,

    #[error("Block anchor name contains invalid name characters")]
    InvalidBlockAnchorName,

    #[error("Attribute {0:?} can not be modified by document")]
    AttributeValueIsLocked(String),

    #[error("Level 0 section headings not supported")]
    Level0SectionHeadingNotSupported,

    #[error("Section heading level skipped (expected {0}, found {1})")]
    SectionHeadingLevelSkipped(usize, usize),

    #[error("Section heading level exceeds maximum (maximum 5, found {0})")]
    SectionHeadingLevelExceedsMaximum(usize),
}

impl std::fmt::Debug for WarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningType::AttributeValueMissingTerminatingQuote => {
                write!(f, "WarningType::AttributeValueMissingTerminatingQuote")
            }

            WarningType::DocumentHeaderNotTerminated => {
                write!(f, "WarningType::DocumentHeaderNotTerminated")
            }

            WarningType::EmptyAttributeValue => write!(f, "WarningType::EmptyAttributeValue"),
            WarningType::EmptyShorthandItem => write!(f, "WarningType::EmptyShorthandItem"),
            WarningType::InvalidMacroName => write!(f, "WarningType::InvalidMacroName"),

            WarningType::MediaMacroMissingTarget => {
                write!(f, "WarningType::MediaMacroMissingTarget")
            }

            WarningType::MacroMissingAttributeList => {
                write!(f, "WarningType::MacroMissingAttributeList")
            }

            WarningType::MacroMissingDoubleColon => {
                write!(f, "WarningType::MacroMissingDoubleColon")
            }

            WarningType::MissingCommaAfterQuotedAttributeValue => {
                write!(f, "WarningType::MissingCommaAfterQuotedAttributeValue")
            }

            WarningType::UnterminatedDelimitedBlock => {
                write!(f, "WarningType::UnterminatedDelimitedBlock")
            }

            WarningType::MissingBlockAfterTitleOrAttributeList => {
                write!(f, "WarningType::MissingBlockAfterTitleOrAttributeList")
            }

            WarningType::EmptyBlockAnchorName => write!(f, "WarningType::EmptyBlockAnchorName"),
            WarningType::InvalidBlockAnchorName => write!(f, "WarningType::InvalidBlockAnchorName"),

            WarningType::AttributeValueIsLocked(value) => f
                .debug_tuple("WarningType::AttributeValueIsLocked")
                .field(value)
                .finish(),

            WarningType::Level0SectionHeadingNotSupported => {
                write!(f, "WarningType::Level0SectionHeadingNotSupported")
            }

            WarningType::SectionHeadingLevelSkipped(expected, found) => f
                .debug_tuple("WarningType::SectionHeadingLevelSkipped")
                .field(expected)
                .field(found)
                .finish(),

            WarningType::SectionHeadingLevelExceedsMaximum(found) => f
                .debug_tuple("WarningType::SectionHeadingLevelExceedsMaximum")
                .field(found)
                .finish(),
        }
    }
}

/// Return type used to signal one or more possible parse error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MatchAndWarnings<'src, T> {
    /// Matched item. Typically either `MatchedItem<X>` or
    /// `Option<MatchedItem<X>>`.
    pub(crate) item: T,

    /// Possible parse errors.
    pub(crate) warnings: Vec<Warning<'src>>,
}

impl<T> MatchAndWarnings<'_, T> {
    #[cfg(test)]
    #[inline(always)]
    #[track_caller]
    #[allow(clippy::panic)] // since not actually in production code
    pub(crate) fn unwrap_if_no_warnings(self) -> T {
        if self.warnings.is_empty() {
            self.item
        } else {
            panic!(
                "expected self.warnings to be empty\n\nfound warnings = {warnings:#?}\n",
                warnings = self.warnings
            );
        }
    }
}
