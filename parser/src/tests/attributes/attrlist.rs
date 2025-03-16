use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::Attrlist,
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = Attrlist::parse(Span::new("abc")).unwrap_if_no_warnings();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn empty_source() {
    let mi = Attrlist::parse(Span::new("")).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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

    assert!(mi.item.named_attribute("foo").is_none());

    assert!(mi.item.nth_attribute(0).is_none());
    assert!(mi.item.nth_attribute(1).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());
    assert!(mi.item.named_or_positional_attribute("foo", 1).is_none());
    assert!(mi.item.named_or_positional_attribute("foo", 42).is_none());

    assert!(mi.item.id().is_none());

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
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
    let mi = Attrlist::parse(Span::new("Sunset,300,400")).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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

    assert!(mi.item.named_attribute("foo").is_none());
    assert!(mi.item.nth_attribute(0).is_none());
    assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

    assert!(mi.item.id().is_none());

    assert_eq!(
        mi.item.nth_attribute(1).unwrap(),
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
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
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
        mi.item.nth_attribute(2).unwrap(),
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
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
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
        mi.item.nth_attribute(3).unwrap(),
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
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
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

    assert!(mi.item.nth_attribute(4).is_none());

    assert!(mi.item.named_or_positional_attribute("height", 4).is_none());

    assert!(mi.item.nth_attribute(42).is_none());

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "Sunset,300,400",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
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
    let mi = Attrlist::parse(Span::new("alt=Sunset,width=300,height=400")).unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
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

    assert!(mi.item.named_attribute("foo").is_none());
    assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

    assert_eq!(
        mi.item.named_attribute("alt").unwrap(),
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
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
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
        mi.item.named_attribute("width").unwrap(),
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
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
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
        mi.item.named_attribute("height").unwrap(),
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
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
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

    assert!(mi.item.nth_attribute(0).is_none());
    assert!(mi.item.nth_attribute(1).is_none());
    assert!(mi.item.nth_attribute(2).is_none());
    assert!(mi.item.nth_attribute(3).is_none());
    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert!(mi.item.id().is_none());

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "alt=Sunset,width=300,height=400",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 32,
            offset: 31
        }
    );
}

#[test]
fn err_unparsed_remainder_after_value() {
    let maw = Attrlist::parse(Span::new("alt=\"Sunset\"width=300"));

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
        TAttrlist {
            attributes: vec!(TElementAttribute {
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
                    col: 6,
                    offset: 5,
                },
                source: TSpan {
                    data: "alt=\"Sunset\"",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            },),
            source: TSpan {
                data: "alt=\"Sunset\"width=300",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 22,
            offset: 21
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "width=300",
                line: 1,
                col: 13,
                offset: 12,
            },
            warning: WarningType::MissingCommaAfterQuotedAttributeValue,
        }]
    );
}

