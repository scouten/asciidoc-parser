use pretty_assertions_sorted::assert_eq;

use crate::{
    HasSpan, Parser, attributes::AttrlistContext, parser::ModificationContext, tests::prelude::*,
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let p = Parser::default();
    let b1 =
        crate::attributes::Attrlist::parse(crate::Span::new("abc"), &p, AttrlistContext::Inline)
            .unwrap_if_no_warnings();

    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn impl_default() {
    let attrlist = crate::attributes::Attrlist::default();

    assert_eq!(
        attrlist,
        Attrlist {
            attributes: &[],
            anchor: None,
            source: Span {
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

    assert!(attrlist.id().is_none());
    assert!(attrlist.roles().is_empty());

    assert_eq!(
        attrlist.span(),
        Span {
            data: "",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}

#[test]
fn empty_source() {
    let p = Parser::default();

    let mi =
        crate::attributes::Attrlist::parse(crate::Span::default(), &p, AttrlistContext::Inline)
            .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[],
            anchor: None,
            source: Span {
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
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 1,
            offset: 0
        }
    );
}

#[test]
fn empty_positional_attributes() {
    let p = Parser::default();

    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new(",300,400"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: ""
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "300"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "400"
                }
            ],
            anchor: None,
            source: Span {
                data: ",300,400",
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
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.nth_attribute(1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: ""
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: ""
        }
    );

    assert_eq!(
        mi.item.nth_attribute(2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.named_or_positional_attribute("height", 4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert_eq!(
        mi.item.span(),
        Span {
            data: ",300,400",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 9,
            offset: 8
        }
    );
}

#[test]
fn only_positional_attributes() {
    let p = Parser::default();

    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new("Sunset,300,400"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: None,
                    shorthand_items: &["Sunset"],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "300"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "400"
                }
            ],
            anchor: None,
            source: Span {
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
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.nth_attribute(1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.named_or_positional_attribute("height", 4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "Sunset,300,400",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 15,
            offset: 14
        }
    );
}

#[test]
fn trim_trailing_space() {
    let p = Parser::default();

    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new("Sunset ,300 , 400"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: None,
                    shorthand_items: &["Sunset"],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "300"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "400"
                }
            ],
            anchor: None,
            source: Span {
                data: "Sunset ,300 , 400",
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
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.nth_attribute(1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.named_or_positional_attribute("height", 4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "Sunset ,300 , 400",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 18,
            offset: 17
        }
    );
}

#[test]
fn only_named_attributes() {
    let p = Parser::default();

    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new("alt=Sunset,width=300,height=400"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: Some("alt"),
                    shorthand_items: &[],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: Some("width"),
                    shorthand_items: &[],
                    value: "300"
                },
                ElementAttribute {
                    name: Some("height"),
                    shorthand_items: &[],
                    value: "400"
                }
            ],
            anchor: None,
            source: Span {
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
        ElementAttribute {
            name: Some("alt"),
            shorthand_items: &[],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: Some("alt"),
            shorthand_items: &[],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_attribute("width").unwrap(),
        ElementAttribute {
            name: Some("width"),
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
        ElementAttribute {
            name: Some("width"),
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.named_attribute("height").unwrap(),
        ElementAttribute {
            name: Some("height"),
            shorthand_items: &[],
            value: "400"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
        ElementAttribute {
            name: Some("height"),
            shorthand_items: &[],
            value: "400"
        }
    );

    assert!(mi.item.nth_attribute(0).is_none());
    assert!(mi.item.nth_attribute(1).is_none());
    assert!(mi.item.nth_attribute(2).is_none());
    assert!(mi.item.nth_attribute(3).is_none());
    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "alt=Sunset,width=300,height=400",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 32,
            offset: 31
        }
    );
}

#[test]
fn ignore_named_attribute_with_none_value() {
    let p = Parser::default();
    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new("alt=Sunset,width=None,height=400"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: Some("alt"),
                    shorthand_items: &[],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: Some("height"),
                    shorthand_items: &[],
                    value: "400"
                }
            ],
            anchor: None,
            source: Span {
                data: "alt=Sunset,width=None,height=400",
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
        ElementAttribute {
            name: Some("alt"),
            shorthand_items: &[],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: Some("alt"),
            shorthand_items: &[],
            value: "Sunset"
        }
    );

    assert!(mi.item.named_attribute("width").is_none());
    assert!(mi.item.named_or_positional_attribute("width", 2).is_none());

    assert_eq!(
        mi.item.named_attribute("height").unwrap(),
        ElementAttribute {
            name: Some("height"),
            shorthand_items: &[],
            value: "400"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("height", 2).unwrap(),
        ElementAttribute {
            name: Some("height"),
            shorthand_items: &[],
            value: "400"
        }
    );

    assert!(mi.item.nth_attribute(0).is_none());
    assert!(mi.item.nth_attribute(1).is_none());
    assert!(mi.item.nth_attribute(2).is_none());
    assert!(mi.item.nth_attribute(3).is_none());
    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "alt=Sunset,width=None,height=400",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 33,
            offset: 32
        }
    );
}

#[test]
fn err_unparsed_remainder_after_value() {
    let p = Parser::default();

    let maw = crate::attributes::Attrlist::parse(
        crate::Span::new("alt=\"Sunset\"width=300"),
        &p,
        AttrlistContext::Inline,
    );

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[ElementAttribute {
                name: Some("alt"),
                shorthand_items: &[],
                value: "Sunset"
            }],
            anchor: None,
            source: Span {
                data: "alt=\"Sunset\"width=300",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 22,
            offset: 21
        }
    );

    assert_eq!(
        maw.warnings,
        vec![Warning {
            source: Span {
                data: "alt=\"Sunset\"width=300",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::MissingCommaAfterQuotedAttributeValue,
        }]
    );
}

#[test]
fn propagates_error_from_element_attribute() {
    let p = Parser::default();

    let maw = crate::attributes::Attrlist::parse(
        crate::Span::new("foo%#id"),
        &p,
        AttrlistContext::Inline,
    );

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[ElementAttribute {
                name: None,
                shorthand_items: &["foo", "#id"],
                value: "foo%#id"
            }],
            anchor: None,
            source: Span {
                data: "foo%#id",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 8,
            offset: 7
        }
    );

    assert_eq!(
        maw.warnings,
        vec![Warning {
            source: Span {
                data: "foo%#id",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::EmptyShorthandItem,
        }]
    );
}

#[test]
fn anchor_syntax() {
    let p = Parser::default();

    let maw = crate::attributes::Attrlist::parse(
        crate::Span::new("[notice]"),
        &p,
        AttrlistContext::Inline,
    );

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[],
            anchor: Some("notice"),
            source: Span {
                data: "[notice]",
                line: 1,
                col: 1,
                offset: 0
            }
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 9,
            offset: 8
        }
    );

    assert!(maw.warnings.is_empty());
}

mod id {
    use pretty_assertions_sorted::assert_eq;

    use crate::{HasSpan, Parser, attributes::AttrlistContext, tests::prelude::*};

    #[test]
    fn via_shorthand_syntax() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("#goals"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &["#goals"],
                    value: "#goals"
                }],
                anchor: None,
                source: Span {
                    data: "#goals",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());
        assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

        assert_eq!(mi.item.id().unwrap(), "goals");

        assert!(mi.item.roles().is_empty());

        assert_eq!(
            mi.item.span(),
            Span {
                data: "#goals",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn via_named_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo=bar,id=goals"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: Some("foo"),
                        shorthand_items: &[],
                        value: "bar"
                    },
                    ElementAttribute {
                        name: Some("id"),
                        shorthand_items: &[],
                        value: "goals"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo=bar,id=goals",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            ElementAttribute {
                name: Some("foo"),
                shorthand_items: &[],
                value: "bar"
            }
        );

        assert_eq!(
            mi.item.named_attribute("id").unwrap(),
            ElementAttribute {
                name: Some("id"),
                shorthand_items: &[],
                value: "goals"
            }
        );

        assert_eq!(mi.item.id().unwrap(), "goals");

        assert!(mi.item.roles().is_empty());

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
    fn via_block_anchor_syntax() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("[goals]"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[],
                anchor: Some("goals"),
                source: Span {
                    data: "[goals]",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn shorthand_only_first_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo,blah#goals"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: None,
                        shorthand_items: &["foo"],
                        value: "foo"
                    },
                    ElementAttribute {
                        name: None,
                        shorthand_items: &[],
                        value: "blah#goals"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo,blah#goals",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.roles().is_empty());

        assert_eq!(
            mi.after,
            Span {
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

    use crate::{HasSpan, Parser, attributes::AttrlistContext, tests::prelude::*};

    #[test]
    fn via_shorthand_syntax() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new(".rolename"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &[".rolename"],
                    value: ".rolename"
                }],
                anchor: None,
                source: Span {
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

        assert_eq!(roles.next().unwrap(), &"rolename");

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.item.span(),
            Span {
                data: ".rolename",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 10,
                offset: 9
            }
        );
    }

    #[test]
    fn via_shorthand_syntax_trim_trailing_whitespace() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new(".rolename "),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &[".rolename"],
                    value: ".rolename"
                }],
                anchor: None,
                source: Span {
                    data: ".rolename ",
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

        assert_eq!(roles.next().unwrap(), &"rolename");
        assert!(roles.next().is_none());

        assert_eq!(
            mi.item.span(),
            Span {
                data: ".rolename ",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );
    }

    #[test]
    fn multiple_roles_via_shorthand_syntax() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new(".role1.role2.role3"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &[".role1", ".role2", ".role3"],
                    value: ".role1.role2.role3"
                }],
                anchor: None,
                source: Span {
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

        assert_eq!(roles.next().unwrap(), &"role1");

        assert_eq!(roles.next().unwrap(), &"role2");

        assert_eq!(roles.next().unwrap(), &"role3");

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.item.span(),
            Span {
                data: ".role1.role2.role3",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 19,
                offset: 18
            }
        );
    }

    #[test]
    fn multiple_roles_via_shorthand_syntax_trim_whitespace() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new(".role1 .role2 .role3 "),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &[".role1", ".role2", ".role3"],
                    value: ".role1 .role2 .role3"
                }],
                anchor: None,
                source: Span {
                    data: ".role1 .role2 .role3 ",
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

        assert_eq!(roles.next().unwrap(), &"role1");

        assert_eq!(roles.next().unwrap(), &"role2");

        assert_eq!(roles.next().unwrap(), &"role3");

        assert!(roles.next().is_none(),);

        assert_eq!(
            mi.item.span(),
            Span {
                data: ".role1 .role2 .role3 ",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 22,
                offset: 21
            }
        );
    }

    #[test]
    fn via_named_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo=bar,role=role1"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: Some("foo"),
                        shorthand_items: &[],
                        value: "bar"
                    },
                    ElementAttribute {
                        name: Some("role"),
                        shorthand_items: &[],
                        value: "role1"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo=bar,role=role1",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            ElementAttribute {
                name: Some("foo"),
                shorthand_items: &[],
                value: "bar"
            }
        );

        assert_eq!(
            mi.item.named_attribute("role").unwrap(),
            ElementAttribute {
                name: Some("role"),
                shorthand_items: &[],
                value: "role1"
            }
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();
        assert_eq!(roles.next().unwrap(), &"role1");
        assert!(roles.next().is_none());

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 19,
                offset: 18
            }
        );
    }

    #[test]
    fn multiple_roles_via_named_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo=bar,role=role1 role2   role3 "),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: Some("foo"),
                        shorthand_items: &[],
                        value: "bar"
                    },
                    ElementAttribute {
                        name: Some("role"),
                        shorthand_items: &[],
                        value: "role1 role2   role3"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo=bar,role=role1 role2   role3 ",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            ElementAttribute {
                name: Some("foo"),
                shorthand_items: &[],
                value: "bar"
            }
        );

        assert_eq!(
            mi.item.named_attribute("role").unwrap(),
            ElementAttribute {
                name: Some("role"),
                shorthand_items: &[],
                value: "role1 role2   role3"
            }
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(roles.next().unwrap(), &"role1");
        assert_eq!(roles.next().unwrap(), &"role2");
        assert_eq!(roles.next().unwrap(), &"role3");
        assert!(roles.next().is_none());

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 34,
                offset: 33
            }
        );
    }

    #[test]
    fn shorthand_role_and_named_attribute_role() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("#foo.sh1.sh2,role=na1 na2   na3 "),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: None,
                        shorthand_items: &["#foo", ".sh1", ".sh2"],
                        value: "#foo.sh1.sh2"
                    },
                    ElementAttribute {
                        name: Some("role"),
                        shorthand_items: &[],
                        value: "na1 na2   na3"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "#foo.sh1.sh2,role=na1 na2   na3 ",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());

        assert_eq!(
            mi.item.named_attribute("role").unwrap(),
            ElementAttribute {
                name: Some("role"),
                shorthand_items: &[],
                value: "na1 na2   na3"
            }
        );

        let roles = mi.item.roles();
        let mut roles = roles.iter();

        assert_eq!(roles.next().unwrap(), &"sh1");
        assert_eq!(roles.next().unwrap(), &"sh2");
        assert_eq!(roles.next().unwrap(), &"na1");
        assert_eq!(roles.next().unwrap(), &"na2");
        assert_eq!(roles.next().unwrap(), &"na3");
        assert!(roles.next().is_none());

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 33,
                offset: 32
            }
        );
    }

    #[test]
    fn shorthand_only_first_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo,blah.rolename"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: None,
                        shorthand_items: &["foo"],
                        value: "foo"
                    },
                    ElementAttribute {
                        name: None,
                        shorthand_items: &[],
                        value: "blah.rolename"
                    },
                ],
                anchor: None,
                source: Span {
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
            Span {
                data: "",
                line: 1,
                col: 18,
                offset: 17
            }
        );
    }
}

