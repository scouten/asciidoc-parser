use nom::{
    error::{ErrorKind, ParseError},
    IResult,
};
use nom_span::Spanned;

/// The error type for AsciiDoc parsing operations.
#[non_exhaustive]
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    /// AsciiDoc data was incomplete.
    #[error("Incomplete data, missing: {0:?}")]
    Incomplete(nom::Needed),

    /// Error from nom parsing framework.
    #[error("nom error: {0:?}")]
    NomError(ErrorKind),

    /// Error with location info.
    #[error("temporary error from nom: {0:?}")]
    TemporaryError(String),
}

impl<'a> ParseError<Spanned<&'a str>> for Error {
    fn from_error_kind(_input: Spanned<&'a str>, kind: ErrorKind) -> Self {
        Error::NomError(kind)
    }

    fn append(_input: Spanned<&'a str>, kind: ErrorKind, _other: Self) -> Self {
        Error::NomError(kind)
    }
}

impl From<nom::Err<Error>> for Error {
    fn from(e: nom::Err<Error>) -> Self {
        match e {
            nom::Err::Incomplete(n) => Self::Incomplete(n),
            nom::Err::Error(e) | nom::Err::Failure(e) => e,
        }
    }
}

impl From<nom::Err<nom::error::Error<Spanned<&str>>>> for Error {
    fn from(e: nom::Err<nom::error::Error<Spanned<&str>>>) -> Self {
        match e {
            nom::Err::Incomplete(n) => Self::Incomplete(n),
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                Self::TemporaryError(format!("TEMPORARY: {e:#?}"))
            } // TO DO: Find better solution for error lifetime issues.
        }
    }
}

/// Holds the result of AsciiDoc parsing functions.
///
/// Note that this type is also a [`Result`], so the usual functions (`map`,
/// `unwrap`, etc.) are available.
#[allow(dead_code)] // TEMPORARY
pub type ParseResult<'a, T, E = Error> = IResult<&'a str, T, E>;
