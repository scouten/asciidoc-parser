use pretty_assertions_sorted::assert_eq;

use crate::{
    HasSpan, Parser, Span,
    blocks::{Block, MediaType},
    tests::fixtures::{
        TSpan,
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMediaBlock, TSimpleBlock},
        content::TContent,
        warnings::TWarning,
    },
    warnings::WarningType,
};

// NOTE: The "error" cases from the MediaBlock parser test suite are not
// necessarily error cases here because we can reparse as SimpleBlock.

#[test]
fn err_inline_syntax() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("foo:bar[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "foo:bar[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "foo:bar[]",
            },
            source: TSpan {
                data: "foo:bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        }),
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo:bar[]",
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
            col: 10,
            offset: 9
        }
    );
}

#[test]
fn err_no_attr_list() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("image::bar"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "image::bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "image::bar",
            },
            source: TSpan {
                data: "image::bar",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        }),
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::bar",
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
            col: 11,
            offset: 10
        }
    );
}

#[test]
fn err_attr_list_not_closed() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("image::bar[blah"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "image::bar[blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "image::bar[blah",
            },
            source: TSpan {
                data: "image::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::bar[blah",
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
            col: 16,
            offset: 15
        }
    );
}

#[test]
fn err_unexpected_after_attr_list() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("image::bar[blah]bonus"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "image::bar[blah]bonus",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "image::bar[blah]bonus",
            },
            source: TSpan {
                data: "image::bar[blah]bonus",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::bar[blah]bonus",
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
            col: 22,
            offset: 21,
        }
    );
}

#[test]
fn rejects_image_with_no_target() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("image::[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "image::[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "image::[]",
            },
            source: TSpan {
                data: "image::[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        },)
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::[]",
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
            col: 10,
            offset: 9
        }
    );
}

#[test]
fn has_target() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("image::bar[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Media(TMediaBlock {
            type_: MediaType::Image,
            target: TSpan {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: TAttrlist {
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: TSpan {
                data: "image::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::bar[]",
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
            col: 13,
            offset: 12
        }
    );
}

#[test]
fn has_target_and_macro_attrlist() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("image::bar[blah]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Media(TMediaBlock {
            type_: MediaType::Image,
            target: TSpan {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
                    shorthand_items: vec!["blah"],
                    value: "blah"
                }),
                source: TSpan {
                    data: "blah",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: TSpan {
                data: "image::bar[blah]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::bar[blah]",
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
            col: 17,
            offset: 16
        }
    );
}

#[test]
fn warn_macro_attrlist_has_extra_comma() {
    let mut parser = Parser::default();

    let maw = Block::parse(
        Span::new("image::bar[alt=Sunset,width=300,,height=400]"),
        &mut parser,
    );

    let mi = maw.item.as_ref().unwrap().clone();

    assert_eq!(
        mi.item,
        TBlock::Media(TMediaBlock {
            type_: MediaType::Image,
            target: TSpan {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: Some("alt"),
                        shorthand_items: vec![],
                        value: "Sunset"
                    },
                    TElementAttribute {
                        name: Some("width"),
                        shorthand_items: vec![],
                        value: "300"
                    },
                    TElementAttribute {
                        name: Some("height"),
                        shorthand_items: vec![],
                        value: "400"
                    }
                ),
                source: TSpan {
                    data: "alt=Sunset,width=300,,height=400",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: TSpan {
                data: "image::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "image::bar[alt=Sunset,width=300,,height=400]",
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
            col: 45,
            offset: 44
        }
    );
    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "alt=Sunset,width=300,,height=400",
                line: 1,
                col: 12,
                offset: 11,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}

#[test]
fn has_title() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new(".macro title\nimage::bar[]\n"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Media(TMediaBlock {
            type_: MediaType::Image,
            target: TSpan {
                data: "bar",
                line: 2,
                col: 8,
                offset: 20,
            },
            macro_attrlist: TAttrlist {
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 2,
                    col: 12,
                    offset: 24,
                }
            },
            source: TSpan {
                data: ".macro title\nimage::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: Some(TSpan {
                data: "macro title",
                line: 1,
                col: 2,
                offset: 1,
            },),
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: ".macro title\nimage::bar[]",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 1,
            offset: 26
        }
    );
}