mod options {
    use pretty_assertions_sorted::assert_eq;

    use crate::{HasSpan, Parser, attributes::AttrlistContext, tests::prelude::*};

    #[test]
    fn via_shorthand_syntax() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("%option"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &["%option"],
                    value: "%option"
                }],
                anchor: None,
                source: Span {
                    data: "%option",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());
        assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(options.next().unwrap(), &"option",);

        assert!(options.next().is_none());

        assert!(mi.item.has_option("option"));
        assert!(!mi.item.has_option("option1"));

        assert_eq!(
            mi.item.span(),
            Span {
                data: "%option",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn multiple_options_via_shorthand_syntax() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("%option1%option2%option3"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[ElementAttribute {
                    name: None,
                    shorthand_items: &["%option1", "%option2", "%option3",],
                    value: "%option1%option2%option3"
                }],
                anchor: None,
                source: Span {
                    data: "%option1%option2%option3",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none());
        assert!(mi.item.named_or_positional_attribute("foo", 0).is_none());

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(options.next().unwrap(), &"option1");
        assert_eq!(options.next().unwrap(), &"option2");
        assert_eq!(options.next().unwrap(), &"option3");
        assert!(options.next().is_none());

        assert!(mi.item.has_option("option1"));
        assert!(mi.item.has_option("option2"));
        assert!(mi.item.has_option("option3"));
        assert!(!mi.item.has_option("option4"));

        assert_eq!(
            mi.item.span(),
            Span {
                data: "%option1%option2%option3",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 25,
                offset: 24
            }
        );
    }

    #[test]
    fn via_options_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo=bar,options=option1"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: Some("foo"),
                        shorthand_items: &[],
                        value: "bar"
                    },
                    ElementAttribute {
                        name: Some("options"),
                        shorthand_items: &[],
                        value: "option1"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo=bar,options=option1",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            ElementAttribute {
                name: Some("foo"),
                shorthand_items: &[],
                value: "bar"
            }
        );

        assert_eq!(
            mi.item.named_attribute("options").unwrap(),
            ElementAttribute {
                name: Some("options"),
                shorthand_items: &[],
                value: "option1"
            }
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(options.next().unwrap(), &"option1");
        assert!(options.next().is_none());

        assert!(mi.item.has_option("option1"));
        assert!(!mi.item.has_option("option2"));

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 24,
                offset: 23
            }
        );
    }

    #[test]
    fn via_opts_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo=bar,opts=option1"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: Some("foo"),
                        shorthand_items: &[],
                        value: "bar"
                    },
                    ElementAttribute {
                        name: Some("opts"),
                        shorthand_items: &[],
                        value: "option1"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo=bar,opts=option1",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            ElementAttribute {
                name: Some("foo"),
                shorthand_items: &[],
                value: "bar"
            }
        );

        assert_eq!(
            mi.item.named_attribute("opts").unwrap(),
            ElementAttribute {
                name: Some("opts"),
                shorthand_items: &[],
                value: "option1"
            }
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(options.next().unwrap(), &"option1");
        assert!(options.next().is_none());

        assert!(!mi.item.has_option("option"));
        assert!(mi.item.has_option("option1"));
        assert!(!mi.item.has_option("option2"));

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 21,
                offset: 20
            }
        );
    }

    #[test]
    fn multiple_options_via_named_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo=bar,options=\"option1,option2,option3\""),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: Some("foo"),
                        shorthand_items: &[],
                        value: "bar"
                    },
                    ElementAttribute {
                        name: Some("options"),
                        shorthand_items: &[],
                        value: "option1,option2,option3"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo=bar,options=\"option1,option2,option3\"",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert_eq!(
            mi.item.named_attribute("foo").unwrap(),
            ElementAttribute {
                name: Some("foo"),
                shorthand_items: &[],
                value: "bar"
            }
        );

        assert_eq!(
            mi.item.named_attribute("options").unwrap(),
            ElementAttribute {
                name: Some("options"),
                shorthand_items: &[],
                value: "option1,option2,option3"
            }
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(options.next().unwrap(), &"option1");
        assert_eq!(options.next().unwrap(), &"option2");
        assert_eq!(options.next().unwrap(), &"option3");
        assert!(options.next().is_none());

        assert!(mi.item.has_option("option1"));
        assert!(mi.item.has_option("option2"));
        assert!(mi.item.has_option("option3"));
        assert!(!mi.item.has_option("option4"));

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 42,
                offset: 41
            }
        );
    }

    #[test]
    fn shorthand_option_and_named_attribute_option() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("#foo%sh1%sh2,options=\"na1,na2,na3\""),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: None,
                        shorthand_items: &["#foo", "%sh1", "%sh2"],
                        value: "#foo%sh1%sh2"
                    },
                    ElementAttribute {
                        name: Some("options"),
                        shorthand_items: &[],
                        value: "na1,na2,na3"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "#foo%sh1%sh2,options=\"na1,na2,na3\"",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        assert!(mi.item.named_attribute("foo").is_none(),);

        assert_eq!(
            mi.item.named_attribute("options").unwrap(),
            ElementAttribute {
                name: Some("options"),
                shorthand_items: &[],
                value: "na1,na2,na3"
            }
        );

        let options = mi.item.options();
        let mut options = options.iter();

        assert_eq!(options.next().unwrap(), &"sh1");
        assert_eq!(options.next().unwrap(), &"sh2");
        assert_eq!(options.next().unwrap(), &"na1");
        assert_eq!(options.next().unwrap(), &"na2");
        assert_eq!(options.next().unwrap(), &"na3");
        assert!(options.next().is_none(),);

        assert!(mi.item.has_option("sh1"));
        assert!(mi.item.has_option("sh2"));
        assert!(!mi.item.has_option("sh3"));
        assert!(mi.item.has_option("na1"));
        assert!(mi.item.has_option("na2"));
        assert!(mi.item.has_option("na3"));
        assert!(!mi.item.has_option("na4"));

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 35,
                offset: 34
            }
        );
    }

    #[test]
    fn shorthand_only_first_attribute() {
        let p = Parser::default();

        let mi = crate::attributes::Attrlist::parse(
            crate::Span::new("foo,blah%option"),
            &p,
            AttrlistContext::Inline,
        )
        .unwrap_if_no_warnings();

        assert_eq!(
            mi.item,
            Attrlist {
                attributes: &[
                    ElementAttribute {
                        name: None,
                        shorthand_items: &["foo"],
                        value: "foo"
                    },
                    ElementAttribute {
                        name: None,
                        shorthand_items: &[],
                        value: "blah%option"
                    },
                ],
                anchor: None,
                source: Span {
                    data: "foo,blah%option",
                    line: 1,
                    col: 1,
                    offset: 0
                }
            }
        );

        let options = mi.item.options();
        assert_eq!(options.iter().len(), 0);

        assert!(!mi.item.has_option("option"));

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 16,
                offset: 15
            }
        );
    }
}

