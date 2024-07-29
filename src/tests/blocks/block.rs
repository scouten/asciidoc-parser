mod simple {
    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
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
        let b1 = Block::parse(Span::new("abc")).unwrap();
        let b2 = b1.t.clone();
        assert_eq!(b1.t, b2);
    }

    #[test]
    fn err_empty_source() {
        assert!(Block::parse(Span::new("")).is_none());
    }

    #[test]
    fn err_only_spaces() {
        assert!(Block::parse(Span::new("    ")).is_none());
    }

    #[test]
    fn single_line() {
        let pr = Block::parse(Span::new("abc")).unwrap();

        assert_eq!(
            pr.t,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(pr.t.content_model(), ContentModel::Simple);
        assert_eq!(pr.t.context().deref(), "paragraph");
        assert_eq!(pr.t.nested_blocks().next(), None);

        assert_eq!(
            pr.rem,
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
        let pr = Block::parse(Span::new("abc\ndef")).unwrap();

        assert_eq!(
            pr.t,
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
            pr.t.span(),
            TSpan {
                data: "abc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 7
            }
        );
    }

    #[test]
    fn consumes_blank_lines_after() {
        let pr = Block::parse(Span::new("abc\n\ndef")).unwrap();

        assert_eq!(
            pr.t,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 3,
                col: 1,
                offset: 5
            }
        );
    }
}

mod r#macro {
    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            attributes::{TAttrlist, TElementAttribute},
            blocks::{TBlock, TMacroBlock, TSimpleBlock},
            inlines::{TInline, TInlineMacro},
            TSpan,
        },
        HasSpan, Span,
    };

    // NOTE: The "error" cases from the MacroBlock parser test suite are not
    // necessarily error cases here because we can reparse as SimpleBlock.

    #[test]
    fn err_inline_syntax() {
        let pr = Block::parse(Span::new("foo:bar[]")).unwrap();

        assert_eq!(
            pr.t,
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
            pr.t.span(),
            TSpan {
                data: "foo:bar[]",
                line: 1,
                col: 1,
                offset: 0,
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
    fn err_no_attr_list() {
        let pr = Block::parse(Span::new("foo::bar")).unwrap();

        assert_eq!(
            pr.t,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "foo::bar",
                line: 1,
                col: 1,
                offset: 0,
            }))),
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "foo::bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 9,
                offset: 8
            }
        );
    }

    #[test]
    fn err_attr_list_not_closed() {
        let pr = Block::parse(Span::new("foo::bar[blah")).unwrap();

        assert_eq!(
            pr.t,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "foo::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "foo::bar[blah",
                line: 1,
                col: 1,
                offset: 0,
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
    fn err_unexpected_after_attr_list() {
        let pr = Block::parse(Span::new("foo::bar[blah]bonus")).unwrap();

        assert_eq!(
            pr.t,
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
            pr.t.span(),
            TSpan {
                data: "foo::bar[blah]bonus",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 20,
                offset: 19
            }
        );
    }

    #[test]
    fn simplest_block_macro() {
        let pr = Block::parse(Span::new("foo::[]")).unwrap();

        assert_eq!(
            pr.t,
            TBlock::Macro(TMacroBlock {
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
            })
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "foo::[]",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(pr.t.content_model(), ContentModel::Simple);
        assert_eq!(pr.t.context().deref(), "paragraph");
        assert_eq!(pr.t.nested_blocks().next(), None);

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
        let pr = Block::parse(Span::new("foo::bar[]")).unwrap();

        assert_eq!(
            pr.t,
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
            })
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "foo::bar[]",
                line: 1,
                col: 1,
                offset: 0,
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
        let pr = Block::parse(Span::new("foo::bar[blah]")).unwrap();

        assert_eq!(
            pr.t,
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
            })
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "foo::bar[blah]",
                line: 1,
                col: 1,
                offset: 0,
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
}

mod section {
    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TSectionBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        HasSpan, Span,
    };

    #[test]
    fn err_missing_space_before_title() {
        let pr = Block::parse(Span::new("=blah blah")).unwrap();

        assert_eq!(
            pr.t,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "=blah blah",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "=blah blah",
                line: 1,
                col: 1,
                offset: 0,
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
    fn simplest_section_block() {
        let pr = Block::parse(Span::new("== Section Title")).unwrap();

        assert_eq!(pr.t.content_model(), ContentModel::Compound);
        assert_eq!(pr.t.context().deref(), "section");

        assert_eq!(
            pr.t,
            TBlock::Section(TSectionBlock {
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
            })
        );

        assert_eq!(pr.t.nested_blocks().next(), None);

        assert_eq!(
            pr.rem,
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
        let pr = Block::parse(Span::new("== Section Title\n\nabc")).unwrap();

        assert_eq!(pr.t.content_model(), ContentModel::Compound);
        assert_eq!(pr.t.context().deref(), "section");

        assert_eq!(
            pr.t,
            TBlock::Section(TSectionBlock {
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
            })
        );

        let mut nested_blocks = pr.t.nested_blocks();

        assert_eq!(
            nested_blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 3,
                col: 1,
                offset: 18,
            })))
        );

        assert_eq!(nested_blocks.next(), None);

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "== Section Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 3,
                col: 4,
                offset: 21
            }
        );
    }

    // TO DO: Add more test cases here as SectionBlock is finalized.
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
