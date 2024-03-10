use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::Attrlist,
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        TSpan,
    },
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let (_, b1) = Attrlist::parse(Span::new("abc", true)).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let (rem, attrlist) = Attrlist::parse(Span::new("", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    assert_eq!(
        attrlist,
        TAttrlist {
            attributes: vec!(),
            source: TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert!(attrlist.named_attribute("foo").is_none());

    assert!(attrlist.nth_attribute(0).is_none());
    assert!(attrlist.nth_attribute(1).is_none());
    assert!(attrlist.nth_attribute(42).is_none());

    assert!(attrlist.named_or_positional_attribute("foo", 0).is_none());
    assert!(attrlist.named_or_positional_attribute("foo", 1).is_none());
    assert!(attrlist.named_or_positional_attribute("foo", 42).is_none());

    assert_eq!(
        attrlist.span(),
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}

#[test]
fn only_positional_attributes() {
    let (rem, attrlist) = Attrlist::parse(Span::new("Sunset,300,400", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 15,
            offset: 14
        }
    );

    assert_eq!(
        attrlist,
        TAttrlist {
            attributes: vec!(
                TElementAttribute {
                    name: None,
                    value: TSpan {
                        data: "Sunset",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    source: TSpan {
                        data: "Sunset",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                TElementAttribute {
                    name: None,
                    value: TSpan {
                        data: "300",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    source: TSpan {
                        data: "300",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                },
                TElementAttribute {
                    name: None,
                    value: TSpan {
                        data: "400",
                        line: 1,
                        col: 12,
                        offset: 11,
                    },
                    source: TSpan {
                        data: "400",
                        line: 1,
                        col: 12,
                        offset: 11,
                    },
                }
            ),
            source: TSpan {
                data: "Sunset,300,400",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert!(attrlist.named_attribute("foo").is_none());
    assert!(attrlist.nth_attribute(0).is_none());
    assert!(attrlist.named_or_positional_attribute("foo", 0).is_none());

    assert_eq!(
        attrlist.nth_attribute(1).unwrap(),
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "Sunset",
                line: 1,
                col: 1,
                offset: 0,
            },
            source: TSpan {
                data: "Sunset",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attrlist.named_or_positional_attribute("alt", 1).unwrap(),
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "Sunset",
                line: 1,
                col: 1,
                offset: 0,
            },
            source: TSpan {
                data: "Sunset",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attrlist.nth_attribute(2).unwrap(),
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "300",
                line: 1,
                col: 8,
                offset: 7,
            },
            source: TSpan {
                data: "300",
                line: 1,
                col: 8,
                offset: 7,
            },
        }
    );

    assert_eq!(
        attrlist.named_or_positional_attribute("width", 2).unwrap(),
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "300",
                line: 1,
                col: 8,
                offset: 7,
            },
            source: TSpan {
                data: "300",
                line: 1,
                col: 8,
                offset: 7,
            },
        }
    );

    assert_eq!(
        attrlist.nth_attribute(3).unwrap(),
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "400",
                line: 1,
                col: 12,
                offset: 11,
            },
            source: TSpan {
                data: "400",
                line: 1,
                col: 12,
                offset: 11,
            },
        }
    );

    assert_eq!(
        attrlist.named_or_positional_attribute("height", 3).unwrap(),
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "400",
                line: 1,
                col: 12,
                offset: 11,
            },
            source: TSpan {
                data: "400",
                line: 1,
                col: 12,
                offset: 11,
            },
        }
    );

    assert!(attrlist.nth_attribute(4).is_none());

    assert!(attrlist
        .named_or_positional_attribute("height", 4)
        .is_none());

    assert!(attrlist.nth_attribute(42).is_none());

    assert_eq!(
        attrlist.span(),
        TSpan {
            data: "Sunset,300,400",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}

#[test]
fn only_named_attributes() {
    let (rem, attrlist) =
        Attrlist::parse(Span::new("alt=Sunset,width=300,height=400", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 32,
            offset: 31
        }
    );

    assert_eq!(
        attrlist,
        TAttrlist {
            attributes: vec!(
                TElementAttribute {
                    name: Some(TSpan {
                        data: "alt",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },),
                    value: TSpan {
                        data: "Sunset",
                        line: 1,
                        col: 5,
                        offset: 4,
                    },
                    source: TSpan {
                        data: "alt=Sunset",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                TElementAttribute {
                    name: Some(TSpan {
                        data: "width",
                        line: 1,
                        col: 12,
                        offset: 11,
                    },),
                    value: TSpan {
                        data: "300",
                        line: 1,
                        col: 18,
                        offset: 17,
                    },
                    source: TSpan {
                        data: "width=300",
                        line: 1,
                        col: 12,
                        offset: 11,
                    },
                },
                TElementAttribute {
                    name: Some(TSpan {
                        data: "height",
                        line: 1,
                        col: 22,
                        offset: 21,
                    },),
                    value: TSpan {
                        data: "400",
                        line: 1,
                        col: 29,
                        offset: 28,
                    },
                    source: TSpan {
                        data: "height=400",
                        line: 1,
                        col: 22,
                        offset: 21,
                    },
                }
            ),
            source: TSpan {
                data: "alt=Sunset,width=300,height=400",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert!(attrlist.named_attribute("foo").is_none());
    assert!(attrlist.named_or_positional_attribute("foo", 0).is_none());

    assert_eq!(
        attrlist.named_attribute("alt").unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "alt",
                line: 1,
                col: 1,
                offset: 0,
            },),
            value: TSpan {
                data: "Sunset",
                line: 1,
                col: 5,
                offset: 4,
            },
            source: TSpan {
                data: "alt=Sunset",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attrlist.named_or_positional_attribute("alt", 1).unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "alt",
                line: 1,
                col: 1,
                offset: 0,
            },),
            value: TSpan {
                data: "Sunset",
                line: 1,
                col: 5,
                offset: 4,
            },
            source: TSpan {
                data: "alt=Sunset",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attrlist.named_attribute("width").unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "width",
                line: 1,
                col: 12,
                offset: 11,
            },),
            value: TSpan {
                data: "300",
                line: 1,
                col: 18,
                offset: 17,
            },
            source: TSpan {
                data: "width=300",
                line: 1,
                col: 12,
                offset: 11,
            },
        }
    );

    assert_eq!(
        attrlist.named_or_positional_attribute("width", 2).unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "width",
                line: 1,
                col: 12,
                offset: 11,
            },),
            value: TSpan {
                data: "300",
                line: 1,
                col: 18,
                offset: 17,
            },
            source: TSpan {
                data: "width=300",
                line: 1,
                col: 12,
                offset: 11,
            },
        }
    );

    assert_eq!(
        attrlist.named_attribute("height").unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "height",
                line: 1,
                col: 22,
                offset: 21,
            },),
            value: TSpan {
                data: "400",
                line: 1,
                col: 29,
                offset: 28,
            },
            source: TSpan {
                data: "height=400",
                line: 1,
                col: 22,
                offset: 21,
            },
        }
    );

    assert_eq!(
        attrlist.named_or_positional_attribute("height", 3).unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "height",
                line: 1,
                col: 22,
                offset: 21,
            },),
            value: TSpan {
                data: "400",
                line: 1,
                col: 29,
                offset: 28,
            },
            source: TSpan {
                data: "height=400",
                line: 1,
                col: 22,
                offset: 21,
            },
        }
    );

    assert!(attrlist.nth_attribute(0).is_none());
    assert!(attrlist.nth_attribute(1).is_none());
    assert!(attrlist.nth_attribute(2).is_none());
    assert!(attrlist.nth_attribute(3).is_none());
    assert!(attrlist.nth_attribute(4).is_none());
    assert!(attrlist.nth_attribute(42).is_none());

    assert_eq!(
        attrlist.span(),
        TSpan {
            data: "alt=Sunset,width=300,height=400",
            line: 1,
            col: 1,
            offset: 0
        }
    );
}
