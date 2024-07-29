use std::ops::Deref;

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
    let b1 = MacroBlock::parse(Span::new("foo::[]")).unwrap().t;
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn err_empty_source() {
    assert!(MacroBlock::parse(Span::new("")).is_none());
}

#[test]
fn err_only_spaces() {
    assert!(MacroBlock::parse(Span::new("    ")).is_none());
}

#[test]
fn err_not_ident() {
    assert!(MacroBlock::parse(Span::new("foo^xyz::bar[]")).is_none());
}
#[test]
fn err_inline_syntax() {
    assert!(MacroBlock::parse(Span::new("foo:bar[]")).is_none());
}

#[test]
fn err_no_attr_list() {
    assert!(MacroBlock::parse(Span::new("foo::bar")).is_none());
}

#[test]
fn err_attr_list_not_closed() {
    assert!(MacroBlock::parse(Span::new("foo::bar[blah")).is_none());
}

#[test]
fn err_unexpected_after_attr_list() {
    assert!(MacroBlock::parse(Span::new("foo::bar[blah]bonus")).is_none());
}

#[test]
fn simplest_block_macro() {
    let pr = MacroBlock::parse(Span::new("foo::[]")).unwrap();

    assert_eq!(pr.t.content_model(), ContentModel::Simple);
    assert_eq!(pr.t.context().deref(), "paragraph");

    assert_eq!(
        pr.t,
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

    assert_eq!(
        pr.rem,
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
    let pr = MacroBlock::parse(Span::new("foo::bar[]")).unwrap();

    assert_eq!(
        pr.t,
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

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 11,
            offset: 10
        }
    );
}

#[test]
fn has_target_and_attrlist() {
    let pr = MacroBlock::parse(Span::new("foo::bar[blah]")).unwrap();

    assert_eq!(
        pr.t,
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

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 15,
            offset: 14
        }
    );
}
