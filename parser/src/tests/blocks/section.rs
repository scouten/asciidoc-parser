use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    blocks::{ContentModel, IsBlock, MediaType, SectionBlock, metadata::BlockMetadata},
    content::SubstitutionGroup,
    tests::fixtures::{
        Span,
        attributes::{Attrlist, TElementAttribute},
        blocks::{TBlock, TMediaBlock, TSectionBlock, TSimpleBlock},
        content::TContent,
        warnings::TWarning,
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();

    let b1 = SectionBlock::parse(&BlockMetadata::new("== Section Title"), &mut parser).unwrap();

    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    let mut parser = Parser::default();
    assert!(SectionBlock::parse(&BlockMetadata::new(""), &mut parser).is_none());
}

#[test]
fn err_only_spaces() {
    let mut parser = Parser::default();
    assert!(SectionBlock::parse(&BlockMetadata::new("    "), &mut parser).is_none());
}

#[test]
fn err_not_section() {
    let mut parser = Parser::default();
    assert!(SectionBlock::parse(&BlockMetadata::new("blah blah"), &mut parser).is_none());
}

#[test]
fn err_missing_space_before_title() {
    let mut parser = Parser::default();
    assert!(SectionBlock::parse(&BlockMetadata::new("=blah blah"), &mut parser).is_none());
}

#[test]
fn simplest_section_block() {
    let mut parser = Parser::default();

    let mi = SectionBlock::parse(&BlockMetadata::new("== Section Title"), &mut parser)
        .unwrap()
        .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: &[],
            source: Span {
                data: "== Section Title",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

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
fn has_child_block() {
    let mut parser = Parser::default();

    let mi = SectionBlock::parse(&BlockMetadata::new("== Section Title\n\nabc"), &mut parser)
        .unwrap()
        .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: Span {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 18,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 3,
            col: 4,
            offset: 21
        }
    );
}

#[test]
fn has_macro_block_with_extra_blank_line() {
    let mut parser = Parser::default();

    let mi = SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]\n\n"),
        &mut parser,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: &[TBlock::Media(TMediaBlock {
                type_: MediaType::Image,
                target: Span {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
                },
                macro_attrlist: Attrlist {
                    attributes: &[
                        TElementAttribute {
                            name: Some("alt"),
                            shorthand_items: &[],
                            value: "Sunset"
                        },
                        TElementAttribute {
                            name: Some("width"),
                            shorthand_items: &[],
                            value: "300"
                        },
                        TElementAttribute {
                            name: Some("height"),
                            shorthand_items: &[],
                            value: "400"
                        }
                    ],
                    source: Span {
                        data: "alt=Sunset,width=300,height=400",
                        line: 3,
                        col: 12,
                        offset: 29,
                    }
                },
                source: Span {
                    data: "image::bar[alt=Sunset,width=300,height=400]",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 5,
            col: 1,
            offset: 63
        }
    );
}

#[test]
fn has_child_block_with_errors() {
    let mut parser = Parser::default();

    let maw = SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]"),
        &mut parser,
    )
    .unwrap();

    let mi = maw.item.clone();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: &[TBlock::Media(TMediaBlock {
                type_: MediaType::Image,
                target: Span {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
                },
                macro_attrlist: Attrlist {
                    attributes: &[
                        TElementAttribute {
                            name: Some("alt"),
                            shorthand_items: &[],
                            value: "Sunset"
                        },
                        TElementAttribute {
                            name: Some("width"),
                            shorthand_items: &[],
                            value: "300"
                        },
                        TElementAttribute {
                            name: Some("height"),
                            shorthand_items: &[],
                            value: "400"
                        }
                    ],
                    source: Span {
                        data: "alt=Sunset,width=300,,height=400",
                        line: 3,
                        col: 12,
                        offset: 29,
                    }
                },
                source: Span {
                    data: "image::bar[alt=Sunset,width=300,,height=400]",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 3,
            col: 45,
            offset: 62
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: Span {
                data: "alt=Sunset,width=300,,height=400",
                line: 3,
                col: 12,
                offset: 29,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}

#[test]
fn dont_stop_at_child_section() {
    let mut parser = Parser::default();

    let mi = SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nabc\n\n=== Section 2\n\ndef"),
        &mut parser,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: &[
                TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        rendered: "abc",
                    },
                    source: Span {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 18,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                }),
                TBlock::Section(TSectionBlock {
                    level: 2,
                    section_title: Span {
                        data: "Section 2",
                        line: 5,
                        col: 5,
                        offset: 27,
                    },
                    blocks: &[TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: Span {
                                data: "def",
                                line: 7,
                                col: 1,
                                offset: 38,
                            },
                            rendered: "def",
                        },
                        source: Span {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 38,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "=== Section 2\n\ndef",
                        line: 5,
                        col: 1,
                        offset: 23,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                })
            ],
            source: Span {
                data: "== Section Title\n\nabc\n\n=== Section 2\n\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 7,
            col: 4,
            offset: 41
        }
    );
}

#[test]
fn stop_at_peer_section() {
    let mut parser = Parser::default();

    let mi = SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nabc\n\n== Section 2\n\ndef"),
        &mut parser,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: Span {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 18,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "== Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 23
        }
    );
}

#[test]
fn stop_at_ancestor_section() {
    let mut parser = Parser::default();

    let mi = SectionBlock::parse(
        &BlockMetadata::new("=== Section Title\n\nabc\n\n== Section 2\n\ndef"),
        &mut parser,
    )
    .unwrap()
    .unwrap_if_no_warnings();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 2,
            section_title: Span {
                data: "Section Title",
                line: 1,
                col: 5,
                offset: 4,
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: Span {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 19,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "=== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "== Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 24
        }
    );
}
