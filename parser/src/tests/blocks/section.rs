use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    blocks::{ContentModel, IsBlock, MediaType, metadata::BlockMetadata},
    content::SubstitutionGroup,
    tests::prelude::*,
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let b1 = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    assert!(
        crate::blocks::SectionBlock::parse(&BlockMetadata::new(""), &mut parser, &mut warnings)
            .is_none()
    );
}

#[test]
fn err_only_spaces() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    assert!(
        crate::blocks::SectionBlock::parse(&BlockMetadata::new("    "), &mut parser, &mut warnings)
            .is_none()
    );
}

#[test]
fn err_not_section() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    assert!(
        crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("blah blah"),
            &mut parser,
            &mut warnings
        )
        .is_none()
    );
}

#[test]
fn err_missing_space_before_title() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    assert!(
        crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("=blah blah"),
            &mut parser,
            &mut warnings
        )
        .is_none()
    );
}

#[test]
fn simplest_section_block() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
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
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nabc"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]\n\n"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Media(MediaBlock {
                type_: MediaType::Image,
                target: Span {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
                },
                macro_attrlist: Attrlist {
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
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Media(MediaBlock {
                type_: MediaType::Image,
                target: Span {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
                },
                macro_attrlist: Attrlist {
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
        warnings,
        vec![Warning {
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
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nabc\n\n=== Section 2\n\ndef"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[
                Block::Simple(SimpleBlock {
                    content: Content {
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
                Block::Section(SectionBlock {
                    level: 2,
                    section_title: Content {
                        original: Span {
                            data: "Section 2",
                            line: 5,
                            col: 5,
                            offset: 27,
                        },
                        rendered: "Section 2",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
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
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section Title\n\nabc\n\n== Section 2\n\ndef"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("=== Section Title\n\nabc\n\n== Section 2\n\ndef"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 2,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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

#[test]
fn section_title_with_markup() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section with *bold* text"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(
        mi.item.section_title_source(),
        Span {
            data: "Section with *bold* text",
            line: 1,
            col: 4,
            offset: 3,
        }
    );

    assert_eq!(
        mi.item.section_title(),
        "Section with <strong>bold</strong> text"
    );
}

#[test]
fn section_title_with_special_chars() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Section with <brackets> & ampersands"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(
        mi.item.section_title_source(),
        Span {
            data: "Section with <brackets> & ampersands",
            line: 1,
            col: 4,
            offset: 3,
        }
    );

    assert_eq!(
        mi.item.section_title(),
        "Section with &lt;brackets&gt; &amp; ampersands"
    );
}

#[test]
fn err_level_0_section_heading() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let result = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("= Document Title"),
        &mut parser,
        &mut warnings,
    );

    assert!(result.is_none());

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "= Document Title",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::Level0SectionHeadingNotSupported,
        }]
    );
}

#[test]
fn err_section_heading_level_exceeds_maximum() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let result = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("======= Level 6 Section"),
        &mut parser,
        &mut warnings,
    );

    assert!(result.is_none());

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "======= Level 6 Section",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::SectionHeadingLevelExceedsMaximum(6),
        }]
    );
}

#[test]
fn valid_maximum_level_5_section() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("====== Level 5 Section"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert!(warnings.is_empty());

    assert_eq!(mi.item.level(), 5);
    assert_eq!(mi.item.section_title(), "Level 5 Section");
}

// ===== Markdown-style heading tests =====

#[test]
fn md_err_missing_space_before_title() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    assert!(
        crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("#blah blah"),
            &mut parser,
            &mut warnings
        )
        .is_none()
    );
}

#[test]
fn md_simplest_section_block() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section Title"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[],
            source: Span {
                data: "## Section Title",
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
fn md_has_child_block() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section Title\n\nabc"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
                data: "## Section Title\n\nabc",
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
fn md_has_macro_block_with_extra_blank_line() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]\n\n"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Media(MediaBlock {
                type_: MediaType::Image,
                target: Span {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
                },
                macro_attrlist: Attrlist {
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
                data: "## Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]",
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
fn md_has_child_block_with_errors() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Media(MediaBlock {
                type_: MediaType::Image,
                target: Span {
                    data: "bar",
                    line: 3,
                    col: 8,
                    offset: 25,
                },
                macro_attrlist: Attrlist {
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
                data: "## Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
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
        warnings,
        vec![Warning {
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
fn md_dont_stop_at_child_section() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section Title\n\nabc\n\n### Section 2\n\ndef"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[
                Block::Simple(SimpleBlock {
                    content: Content {
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
                Block::Section(SectionBlock {
                    level: 2,
                    section_title: Content {
                        original: Span {
                            data: "Section 2",
                            line: 5,
                            col: 5,
                            offset: 27,
                        },
                        rendered: "Section 2",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
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
                        data: "### Section 2\n\ndef",
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
                data: "## Section Title\n\nabc\n\n### Section 2\n\ndef",
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
fn md_stop_at_peer_section() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section Title\n\nabc\n\n## Section 2\n\ndef"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 1,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
                data: "## Section Title\n\nabc",
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
            data: "## Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 23
        }
    );
}

#[test]
fn md_stop_at_ancestor_section() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("### Section Title\n\nabc\n\n## Section 2\n\ndef"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.raw_context().deref(), "section");
    assert_eq!(mi.item.resolved_context().deref(), "section");
    assert!(mi.item.declared_style().is_none());
    assert_eq!(mi.item.id().unwrap(), "_section_title");
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item,
        SectionBlock {
            level: 2,
            section_title: Content {
                original: Span {
                    data: "Section Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                rendered: "Section Title",
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
                data: "### Section Title\n\nabc",
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
            data: "## Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 24
        }
    );
}

#[test]
fn md_section_title_with_markup() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section with *bold* text"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(
        mi.item.section_title_source(),
        Span {
            data: "Section with *bold* text",
            line: 1,
            col: 4,
            offset: 3,
        }
    );

    assert_eq!(
        mi.item.section_title(),
        "Section with <strong>bold</strong> text"
    );
}

#[test]
fn md_section_title_with_special_chars() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Section with <brackets> & ampersands"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(
        mi.item.section_title_source(),
        Span {
            data: "Section with <brackets> & ampersands",
            line: 1,
            col: 4,
            offset: 3,
        }
    );

    assert_eq!(
        mi.item.section_title(),
        "Section with &lt;brackets&gt; &amp; ampersands"
    );
}

#[test]
fn md_err_level_0_section_heading() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let result = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("# Document Title"),
        &mut parser,
        &mut warnings,
    );

    assert!(result.is_none());

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "# Document Title",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::Level0SectionHeadingNotSupported,
        }]
    );
}

