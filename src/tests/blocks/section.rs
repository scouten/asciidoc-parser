use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock, SectionBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMacroBlock, TSectionBlock, TSimpleBlock},
        inlines::TInline,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = SectionBlock::parse(Span::new("== Section Title"), None).unwrap();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    assert!(SectionBlock::parse(Span::new(""), None).is_none());
}

#[test]
fn err_only_spaces() {
    assert!(SectionBlock::parse(Span::new("    "), None).is_none());
}

#[test]
fn err_not_section() {
    assert!(SectionBlock::parse(Span::new("blah blah"), None).is_none());
}

#[test]
fn err_missing_space_before_title() {
    assert!(SectionBlock::parse(Span::new("=blah blah"), None).is_none());
}

#[test]
fn simplest_section_block() {
    let mi = SectionBlock::parse(Span::new("== Section Title"), None)
        .unwrap()
        .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![],
            source: TSpan {
                data: "== Section Title",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None
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
fn has_child_block() {
    let mi = SectionBlock::parse(Span::new("== Section Title\n\nabc"), None)
        .unwrap()
        .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                }),
                title: None
            })],
            source: TSpan {
                data: "== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 4,
            offset: 21
        }
    );
}

#[test]
fn has_macro_block_with_extra_blank_line() {
    let mi = SectionBlock::parse(
        Span::new("== Section Title\n\nfoo::bar[alt=Sunset,width=300,height=400]\n\n"),
        None,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![TBlock::Macro(TMacroBlock {
                name: TSpan {
                    data: "foo",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                target: Some(TSpan {
                    data: "bar",
                    line: 3,
                    col: 6,
                    offset: 23,
                }),
                attrlist: TAttrlist {
                    attributes: vec!(
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "alt",
                                line: 3,
                                col: 10,
                                offset: 27,
                            }),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "Sunset",
                                line: 3,
                                col: 14,
                                offset: 31,
                            },
                            source: TSpan {
                                data: "alt=Sunset",
                                line: 3,
                                col: 10,
                                offset: 27,
                            },
                        },
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "width",
                                line: 3,
                                col: 21,
                                offset: 38,
                            }),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "300",
                                line: 3,
                                col: 27,
                                offset: 44,
                            },
                            source: TSpan {
                                data: "width=300",
                                line: 3,
                                col: 21,
                                offset: 38,
                            },
                        },
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "height",
                                line: 3,
                                col: 31,
                                offset: 48,
                            }),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "400",
                                line: 3,
                                col: 38,
                                offset: 55,
                            },
                            source: TSpan {
                                data: "height=400",
                                line: 3,
                                col: 31,
                                offset: 48,
                            },
                        }
                    ),
                    source: TSpan {
                        data: "alt=Sunset,width=300,height=400",
                        line: 3,
                        col: 10,
                        offset: 27,
                    }
                },
                source: TSpan {
                    data: "foo::bar[alt=Sunset,width=300,height=400]",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title: None
            })],
            source: TSpan {
                data: "== Section Title\n\nfoo::bar[alt=Sunset,width=300,height=400]\n\n",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 5,
            col: 1,
            offset: 61
        }
    );
}

#[test]
fn has_child_block_with_errors() {
    let maw = SectionBlock::parse(
        Span::new("== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]"),
        None,
    )
    .unwrap();

    let mi = maw.item.clone();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![TBlock::Macro(TMacroBlock {
                name: TSpan {
                    data: "foo",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                target: Some(TSpan {
                    data: "bar",
                    line: 3,
                    col: 6,
                    offset: 23,
                }),
                attrlist: TAttrlist {
                    attributes: vec!(
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "alt",
                                line: 3,
                                col: 10,
                                offset: 27,
                            }),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "Sunset",
                                line: 3,
                                col: 14,
                                offset: 31,
                            },
                            source: TSpan {
                                data: "alt=Sunset",
                                line: 3,
                                col: 10,
                                offset: 27,
                            },
                        },
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "width",
                                line: 3,
                                col: 21,
                                offset: 38,
                            }),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "300",
                                line: 3,
                                col: 27,
                                offset: 44,
                            },
                            source: TSpan {
                                data: "width=300",
                                line: 3,
                                col: 21,
                                offset: 38,
                            },
                        },
                        TElementAttribute {
                            name: Some(TSpan {
                                data: "height",
                                line: 3,
                                col: 32,
                                offset: 49,
                            }),
                            shorthand_items: vec![],
                            value: TSpan {
                                data: "400",
                                line: 3,
                                col: 39,
                                offset: 56,
                            },
                            source: TSpan {
                                data: "height=400",
                                line: 3,
                                col: 32,
                                offset: 49,
                            },
                        }
                    ),
                    source: TSpan {
                        data: "alt=Sunset,width=300,,height=400",
                        line: 3,
                        col: 10,
                        offset: 27,
                    }
                },
                source: TSpan {
                    data: "foo::bar[alt=Sunset,width=300,,height=400]",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title: None,
            })],
            source: TSpan {
                data: "== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 43,
            offset: 60
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: ",",
                line: 3,
                col: 30,
                offset: 47,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}

#[test]
fn dont_stop_at_child_section() {
    let mi = SectionBlock::parse(
        Span::new("== Section Title\n\nabc\n\n=== Section 2\n\ndef"),
        None,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![
                TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 18,
                    }),
                    title: None
                }),
                TBlock::Section(TSectionBlock {
                    level: 2,
                    section_title: TSpan {
                        data: "Section 2",
                        line: 5,
                        col: 5,
                        offset: 27,
                    },
                    blocks: vec![TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 38,
                        }),
                        title: None
                    })],
                    source: TSpan {
                        data: "=== Section 2\n\ndef",
                        line: 5,
                        col: 1,
                        offset: 23,
                    },
                    title: None
                })
            ],
            source: TSpan {
                data: "== Section Title\n\nabc\n\n=== Section 2\n\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 7,
            col: 4,
            offset: 41
        }
    );
}

#[test]
fn stop_at_peer_section() {
    let mi = SectionBlock::parse(
        Span::new("== Section Title\n\nabc\n\n== Section 2\n\ndef"),
        None,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                }),
                title: None
            })],
            source: TSpan {
                // TO DO: Fix bug that includes blank lines.
                data: "== Section Title\n\nabc\n\n",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "== Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 23
        }
    );
}

#[test]
fn stop_at_ancestor_section() {
    let mi = SectionBlock::parse(
        Span::new("=== Section Title\n\nabc\n\n== Section 2\n\ndef"),
        None,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 2,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 5,
                offset: 4,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                }),
                title: None
            })],
            source: TSpan {
                // TO DO: Fix bug that includes blank lines.
                data: "=== Section Title\n\nabc\n\n",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "== Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 24
        }
    );
}
