use nom::error::ParseError;

use crate::Span;

// TO DO: I think this crate is moving in the direction of being infallible
// (i.e. error conditions are reflected as annotations rather than overall
// failures). So this will hopefully be unnecessary.

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

impl<'a> ParseError<Span<'a>> for Error<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::NomError(nom::error::Error::new(input, kind))
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        // TO DO: Fix or remove.
        other
    }
}
