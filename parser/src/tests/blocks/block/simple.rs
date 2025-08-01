use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    HasSpan, Parser, Span,
    blocks::{Block, ContentModel, IsBlock},
    content::SubstitutionGroup,
    tests::fixtures::{
        TSpan,
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TSimpleBlock},
        content::TContent,
        warnings::TWarning,
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();

    let b1 = Block::parse(Span::new("abc"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    let mut parser = Parser::default();

    assert!(
        Block::parse(Span::new(""), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn err_only_spaces() {
    let mut parser = Parser::default();

    assert!(
        Block::parse(Span::new("    "), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn single_line() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("abc"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "abc",
            },
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
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
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("abc\ndef"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "abc\ndef",
            },
            source: TSpan {
                data: "abc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new(".simple block\nabc\ndef\n"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc\ndef",
                    line: 2,
                    col: 1,
                    offset: 14,
                },
                rendered: "abc\ndef",
            },
            source: TSpan {
                data: ".simple block\nabc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: Some(TSpan {
                data: "simple block",
                line: 1,
                col: 2,
                offset: 1,
            },),
            title: Some("simple block"),
            anchor: None,
            attrlist: None,
        })
    );
}

#[test]
fn attrlist() {
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("[sidebar]\nabc\ndef\n"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc\ndef",
                    line: 2,
                    col: 1,
                    offset: 10,
                },
                rendered: "abc\ndef",
            },
            source: TSpan {
                data: "[sidebar]\nabc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: &[TElementAttribute {
                    name: None,
                    shorthand_items: &["sidebar"],
                    value: "sidebar"
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
            data: "[sidebar]\nabc\ndef",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert!(mi.item.anchor().is_none());

    assert_eq!(
        mi.item.attrlist().unwrap(),
        TAttrlist {
            attributes: &[TElementAttribute {
                name: None,
                shorthand_items: &["sidebar"],
                value: "sidebar"
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new(".title\n[sidebar]\nabc\ndef\n"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc\ndef",
                    line: 3,
                    col: 1,
                    offset: 17,
                },
                rendered: "abc\ndef",
            },
            source: TSpan {
                data: ".title\n[sidebar]\nabc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: Some(TSpan {
                data: "title",
                line: 1,
                col: 2,
                offset: 1,
            },),
            title: Some("title"),
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: &[TElementAttribute {
                    name: None,
                    shorthand_items: &["sidebar"],
                    value: "sidebar"
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
            data: ".title\n[sidebar]\nabc\ndef",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert!(mi.item.anchor().is_none());

    assert_eq!(
        mi.item.attrlist().unwrap(),
        TAttrlist {
            attributes: &[TElementAttribute {
                name: None,
                shorthand_items: &["sidebar"],
                value: "sidebar"
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
    let mut parser = Parser::default();

    let mi = Block::parse(Span::new("abc\n\ndef"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "abc",
            },
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
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

#[test]
fn with_block_anchor() {
    let mut parser = Parser::default();

    let mi = Block::parse(
        Span::new("[[notice]]\nThis paragraph gets a lot of attention.\n"),
        &mut parser,
    )
    .unwrap_if_no_warnings()
    .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "This paragraph gets a lot of attention.",
                    line: 2,
                    col: 1,
                    offset: 11,
                },
                rendered: "This paragraph gets a lot of attention.",
            },
            source: TSpan {
                data: "[[notice]]\nThis paragraph gets a lot of attention.",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: Some(TSpan {
                data: "notice",
                line: 1,
                col: 3,
                offset: 2,
            },),
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "[[notice]]\nThis paragraph gets a lot of attention.",
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
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item.anchor().unwrap(),
        TSpan {
            data: "notice",
            line: 1,
            col: 3,
            offset: 2,
        }
    );

    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 1,
            offset: 51
        }
    );
}

#[test]
fn err_empty_block_anchor() {
    let mut parser = Parser::default();

    let maw = Block::parse(
        Span::new("[[]]\nThis paragraph gets a lot of attention.\n"),
        &mut parser,
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "",
                line: 1,
                col: 3,
                offset: 2,
            },
            warning: WarningType::EmptyBlockAnchorName,
        },]
    );

    let mi = maw.item.unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "This paragraph gets a lot of attention.",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "This paragraph gets a lot of attention.",
            },
            source: TSpan {
                data: "[[]]\nThis paragraph gets a lot of attention.",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: Some(TSpan {
                data: "",
                line: 1,
                col: 3,
                offset: 2,
            },),
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "[[]]\nThis paragraph gets a lot of attention.",
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
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item.anchor().unwrap(),
        TSpan {
            data: "",
            line: 1,
            col: 3,
            offset: 2,
        }
    );

    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 1,
            offset: 45
        }
    );
}

#[test]
fn err_invalid_block_anchor() {
    let mut parser = Parser::default();

    let maw = Block::parse(
        Span::new("[[3 blind mice]]\nThis paragraph gets a lot of attention.\n"),
        &mut parser,
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "3 blind mice",
                line: 1,
                col: 3,
                offset: 2,
            },
            warning: WarningType::InvalidBlockAnchorName,
        },]
    );

    let mi = maw.item.unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "This paragraph gets a lot of attention.",
                    line: 2,
                    col: 1,
                    offset: 17,
                },
                rendered: "This paragraph gets a lot of attention.",
            },
            source: TSpan {
                data: "[[3 blind mice]]\nThis paragraph gets a lot of attention.",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: Some(TSpan {
                data: "3 blind mice",
                line: 1,
                col: 3,
                offset: 2,
            },),
            attrlist: None,
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "[[3 blind mice]]\nThis paragraph gets a lot of attention.",
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
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item.anchor().unwrap(),
        TSpan {
            data: "3 blind mice",
            line: 1,
            col: 3,
            offset: 2,
        }
    );

    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 1,
            offset: 57
        }
    );
}

#[test]
fn unterminated_block_anchor() {
    let mut parser = Parser::default();

    let mi = Block::parse(
        Span::new("[[notice]\nThis paragraph gets a lot of attention.\n"),
        &mut parser,
    )
    .unwrap_if_no_warnings()
    .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "This paragraph gets a lot of attention.",
                    line: 2,
                    col: 1,
                    offset: 10,
                },
                rendered: "This paragraph gets a lot of attention.",
            },
            source: TSpan {
                data: "[[notice]\nThis paragraph gets a lot of attention.",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: Some(TAttrlist {
                attributes: &[TElementAttribute {
                    name: None,
                    shorthand_items: &["[notice",],
                    value: "[notice"
                },],
                source: TSpan {
                    data: "[notice",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            },),
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "[[notice]\nThis paragraph gets a lot of attention.",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Simple);
    assert_eq!(mi.item.raw_context().deref(), "paragraph");
    assert_eq!(mi.item.resolved_context().deref(), "paragraph");
    assert_eq!(mi.item.declared_style().unwrap(), "[notice");
    assert_eq!(mi.item.nested_blocks().next(), None);

    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        mi.item.attrlist().unwrap(),
        TAttrlist {
            attributes: &[TElementAttribute {
                name: None,
                shorthand_items: &["[notice"],
                value: "[notice"
            },],
            source: TSpan {
                data: "[notice",
                line: 1,
                col: 2,
                offset: 1,
            },
        },
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 3,
            col: 1,
            offset: 50
        }
    );
}
