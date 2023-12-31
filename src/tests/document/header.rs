use pretty_assertions_sorted::assert_eq;

use crate::{
    document::Header,
    tests::fixtures::{document::THeader, TSpan},
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let h1 = Header::parse(Span::new("= Title", true)).unwrap();
    let h2 = h1.clone();
    assert_eq!(h1, h2);
}

#[test]
fn only_title() {
    let (rem, block) = Header::parse(Span::new("= Just the Title", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 17,
            offset: 16
        }
    );

    assert_eq!(
        block,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            source: TSpan {
                data: "= Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );
}

#[test]
fn trims_leading_spaces_in_title() {
    // This is totally a judgement call on my part. As far as I can tell,
    // the language doesn't describe behavior here.
    let (rem, block) = Header::parse(Span::new("=    Just the Title", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 20,
            offset: 19
        }
    );

    assert_eq!(
        block,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 6,
                offset: 5,
            }),
            source: TSpan {
                data: "=    Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );
}

#[test]
fn trims_trailing_spaces_in_title() {
    let (rem, block) = Header::parse(Span::new("= Just the Title   ", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 20,
            offset: 19
        }
    );

    assert_eq!(
        block,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            source: TSpan {
                data: "= Just the Title   ",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );
}
