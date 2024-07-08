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
