use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/images.adoc");

non_normative!(
    r#"
= Images

There are two AsciiDoc image macro types, block and inline.
As with all macros, the block and inline forms differ by the number of colons that follow the macro name.
The block form uses two colons (`::`), whereas the inline form only uses one (`:`).

"#
);

mod block_image_macro {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::MediaType,
        tests::{
            fixtures::{
                TSpan,
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TMediaBlock, TSimpleBlock},
                content::TContent,
                document::{TDocument, THeader},
            },
            sdd::{non_normative, to_do_verifies, verifies},
        },
    };

    non_normative!(
        r#"
== Block image macro

"#
    );

    #[test]
    fn basic_syntax() {
        verifies!(
            r#"
A [.term]*block image* is displayed as a discrete element, i.e., on its own line, in a document.
A block image is designated by `image` macro name and followed by two colons (`::`)
It's preceded by an empty line, entered on a line by itself, and then followed by an empty line.

.Block image macro
[source#ex-block]
----
Content in document.

include::example$image.adoc[tag=base-co]

Content in document
----
<.> To insert a block image, type the `image` macro name directly followed by two colons (`::`).
<.> After the colons, enter the image file target.
Type a pair of square brackets (`+[]+`) directly after the target to complete the macro.

The result of <<ex-block>> is displayed below.

include::example$image.adoc[tag=base]

"#
        );

        let doc = Parser::default()
            .parse("Content in document.\n\nimage::sunset.jpg[]\n\nContent in document");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: vec![],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: vec![
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "Content in document.",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            rendered: "Content in document.",
                        },
                        source: TSpan {
                            data: "Content in document.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Media(TMediaBlock {
                        type_: MediaType::Image,
                        target: TSpan {
                            data: "sunset.jpg",
                            line: 3,
                            col: 8,
                            offset: 29,
                        },
                        macro_attrlist: TAttrlist {
                            attributes: vec![],
                            source: TSpan {
                                data: "",
                                line: 3,
                                col: 19,
                                offset: 40,
                            },
                        },
                        source: TSpan {
                            data: "image::sunset.jpg[]",
                            line: 3,
                            col: 1,
                            offset: 22,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "Content in document",
                                line: 5,
                                col: 1,
                                offset: 43,
                            },
                            rendered: "Content in document",
                        },
                        source: TSpan {
                            data: "Content in document",
                            line: 5,
                            col: 1,
                            offset: 43,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                source: TSpan {
                    data: "Content in document.\n\nimage::sunset.jpg[]\n\nContent in document",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: vec![],
            }
        );
    }

    non_normative!(
        r#"
include::partial$image-target.adoc[]

"#
    );

    #[test]
    fn optional_attributes() {
        verifies!(
            r#"
You can specify a comma-separated list of optional attributes inside the square brackets or leave them empty.
If you want to specify alt text, enter it inside the square brackets.

.Block image macro with alt text
[source#ex-alt]
----
include::example$image.adoc[tag=alt]
----

"#
        );

        let doc = Parser::default().parse("image::sunset.jpg[Sunset]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: vec![],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: vec![TBlock::Media(TMediaBlock {
                    type_: MediaType::Image,
                    target: TSpan {
                        data: "sunset.jpg",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: vec![TElementAttribute {
                            name: None,
                            value: "Sunset",
                            shorthand_items: vec!["Sunset"],
                        },],
                        source: TSpan {
                            data: "Sunset",
                            line: 1,
                            col: 19,
                            offset: 18,
                        },
                    },
                    source: TSpan {
                        data: "image::sunset.jpg[Sunset]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image::sunset.jpg[Sunset]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: vec![],
            }
        );
    }

    #[test]
    fn alt_text_with_comma() {
        verifies!(
            r#"
If the alt text contains a comma or starts with a valid attribute name followed by an equals sign, you must enclose the alt text in double quotes.
The double quote enclosure effectively escapes the comma from being interpreted as an attribute separator.
See xref:attributes:positional-and-named-attributes.adoc#attribute-list-parsing[Attribute list parsing] to learn how the attribute list in a macro is parsed.

.Block image macro with alt text that contains a comma
[source#ex-alt-with-comma]
----
include::example$image.adoc[tag=alt-with-comma]
----

NOTE: Although you could enclose the alt text in single quotes to escape the comma, doing so implicitly enables substitutions.
Unless you need substitutions to be applied to the alt text, prefer using double quotes as the enclosure.

You can also give the image an ID, title, set its dimensions and make it a link.

"#
        );

        let doc = Parser::default().parse(r#"image::sunset.jpg["Mesa Verde Sunset, by JAVH"]"#);

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: vec![],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: vec![TBlock::Media(TMediaBlock {
                    type_: MediaType::Image,
                    target: TSpan {
                        data: "sunset.jpg",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: vec![TElementAttribute {
                            name: None,
                            value: "Mesa Verde Sunset, by JAVH",
                            shorthand_items: vec!["Mesa Verde Sunset, by JAVH"],
                        },],
                        source: TSpan {
                            data: "\"Mesa Verde Sunset, by JAVH\"",
                            line: 1,
                            col: 19,
                            offset: 18,
                        },
                    },
                    source: TSpan {
                        data: "image::sunset.jpg[\"Mesa Verde Sunset, by JAVH\"]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image::sunset.jpg[\"Mesa Verde Sunset, by JAVH\"]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: vec![],
            }
        );
    }

    non_normative!(
        r#"
NOTE: Although you could enclose the alt text in single quotes to escape the comma, doing so implicitly enables substitutions.
Unless you need substitutions to be applied to the alt text, prefer using double quotes as the enclosure.

"#
    );

    #[test]
    fn with_preamble() {
        verifies!(
            r#"
You can also give the image an ID, title, set its dimensions and make it a link.

.Block image macro with attribute list
[source#ex-attributes]
----
include::example$image.adoc[tag=attr-co]
----
<.> Defines the title of the block image, which gets displayed underneath the image when rendered.
<.> Assigns an ID to the block and makes the image a link.
The `link` attribute can also be defined inside the attribute list of the block macro.
<.> The first positional attribute, _Sunset_, is the image's alt text.
<.> The second and third positional attributes define the width and height, respectively.

The result of <<ex-attributes>> is displayed below.

include::example$image.adoc[tag=attr]

"#
        );

        let doc = Parser::default().parse(".A mountain sunset\n[#img-sunset,link=https://www.flickr.com/photos/javh/5448336655]\nimage::sunset.jpg[Sunset,200,100]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: vec![],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: vec![TBlock::Media(TMediaBlock {
                    type_: MediaType::Image,
                    target: TSpan {
                        data: "sunset.jpg",
                        line: 3,
                        col: 8,
                        offset: 91,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: vec![
                            TElementAttribute {
                                name: None,
                                value: "Sunset",
                                shorthand_items: vec!["Sunset"],
                            },
                            TElementAttribute {
                                name: None,
                                value: "200",
                                shorthand_items: vec![],
                            },
                            TElementAttribute {
                                name: None,
                                value: "100",
                                shorthand_items: vec![],
                            },
                        ],
                        source: TSpan {
                            data: "Sunset,200,100",
                            line: 3,
                            col: 19,
                            offset: 102,
                        },
                    },
                    source: TSpan {
                        data: ".A mountain sunset\n[#img-sunset,link=https://www.flickr.com/photos/javh/5448336655]\nimage::sunset.jpg[Sunset,200,100]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: Some(TSpan {
                        data: "A mountain sunset",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },),
                    anchor: None,
                    attrlist: Some(TAttrlist {
                        attributes: vec![
                            TElementAttribute {
                                name: None,
                                value: "#img-sunset",
                                shorthand_items: vec!["#img-sunset"],
                            },
                            TElementAttribute {
                                name: Some("link",),
                                value: "https://www.flickr.com/photos/javh/5448336655",
                                shorthand_items: vec![],
                            },
                        ],
                        source: TSpan {
                            data: "#img-sunset,link=https://www.flickr.com/photos/javh/5448336655",
                            line: 2,
                            col: 2,
                            offset: 20,
                        },
                    },),
                },),],
                source: TSpan {
                    data: ".A mountain sunset\n[#img-sunset,link=https://www.flickr.com/photos/javh/5448336655]\nimage::sunset.jpg[Sunset,200,100]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: vec![],
            }
        );
    }

    #[ignore]
    #[test]
    fn figure_caption_label() {
        to_do_verifies!(
            r#"
=== Figure caption label

When a title is defined on a block image, the image title will be prefixed by a caption label (Figure) and numbered automatically.
To turn off figure caption labels and numbers, unset the `figure-caption` attribute in the document header.

[source]
----
= Document Title
:figure-caption!:
----

"#
        );
    }
}

mod inline_image_macro {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{Block, IsBlock},
        tests::sdd::{non_normative, verifies},
    };

    non_normative!(
        r#"
== Inline image macro

"#
    );

    #[test]
    fn basic_syntax() {
        verifies!(
            r#"
An [.term]*inline image* is displayed in the flow of another element, such as a paragraph or sidebar block.
The inline image macro is almost identical to the block image macro, except its macro name is followed by a single colon (`:`).

.Inline image macro
[source#ex-inline]
----
Click image:play.png[] to get the party started. <.>

Click image:pause.png[title=Pause] when you need a break. <.>
----
<.> In the flow of an element, enter the macro name and a single colon (`+image:+`), followed by the image target.
Complete the macro with a pair of square brackets (`+[]+`).
<.> You can specify a comma-separated list of attributes inside the square brackets or leave them empty.

The result of <<ex-inline>> is displayed below.

include::example$image.adoc[tag=inline]

"#
        );

        let doc = Parser::default()
            .parse("Click image:play.png[] to get the party started.\n\nClick image:pause.png[title=Pause] when you need a break.");

        let mut blocks = doc.nested_blocks();

        let block1 = blocks.next().unwrap();
        let Block::Simple(sb1) = block1 else {
            panic!("Unexpected block type: {block1:?}");
        };

        assert_eq!(
            sb1.content().rendered(),
            r#"Click <span class="image"><img src="play.png" alt="play"></span> to get the party started."#
        );

        let block2 = blocks.next().unwrap();
        let Block::Simple(sb2) = block2 else {
            panic!("Unexpected block type: {block2:?}");
        };

        assert_eq!(
            sb2.content().rendered(),
            r#"Click <span class="image"><img src="pause.png" alt="pause" title="Pause"></span> when you need a break."#
        );

        assert!(blocks.next().is_none());
    }

    non_normative!(
        r#"
The alt text for an inline image has the same requirements as for a block image, with the added restriction that a closing square bracket must be escaped.

For inline images, the optional title is displayed as a tooltip.
"#
    );
}
