use crate::Span;

#[test]
#[should_panic]
fn incomplete() {
    // Silly test to cover the incomplete case.

    let incomplete: nom::Err<nom::error::Error<Span<'static>>> =
        nom::Err::Incomplete(nom::Needed::Unknown);

    let _panic: crate::Error<'static> = incomplete.into();
}
