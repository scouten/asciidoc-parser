use pretty_assertions_sorted::assert_eq;

use crate::{
    inlines::InlineMacro,
    tests::fixtures::{inlines::TInlineMacro, TSpan},
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = InlineMacro::parse(Span::new("foo:[]")).unwrap().t;
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
fn simplest_block_macro() {
    let pr = InlineMacro::parse(Span::new("foo:[]")).unwrap();

    assert_eq!(
        pr.t,
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
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 7,
            offset: 6
        }
    );
}

#[test]
fn has_target() {
    let pr = InlineMacro::parse(Span::new("foo:bar[]")).unwrap();

    assert_eq!(
        pr.t,
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
        pr.rem,
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
    let pr = InlineMacro::parse(Span::new("foo:bar[blah]")).unwrap();

    assert_eq!(
        pr.t,
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
        pr.rem,
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
    let pr = InlineMacro::parse(Span::new("foo:bar[blah]bonus")).unwrap();

    assert_eq!(
        pr.t,
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
        pr.rem,
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

    let pr = InlineMacro::parse(Span::new("foo::bar[]")).unwrap();

    assert_eq!(
        pr.t,
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
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 11,
            offset: 10
        }
    );
}
