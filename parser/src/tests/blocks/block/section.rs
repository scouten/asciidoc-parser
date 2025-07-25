use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    HasSpan, Parser, Span,
    blocks::{Block, ContentModel, IsBlock, MediaType},
    span::content::SubstitutionGroup,
    tests::fixtures::{
        TSpan,
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMediaBlock, TSectionBlock, TSimpleBlock},
        content::TContent,
        warnings::TWarning,
    },
    warnings::WarningType,
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
                rendered: "=blah blah",
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
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

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
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

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
                    rendered: "abc",
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
                rendered: "abc",
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
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

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
                    rendered: "abc",
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
                rendered: "abc",
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
        Span::new("== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]"),
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
            blocks: vec![TBlock::Media(TMediaBlock {
                type_: MediaType::Image,
                target: TSpan {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
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
                        line: 3,
                        col: 12,
                        offset: 29,
                    }
                },
                source: TSpan {
                    data: "image::bar[alt=Sunset,width=300,,height=400]",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: TSpan {
                data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
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
            data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
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
            col: 45,
            offset: 62
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "alt=Sunset,width=300,,height=400",
                line: 3,
                col: 12,
                offset: 29,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}
