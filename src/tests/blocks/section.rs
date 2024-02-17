use std::ops::Deref;

use nom::{
    bytes::complete::take,
    error::{Error, ErrorKind},
    Err,
};
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
    let (_, b1) = SectionBlock::parse(Span::new("== Section Title", true)).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn err_empty_source() {
    let expected_err = Err::Error(Error::new(Span::new("", true), ErrorKind::TakeTill1));

    let actual_err = SectionBlock::parse(Span::new("", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_only_spaces() {
    let err_span: nom_span::Spanned<&str> = Span::new("    ", true);
    let (err_span, _) = take::<usize, Span, Error<Span>>(4)(err_span).unwrap();

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::TakeTill1));

    let actual_err = SectionBlock::parse(Span::new("    ", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_not_section() {
    let err_span: nom_span::Spanned<&str> = Span::new("blah blah", true);

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::Many1Count));

    let actual_err = SectionBlock::parse(Span::new("blah blah", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_missing_space_before_title() {
    let err_span: nom_span::Spanned<&str> = Span::new("=blah blah", true);
    let (err_span, _) = take::<usize, Span, Error<Span>>(1)(err_span).unwrap();

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::Space));

    let actual_err = SectionBlock::parse(Span::new("=blah blah", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn simplest_section_block() {
    let (rem, block) = SectionBlock::parse(Span::new("== Section Title", true)).unwrap();

    assert_eq!(block.content_model(), ContentModel::Compound);
    assert_eq!(block.context().deref(), "section");

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 17,
            offset: 16
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn has_child_block() {
    let (rem, block) = SectionBlock::parse(Span::new("== Section Title\n\nabc", true)).unwrap();

    assert_eq!(block.content_model(), ContentModel::Compound);
    assert_eq!(block.context().deref(), "section");

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 3,
            col: 4,
            offset: 21
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn stop_at_peer_block() {
    let (rem, block) = SectionBlock::parse(Span::new(
        "== Section Title\n\nabc\n\n== Section 2\n\ndef",
        true,
    ))
    .unwrap();

    assert_eq!(block.content_model(), ContentModel::Compound);
    assert_eq!(block.context().deref(), "section");

    assert_eq!(
        rem,
        TSpan {
            data: "== Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 23
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn stop_at_ancestor_block() {
    let (rem, block) = SectionBlock::parse(Span::new(
        "=== Section Title\n\nabc\n\n== Section 2\n\ndef",
        true,
    ))
    .unwrap();

    assert_eq!(block.content_model(), ContentModel::Compound);
    assert_eq!(block.context().deref(), "section");

    assert_eq!(
        rem,
        TSpan {
            data: "== Section 2\n\ndef",
            line: 5,
            col: 1,
            offset: 24
        }
    );

    assert_eq!(
        block,
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
}
