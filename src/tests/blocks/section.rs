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
fn simplest_section_block() {
    let (rem, block) = SectionBlock::parse(Span::new("== Section Title", true)).unwrap();

    assert_eq!(block.content_model(), ContentModel::Compound);

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

// #[test]
// fn has_target() {
//     let (rem, block) = SectionBlock::parse(Span::new("foo::bar[]",
// true)).unwrap();

//     assert_eq!(
//         rem,
//         TSpan {
//             data: "",
//             line: 1,
//             col: 11,
//             offset: 10
//         }
//     );

//     assert_eq!(
//         block,
//         TSectionBlock {
//             name: TSpan {
//                 data: "foo",
//                 line: 1,
//                 col: 1,
//                 offset: 0,
//             },
//             target: Some(TSpan {
//                 data: "bar",
//                 line: 1,
//                 col: 6,
//                 offset: 5,
//             }),
//             attrlist: None,
//             source: TSpan {
//                 data: "foo::bar[]",
//                 line: 1,
//                 col: 1,
//                 offset: 0,
//             },
//         }
//     );
// }

// #[test]
// fn has_target_and_attrlist() {
//     let (rem, block) = SectionBlock::parse(Span::new("foo::bar[blah]",
// true)).unwrap();

//     assert_eq!(
//         rem,
//         TSpan {
//             data: "",
//             line: 1,
//             col: 15,
//             offset: 14
//         }
//     );

//     assert_eq!(
//         block,
//         TSectionBlock {
//             name: TSpan {
//                 data: "foo",
//                 line: 1,
//                 col: 1,
//                 offset: 0,
//             },
//             target: Some(TSpan {
//                 data: "bar",
//                 line: 1,
//                 col: 6,
//                 offset: 5,
//             }),
//             attrlist: Some(TSpan {
//                 data: "blah",
//                 line: 1,
//                 col: 10,
//                 offset: 9,
//             }),

//             source: TSpan {
//                 data: "foo::bar[blah]",
//                 line: 1,
//                 col: 1,
//                 offset: 0,
//             },
//         }
//     );
// }
