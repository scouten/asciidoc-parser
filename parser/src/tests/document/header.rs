use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    tests::fixtures::{
        Span,
        document::{Attribute, Header, InterpretedValue},
    },
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();

    let h1 = crate::document::Header::parse(crate::Span::new("= Title"), &mut parser)
        .unwrap_if_no_warnings();
    let h2 = h1.clone();

    assert_eq!(h1, h2);
}

#[test]
fn only_title() {
    let mut parser = Parser::default();
    let mi = crate::document::Header::parse(crate::Span::new("= Just the Title"), &mut parser)
        .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Header {
            title_source: Some(Span {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            title: Some("Just the Title"),
            attributes: &[],
            source: Span {
                data: "= Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
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
    let mi = crate::document::Header::parse(crate::Span::new("=    Just the Title"), &mut parser)
        .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Header {
            title_source: Some(Span {
                data: "Just the Title",
                line: 1,
                col: 6,
                offset: 5,
            }),
            title: Some("Just the Title"),
            attributes: &[],
            source: Span {
                data: "=    Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
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
    let mi = crate::document::Header::parse(crate::Span::new("= Just the Title   "), &mut parser)
        .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Header {
            title_source: Some(Span {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            title: Some("Just the Title"),
            attributes: &[],
            source: Span {
                data: "= Just the Title",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
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

    let mi = crate::document::Header::parse(
        crate::Span::new("= Just the Title\n:foo: bar\n\nblah"),
        &mut parser,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Header {
            title_source: Some(Span {
                data: "Just the Title",
                line: 1,
                col: 3,
                offset: 2,
            }),
            title: Some("Just the Title"),
            attributes: &[Attribute {
                name: Span {
                    data: "foo",
                    line: 2,
                    col: 2,
                    offset: 18,
                },
                value_source: Some(Span {
                    data: "bar",
                    line: 2,
                    col: 7,
                    offset: 23,
                }),
                value: InterpretedValue::Value("bar"),
                source: Span {
                    data: ":foo: bar",
                    line: 2,
                    col: 1,
                    offset: 17,
                }
            }],
            source: Span {
                data: "= Just the Title\n:foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 4,
            col: 1,
            offset: 28
        }
    );
}

#[test]
fn title_applies_header_substitutions() {
    let mut parser = Parser::default();

    let mi = crate::document::Header::parse(
        crate::Span::new("= The Title & Some{sp}Nonsense\n:foo: bar\n\nblah"),
        &mut parser,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Header {
            title_source: Some(Span {
                data: "The Title & Some{sp}Nonsense",
                line: 1,
                col: 3,
                offset: 2,
            }),
            title: Some("The Title &amp; Some Nonsense"),
            attributes: &[Attribute {
                name: Span {
                    data: "foo",
                    line: 2,
                    col: 2,
                    offset: 32,
                },
                value_source: Some(Span {
                    data: "bar",
                    line: 2,
                    col: 7,
                    offset: 37,
                }),
                value: InterpretedValue::Value("bar"),
                source: Span {
                    data: ":foo: bar",
                    line: 2,
                    col: 1,
                    offset: 31,
                }
            }],
            source: Span {
                data: "= The Title & Some{sp}Nonsense\n:foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 4,
            col: 1,
            offset: 42
        }
    );
}

#[test]
fn attribute_without_title() {
    let mut parser = Parser::default();
    let mi = crate::document::Header::parse(crate::Span::new(":foo: bar\n\nblah"), &mut parser)
        .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Header {
            title_source: None,
            title: None,
            attributes: &[Attribute {
                name: Span {
                    data: "foo",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: Some(Span {
                    data: "bar",
                    line: 1,
                    col: 7,
                    offset: 6,
                }),
                value: InterpretedValue::Value("bar"),
                source: Span {
                    data: ":foo: bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            }],
            source: Span {
                data: ":foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 3,
            col: 1,
            offset: 11
        }
    );
}
