use std::ops::Deref;

use nom::{
    bytes::complete::take,
    error::{Error, ErrorKind},
    Err,
};
use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock, MacroBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::TMacroBlock,
        TSpan,
    },
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let (_, b1) = MacroBlock::parse(Span::new("foo::[]")).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn err_empty_source() {
    let expected_err = Err::Error(Error::new(Span::new(""), ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_only_spaces() {
    let expected_err = Err::Error(Error::new(Span::new(""), ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("    ")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_not_ident() {
    let err_span = Span::new("foo^xyz::bar[]");
    let (err_span, _) = take::<usize, Span, Error<Span>>(3)(err_span).unwrap();
    let (_, err_span) = take::<usize, Span, Error<Span>>(10)(err_span).unwrap();

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("foo^xyz::bar[]")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}
#[test]
fn err_inline_syntax() {
    let err_span = Span::new("foo:bar[]");
    let (err_span, _) = take::<usize, Span, Error<Span>>(3)(err_span).unwrap();
    let (_, err_span) = take::<usize, Span, Error<Span>>(5)(err_span).unwrap();

    let expected_err: Err<Error<Span>> = Err::Error(Error::new(err_span, ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("foo:bar[]")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_no_attr_list() {
    let err_span = Span::new("foo::bar");

    let expected_err: Err<Error<Span>> = Err::Error(Error::new(err_span, ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("foo::bar")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_attr_list_not_closed() {
    let err_span = Span::new("foo::bar[blah");

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("foo::bar[blah")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_unexpected_after_attr_list() {
    let err_span = Span::new("foo::bar[blah]bonus");

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::Tag));

    let actual_err = MacroBlock::parse(Span::new("foo::bar[blah]bonus")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn simplest_block_macro() {
    let (rem, block) = MacroBlock::parse(Span::new("foo::[]")).unwrap();

    assert_eq!(block.content_model(), ContentModel::Simple);
    assert_eq!(block.context().deref(), "paragraph");

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 8,
            offset: 7
        }
    );

    assert_eq!(
        block,
        TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: None,
            attrlist: TAttrlist {
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
        }
    );
}

#[test]
fn has_target() {
    let (rem, block) = MacroBlock::parse(Span::new("foo::bar[]")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 11,
            offset: 10
        }
    );

    assert_eq!(
        block,
        TMacroBlock {
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
            attrlist: TAttrlist {
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
        }
    );
}

#[test]
fn has_target_and_attrlist() {
    let (rem, block) = MacroBlock::parse(Span::new("foo::bar[blah]")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 15,
            offset: 14
        }
    );

    assert_eq!(
        block,
        TMacroBlock {
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
            attrlist: TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
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
        }
    );
}
