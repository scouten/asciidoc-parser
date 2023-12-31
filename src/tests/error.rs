use crate::{tests::fixtures::TSpan, Span};

#[test]
fn incomplete() {
    // Silly test to cover the incomplete case.

    let incomplete: nom::Err<nom::error::Error<Span<'static>>> =
        nom::Err::Error(nom::error::Error {
            input: Span::new("abc", true),
            code: nom::error::ErrorKind::NonEmpty,
        });

    let e: crate::Error<'static> = incomplete.into();

    // Convert to `if let` when we add other error kinds.
    let crate::Error::NomError(ne) = e;

    assert_eq!(
        ne.input,
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(ne.code, nom::error::ErrorKind::NonEmpty);
}
