mod simple {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel},
        tests::fixtures::{
            blocks::{TBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        HasSpan, Span,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let (_, b1) = Block::parse(Span::new("abc", true)).unwrap();
        let b2 = b1.clone();
        assert_eq!(b1, b2);
    }

    #[test]
    fn empty_source() {
        let expected_err = Err::Error(Error::new(Span::new("", true), ErrorKind::TakeTill1));

        let actual_err = Block::parse(Span::new("", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn only_spaces() {
        let err = Block::parse(Span::new("    ", true)).unwrap_err();

        let Err::Error(e) = err else {
            panic!("Expected Err::Error: {err:#?}");
        };

        assert_eq!(e.code, ErrorKind::TakeTill1);

        assert_eq!(
            e.input,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn single_line() {
        let (rem, block) = Block::parse(Span::new("abc", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(block.content_model(), ContentModel::Simple);
    }

    #[test]
    fn multiple_lines() {
        let (rem, block) = Block::parse(Span::new("abc\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 7
            }
        );

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock(TInline::Sequence(
                vec![
                    TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }),
                    TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 2,
                        col: 1,
                        offset: 4,
                    }),
                ],
                TSpan {
                    data: "abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            )))
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "abc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn consumes_blank_lines_after() {
        let (rem, block) = Block::parse(Span::new("abc\n\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 3,
                col: 1,
                offset: 5
            }
        );

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod r#macro {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel},
        tests::fixtures::{
            blocks::{TBlock, TMacroBlock, TSimpleBlock},
            inlines::{TInline, TInlineMacro},
            TSpan,
        },
        HasSpan, Span,
    };

    // NOTE: The "error" cases from the MacroBlock parser are not
    // necessarily error cases here because we can reparse as SimpleBlock.

    #[test]
    fn err_inline_syntax() {
        let (rem, block) = Block::parse(Span::new("foo:bar[]", true)).unwrap();

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
            TBlock::Simple(TSimpleBlock(TInline::Macro(TInlineMacro {
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
                },),
                attrlist: None,
                source: TSpan {
                    data: "foo:bar[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }))),
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo:bar[]",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn err_no_attr_list() {
        let (rem, block) = Block::parse(Span::new("foo::bar", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 9,
                offset: 8
            }
        );

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "foo::bar",
                line: 1,
                col: 1,
                offset: 0,
            }))),
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo::bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn err_attr_list_not_closed() {
        let (rem, block) = Block::parse(Span::new("foo::bar[blah", true)).unwrap();

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
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "foo::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn err_unexpected_after_attr_list() {
        let (rem, block) = Block::parse(Span::new("foo::bar[blah]bonus", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 20,
                offset: 19
            }
        );

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock(TInline::Sequence(
                vec![
                    TInline::Macro(TInlineMacro {
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
                        },),
                        attrlist: Some(TSpan {
                            data: "blah",
                            line: 1,
                            col: 10,
                            offset: 9,
                        },),
                        source: TSpan {
                            data: "foo::bar[blah]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },),
                    TInline::Uninterpreted(TSpan {
                        data: "bonus",
                        line: 1,
                        col: 15,
                        offset: 14,
                    },),
                ],
                TSpan {
                    data: "foo::bar[blah]bonus",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            )))
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo::bar[blah]bonus",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn simplest_block_macro() {
        let (rem, block) = Block::parse(Span::new("foo::[]", true)).unwrap();

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
            TBlock::Macro(TMacroBlock {
                name: TSpan {
                    data: "foo",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                target: None,
                attrlist: None,
                source: TSpan {
                    data: "foo::[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            })
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo::[]",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(block.content_model(), ContentModel::Simple,);
    }

    #[test]
    fn has_target() {
        let (rem, block) = Block::parse(Span::new("foo::bar[]", true)).unwrap();

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
            TBlock::Macro(TMacroBlock {
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
                attrlist: None,
                source: TSpan {
                    data: "foo::bar[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            })
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn has_target_and_attrlist() {
        let (rem, block) = Block::parse(Span::new("foo::bar[blah]", true)).unwrap();

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
            TBlock::Macro(TMacroBlock {
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
                attrlist: Some(TSpan {
                    data: "blah",
                    line: 1,
                    col: 10,
                    offset: 9,
                }),

                source: TSpan {
                    data: "foo::bar[blah]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            })
        );

        assert_eq!(
            block.span(),
            TSpan {
                data: "foo::bar[blah]",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod content_model {
    use crate::blocks::ContentModel;

    #[test]
    fn impl_copy() {
        // Silly test to mark the #[derive(...)] line as covered.
        let c1 = ContentModel::Simple;
        let c2 = c1;
        assert_eq!(c1, c2);
    }
}
