use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/links.adoc");

non_normative!(
    r#"
= Links
:url-url-def: https://en.wikipedia.org/wiki/URL

AsciiDoc offers a variety of ways of creating links (aka hyperlinks) to other addressable resources.
The pages in this section document how to add and customize links in AsciiDoc.

== URLs and links

The [.term]*target* of a link is a {url-url-def}[Uniform Resource Locator^] (URL), otherwise known as a web address.
The text the reader clicks to navigate to that target is referred to as the [.term]*link text*.

NOTE: You may sometimes see the term URI used in place of a URL.
Although the URI is more technically correct in some cases, URL is the accepted term.

The URL is the web address of a unique resource.
A URL can either be absolute or relative.
An absolute URL consists of a scheme, an authority (i.e., domain name), a path with an optional file extension, and metadata in the form of a query string and fragment (e.g., `\https://example.org/asciidoc/links.html?source=home`).
You may recognize an absolute URL as what you type in the location bar of a web browser, such as the one for this page.
A relative URL is the portion of an absolute URL that starts after either the root path or a subpath (e.g., `guides/getting-started.html`).

Since an absolute URL has a distinct, recognizable syntax, an AsciiDoc processor will detect URLs (unless escaped) and automatically convert them to links wherever the macros substitution step is applied.
This also works for bare email addresses.
You can learn more about this behavior in xref:autolinks.adoc[].
To make a link to a relative URL, you must be specify it explicitly as the target of a xref:link-macro.adoc[link macro].

== Link-related macros

Instead of showing the bare URL or email address as the link text, you may want to customize that text.
Or perhaps you want to apply attributes to the link, such as a role.
To do so, you'd use either the xref:url-macro.adoc[URL macro] or, if you're linking to xref:complex-urls.adoc[a complex URL], the more decisive xref:link-macro.adoc[link macro].
(You can also use the xref:link-macro.adoc[link macro] to make a link to an addressable resource using a relative URL or a URL that is not otherwise recognized as an absolute URL).

When linking to an email address, you can use the specialized xref:mailto-macro.adoc[mailto macro] to enhance the link with prepopulated subject and body text.

"#
);

mod encode_reserved_characters {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        tests::{
            fixtures::{
                TSpan,
                blocks::{TBlock, TSimpleBlock},
                content::TContent,
                document::{TDocument, THeader},
            },
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
== Encode reserved characters

If the URL contains reserved characters, such as double quote (`"`), space, or an unconstrained AsciiDoc formatting mark, you'll need to encode these characters using URI encoding.
For example, a double quote is encoded as `%22`.
An underscore is encoded as `%5F`.
If you do not encode these characters, the URL may be mangled or cause the processor to fail.

"#
    );

    #[test]
    fn example() {
        verifies!(
            r#"
Let's assume we are creating a URL that contains a query string parameter named `q` that contains reserved characters:

....
https://example.org?q=label:"Requires docs"
....

To encode a URL, open the development tools in your browser and pass the URL to the `encodeURI` function:

[,js]
----
encodeURI('http://example.org?q=label:"Requires docs"')
----

Here's the encoded URL that we'd use in the AsciiDoc document:

....
https://example.org?q=label:%22Requires%20docs%22
....

Depending on the capabilities of the web application, the space character can be encoded as `+` instead of `%20`.

"#
        );

        let doc = Parser::default().parse("https://example.org?q=label:%22Requires%20docs%22");

        dbg!(&doc);

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
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://example.org?q=label:%22Requires%20docs%22",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<a href=\"https://example.org?q=label:%22Requires%20docs%22\" class=\"bare\">https://example.org?q=label:%22Requires%20docs%22</a>",
                    },
                    source: TSpan {
                        data: "https://example.org?q=label:%22Requires%20docs%22",
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
                    data: "https://example.org?q=label:%22Requires%20docs%22",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}

mod hide_uri_scheme {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        tests::{
            fixtures::{
                TSpan,
                blocks::{TBlock, TSimpleBlock},
                content::TContent,
                document::{TAttribute, TDocument, THeader, TInterpretedValue},
            },
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
== Hide the URL scheme

If the link text is a bare URL (aka URI), whether that link was created automatically or using a link-related macro, you can configure the AsciiDoc processor to hide the scheme (e.g., _https://_).
Hiding the scheme can make the URL more readable--perhaps even recognizable--to a person less familiar with technical nomenclature.

"#
    );

    #[test]
    fn example() {
        verifies!(
            r#"
To configure the AsciiDoc processor to display the linked URL without the scheme part, set the `hide-uri-scheme` attribute in the header of the AsciiDoc document.

[source]
----
= Document Title
:hide-uri-scheme: <.>

https://asciidoctor.org
----
<.> Note the use of `uri` instead of `url` in the attribute name.

When the `hide-uri-scheme` attribute is set, the above URL will be displayed to the reader as follows:

====
https://asciidoctor.org[asciidoctor.org]
====

Note the absence of _https://_ in the URL.
The prefix will still be present in the link target.

"#
        );

        let doc = Parser::default()
            .parse("= Document Title\n:hide-uri-scheme:\n\nhttps://asciidoctor.org");

        dbg!(&doc);

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: Some(TSpan {
                        data: "Document Title",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },),
                    title: Some("Document Title",),
                    attributes: &[TAttribute {
                        name: TSpan {
                            data: "hide-uri-scheme",
                            line: 2,
                            col: 2,
                            offset: 18,
                        },
                        value_source: None,
                        value: TInterpretedValue::Set,
                        source: TSpan {
                            data: ":hide-uri-scheme:",
                            line: 2,
                            col: 1,
                            offset: 17,
                        },
                    },],
                    source: TSpan {
                        data: "= Document Title\n:hide-uri-scheme:",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://asciidoctor.org",
                            line: 4,
                            col: 1,
                            offset: 36,
                        },
                        rendered: r#"<a href="https://asciidoctor.org" class="bare">asciidoctor.org</a>"#,
                        // Expected output verified by running Asciidoc locally.
                    },
                    source: TSpan {
                        data: "https://asciidoctor.org",
                        line: 4,
                        col: 1,
                        offset: 36,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "= Document Title\n:hide-uri-scheme:\n\nhttps://asciidoctor.org",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}
