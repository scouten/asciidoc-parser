use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser, Span,
    blocks::{Block, ContentModel, IsBlock},
    content::SubstitutionGroup,
    document::Attribute,
    tests::fixtures::{
        TSpan,
        blocks::TBlock,
        document::{TAttribute, TInterpretedValue},
    },
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let h1 = Attribute::parse(Span::new(":foo: bar"), &Parser::default()).unwrap();
    let h2 = h1.clone();
    assert_eq!(h1, h2);
}

#[test]
fn simple_value() {
    let mi = Attribute::parse(Span::new(":foo: bar\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(TSpan {
                data: "bar",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar"),
            source: TSpan {
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
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 10
        }
    );
}

#[test]
fn no_value() {
    let mi = Attribute::parse(Span::new(":foo:\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Set,
            source: TSpan {
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
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 6
        }
    );
}

#[test]
fn name_with_hyphens() {
    let mi = Attribute::parse(Span::new(":name-with-hyphen:"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "name-with-hyphen",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Set,
            source: TSpan {
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
        TSpan {
            data: "",
            line: 1,
            col: 19,
            offset: 18
        }
    );
}

#[test]
fn unset_prefix() {
    let mi = Attribute::parse(Span::new(":!foo:\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 3,
                offset: 2,
            },
            value_source: None,
            value: TInterpretedValue::Unset,
            source: TSpan {
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
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );
}

#[test]
fn unset_postfix() {
    let mi = Attribute::parse(Span::new(":foo!:\nblah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Unset,
            source: TSpan {
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
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );
}

#[test]
fn err_unset_prefix_and_postfix() {
    assert!(Attribute::parse(Span::new(":!foo!:\nblah"), &Parser::default()).is_none());
}

#[test]
fn err_invalid_ident1() {
    assert!(Attribute::parse(Span::new(":@invalid:\nblah"), &Parser::default()).is_none());
}

#[test]
fn err_invalid_ident2() {
    assert!(Attribute::parse(Span::new(":invalid@:\nblah"), &Parser::default()).is_none());
}

#[test]
fn err_invalid_ident3() {
    assert!(Attribute::parse(Span::new(":-invalid:\nblah"), &Parser::default()).is_none());
}

#[test]
fn value_with_soft_wrap() {
    let mi = Attribute::parse(Span::new(":foo: bar \\\n blah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(TSpan {
                data: "bar \\\n blah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar blah"),
            source: TSpan {
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
        TSpan {
            data: "",
            line: 2,
            col: 6,
            offset: 17
        }
    );
}

#[test]
fn value_with_hard_wrap() {
    let mi = Attribute::parse(Span::new(":foo: bar + \\\n blah"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(TSpan {
                data: "bar + \\\n blah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar\nblah"),
            source: TSpan {
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
        TSpan {
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
    let maw = Block::parse(Span::new(":foo: bar\nblah"), &mut parser);

    let mi = maw.item.unwrap();
    let block = mi.item;

    assert_eq!(
        block,
        TBlock::DocumentAttribute(TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: Some(TSpan {
                data: "bar",
                line: 1,
                col: 7,
                offset: 6,
            }),
            value: TInterpretedValue::Value("bar"),
            source: TSpan {
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

    let Block::DocumentAttribute(attr) = block else {
        panic!("Wrong type");
    };

    assert_eq!(attr.value(), TInterpretedValue::Value("bar"));

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 10
        }
    );
}
