use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMacroBlock, TSimpleBlock},
        content::TContent,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    HasSpan, Parser, Span,
};

// NOTE: The "error" cases from the MacroBlock parser test suite are not
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
                rendered: None,
                substitutions: vec!(),
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

    let mi = Block::parse(Span::new("foo::bar"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "foo::bar",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: None,
                substitutions: vec!(),
            },
            source: TSpan {
                data: "foo::bar",
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("foo::bar[blah"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "foo::bar[blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: None,
                substitutions: vec!(),
            },
            source: TSpan {
                data: "foo::bar[blah",
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("foo::bar[blah]bonus"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "foo::bar[blah]bonus",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: None,
                substitutions: vec!(),
            },
            source: TSpan {
                data: "foo::bar[blah]bonus",
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("foo::[]"), &mut parser)
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
            anchor: None,
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
    assert_eq!(mi.item.raw_context().deref(), "paragraph");
    assert_eq!(mi.item.resolved_context().deref(), "paragraph");
    assert_eq!(mi.item.nested_blocks().next(), None);
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());

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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("foo::bar[]"), &mut parser)
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
            anchor: None,
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("foo::bar[blah]"), &mut parser)
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
            anchor: None,
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
    let mut parser = Parser::default();

    let maw = Block::parse(
        Span::new("foo::bar[alt=Sunset,width=300,,height=400]"),
        &mut parser,
    );

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
            anchor: None,
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new(".macro title\nfoo::bar[]\n"), &mut parser)
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
            anchor: None,
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
