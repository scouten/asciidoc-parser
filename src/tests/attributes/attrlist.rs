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
    let b1 = Attrlist::parse(Span::new("abc")).unwrap();
    let b2 = b1.t.clone();
    assert_eq!(b1.t, b2);
}

#[test]
fn empty_source() {
    let pr = Attrlist::parse(Span::new("")).unwrap();

    assert_eq!(
        pr.t,
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

    assert!(pr.t.named_attribute("foo").is_none());

    assert!(pr.t.nth_attribute(0).is_none());
    assert!(pr.t.nth_attribute(1).is_none());
    assert!(pr.t.nth_attribute(42).is_none());

    assert!(pr.t.named_or_positional_attribute("foo", 0).is_none());
    assert!(pr.t.named_or_positional_attribute("foo", 1).is_none());
    assert!(pr.t.named_or_positional_attribute("foo", 42).is_none());

    assert_eq!(
        pr.t.span(),
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0
        }
    );
}

#[test]
fn only_positional_attributes() {
    let pr = Attrlist::parse(Span::new("Sunset,300,400")).unwrap();

    assert_eq!(
        pr.t,
        TAttrlist {
            attributes: vec!(
                TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "Sunset",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }],
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
                    shorthand_items: vec![],
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
                    shorthand_items: vec![],
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

    assert!(pr.t.named_attribute("foo").is_none());
    assert!(pr.t.nth_attribute(0).is_none());
    assert!(pr.t.named_or_positional_attribute("foo", 0).is_none());

    assert_eq!(
        pr.t.nth_attribute(1).unwrap(),
        TElementAttribute {
            name: None,
            shorthand_items: vec![TSpan {
                data: "Sunset",
                line: 1,
                col: 1,
                offset: 0,
            }],
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
        pr.t.named_or_positional_attribute("alt", 1).unwrap(),
        TElementAttribute {
            name: None,
            shorthand_items: vec![TSpan {
                data: "Sunset",
                line: 1,
                col: 1,
                offset: 0,
            }],
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
        pr.t.nth_attribute(2).unwrap(),
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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
        pr.t.named_or_positional_attribute("width", 2).unwrap(),
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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
        pr.t.nth_attribute(3).unwrap(),
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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
        pr.t.named_or_positional_attribute("height", 3).unwrap(),
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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

    assert!(pr.t.nth_attribute(4).is_none());

    assert!(pr.t.named_or_positional_attribute("height", 4).is_none());

    assert!(pr.t.nth_attribute(42).is_none());

    assert_eq!(
        pr.t.span(),
        TSpan {
            data: "Sunset,300,400",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 15,
            offset: 14
        }
    );
}

#[test]
fn only_named_attributes() {
    let pr = Attrlist::parse(Span::new("alt=Sunset,width=300,height=400")).unwrap();

    assert_eq!(
        pr.t,
        TAttrlist {
            attributes: vec!(
                TElementAttribute {
                    name: Some(TSpan {
                        data: "alt",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },),
                    shorthand_items: vec![],
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
                    shorthand_items: vec![],
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
                    shorthand_items: vec![],
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

    assert!(pr.t.named_attribute("foo").is_none());
    assert!(pr.t.named_or_positional_attribute("foo", 0).is_none());

    assert_eq!(
        pr.t.named_attribute("alt").unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "alt",
                line: 1,
                col: 1,
                offset: 0,
            },),
            shorthand_items: vec![],
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
        pr.t.named_or_positional_attribute("alt", 1).unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "alt",
                line: 1,
                col: 1,
                offset: 0,
            },),
            shorthand_items: vec![],
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
        pr.t.named_attribute("width").unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "width",
                line: 1,
                col: 12,
                offset: 11,
            },),
            shorthand_items: vec![],
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
        pr.t.named_or_positional_attribute("width", 2).unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "width",
                line: 1,
                col: 12,
                offset: 11,
            },),
            shorthand_items: vec![],
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
        pr.t.named_attribute("height").unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "height",
                line: 1,
                col: 22,
                offset: 21,
            },),
            shorthand_items: vec![],
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
        pr.t.named_or_positional_attribute("height", 3).unwrap(),
        TElementAttribute {
            name: Some(TSpan {
                data: "height",
                line: 1,
                col: 22,
                offset: 21,
            },),
            shorthand_items: vec![],
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

    assert!(pr.t.nth_attribute(0).is_none());
    assert!(pr.t.nth_attribute(1).is_none());
    assert!(pr.t.nth_attribute(2).is_none());
    assert!(pr.t.nth_attribute(3).is_none());
    assert!(pr.t.nth_attribute(4).is_none());
    assert!(pr.t.nth_attribute(42).is_none());

    assert_eq!(
        pr.t.span(),
        TSpan {
            data: "alt=Sunset,width=300,height=400",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 32,
            offset: 31
        }
    );
}

#[test]
fn err_double_comma() {
    assert!(Attrlist::parse(Span::new("alt=Sunset,width=300,,height=400")).is_none());
}
