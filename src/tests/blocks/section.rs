use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock, SectionBlock},
    tests::fixtures::{
        blocks::{TBlock, TSectionBlock, TSimpleBlock},
        inlines::TInline,
        TSpan,
    },
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = SectionBlock::parse(Span::new("== Section Title")).unwrap();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    assert!(SectionBlock::parse(Span::new("")).is_none());
}

#[test]
fn err_only_spaces() {
    assert!(SectionBlock::parse(Span::new("    ")).is_none());
}

#[test]
fn err_not_section() {
    assert!(SectionBlock::parse(Span::new("blah blah")).is_none());
}

#[test]
fn err_missing_space_before_title() {
    assert!(SectionBlock::parse(Span::new("=blah blah")).is_none());
}

#[test]
fn simplest_section_block() {
    let mi = SectionBlock::parse(Span::new("== Section Title")).unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
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
    let mi = SectionBlock::parse(Span::new("== Section Title\n\nabc")).unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
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
fn dont_stop_at_peer_section() {
    let mi =
        SectionBlock::parse(Span::new("== Section Title\n\nabc\n\n=== Section 2\n\ndef")).unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 1,
            title: TSpan {
                data: "Section Title",
                line: 1,
                col: 4,
                offset: 3,
            },
            blocks: vec![
                TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 18,
                }),)),
                TBlock::Section(TSectionBlock {
                    level: 2,
                    title: TSpan {
                        data: "Section 2",
                        line: 5,
                        col: 5,
                        offset: 27,
                    },
                    blocks: vec![TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                        TSpan {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 38,
                        }
                    ),))],
                    source: TSpan {
                        data: "=== Section 2\n\ndef",
                        line: 5,
                        col: 1,
                        offset: 23,
                    }
                })
            ],
            source: TSpan {
                data: "== Section Title\n\nabc\n\n=== Section 2\n\ndef",
                line: 1,
                col: 1,
                offset: 0,
            }
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
    let mi =
        SectionBlock::parse(Span::new("== Section Title\n\nabc\n\n== Section 2\n\ndef")).unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
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
                // TO DO: Fix bug that includes blank lines.
                data: "== Section Title\n\nabc\n\n",
                line: 1,
                col: 1,
                offset: 0,
            },
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
    let mi =
        SectionBlock::parse(Span::new("=== Section Title\n\nabc\n\n== Section 2\n\ndef")).unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().deref(), "section");

    assert_eq!(
        mi.item,
        TSectionBlock {
            level: 2,
            title: TSpan {
                data: "Section Title",
                line: 1,
                col: 5,
                offset: 4,
            },
            blocks: vec![TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                }
            )))],
            source: TSpan {
                // TO DO: Fix bug that includes blank lines.
                data: "=== Section Title\n\nabc\n\n",
                line: 1,
                col: 1,
                offset: 0,
            },
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
