use pretty_assertions_sorted::assert_eq;

use crate::{
    inlines::{Inline, InlineMacro},
    span::HasSpan,
    tests::fixtures::{
        inlines::{TInline, TInlineMacro},
        TSpan,
    },
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = InlineMacro::parse(Span::new("foo:[]")).unwrap().item;
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    assert!(InlineMacro::parse(Span::new("")).is_none());
}

#[test]
fn only_spaces() {
    assert!(InlineMacro::parse(Span::new("    ")).is_none());
}

#[test]
fn err_not_ident() {
    assert!(InlineMacro::parse(Span::new("foo^xyz:bar[]")).is_none());
}

#[test]
fn err_no_attr_list() {
    assert!(InlineMacro::parse(Span::new("foo:bar")).is_none());
}

#[test]
fn err_attr_list_not_closed() {
    assert!(InlineMacro::parse(Span::new("foo:bar[blah")).is_none());
}

#[test]
fn simplest_inline_macro() {
    let mi = InlineMacro::parse(Span::new("foo:[]")).unwrap();

    assert_eq!(
        mi.item,
        TInlineMacro {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: None,
            attrlist: None,
            source: TSpan {
                data: "foo:[]",
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
            col: 7,
            offset: 6
        }
    );
}

#[test]
fn simplest_inline_macro_via_inline_parse() {
    let mi = Inline::parse(Span::new("foo:[]")).unwrap();

    assert_eq!(
        mi.item,
        TInline::Macro(TInlineMacro {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: None,
            attrlist: None,
            source: TSpan {
                data: "foo:[]",
                line: 1,
                col: 1,
                offset: 0,
            },
        })
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 7,
            offset: 6
        }
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "foo:[]",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}

#[test]
fn has_target() {
    let mi = InlineMacro::parse(Span::new("foo:bar[]")).unwrap();

    assert_eq!(
        mi.item,
        TInlineMacro {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: "bar",
                line: 1,
                col: 5,
                offset: 4,
            }),
            attrlist: None,
            source: TSpan {
                data: "foo:bar[]",
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
            col: 10,
            offset: 9
        }
    );
}

#[test]
fn has_target_and_attrlist() {
    let mi = InlineMacro::parse(Span::new("foo:bar[blah]")).unwrap();

    assert_eq!(
        mi.item,
        TInlineMacro {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: "bar",
                line: 1,
                col: 5,
                offset: 4,
            }),
            attrlist: Some(TSpan {
                data: "blah",
                line: 1,
                col: 9,
                offset: 8,
            }),

            source: TSpan {
                data: "foo:bar[blah]",
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
            col: 14,
            offset: 13
        }
    );
}

#[test]
fn doesnt_consume_after_attr_list() {
    let mi = InlineMacro::parse(Span::new("foo:bar[blah]bonus")).unwrap();

    assert_eq!(
        mi.item,
        TInlineMacro {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: "bar",
                line: 1,
                col: 5,
                offset: 4,
            }),
            attrlist: Some(TSpan {
                data: "blah",
                line: 1,
                col: 9,
                offset: 8,
            }),

            source: TSpan {
                data: "foo:bar[blah]",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "bonus",
            line: 1,
            col: 14,
            offset: 13
        }
    );
}

#[test]
fn okish_block_syntax() {
    // TO DO: Should this be an error? Or is the second colon part of the target?

    let mi = InlineMacro::parse(Span::new("foo::bar[]")).unwrap();

    assert_eq!(
        mi.item,
        TInlineMacro {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: Some(TSpan {
                data: ":bar",
                line: 1,
                col: 5,
                offset: 4,
            }),
            attrlist: None,

            source: TSpan {
                data: "foo::bar[]",
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
            col: 11,
            offset: 10
        }
    );
}
