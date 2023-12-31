use nom::IResult;

use crate::Span;

/// The error type for AsciiDoc parsing operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    /// Error from nom parsing framework.
    #[error("nom error: {0:?}")]
    NomError(nom::error::Error<Span<'a>>),
}

impl<'a> From<nom::Err<nom::error::Error<Span<'a>>>> for Error<'a> {
    fn from(e: nom::Err<nom::error::Error<Span<'a>>>) -> Self {
        match e {
            nom::Err::Incomplete(_n) => unreachable!("We don't do streaming parsing"),
            nom::Err::Error(e) | nom::Err::Failure(e) => Self::NomError(e),
        }
    }
}

/// Holds the result of AsciiDoc parsing functions.
///
/// Note that this type is also a [`Result`], so the usual functions (`map`,
/// `unwrap`, etc.) are available.
pub type ParseResult<'a, T, E = Error<'a>> = IResult<&'a str, T, E>;
