use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMacroBlock, TSectionBlock, TSimpleBlock},
        content::TContent,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    HasSpan, Parser, Span,
};

#[test]
fn err_missing_space_before_title() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("=blah blah"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "=blah blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: None,
                substitutions: vec!(),
            },
            source: TSpan {
                data: "=blah blah",
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("== Section Title"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
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
            title: None,
            anchor: None,
            attrlist: None,
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("== Section Title\n\nabc"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 18,
                    },
                    rendered: None,
                    substitutions: vec!(),
                },
                source: TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: TSpan {
                data: "== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    let mut nested_blocks = mi.item.nested_blocks();

    assert_eq!(
        nested_blocks.next().unwrap(),
        &TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                rendered: None,
                substitutions: vec!(),
            },
            source: TSpan {
                data: "abc",
                line: 3,
                col: 1,
                offset: 18,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
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
fn title() {
    let mut parser = Parser::default();

    let mi = Block::parse(
        Span::new(".other section title\n== Section Title\n\nabc"),
        &mut parser,
    )
    .unwrap_if_no_warnings()
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
            level: 1,
            section_title: TSpan {
                data: "Section Title",
                line: 2,
                col: 4,
                offset: 24,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "abc",
                        line: 4,
                        col: 1,
                        offset: 39,
                    },
                    rendered: None,
                    substitutions: vec!(),
                },
                source: TSpan {
                    data: "abc",
                    line: 4,
                    col: 1,
                    offset: 39,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: TSpan {
                data: ".other section title\n== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: Some(TSpan {
                data: "other section title",
                line: 1,
                col: 2,
                offset: 1,
            },),
            anchor: None,
            attrlist: None,
        })
    );

    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());

    assert_eq!(
        mi.item.title().unwrap(),
        TSpan {
            data: "other section title",
            line: 1,
            col: 2,
            offset: 1,
        }
    );

    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());

    let mut nested_blocks = mi.item.nested_blocks();

    assert_eq!(
        nested_blocks.next().unwrap(),
        &TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc",
                    line: 4,
                    col: 1,
                    offset: 39,
                },
                rendered: None,
                substitutions: vec!(),
            },
            source: TSpan {
                data: "abc",
                line: 4,
                col: 1,
                offset: 39,
            },
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    assert_eq!(nested_blocks.next(), None);

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: ".other section title\n== Section Title\n\nabc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 4,
            col: 4,
            offset: 42
        }
    );
}

#[test]
fn warn_child_attrlist_has_extra_comma() {
    let mut parser = Parser::default();

    let maw = Block::parse(
        Span::new("== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]"),
        &mut parser,
    );

    let mi = maw.item.as_ref().unwrap().clone();

    assert_eq!(
        mi.item,
        TBlock::Section(TSectionBlock {
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
                macro_attrlist: TAttrlist {
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
                anchor: None,
                attrlist: None,
            })],
            source: TSpan {
                data: "== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
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
