use crate::Span;

#[test]
#[should_panic]
fn incomplete() {
    // Silly test to cover the incomplete case.

    let incomplete: nom::Err<nom::error::Error<Span<'static>>> =
        nom::Err::Incomplete(nom::Needed::Unknown);

    let _panic: crate::Error<'static> = incomplete.into();
}

#[test]
fn from_error_kind() {
    use nom::error::ParseError;

    let span: Span = Span::new("abc");
    let _err = crate::Error::from_error_kind(span, nom::error::ErrorKind::Eof);
    // nom's error types don't implement Eq, so … 🤷🏻‍♂️
    // This test case is likely temporary while we explore dropping
    // the dependency on nom altogether.
}

#[test]
fn append() {
    use nom::error::ParseError;

    let span1: Span = Span::new("abc");
    let err1 = crate::Error::from_error_kind(span1, nom::error::ErrorKind::Eof);

    let span2: Span = Span::new("ghi");
    let _err2 = crate::Error::append(span2, nom::error::ErrorKind::Eof, err1);
    // nom's error types don't implement Eq, so … 🤷🏻‍♂️
    // This test case is likely temporary while we explore dropping
    // the dependency on nom altogether.
}
