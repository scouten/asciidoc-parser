use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/image-link.adoc");

non_normative!(
    r#"
= Add Link to Image

You can turn an image into a link by using the `link` attribute.

"#
);

mod link_attribute {
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
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
== link attribute

The link attribute on a block or image macro acts as though the image is wrapped in a link macro.
While it's possible to wrap an inline image macro in a link macro, that combination is not well supported and may introduce subtle parsing problems.
Therefore, you should use the `link` attribute on the image macro instead.

The value of the `link` attribute is akin to the target of the link macro.
It can point to any URL or relative path.

"#
    );

    #[test]
    fn block_image_macro_with_link_1() {
        verifies!(
            r#"
For a block image macro, the `link` attribute can be added to the block attribute line above the macro or inside the contents of the macro.

----
[link=https://example.org]
image::logo.png[Logo]
----

"#
        );

        let doc = Parser::default().parse("[link=https://example.org]\nimage::logo.png[Logo]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: &[],
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
                        data: "logo.png",
                        line: 2,
                        col: 8,
                        offset: 34,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: &[TElementAttribute {
                            name: None,
                            value: "Logo",
                            shorthand_items: &["Logo"],
                        },],
                        source: TSpan {
                            data: "Logo",
                            line: 2,
                            col: 17,
                            offset: 43,
                        },
                    },
                    source: TSpan {
                        data: "[link=https://example.org]\nimage::logo.png[Logo]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: Some(TAttrlist {
                        attributes: &[TElementAttribute {
                            name: Some("link",),
                            value: "https://example.org",
                            shorthand_items: &[],
                        },],
                        source: TSpan {
                            data: "link=https://example.org",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: TSpan {
                    data: "[link=https://example.org]\nimage::logo.png[Logo]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn block_image_macro_with_link_2() {
        verifies!(
            r#"
or

----
image::logo.png[Logo,link=https://example.org]
----

"#
        );

        let doc = Parser::default().parse("image::logo.png[Logo,link=https://example.org]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: &[],
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
                        data: "logo.png",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: &[
                            TElementAttribute {
                                name: None,
                                value: "Logo",
                                shorthand_items: &["Logo"],
                            },
                            TElementAttribute {
                                name: Some("link",),
                                value: "https://example.org",
                                shorthand_items: &[],
                            },
                        ],
                        source: TSpan {
                            data: "Logo,link=https://example.org",
                            line: 1,
                            col: 17,
                            offset: 16,
                        },
                    },
                    source: TSpan {
                        data: "image::logo.png[Logo,link=https://example.org]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image::logo.png[Logo,link=https://example.org]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn inline_macro_with_link() {
        verifies!(
            r#"
For an inline macro, the `link` attribute must be added inside the contents of the macro.

----
image:apply.jpg[Apply,link=https://apply.example.org] today!
----

"#
        );

        let doc =
            Parser::default().parse("image:apply.jpg[Apply,link=https://apply.example.org] today!");

        dbg!(&doc);

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: vec![TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "image:apply.jpg[Apply,link=https://apply.example.org] today!",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<span class=\"image\"><a class=\"image\" href=\"https://apply.example.org\"><img src=\"apply.jpg\" alt=\"Apply\"></a></span> today!",
                    },
                    source: TSpan {
                        data: "image:apply.jpg[Apply,link=https://apply.example.org] today!",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image:apply.jpg[Apply,link=https://apply.example.org] today!",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}

mod link_controls {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::MediaType,
        tests::{
            fixtures::{
                TSpan,
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TMediaBlock},
                document::{TDocument, THeader},
            },
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
== Link controls

When using the `link` attribute, you can also use the same controls supported by the link macro to control how the link is constructed.
"#
    );

    #[test]
    fn example() {
        verifies!(
            r#"
Those controls are as follows:

* `window` attribute - instructs the browser to open the link in the specified named window
* `nofollow` option - instructs search engines to not follow the link
* `noopener` option - instructs the browser to navigate to the target without granting the new browsing context access to the original document

When the value of `window` attribute is *_blank*, the `noopener` option is automatically enabled.

Here's an example that shows how to use these controls.

----
image::logo.png[Logo,link=https://example.org,window=_blank,opts=nofollow]
----
"#
        );

        let doc = Parser::default().parse(
            "image::logo.png[Logo,link=https://example.org,window=_blank,opts=nofollow]
",
        );

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title: None,
                    attributes: &[],
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
                        data: "logo.png",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: &[
                            TElementAttribute {
                                name: None,
                                value: "Logo",
                                shorthand_items: &["Logo"],
                            },
                            TElementAttribute {
                                name: Some("link",),
                                value: "https://example.org",
                                shorthand_items: &[],
                            },
                            TElementAttribute {
                                name: Some("window",),
                                value: "_blank",
                                shorthand_items: &[],
                            },
                            TElementAttribute {
                                name: Some("opts",),
                                value: "nofollow",
                                shorthand_items: &[],
                            },
                        ],
                        source: TSpan {
                            data: "Logo,link=https://example.org,window=_blank,opts=nofollow",
                            line: 1,
                            col: 17,
                            offset: 16,
                        },
                    },
                    source: TSpan {
                        data: "image::logo.png[Logo,link=https://example.org,window=_blank,opts=nofollow]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image::logo.png[Logo,link=https://example.org,window=_blank,opts=nofollow]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    non_normative!(
        r#"

Refer to the xref:link-macro-attribute-parsing.adoc#target-a-separate-window[Target a separate window] section in the link macro documentation for more information about how these link controls work.
"#
    );
}
