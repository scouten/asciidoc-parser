use nom::{multi::many1, IResult};

use crate::primitives::non_empty_line;

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'a> {
    /// Lines that were found.
    pub inlines: Vec<&'a str>,
}

impl<'a> SimpleBlock<'a> {
    /// Parse a byte-slice as a simple AsciiDoc block.
    ///
    /// Returns a tuple of the remaining input and the simple block.
    #[allow(dead_code)] // TEMPORARY
    pub fn from_str(i: &'a str) -> IResult<&str, Self> {
        let (i, inlines) = many1(non_empty_line)(i)?;

        Ok((i, Self { inlines }))
    }
}
