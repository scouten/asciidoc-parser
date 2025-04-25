use pretty_assertions_sorted::assert_eq;

use crate::{
    document::Header,
    tests::fixtures::{
        document::{TAttribute, THeader, TRawAttributeValue},
        TSpan,
    },
    Parser, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();

    let h1 = Header::parse(Span::new("= Title"), &mut parser).unwrap_if_no_warnings();
    let h2 = h1.clone();

    assert_eq!(h1, h2);
}

#[test]
fn only_title() {
    let mut parser = Parser::default();

    let mi = Header::parse(Span::new("= Just the Title"), &mut parser).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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
        mi.after,
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
    let mut parser = Parser::default();

    let mi = Header::parse(Span::new("=    Just the Title"), &mut parser).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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
        mi.after,
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
    let mut parser = Parser::default();

    let mi = Header::parse(Span::new("= Just the Title   "), &mut parser).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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
        mi.after,
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
    let mut parser = Parser::default();

    let mi = Header::parse(
        Span::new("= Just the Title\n:foo: bar\n\nblah"),
        &mut parser,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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
                    data: ":foo: bar",
                    line: 2,
                    col: 1,
                    offset: 17,
                }
            }],
            source: TSpan {
                data: "= Just the Title\n:foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 4,
            col: 1,
            offset: 28
        }
    );
}

#[test]
fn attribute_without_title() {
    let mut parser = Parser::default();

    let mi = Header::parse(Span::new(":foo: bar\n\nblah"), &mut parser).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        THeader {
            title: None,
            attributes: vec![TAttribute {
                name: TSpan {
                    data: "foo",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value: TRawAttributeValue::Value(TSpan {
                    data: "bar",
                    line: 1,
                    col: 7,
                    offset: 6,
                }),
                source: TSpan {
                    data: ":foo: bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            }],
            source: TSpan {
                data: ":foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 3,
            col: 1,
            offset: 11
        }
    );
}
