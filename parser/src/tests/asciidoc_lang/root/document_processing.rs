use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/ROOT/pages/document-processing.adoc");
// Tracking commit c45aa60, current as of 2024-10-26.

non_normative!(
    r#"
= Document Processing

AsciiDoc is specifically a writing format, not a publishing format.
In other words, it's not WYSIWYG like when you write in a word processor.
Instead, what you write is the AsciiDoc source.
You then use an AsciiDoc processor, such as Asciidoctor, to convert the AsciiDoc source into a publishable format.
It's this output that you publish.

Converting the AsciiDoc source is an opportunity to interpret and embellish your content to get more out of it than what you put in.
The work of converting the AsciiDoc source to another format is handled by a converter.
While there is a strong relationship between the language and the converters, these two aspects are not explicitly coupled.

An AsciiDoc processor provides several built-in converters, including ones for making HTML and DocBook.
To activate one of these converters, you set the backend on the document (default: html).
The backend is a keyword that tells the processor which output format you want to make.
The processor then selects the converter that makes that output format.
For example, the HTML converter handles the html backend to make HTML output.

An AsciiDoc processor actually works in two steps.
First, it parses the AsciiDoc document.
This parsing produces a structured document that reflects the written structure and interprets all the meaningful markup.
The processor then passes this structured document to the converter to transform it into the output format.

In short, the processor accepts a string (which may be read from a file), parses it into a structure document, then produces another string (which may be written to a file).
"#
);
