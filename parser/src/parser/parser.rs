use crate::Document;

/// The [`Parser`] struct and its related structs allow a caller to configure
/// how AsciiDoc parsing occurs and then to initiate the parsing process.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Parser {}

impl Parser {
    /// Parse a UTF-8 string as an AsciiDoc document.
    ///
    /// Note that the document references the underlying source string and
    /// necessarily has the same lifetime as the source.
    ///
    /// The `Document` data structure returned by this call and nearly all data
    /// structures contained within it are gated by the lifetime of the `source`
    /// text passed in to this function. For that reason all of those data
    /// structures are given the lifetime `'src`.
    ///
    /// **IMPORTANT:** The AsciiDoc language documentation states that UTF-16
    /// encoding is allowed if a byte-order-mark (BOM) is present at the
    /// start of a file. This format is not directly supported by the
    /// `asciidoc-parser` crate. Any UTF-16 content must be re-encoded as
    /// UTF-8 prior to parsing.
    ///
    /// # Warnings, not errors
    ///
    /// Any UTF-8 string is a valid AsciiDoc document, so this function does not
    /// return an [`Option`] or [`Result`] data type. There may be any number of
    /// character sequences that have ambiguous or potentially unintended
    /// meanings. For that reason, a caller is advised to review the warnings
    /// provided via the [`warnings()`] iterator.
    ///
    /// [`warnings()`]: Document::warnings
    pub fn parse<'src>(&self, source: &'src str) -> Document<'src> {
        let mut temp_copy = self.clone();
        Document::parse(source, &mut temp_copy)
    }
}