#[test]
fn md_err_section_heading_level_exceeds_maximum() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let result = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("####### Level 6 Section"),
        &mut parser,
        &mut warnings,
    );

    assert!(result.is_none());

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "####### Level 6 Section",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::SectionHeadingLevelExceedsMaximum(6),
        }]
    );
}

#[test]
fn md_valid_maximum_level_5_section() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("###### Level 5 Section"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert!(warnings.is_empty());

    assert_eq!(mi.item.level(), 5);
    assert_eq!(mi.item.section_title(), "Level 5 Section");
}

#[test]
fn warn_section_level_skipped_asciidoc() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Level 1\n\n==== Level 3 (skipped level 2)"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.level(), 1);
    assert_eq!(mi.item.section_title(), "Level 1");
    assert_eq!(mi.item.nested_blocks().len(), 1);

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "==== Level 3 (skipped level 2)",
                line: 3,
                col: 1,
                offset: 12,
            },
            warning: WarningType::SectionHeadingLevelSkipped(1, 3),
        }]
    );
}

#[test]
fn warn_section_level_skipped_markdown() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("## Level 1\n\n#### Level 3 (skipped level 2)"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.level(), 1);
    assert_eq!(mi.item.section_title(), "Level 1");
    assert_eq!(mi.item.nested_blocks().len(), 1);

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "#### Level 3 (skipped level 2)",
                line: 3,
                col: 1,
                offset: 12,
            },
            warning: WarningType::SectionHeadingLevelSkipped(1, 3),
        }]
    );
}

#[test]
fn warn_multiple_section_levels_skipped() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Level 1\n\n===== Level 4 (skipped levels 2 and 3)"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.level(), 1);
    assert_eq!(mi.item.section_title(), "Level 1");
    assert_eq!(mi.item.nested_blocks().len(), 1);

    assert_eq!(
        warnings,
        vec![Warning {
            source: Span {
                data: "===== Level 4 (skipped levels 2 and 3)",
                line: 3,
                col: 1,
                offset: 12,
            },
            warning: WarningType::SectionHeadingLevelSkipped(1, 4),
        }]
    );
}

#[test]
fn no_warning_for_consecutive_section_levels() {
    let mut parser = Parser::default();
    let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

    let mi = crate::blocks::SectionBlock::parse(
        &BlockMetadata::new("== Level 1\n\n=== Level 2 (no skip)"),
        &mut parser,
        &mut warnings,
    )
    .unwrap();

    assert_eq!(mi.item.level(), 1);
    assert_eq!(mi.item.section_title(), "Level 1");
    assert_eq!(mi.item.nested_blocks().len(), 1);

    assert!(warnings.is_empty());
}

#[test]
fn section_id_generation_basic() {
    let input = "== Section One";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), Some("_section_one"));
    } else {
        panic!("Expected section block");
    }
}

#[test]
fn section_id_generation_with_special_characters() {
    let input = "== We're back! & Company";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), Some("_were_back_company"));
    } else {
        panic!("Expected section block");
    }
}

#[test]
fn section_id_generation_with_entities() {
    let input = "== Ben &amp; Jerry &#34;Ice Cream&#34;";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), Some("_ben_jerry_ice_cream"));
    } else {
        panic!("Expected section block");
    }
}

#[test]
fn section_id_generation_disabled_when_sectids_unset() {
    let input = ":!sectids:\n\n== Section One";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), None);
    } else {
        panic!("Expected section block");
    }
}

#[test]
fn section_id_generation_with_custom_prefix() {
    let input = ":idprefix: id_\n\n== Section One";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), Some("id_section_one"));
    } else {
        panic!("Expected section block");
    }
}

#[test]
fn section_id_generation_with_custom_separator() {
    let input = ":idseparator: -\n\n== Section One";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), Some("_section-one"));
    } else {
        panic!("Expected section block");
    }
}

#[test]
fn section_id_generation_with_empty_prefix() {
    let input = ":idprefix:\n\n== Section One";
    let mut parser = Parser::default();
    let document = parser.parse(input);

    if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
        assert_eq!(section.id(), Some("section_one"));
    } else {
        panic!("Expected section block");
    }
}
