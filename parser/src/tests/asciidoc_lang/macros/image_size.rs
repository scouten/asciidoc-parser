use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/image-size.adoc");

non_normative!(
    r#"
= Adjust Image Sizes
:url-w3-dimensions: https://www.w3.org/TR/2014/REC-html5-20141028/embedded-content-0.html#dimension-attributes
:url-discuss-measure: https://discuss.asciidoctor.org/Unit-of-measure-for-image-dimensions-td3040.html#a3222

Since images often need to be sized according to the medium, there are several ways to specify an image size.

In most output formats, the specified width is obeyed unless the image would exceed the content width or height, in which case it scaled to fit while maintaining the original aspect ratio (i.e., responsive scaling).

"#
);

mod width_and_height_attributes {
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
== width and height attributes

The primary way to specify the size of an image is to define the `width` and `height` attributes on the image macro.
"#
    );

    #[test]
    fn positional_attributes() {
        verifies!(
            r#"
Since these two attributes are so common, they're mapped as the second and third (unnamed) positional attributes on both image macros.

[source]
----
image::flower.jpg[Flower,640,480]
----

"#
        );

        let doc = Parser::default().parse("image::flower.jpg[Flower,640,480]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Media(TMediaBlock {
                    type_: MediaType::Image,
                    target: TSpan {
                        data: "flower.jpg",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: &[
                            TElementAttribute {
                                name: None,
                                value: "Flower",
                                shorthand_items: &["Flower"],
                            },
                            TElementAttribute {
                                name: None,
                                value: "640",
                                shorthand_items: &[],
                            },
                            TElementAttribute {
                                name: None,
                                value: "480",
                                shorthand_items: &[],
                            },
                        ],
                        source: TSpan {
                            data: "Flower,640,480",
                            line: 1,
                            col: 19,
                            offset: 18,
                        },
                    },
                    source: TSpan {
                        data: "image::flower.jpg[Flower,640,480]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image::flower.jpg[Flower,640,480]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn named_attributes() {
        verifies!(
            r#"
That's equivalent to the long-hand version:

[source]
----
image::flower.jpg[alt=Flower,width=640,height=480]
----

"#
        );

        let doc = Parser::default().parse("image::flower.jpg[alt=Flower,width=640,height=480]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Media(TMediaBlock {
                    type_: MediaType::Image,
                    target: TSpan {
                        data: "flower.jpg",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: &[
                            TElementAttribute {
                                name: Some("alt",),
                                value: "Flower",
                                shorthand_items: &[],
                            },
                            TElementAttribute {
                                name: Some("width",),
                                value: "640",
                                shorthand_items: &[],
                            },
                            TElementAttribute {
                                name: Some("height",),
                                value: "480",
                                shorthand_items: &[],
                            },
                        ],
                        source: TSpan {
                            data: "alt=Flower,width=640,height=480",
                            line: 1,
                            col: 19,
                            offset: 18,
                        },
                    },
                    source: TSpan {
                        data: "image::flower.jpg[alt=Flower,width=640,height=480]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "image::flower.jpg[alt=Flower,width=640,height=480]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}

non_normative!(
    r#"
The value of the width and height attributes should be an integer without a unit.
The px unit is implied.
Although the processor may allow it, you should never rely on a % value.
While the % unit was supported in older versions of HTML, it was removed starting in HTML 5.
If you need to specify a % value for PDF or DocBook output, use `pdfwidth` or `scaledwidth`, respectively.
To scale the image relative to the content area in HTML output, use a role.

While the values of `width` and `height` can be used to scale the image, these attributes are primarily intended to specify the {url-w3-dimensions}[intrinsic size of the image in CSS pixels^].
The `width` and `height` attributes are mapped to attributes of the same name on the `<img>` element in the HTML output.
These attributes are important because they provide a hint to the browser to tell it how much space to reserve for the image during layout to minimize page reflows.
The `height` attribute should only be specified if the `width` attribute is also specified, and it should respect the aspect ratio of the image.

.Automatic image scaling
****
The default Asciidoctor stylesheet implements responsive images (using width-wise scaling).
If the width of the content area is smaller than the width of the image, the image will be scaled down to fit.
To support this feature, the intrinsic aspect ratio of the image is preserved at all sizes.

Thus, when specifying the image's dimensions, you should choose values that honor the intrinsic aspect ratio of the image.
If the values don't respect the aspect ratio, the height is ignored by the browser.
****

== pdfwidth attribute

AsciiDoc recognizes the following attributes to size images when converting to PDF using Asciidoctor PDF:

* `pdfwidth` - The preferred width of the image in the PDF when converting using Asciidoctor PDF.

The `pdfwidth` attribute accepts the following units:

[horizontal]
px:: Output device pixels (assumed to be 96 dpi)
pt (or none):: Points (1/72 of an inch)
pc:: Picas (1/6 of an inch)
cm:: Centimeters
mm:: Millimeters
in:: Inches
%:: Percentage of the content width (area between margins)
vw:: Percentage of the page width (edge to edge)
iw:: Percentage of the intrinsic width of the image

If `pdfwidth` is not provided, Asciidoctor PDF also accepts `scaledwidth`, or `width` (no units, assumed to be pixels), in that order.
ifeval::["{url-project}"=="https://asciidoctor.org"]
See xref:pdf-converter::image-scaling.adoc[image scaling in Asciidoctor PDF] for more details.
endif::[]

== scaledwidth attribute

AsciiDoc recognizes the following attributes to size images when converting to DocBook or when converting to PDF using Asciidoctor PDF.
The `scaledwidth` attribute is ignored by other converters.

* `scaledwidth` - The preferred width of the image when converting to PDF using the DocBook toolchain. (Mutually exclusive with `scale`).
* `scale` - Scales the original image size by this amount when converting to PDF using the DocBook toolchain. (Mutually exclusive with `scaledwidth`).

`scaledwidth` sizes images much like `pdfwidth`, except it does not accept the vw unit.

The value of `scaledwidth` when used with DocBook can have the following units:

[horizontal]
px:: Output device pixels (assumed to be 72 dpi)
pt:: Points (1/72 of an inch)
pc:: Picas (1/6 of an inch)
cm:: Centimeters
mm:: Millimeters
in:: Inches
em:: Ems (current font size)
% (or no units):: Percentage of the content width (area between margins)

The `scaledwidth` attribute in AsciiDoc is mapped to the `width` attribute on the `imagedata` tag in DocBook, whereas the `width` attribute in AsciiDoc is mapped to the `contentwidth` attribute on the `imagedata` tag in DocBook.
If both the `width` and `scaledwidth` attributes are specified in AsciiDoc, the `scaledwidth` tags precedence, so the DocBook output will only have the `width` attribute.

== Image sizing recap

.Image sizing attributes
[%autowidth]
|====
|Backend |Absolute size |Relative to original size |Relative to content width |Relative to page width

|html
|width=120 +
(assumed to be px)
|Not possible
|role=half-width
|role=half-view-width

|pdf
|pdfwidth=100mm +
(or cm, in, pc, pt, px)
|Not possible +
(support for the scale attribute is pending)
|pdfwidth=80%
|pdfwidth=50vw

|docbook, pdf
|scaledwidth=100mm +
(or cm, em, in, pc, pt, px)
|scale=75
|scaledwidth=50%
|Not possible
|====

Here's an example of how you might bring these attributes together to control the size of an image in various output formats:

[source]
----
image::flower.jpg[Flower,640,480,pdfwidth=50%,scaledwidth=50%]
----

If the cascading behavior of the sizing attributes does not work for your use case, you might consider a document attribute to set the attribute that is suitable for the backend you are using.
Consider the following example:

[source,indent=0]
----
 ifdef::backend-html5[]
 :twoinches: width=144
 // using a role requires adding a corresponding rule to the CSS
 :full-width: role=full-width
 :half-width: role=half-width
 :half-size: role=half-size
 :thumbnail: width=60
 endif::[]
 ifdef::backend-pdf[]
 :twoinches: pdfwidth=2in
 // NOTE use pdfwidth=100vw to make the image stretch edge to edge
 :full-width: pdfwidth=100%
 :half-width: pdfwidth=50%
 // NOTE scale is not yet supported by the PDF converter
 :half-size: pdfwidth=50%
 :thumbnail: pdfwidth=20mm
 endif::[]
 ifdef::backend-docbook5[]
 :twoinches: scaledwidth=2in
 :full-width: scaledwidth=100%
 :half-width: scaledwidth=50%
 :half-size: scale=50
 :thumbnail: scaledwidth=20mm
 endif::[]
----

Then you can specify the image to be half the width of the content area using the following syntax:

[source]
----
image::image.jpg[{half-width}]
----

In addition to providing consistency across your document, this technique will help insulate you from future changes.
For a more detailed example, see {url-discuss-measure}[this thread^] on the discussion list.
"#
);
