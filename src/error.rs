use nom::{
    error::{ErrorKind, FromExternalError, ParseError},
    IResult,
};

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
}

impl<'a> ParseError<&'a str> for Error {
    fn from_error_kind(_input: &'a str, kind: ErrorKind) -> Self {
        Error::NomError(kind)
    }

    fn append(_input: &'a str, kind: ErrorKind, _other: Self) -> Self {
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

impl<I, E> FromExternalError<I, E> for Error {
    fn from_external_error(_input: I, kind: ErrorKind, _e: E) -> Error {
        Error::NomError(kind)
    }
}

/// Holds the result of AsciiDoc parsing functions.
///
/// Note that this type is also a [`Result`], so the usual functions (`map`,
/// `unwrap`, etc.) are available.
#[allow(dead_code)] // TEMPORARY
pub type ParseResult<'a, T, E = Error> = IResult<&'a str, T, E>;
