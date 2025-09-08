use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    blocks::{ContentModel, IsBlock, MediaBlock, MediaType, metadata::BlockMetadata},
    content::SubstitutionGroup,
    tests::fixtures::{
        Span,
        attributes::{Attrlist, TElementAttribute},
        blocks::TMediaBlock,
        warnings::TWarning,
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();

    let b1 = MediaBlock::parse(&BlockMetadata::new("image::foo.jpg[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap()
        .item;

    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn err_empty_source() {
    let mut parser = Parser::default();

    assert!(
        MediaBlock::parse(&BlockMetadata::new(""), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn err_only_spaces() {
    let mut parser = Parser::default();

    assert!(
        MediaBlock::parse(&BlockMetadata::new("    "), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn err_macro_name_not_ident() {
    let mut parser = Parser::default();
    let maw = MediaBlock::parse(&BlockMetadata::new("98xyz::bar[blah,blap]"), &mut parser);

    assert!(maw.item.is_none());
    assert!(maw.warnings.is_empty());
}

#[test]
fn err_missing_double_colon() {
    let mut parser = Parser::default();
    let maw = MediaBlock::parse(&BlockMetadata::new("image:bar[blah,blap]"), &mut parser);

    assert!(maw.item.is_none());

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: Span {
                data: ":bar[blah,blap]",
                line: 1,
                col: 6,
                offset: 5,
            },
            warning: WarningType::MacroMissingDoubleColon,
        }]
    );
}

#[test]
fn err_missing_macro_attrlist() {
    let mut parser = Parser::default();
    let maw = MediaBlock::parse(&BlockMetadata::new("image::barblah,blap]"), &mut parser);

    assert!(maw.item.is_none());

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: Span {
                data: "",
                line: 1,
                col: 21,
                offset: 20,
            },
            warning: WarningType::MacroMissingAttributeList,
        }]
    );
}

#[test]
fn err_unknown_type() {
    let mut parser = Parser::default();

    assert!(
        MediaBlock::parse(&BlockMetadata::new("imagex::bar[]"), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn err_no_attr_list() {
    let mut parser = Parser::default();

    assert!(
        MediaBlock::parse(&BlockMetadata::new("image::bar"), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn err_attr_list_not_closed() {
    let mut parser = Parser::default();

    assert!(
        MediaBlock::parse(&BlockMetadata::new("image::bar[blah"), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn err_unexpected_after_attr_list() {
    let mut parser = Parser::default();

    assert!(
        MediaBlock::parse(&BlockMetadata::new("image::bar[blah]bonus"), &mut parser)
            .unwrap_if_no_warnings()
            .is_none()
    );
}

#[test]
fn simplest_block_macro() {
    let mut parser = Parser::default();

    let mi = MediaBlock::parse(&BlockMetadata::new("image::[]"), &mut parser);

    assert!(mi.item.is_none());

    assert_eq!(
        mi.warnings,
        vec![TWarning {
            source: Span {
                data: "[]",
                line: 1,
                col: 8,
                offset: 7,
            },
            warning: WarningType::MediaMacroMissingTarget,
        }]
    );
}

#[test]
fn has_target() {
    let mut parser = Parser::default();

    let mi = MediaBlock::parse(&BlockMetadata::new("image::bar[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TMediaBlock {
            type_: MediaType::Image,
            target: Span {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: Attrlist {
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: Span {
                data: "image::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 13,
            offset: 12
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Empty);
    assert_eq!(mi.item.raw_context().deref(), "image");
    assert!(mi.item.nested_blocks().next().is_none());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);
}

#[test]
fn has_target_and_attrlist() {
    let mut parser = Parser::default();

    let mi = MediaBlock::parse(&BlockMetadata::new("image::bar[blah]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TMediaBlock {
            type_: MediaType::Image,
            target: Span {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: Attrlist {
                attributes: &[TElementAttribute {
                    name: None,
                    shorthand_items: &["blah"],
                    value: "blah"
                }],
                source: Span {
                    data: "blah",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: Span {
                data: "image::bar[blah]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 17,
            offset: 16
        }
    );
}

#[test]
fn audio() {
    let mut parser = Parser::default();

    let mi = MediaBlock::parse(&BlockMetadata::new("audio::bar[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TMediaBlock {
            type_: MediaType::Audio,
            target: Span {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: Attrlist {
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: Span {
                data: "audio::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 13,
            offset: 12
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Empty);
    assert_eq!(mi.item.raw_context().deref(), "audio");
    assert!(mi.item.nested_blocks().next().is_none());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);
}

#[test]
fn video() {
    let mut parser = Parser::default();

    let mi = MediaBlock::parse(&BlockMetadata::new("video::bar[]"), &mut parser)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TMediaBlock {
            type_: MediaType::Video,
            target: Span {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: Attrlist {
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: Span {
                data: "video::bar[]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 13,
            offset: 12
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Empty);
    assert_eq!(mi.item.raw_context().deref(), "video");
    assert!(mi.item.nested_blocks().next().is_none());
    assert!(mi.item.title_source().is_none());
    assert!(mi.item.title().is_none());
    assert!(mi.item.anchor().is_none());
    assert!(mi.item.attrlist().is_none());
    assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);
}

#[test]
fn err_duplicate_comma() {
    let mut parser = Parser::default();
    let maw = MediaBlock::parse(&BlockMetadata::new("image::bar[blah,,blap]"), &mut parser);

    let mi = maw.item.unwrap().clone();

    assert_eq!(
        mi.item,
        TMediaBlock {
            type_: MediaType::Image,
            target: Span {
                data: "bar",
                line: 1,
                col: 8,
                offset: 7,
            },
            macro_attrlist: Attrlist {
                attributes: &[
                    TElementAttribute {
                        name: None,
                        shorthand_items: &["blah"],
                        value: "blah"
                    },
                    TElementAttribute {
                        name: None,
                        shorthand_items: &[],
                        value: "blap"
                    }
                ],
                source: Span {
                    data: "blah,,blap",
                    line: 1,
                    col: 12,
                    offset: 11,
                }
            },
            source: Span {
                data: "image::bar[blah,,blap]",
                line: 1,
                col: 1,
                offset: 0,
            },
            title_source: None,
            title: None,
            anchor: None,
            attrlist: None,
        }
    );

    assert_eq!(
        mi.after,
        Span {
            data: "",
            line: 1,
            col: 23,
            offset: 22
        }
    );

    assert_eq!(
        maw.warnings,
        vec![TWarning {
            source: Span {
                data: "blah,,blap",
                line: 1,
                col: 12,
                offset: 11,
            },
            warning: WarningType::EmptyAttributeValue,
        }]
    );
}
