use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMacroBlock, TSectionBlock, TSimpleBlock},
        inlines::TInline,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    HasSpan, Span,
};

#[test]
fn err_missing_space_before_title() {
    let mi = Block::parse(Span::new("=blah blah"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
            data: "=blah blah",
            line: 1,
            col: 1,
            offset: 0,
        })))
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "=blah blah",
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
fn simplest_section_block() {
    let mi = Block::parse(Span::new("== Section Title"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
            level: 1,
            title: TSpan {
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
        })
    );

    assert_eq!(mi.item.nested_blocks().next(), None);

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
    let mi = Block::parse(Span::new("== Section Title\n\nabc"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
            level: 1,
            title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                }
            )))],
            source: TSpan {
                data: "== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
        })
    );

    let mut nested_blocks = mi.item.nested_blocks();

    assert_eq!(
        nested_blocks.next().unwrap(),
        &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
            data: "abc",
            line: 3,
            col: 1,
            offset: 18,
        })))
    );

    assert_eq!(nested_blocks.next(), None);

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "== Section Title\n\nabc",
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
            col: 4,
            offset: 21
        }
    );
}

#[test]
fn warn_child_attrlist_has_extra_comma() {
    let maw = Block::parse(Span::new(
        "== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
    ));

    let mi = maw.item.as_ref().unwrap().clone();

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
            level: 1,
            title: TSpan {
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
            })],
            source: TSpan {
                data: "== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0,
            },
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
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