#[test]
fn err_double_comma() {
    let p = Parser::default();

    let maw = crate::attributes::Attrlist::parse(
        crate::Span::new("alt=Sunset,width=300,,height=400"),
        &p,
        AttrlistContext::Inline,
    );

    let mi = maw.item.clone();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: Some("alt"),
                    shorthand_items: &[],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: Some("width"),
                    shorthand_items: &[],
                    value: "300"
                },
                ElementAttribute {
                    name: Some("height"),
                    shorthand_items: &[],
                    value: "400"
                },
            ],
            anchor: None,
            source: Span {
                data: "alt=Sunset,width=300,,height=400",
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
            col: 33,
            offset: 32,
        }
    );

    assert_eq!(
        maw.warnings,
        vec![Warning {
            source: Span {
                data: "alt=Sunset,width=300,,height=400",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}

#[test]
fn applies_attribute_substitution_before_parsing() {
    let p = Parser::default().with_intrinsic_attribute(
        "sunset_dimensions",
        "300,400",
        ModificationContext::Anywhere,
    );

    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new("Sunset,{sunset_dimensions}"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: None,
                    shorthand_items: &["Sunset"],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "300"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "400"
                }
            ],
            anchor: None,
            source: Span {
                data: "Sunset,{sunset_dimensions}",
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
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.nth_attribute(1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "300"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("height", 3).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "400"
        }
    );

    assert!(mi.item.nth_attribute(4).is_none());
    assert!(mi.item.named_or_positional_attribute("height", 4).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "Sunset,{sunset_dimensions}",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 27,
            offset: 26,
        }
    );
}

#[test]
fn ignores_unknown_attribute_when_applying_attribution_substitution() {
    let p = Parser::default().with_intrinsic_attribute(
        "sunset_dimensions",
        "300,400",
        ModificationContext::Anywhere,
    );

    let mi = crate::attributes::Attrlist::parse(
        crate::Span::new("Sunset,{not_sunset_dimensions}"),
        &p,
        AttrlistContext::Inline,
    )
    .unwrap_if_no_warnings();

    assert_eq!(
        mi.item,
        Attrlist {
            attributes: &[
                ElementAttribute {
                    name: None,
                    shorthand_items: &["Sunset"],
                    value: "Sunset"
                },
                ElementAttribute {
                    name: None,
                    shorthand_items: &[],
                    value: "{not_sunset_dimensions}"
                },
            ],
            anchor: None,
            source: Span {
                data: "Sunset,{not_sunset_dimensions}",
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
    assert!(mi.item.roles().is_empty());

    assert_eq!(
        mi.item.nth_attribute(1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("alt", 1).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &["Sunset"],
            value: "Sunset"
        }
    );

    assert_eq!(
        mi.item.nth_attribute(2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "{not_sunset_dimensions}"
        }
    );

    assert_eq!(
        mi.item.named_or_positional_attribute("width", 2).unwrap(),
        ElementAttribute {
            name: None,
            shorthand_items: &[],
            value: "{not_sunset_dimensions}"
        }
    );

    assert!(mi.item.nth_attribute(3).is_none());
    assert!(mi.item.named_or_positional_attribute("height", 3).is_none());
    assert!(mi.item.nth_attribute(42).is_none());

    assert_eq!(
        mi.item.span(),
        Span {
            data: "Sunset,{not_sunset_dimensions}",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 31,
            offset: 30,
        }
    );
}
