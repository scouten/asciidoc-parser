use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMacroBlock, TSimpleBlock},
        inlines::{TInline, TInlineMacro},
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    HasSpan, Span,
};

// NOTE: The "error" cases from the MacroBlock parser test suite are not
// necessarily error cases here because we can reparse as SimpleBlock.

#[test]
fn err_inline_syntax() {
    let mi = Block::parse(Span::new("foo:bar[]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Macro(TInlineMacro {
                name: TSpan {
                    data: "foo",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                target: Some(TSpan {
                    data: "bar",
                    line: 1,
                    col: 5,
                    offset: 4,
                },),
                attrlist: None,
                source: TSpan {
                    data: "foo:bar[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }),
            source: TSpan {
                data: "foo:bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
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
    let mi = Block::parse(Span::new("foo::bar"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Uninterpreted(TSpan {
                data: "foo::bar",
                line: 1,
                col: 1,
                offset: 0,
            }),
            source: TSpan {
                data: "foo::bar",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        }),
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::bar",
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
            col: 9,
            offset: 8
        }
    );
}

#[test]
fn err_attr_list_not_closed() {
    let mi = Block::parse(Span::new("foo::bar[blah"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Uninterpreted(TSpan {
                data: "foo::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
            }),
            source: TSpan {
                data: "foo::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::bar[blah",
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
            col: 14,
            offset: 13
        }
    );
}

#[test]
fn err_unexpected_after_attr_list() {
    let mi = Block::parse(Span::new("foo::bar[blah]bonus"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Sequence(
                vec![
                    TInline::Macro(TInlineMacro {
                        name: TSpan {
                            data: "foo",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        target: Some(TSpan {
                            data: ":bar",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },),
                        attrlist: Some(TSpan {
                            data: "blah",
                            line: 1,
                            col: 10,
                            offset: 9,
                        },),
                        source: TSpan {
                            data: "foo::bar[blah]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },),
                    TInline::Uninterpreted(TSpan {
                        data: "bonus",
                        line: 1,
                        col: 15,
                        offset: 14,
                    },),
                ],
                TSpan {
                    data: "foo::bar[blah]bonus",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            ),
            source: TSpan {
                data: "foo::bar[blah]bonus",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::bar[blah]bonus",
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
            col: 20,
            offset: 19
        }
    );
}

#[test]
fn simplest_block_macro() {
    let mi = Block::parse(Span::new("foo::[]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Macro(TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: None,
            macro_attrlist: TAttrlist {
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 7,
                    offset: 6,
                }
            },
            source: TSpan {
                data: "foo::[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::[]",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Simple);
    assert_eq!(mi.item.context().deref(), "paragraph");
    assert_eq!(mi.item.nested_blocks().next(), None);
    assert!(mi.item.title().is_none());

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 8,
            offset: 7
        }
    );
}

#[test]
fn has_target() {
    let mi = Block::parse(Span::new("foo::bar[]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Macro(TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: "bar",
                line: 1,
                col: 6,
                offset: 5,
            }),
            macro_attrlist: TAttrlist {
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 10,
                    offset: 9,
                }
            },
            source: TSpan {
                data: "foo::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::bar[]",
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
fn has_target_and_macro_attrlist() {
    let mi = Block::parse(Span::new("foo::bar[blah]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Macro(TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: "bar",
                line: 1,
                col: 6,
                offset: 5,
            }),
            macro_attrlist: TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "blah",
                        line: 1,
                        col: 10,
                        offset: 9,
                    }],
                    value: TSpan {
                        data: "blah",
                        line: 1,
                        col: 10,
                        offset: 9,
                    },
                    source: TSpan {
                        data: "blah",
                        line: 1,
                        col: 10,
                        offset: 9,
                    },
                }),
                source: TSpan {
                    data: "blah",
                    line: 1,
                    col: 10,
                    offset: 9,
                }
            },
            source: TSpan {
                data: "foo::bar[blah]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::bar[blah]",
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
fn warn_macro_attrlist_has_extra_comma() {
    let maw = Block::parse(Span::new("foo::bar[alt=Sunset,width=300,,height=400]"));

    let mi = maw.item.as_ref().unwrap().clone();

    assert_eq!(
        mi.item,
        TBlock::Macro(TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: "bar",
                line: 1,
                col: 6,
                offset: 5,
            }),
            macro_attrlist: TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "alt",
                            line: 1,
                            col: 10,
                            offset: 9,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "Sunset",
                            line: 1,
                            col: 14,
                            offset: 13,
                        },
                        source: TSpan {
                            data: "alt=Sunset",
                            line: 1,
                            col: 10,
                            offset: 9,
                        },
                    },
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "width",
                            line: 1,
                            col: 21,
                            offset: 20,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "300",
                            line: 1,
                            col: 27,
                            offset: 26,
                        },
                        source: TSpan {
                            data: "width=300",
                            line: 1,
                            col: 21,
                            offset: 20,
                        },
                    },
                    TElementAttribute {
                        name: Some(TSpan {
                            data: "height",
                            line: 1,
                            col: 32,
                            offset: 31,
                        }),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "400",
                            line: 1,
                            col: 39,
                            offset: 38,
                        },
                        source: TSpan {
                            data: "height=400",
                            line: 1,
                            col: 32,
                            offset: 31,
                        },
                    }
                ),
                source: TSpan {
                    data: "alt=Sunset,width=300,,height=400",
                    line: 1,
                    col: 10,
                    offset: 9,
                }
            },
            source: TSpan {
                data: "foo::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo::bar[alt=Sunset,width=300,,height=400]",
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
            col: 43,
            offset: 42
        }
    );
    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: ",",
                line: 1,
                col: 30,
                offset: 29,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}

#[test]
fn has_title() {
    let mi = Block::parse(Span::new(".macro title\nfoo::bar[]\n"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Macro(TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 2,
                col: 1,
                offset: 13,
            },
            target: Some(TSpan {
                data: "bar",
                line: 2,
                col: 6,
                offset: 18,
            }),
            macro_attrlist: TAttrlist {
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 2,
                    col: 10,
                    offset: 22,
                }
            },
            source: TSpan {
                data: ".macro title\nfoo::bar[]",
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
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: ".macro title\nfoo::bar[]",
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
            offset: 24
        }
    );
}
