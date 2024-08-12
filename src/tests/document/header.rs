use pretty_assertions_sorted::assert_eq;

use crate::{
    document::Header,
    tests::fixtures::{
        document::{TAttribute, THeader, TRawAttributeValue},
        TSpan,
    },
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let h1 = Header::parse(Span::new("= Title")).unwrap();
    let h2 = h1.clone();
    assert_eq!(h1, h2);
}

#[test]
fn only_title() {
    let pr = Header::parse(Span::new("= Just the Title")).unwrap();

    assert_eq!(
        pr.t,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            attributes: vec![],
            source: TSpan {
                data: "= Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 17,
            offset: 16
        }
    );
}

#[test]
fn trims_leading_spaces_in_title() {
    // This is totally a judgement call on my part. As far as I can tell,
    // the language doesn't describe behavior here.
    let pr = Header::parse(Span::new("=    Just the Title")).unwrap();

    assert_eq!(
        pr.t,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 6,
                offset: 5,
            }),
            attributes: vec![],
            source: TSpan {
                data: "=    Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 20,
            offset: 19
        }
    );
}

#[test]
fn trims_trailing_spaces_in_title() {
    let pr = Header::parse(Span::new("= Just the Title   ")).unwrap();

    assert_eq!(
        pr.t,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            attributes: vec![],
            source: TSpan {
                data: "= Just the Title   ",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 20,
            offset: 19
        }
    );
}

#[test]
fn title_and_attribute() {
    let pr = Header::parse(Span::new("= Just the Title\n:foo: bar\n\nblah")).unwrap();

    assert_eq!(
        pr.t,
        THeader {
            title: Some(TSpan {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            attributes: vec![TAttribute {
                name: TSpan {
                    data: "foo",
                    line: 2,
                    col: 2,
                    offset: 18,
                },
                value: TRawAttributeValue::Value(TSpan {
                    data: "bar",
                    line: 2,
                    col: 7,
                    offset: 23,
                }),
                source: TSpan {
                    data: ":foo: bar\n",
                    line: 2,
                    col: 1,
                    offset: 17,
                }
            }],
            source: TSpan {
                data: "= Just the Title\n:foo: bar\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "blah",
            line: 4,
            col: 1,
            offset: 28
        }
    );
}