#[test]
fn propagates_error_from_element_attribute() {
    let maw = Attrlist::parse(Span::new("foo%#id"));

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
        TAttrlist {
            attributes: vec!(TElementAttribute {
                name: None,
                shorthand_items: vec![
                    TSpan {
                        data: "foo",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    TSpan {
                        data: "#id",
                        line: 1,
                        col: 5,
                        offset: 4,
                    },
                ],
                value: TSpan {
                    data: "foo%#id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "foo%#id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            },),
            source: TSpan {
                data: "foo%#id",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 8,
            offset: 7
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "%",
                line: 1,
                col: 4,
                offset: 3,
            },
            warning: WarningType::EmptyShorthandItem,
        }]
    );
}

mod id {
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
    fn via_shorthand_syntax() {
        let mi = Attrlist::parse(Span::new("#goals")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "#goals",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }],
                    value: TSpan {
                        data: "#goals",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    source: TSpan {
                        data: "#goals",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },),
                source: TSpan {
                    data: "#goals",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());
        assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

        assert_eq!(
            mi.item.id().unwrap(),
            TSpan {
                data: "goals",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "#goals",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn via_named_attribute() {
        let mi = Attrlist::parse(Span::new("foo=bar,id=goals")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "bar",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        source: TSpan {
                            data: "foo=bar",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "id",
                            line: 1,
                            col: 9,
                            offset: 8,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "goals",
                            line: 1,
                            col: 12,
                            offset: 11,
                        },
                        source: TSpan {
                            data: "id=goals",
                            line: 1,
                            col: 9,
                            offset: 8,
                        },
                    },
                ),
                source: TSpan {
                    data: "foo=bar,id=goals",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "foo",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "bar",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                source: TSpan {
                    data: "foo=bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            mi.item.named_attribute("id").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "id",
                    line: 1,
                    col: 9,
                    offset: 8,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "goals",
                    line: 1,
                    col: 12,
                    offset: 11,
                },
                source: TSpan {
                    data: "id=goals",
                    line: 1,
                    col: 9,
                    offset: 8,
                },
            }
        );

        assert_eq!(
            mi.item.id().unwrap(),
            TSpan {
                data: "goals",
                line: 1,
                col: 12,
                offset: 11,
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
    #[should_panic]
    fn via_block_anchor_syntax() {
        let _pr = Attrlist::parse(Span::new("[goals]")).unwrap_if_no_warnings();

        // TO DO (#122): Parse block anchor syntax
    }

    #[test]
    fn shorthand_only_first_attribute() {
        let mi = Attrlist::parse(Span::new("foo,blah#goals")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        }],
                        value: TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        source: TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "blah#goals",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        source: TSpan {
                            data: "blah#goals",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                    },
                ),
                source: TSpan {
                    data: "foo,blah#goals",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.id().is_none());

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 15,
                offset: 14
            }
        );
    }
}

mod roles {
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
    fn via_shorthand_syntax() {
        let mi = Attrlist::parse(Span::new(".rolename")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: ".rolename",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }],
                    value: TSpan {
                        data: ".rolename",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    source: TSpan {
                        data: ".rolename",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },),
                source: TSpan {
                    data: ".rolename",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());
        assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "rolename",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".rolename",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 10,
                offset: 9
            }
        );
    }

    #[test]
    fn multiple_roles_via_shorthand_syntax() {
        let mi = Attrlist::parse(Span::new(".role1.role2.role3")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
                    shorthand_items: vec![
                        TSpan {
                            data: ".role1",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        TSpan {
                            data: ".role2",
                            line: 1,
                            col: 7,
                            offset: 6,
                        },
                        TSpan {
                            data: ".role3",
                            line: 1,
                            col: 13,
                            offset: 12,
                        }
                    ],
                    value: TSpan {
                        data: ".role1.role2.role3",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    source: TSpan {
                        data: ".role1.role2.role3",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },),
                source: TSpan {
                    data: ".role1.role2.role3",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());
        assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role1",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role2",
                line: 1,
                col: 8,
                offset: 7,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role3",
                line: 1,
                col: 14,
                offset: 13,
            }
        );

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".role1.role2.role3",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 19,
                offset: 18
            }
        );
    }

    #[test]
    fn via_named_attribute() {
        let mi = Attrlist::parse(Span::new("foo=bar,role=role1")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "bar",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        source: TSpan {
                            data: "foo=bar",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "role",
                            line: 1,
                            col: 9,
                            offset: 8,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "role1",
                            line: 1,
                            col: 14,
                            offset: 13,
                        },
                        source: TSpan {
                            data: "role=role1",
                            line: 1,
                            col: 9,
                            offset: 8,
                        },
                    },
                ),
                source: TSpan {
                    data: "foo=bar,role=role1",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "foo",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "bar",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                source: TSpan {
                    data: "foo=bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            mi.item.named_attribute("role").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "role",
                    line: 1,
                    col: 9,
                    offset: 8,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "role1",
                    line: 1,
                    col: 14,
                    offset: 13,
                },
                source: TSpan {
                    data: "role=role1",
                    line: 1,
                    col: 9,
                    offset: 8,
                },
            }
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role1",
                line: 1,
                col: 14,
                offset: 13,
            }
        );

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 19,
                offset: 18
            }
        );
    }

    #[test]
    fn multiple_roles_via_named_attribute() {
        let mi =
            Attrlist::parse(Span::new("foo=bar,role=role1 role2   role3 ")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "bar",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        source: TSpan {
                            data: "foo=bar",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "role",
                            line: 1,
                            col: 9,
                            offset: 8,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "role1 role2   role3 ",
                            line: 1,
                            col: 14,
                            offset: 13,
                        },
                        source: TSpan {
                            data: "role=role1 role2   role3 ",
                            line: 1,
                            col: 9,
                            offset: 8,
                        },
                    },
                ),
                source: TSpan {
                    data: "foo=bar,role=role1 role2   role3 ",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "foo",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "bar",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                source: TSpan {
                    data: "foo=bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            mi.item.named_attribute("role").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "role",
                    line: 1,
                    col: 9,
                    offset: 8,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "role1 role2   role3 ",
                    line: 1,
                    col: 14,
                    offset: 13,
                },
                source: TSpan {
                    data: "role=role1 role2   role3 ",
                    line: 1,
                    col: 9,
                    offset: 8,
                },
            }
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role1",
                line: 1,
                col: 14,
                offset: 13,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role2",
                line: 1,
                col: 20,
                offset: 19,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "role3",
                line: 1,
                col: 28,
                offset: 27,
            }
        );

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 34,
                offset: 33
            }
        );
    }

    #[test]
    fn shorthand_role_and_named_attribute_role() {
        let mi =
            Attrlist::parse(Span::new("#foo.sh1.sh2,role=na1 na2   na3 ")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![
                            TSpan {
                                data: "#foo",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            TSpan {
                                data: ".sh1",
                                line: 1,
                                col: 5,
                                offset: 4,
                            },
                            TSpan {
                                data: ".sh2",
                                line: 1,
                                col: 9,
                                offset: 8,
                            }
                        ],
                        value: TSpan {
                            data: "#foo.sh1.sh2",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        source: TSpan {
                            data: "#foo.sh1.sh2",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "role",
                            line: 1,
                            col: 14,
                            offset: 13,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "na1 na2   na3 ",
                            line: 1,
                            col: 19,
                            offset: 18,
                        },
                        source: TSpan {
                            data: "role=na1 na2   na3 ",
                            line: 1,
                            col: 14,
                            offset: 13,
                        },
                    },
                ),
                source: TSpan {
                    data: "#foo.sh1.sh2,role=na1 na2   na3 ",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none(),);

        assert_eq!(
            mi.item.named_attribute("role").unwrap(),
            TElementAttribute {
                name: Some(TSpan {
                    data: "role",
                    line: 1,
                    col: 14,
                    offset: 13,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "na1 na2   na3 ",
                    line: 1,
                    col: 19,
                    offset: 18,
                },
                source: TSpan {
                    data: "role=na1 na2   na3 ",
                    line: 1,
                    col: 14,
                    offset: 13,
                },
            }
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "sh1",
                line: 1,
                col: 6,
                offset: 5,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "sh2",
                line: 1,
                col: 10,
                offset: 9,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "na1",
                line: 1,
                col: 19,
                offset: 18,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "na2",
                line: 1,
                col: 23,
                offset: 22,
            }
        );

        assert_eq!(
            roles.next().unwrap(),
            TSpan {
                data: "na3",
                line: 1,
                col: 29,
                offset: 28,
            }
        );

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 33,
                offset: 32
            }
        );
    }

    #[test]
    fn shorthand_only_first_attribute() {
        let mi = Attrlist::parse(Span::new("foo,blah.rolename")).unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        }],
                        value: TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        source: TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "blah.rolename",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        source: TSpan {
                            data: "blah.rolename",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                    },
                ),
                source: TSpan {
                    data: "foo,blah.rolename",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        let roles = mi.item.roles();
        assert_eq!(roles.iter().len(), 0);

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 18,
                offset: 17
            }
        );
    }
}

#[test]
fn err_double_comma() {
    let maw = Attrlist::parse(Span::new("alt=Sunset,width=300,,height=400"));

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
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
                        col: 23,
                        offset: 22,
                    },),
                    shorthand_items: vec![],
                    value: TSpan {
                        data: "400",
                        line: 1,
                        col: 30,
                        offset: 29,
                    },
                    source: TSpan {
                        data: "height=400",
                        line: 1,
                        col: 23,
                        offset: 22,
                    },
                },
            ),
            source: TSpan {
                data: "alt=Sunset,width=300,,height=400",
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
            col: 33,
            offset: 32,
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: ",",
                line: 1,
                col: 21,
                offset: 20,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}
