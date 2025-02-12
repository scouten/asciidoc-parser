use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/ROOT/pages/index.adoc");

non_normative!(
    r#"
= AsciiDoc Language Documentation
:navtitle: Introduction
:url-asciidoc-lang: https://projects.eclipse.org/projects/technology.asciidoc

== About AsciiDoc

AsciiDoc is a lightweight and semantic markup language primarily designed for writing technical documentation.
The language can be used to produce a variety of presentation-rich output formats, all from content encoded in a concise, human-readable, plain text format.

The AsciiDoc syntax is intuitive because it builds on well-established, plain text conventions for marking up and structuring text.
Someone unfamiliar with AsciiDoc can probably guess the purpose of many of its syntax elements just by looking at them.
That's because the elements of the syntax were carefully chosen to look like what they mean (a practice long employed by the tech industry).

The AsciiDoc language isn't coupled to the output format it produces.
An AsciiDoc processor can parse and comprehend an AsciiDoc source document and convert the parsed document structure into one or more output formats, such as HTML, PDF, EPUB3, man(ual) page, or DocBook.
The ability to produce multiple output formats is one of the main advantages of AsciiDoc.
This capability enables it to be used in static site generators, IDEs, git tools and services, CI/CD systems, and other software.

AsciiDoc bridges the gap between ease of writing and the rigorous requirements of technical authoring and publishing.
AsciiDoc only requires a text editor to read or write, thereby offering a low bar to getting started.

== About this documentation

You're reading the user-facing documentation for the AsciiDoc language as it's implemented in xref:asciidoctor::index.adoc[Asciidoctor].
This documentation does not cover how to set up and use Asciidoctor to process AsciiDoc content.
You can find that documentation in the xref:asciidoctor::index.adoc[Asciidoctor] section of this website.

This documentation has been submitted as the initial contribution for the {url-asciidoc-lang}[AsciiDoc Language project at Eclipse^].
That project will use this documentation as the basis for drafting a specification for the AsciiDoc language.
It will also be used as the draft of the user-facing guide for the AsciiDoc Language, which will also be maintained by that project.

Until the first version of the AsciiDoc Language Specification is ratified, AsciiDoc is defined by the Asciidoctor implementation.
There is no other official definition of the language.

The documentation for AsciiDoc will remain on this site until the AsciiDoc Language project starts publishing its own documentation for the AsciiDoc Language.

Until then, let's get started!
"#
);
