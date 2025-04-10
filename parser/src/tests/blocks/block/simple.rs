use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TSimpleBlock},
        inlines::TInline,
        TSpan,
    },
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = Block::parse(Span::new("abc"))
        .unwrap_if_no_warnings()
        .unwrap();

    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    assert!(Block::parse(Span::new(""))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn err_only_spaces() {
    assert!(Block::parse(Span::new("    "))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn single_line() {
    let mi = Block::parse(Span::new("abc"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }),
            source: TSpan {
                data: "abc",
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
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Simple);
    assert_eq!(mi.item.raw_context().deref(), "paragraph");
    assert_eq!(mi.item.resolved_context().deref(), "paragraph");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.nested_blocks().next(), None);
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title().is_none());
    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 4,
            offset: 3
        }
    );
}

#[test]
fn multiple_lines() {
    let mi = Block::parse(Span::new("abc\ndef"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Sequence(
                vec![
                    TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }),
                    TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 2,
                        col: 1,
                        offset: 4,
                    }),
                ],
                TSpan {
                    data: "abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            ),
            source: TSpan {
                data: "abc\ndef",
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
            data: "abc\ndef",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 2,
            col: 4,
            offset: 7
        }
    );
}

#[test]
fn title() {
    let mi = Block::parse(Span::new(".simple block\nabc\ndef\n"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Sequence(
                vec![
                    TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 2,
                        col: 1,
                        offset: 14,
                    }),
                    TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 18,
                    }),
                ],
                TSpan {
                    data: "abc\ndef\n",
                    line: 2,
                    col: 1,
                    offset: 14,
                }
            ),
            source: TSpan {
                data: ".simple block\nabc\ndef\n",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: Some(TSpan {
                data: "simple block",
                line: 1,
                col: 2,
                offset: 1,
            },),
            anchor: None,
            attrlist: None,
        })
    );
}

#[test]
fn attrlist() {
    let mi = Block::parse(Span::new("[sidebar]\nabc\ndef\n"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Sequence(
                vec![
                    TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 2,
                        col: 1,
                        offset: 10,
                    },),
                    TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 14,
                    },),
                ],
                TSpan {
                    data: "abc\ndef\n",
                    line: 2,
                    col: 1,
                    offset: 10,
                },
            ),
            source: TSpan {
                data: "[sidebar]\nabc\ndef\n",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: vec![TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "sidebar",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },],
                    value: TSpan {
                        data: "sidebar",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                    source: TSpan {
                        data: "sidebar",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },],
                source: TSpan {
                    data: "sidebar",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            },),
        },)
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "[sidebar]\nabc\ndef\n",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.item.attrlist().unwrap(),
        TAttrlist {
            attributes: vec![TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "sidebar",
                    line: 1,
                    col: 2,
                    offset: 1,
                },],
                value: TSpan {
                    data: "sidebar",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "sidebar",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            },],
            source: TSpan {
                data: "sidebar",
                line: 1,
                col: 2,
                offset: 1,
            },
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 4,
            col: 1,
            offset: 18,
        }
    );
}

#[test]
fn title_and_attrlist() {
    let mi = Block::parse(Span::new(".title\n[sidebar]\nabc\ndef\n"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Sequence(
                vec![
                    TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 17,
                    },),
                    TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 4,
                        col: 1,
                        offset: 21,
                    },),
                ],
                TSpan {
                    data: "abc\ndef\n",
                    line: 3,
                    col: 1,
                    offset: 17,
                },
            ),
            source: TSpan {
                data: ".title\n[sidebar]\nabc\ndef\n",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: Some(TSpan {
                data: "title",
                line: 1,
                col: 2,
                offset: 1,
            },),
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: vec![TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "sidebar",
                        line: 2,
                        col: 2,
                        offset: 8,
                    },],
                    value: TSpan {
                        data: "sidebar",
                        line: 2,
                        col: 2,
                        offset: 8,
                    },
                    source: TSpan {
                        data: "sidebar",
                        line: 2,
                        col: 2,
                        offset: 8,
                    },
                },],
                source: TSpan {
                    data: "sidebar",
                    line: 2,
                    col: 2,
                    offset: 8,
                },
            },),
        },)
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: ".title\n[sidebar]\nabc\ndef\n",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.item.attrlist().unwrap(),
        TAttrlist {
            attributes: vec![TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "sidebar",
                    line: 2,
                    col: 2,
                    offset: 8,
                },],
                value: TSpan {
                    data: "sidebar",
                    line: 2,
                    col: 2,
                    offset: 8,
                },
                source: TSpan {
                    data: "sidebar",
                    line: 2,
                    col: 2,
                    offset: 8,
                },
            },],
            source: TSpan {
                data: "sidebar",
                line: 2,
                col: 2,
                offset: 8,
            },
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 5,
            col: 1,
            offset: 25,
        }
    );
}

#[test]
fn consumes_blank_lines_after() {
    let mi = Block::parse(Span::new("abc\n\ndef"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }),
            source: TSpan {
                data: "abc\n",
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
            data: "abc\n",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "def",
            line: 3,
            col: 1,
            offset: 5
        }
    );
}
