use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{preamble::Preamble, ContentModel, IsBlock, MacroBlock},
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::TMacroBlock,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = MacroBlock::parse(&Preamble::new("foo::[]"))
        .unwrap_if_no_warnings()
        .unwrap()
        .item;

    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn err_empty_source() {
    assert!(MacroBlock::parse(&Preamble::new(""))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn err_only_spaces() {
    assert!(MacroBlock::parse(&Preamble::new("    "))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn err_macro_name_not_ident() {
    let maw = MacroBlock::parse(&Preamble::new("98xyz::bar[blah,blap]"));

    assert!(maw.item.is_none());

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "98xyz::bar[blah,blap]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::InvalidMacroName,
        }]
    );
}

#[test]
fn err_missing_double_colon() {
    let maw = MacroBlock::parse(&Preamble::new("foo:bar[blah,blap]"));

    assert!(maw.item.is_none());

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: ":bar[blah,blap]",
                line: 1,
                col: 4,
                offset: 3,
            },
            warning: WarningType::MacroMissingDoubleColon,
        }]
    );
}

#[test]
fn err_missing_macro_attrlist() {
    let maw = MacroBlock::parse(&Preamble::new("foo::barblah,blap]"));

    assert!(maw.item.is_none());

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: "",
                line: 1,
                col: 19,
                offset: 18,
            },
            warning: WarningType::MacroMissingAttributeList,
        }]
    );
}

#[test]
fn err_no_attr_list() {
    assert!(MacroBlock::parse(&Preamble::new("foo::bar"))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn err_attr_list_not_closed() {
    assert!(MacroBlock::parse(&Preamble::new("foo::bar[blah"))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn err_unexpected_after_attr_list() {
    assert!(MacroBlock::parse(&Preamble::new("foo::bar[blah]bonus"))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn simplest_block_macro() {
    let mi = MacroBlock::parse(&Preamble::new("foo::[]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Simple);
    assert_eq!(mi.item.raw_context().deref(), "paragraph");
    assert_eq!(mi.item.resolved_context().deref(), "paragraph");
    assert!(mi.item.declared_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.attrlist().is_none());

    assert_eq!(
        mi.item,
        TMacroBlock {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            },
            target: None,
            macro_attrlist: TAttrlist {
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
            title: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
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
    let mi = MacroBlock::parse(&Preamble::new("foo::bar[]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
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
            macro_attrlist: TAttrlist {
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
            title: None,
            attrlist: None,
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

#[test]
fn has_target_and_attrlist() {
    let mi = MacroBlock::parse(&Preamble::new("foo::bar[blah]"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
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
            macro_attrlist: TAttrlist {
                attributes: vec!(TElementAttribute {
                    name: None,
                    shorthand_items: vec![TSpan {
                        data: "blah",
                        line: 1,
                        col: 10,
                        offset: 9,
                    }],
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
            title: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 15,
            offset: 14
        }
    );
}

#[test]
fn err_duplicate_comma() {
    let maw = MacroBlock::parse(&Preamble::new("foo::bar[blah,,blap]"));

    let mi = maw.item.unwrap().clone();

    assert_eq!(
        mi.item,
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
            macro_attrlist: TAttrlist {
                attributes: vec!(
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![TSpan {
                            data: "blah",
                            line: 1,
                            col: 10,
                            offset: 9,
                        }],
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
                    },
                    TElementAttribute {
                        name: None,
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "blap",
                            line: 1,
                            col: 16,
                            offset: 15,
                        },
                        source: TSpan {
                            data: "blap",
                            line: 1,
                            col: 16,
                            offset: 15,
                        },
                    }
                ),
                source: TSpan {
                    data: "blah,,blap",
                    line: 1,
                    col: 10,
                    offset: 9,
                }
            },
            source: TSpan {
                data: "foo::bar[blah,,blap]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 21,
            offset: 20
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: TSpan {
                data: ",",
                line: 1,
                col: 14,
                offset: 13,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}
