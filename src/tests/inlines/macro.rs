use nom::{
    bytes::complete::take,
    error::{Error, ErrorKind},
    Err,
};
use pretty_assertions_sorted::assert_eq;

use crate::{
    inlines::InlineMacro,
    tests::fixtures::{inlines::TInlineMacro, TSpan},
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let (_, b1) = InlineMacro::parse(Span::new("foo:[]")).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let expected_err = Err::Error(Error::new(Span::new(""), ErrorKind::Tag));

    let actual_err = InlineMacro::parse(Span::new("")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn only_spaces() {
    let expected_err = Err::Error(Error::new(Span::new("    "), ErrorKind::Tag));

    let actual_err = InlineMacro::parse(Span::new("    ")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_not_ident() {
    let err_span = Span::new("foo^xyz:bar[]");
    let (err_span, _) = take::<usize, Span, Error<Span>>(3)(err_span).unwrap();

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::Tag));

    let actual_err = InlineMacro::parse(Span::new("foo^xyz:bar[]")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_no_attr_list() {
    let err_span = Span::new("foo:bar");
    let (err_span, _) = take::<usize, Span, Error<Span>>(4)(err_span).unwrap();

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::TakeUntil));

    let actual_err = InlineMacro::parse(Span::new("foo:bar")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn err_attr_list_not_closed() {
    let err_span = Span::new("foo:bar[blah");
    let (err_span, _) = take::<usize, Span, Error<Span>>(8)(err_span).unwrap();

    let expected_err = Err::Error(Error::new(err_span, ErrorKind::TakeUntil));

    let actual_err = InlineMacro::parse(Span::new("foo:bar[blah")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn simplest_block_macro() {
    let (rem, block) = InlineMacro::parse(Span::new("foo:[]")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 7,
            offset: 6
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn has_target() {
    let (rem, block) = InlineMacro::parse(Span::new("foo:bar[]")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 10,
            offset: 9
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn has_target_and_attrlist() {
    let (rem, block) = InlineMacro::parse(Span::new("foo:bar[blah]")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 14,
            offset: 13
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn doesnt_consume_after_attr_list() {
    let (rem, block) = InlineMacro::parse(Span::new("foo:bar[blah]bonus")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "bonus",
            line: 1,
            col: 14,
            offset: 13
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn okish_block_syntax() {
    // TO DO: Should this be an error? Or is the second colon part of the target?

    let (rem, block) = InlineMacro::parse(Span::new("foo::bar[]")).unwrap();

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
}
