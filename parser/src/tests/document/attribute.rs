use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    HasSpan, Parser,
    blocks::{Block, ContentModel, IsBlock},
    content::SubstitutionGroup,
    document::Attribute,
    parser::ModificationContext,
    tests::fixtures::{
        Span,
        blocks::{TBlock, TSimpleBlock},
        content::TContent,
        document::{TAttribute, TInterpretedValue},
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let h1 = Attribute::parse(crate::Span::new(":foo: bar"), &Parser::default()).unwrap();
    let h2 = h1.clone();
    assert_eq!(h1, h2);
}

#[test]
fn simple_value() {
    let mi = Attribute::parse(crate::Span::new(":foo: bar\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(Span {
                data: "bar",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar"),
            source: Span {
                data: ":foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Value("bar"));

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 2,
            col: 1,
            offset: 10
        }
    );
}

#[test]
fn no_value() {
    let mi = Attribute::parse(crate::Span::new(":foo:\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Set,
            source: Span {
                data: ":foo:",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Set);

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 2,
            col: 1,
            offset: 6
        }
    );
}

#[test]
fn name_with_hyphens() {
    let mi = Attribute::parse(crate::Span::new(":name-with-hyphen:"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "name-with-hyphen",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Set,
            source: Span {
                data: ":name-with-hyphen:",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Set);

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 19,
            offset: 18
        }
    );
}

#[test]
fn unset_prefix() {
    let mi = Attribute::parse(crate::Span::new(":!foo:\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 3,
                offset: 2,
            },
            value_source: None,
            value: TInterpretedValue::Unset,
            source: Span {
                data: ":!foo:",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Unset);

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );
}

#[test]
fn unset_postfix() {
    let mi = Attribute::parse(crate::Span::new(":foo!:\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Unset,
            source: Span {
                data: ":foo!:",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Unset);

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );
}

#[test]
fn err_unset_prefix_and_postfix() {
    assert!(Attribute::parse(crate::Span::new(":!foo!:\nblah"), &Parser::default()).is_none());
}

#[test]
fn err_invalid_ident1() {
    assert!(Attribute::parse(crate::Span::new(":@invalid:\nblah"), &Parser::default()).is_none());
}

#[test]
fn err_invalid_ident2() {
    assert!(Attribute::parse(crate::Span::new(":invalid@:\nblah"), &Parser::default()).is_none());
}

#[test]
fn err_invalid_ident3() {
    assert!(Attribute::parse(crate::Span::new(":-invalid:\nblah"), &Parser::default()).is_none());
}

#[test]
fn value_with_soft_wrap() {
    let mi = Attribute::parse(crate::Span::new(":foo: bar \\\n blah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(Span {
                data: "bar \\\n blah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar blah"),
            source: Span {
                data: ":foo: bar \\\n blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Value("bar blah"));

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 2,
            col: 6,
            offset: 17
        }
    );
}

#[test]
fn value_with_hard_wrap() {
    let mi = Attribute::parse(
        crate::Span::new(":foo: bar + \\\n blah"),
        &Parser::default(),
    )
    .unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(Span {
                data: "bar + \\\n blah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar\nblah"),
            source: Span {
                data: ":foo: bar + \\\n blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TInterpretedValue::Value("bar\nblah"));

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 2,
            col: 6,
            offset: 19
        }
    );
}

#[test]
fn is_block() {
    let mut parser = Parser::default();
    let maw = Block::parse(crate::Span::new(":foo: bar\nblah"), &mut parser);

    let mi = maw.item.unwrap();
    let block = mi.item;

    assert_eq!(
        block,
        TBlock::DocumentAttribute(TAttribute {
            name: Span {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(Span {
                data: "bar",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar"),
            source: Span {
                data: ":foo: bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        })
    );

    assert_eq!(block.content_model(), ContentModel::Empty);
    assert_eq!(block.raw_context().deref(), "attribute");
    assert!(block.nested_blocks().next().is_none());
    assert!(block.title_source().is_none());
    assert!(block.title().is_none());
    assert!(block.anchor().is_none());
    assert!(block.attrlist().is_none());
    assert_eq!(block.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        block.span(),
        Span {
            data: ":foo: bar",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    let Block::DocumentAttribute(attr) = block else {
        panic!("Wrong type");
    };

    assert_eq!(attr.value(), TInterpretedValue::Value("bar"));

    assert_eq!(
        mi.after,
        Span {
            data: "blah",
            line: 2,
            col: 1,
            offset: 10
        }
    );
}

#[test]
fn affects_document_state() {
    let mut parser =
        Parser::default().with_intrinsic_attribute("agreed", "yes", ModificationContext::Anywhere);

    let doc =
        parser.parse("We are agreed? {agreed}\n\n:agreed: no\n\nAre we still agreed? {agreed}");

    let mut blocks = doc.nested_blocks();

    let block1 = blocks.next().unwrap();

    assert_eq!(
        block1,
        &TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: Span {
                    data: "We are agreed? {agreed}",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "We are agreed? yes",
            },
            source: Span {
                data: "We are agreed? {agreed}",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    let _ = blocks.next().unwrap();

    let block3 = blocks.next().unwrap();

    assert_eq!(
        block3,
        &TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: Span {
                    data: "Are we still agreed? {agreed}",
                    line: 5,
                    col: 1,
                    offset: 38,
                },
                rendered: "Are we still agreed? no",
            },
            source: Span {
                data: "Are we still agreed? {agreed}",
                line: 5,
                col: 1,
                offset: 38,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    let mut warnings = doc.warnings();
    assert!(warnings.next().is_none());
}

#[test]
fn block_enforces_permission() {
    let mut parser =
        Parser::default().with_intrinsic_attribute("agreed", "yes", ModificationContext::ApiOnly);

    let doc = parser.parse("Hello\n\n:agreed: no\n\nAre we agreed? {agreed}");

    let mut blocks = doc.nested_blocks();
    let _ = blocks.next().unwrap();
    let _ = blocks.next().unwrap();
    let block3 = blocks.next().unwrap();

    assert_eq!(
        block3,
        &TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: Span {
                    data: "Are we agreed? {agreed}",
                    line: 5,
                    col: 1,
                    offset: 20,
                },
                rendered: "Are we agreed? yes",
            },
            source: Span {
                data: "Are we agreed? {agreed}",
                line: 5,
                col: 1,
                offset: 20,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        })
    );

    let mut warnings = doc.warnings();
    let warning1 = warnings.next().unwrap();

    dbg!(&warning1);

    assert_eq!(
        &warning1.source,
        Span {
            data: ":agreed: no",
            line: 3,
            col: 1,
            offset: 7,
        }
    );

    assert_eq!(
        warning1.warning,
        WarningType::AttributeValueIsLocked("agreed".to_owned(),)
    );

    assert!(warnings.next().is_none());
}
