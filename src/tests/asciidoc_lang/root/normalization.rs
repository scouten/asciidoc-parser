//! Tracks https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang/-/blob/main/docs/modules/ROOT/pages/normalization.adoc?ref_type=heads
//!
//! Tracking commit 08289a9f, current as of 2024-10-26.
//!
//! See additional test cases with more edge-case coverage in
//! `tests/primitives/line.rs`.

use pretty_assertions_sorted::assert_eq;

use crate::{tests::fixtures::TSpan, Span};

// = Normalization
//
// When an AsciiDoc processor reads the AsciiDoc source, the first thing it does
// is normalize the lines. (This operation can be performed up front or as each
// line is visited).
//
// Normalization consists of the following operations:
//
// * Force the encoding to UTF-8 (An AsciiDoc processor always assumes the
//   content is UTF-8 encoded)

#[test]
fn force_utf8() {
    // Implicit: The asciidoc-parser crate requires a Rust string slice
    // as input, which is guaranteed to be UTF-8.
}

// * Strip trailing spaces from each line (including any end of line character)

#[test]
fn strips_trailing_spaces() {
    let span = Span::new("abc   ");
    let line = span.take_normalized_line();

    assert_eq!(
        line.after,
        TSpan {
            data: "",
            line: 1,
            col: 7,
            offset: 6
        }
    );

    assert_eq!(
        line.item,
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0
        }
    );
}

#[test]
fn strips_trailing_lf() {
    // Should consume but not return \n.

    let span = Span::new("abc  \ndef");
    let line = span.take_normalized_line();

    assert_eq!(
        line.after,
        TSpan {
            data: "def",
            line: 2,
            col: 1,
            offset: 6
        }
    );

    assert_eq!(
        line.item,
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0
        }
    );
}

#[test]
fn strips_trailing_crlf() {
    // Should consume but not return \r\n.

    let span = Span::new("abc  \r\ndef");
    let line = span.take_normalized_line();

    assert_eq!(
        line.after,
        TSpan {
            data: "def",
            line: 2,
            col: 1,
            offset: 7
        }
    );

    assert_eq!(
        line.item,
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0
        }
    );
}

// This normalization is performed independent of any structured context.
// It doesn't matter if the line is part of a literal block or a regular
// paragraph. All lines get normalized.
//
// Normalization is only applied in certain cases to the lines of an include
// file. Only include files that have a recognized AsciiDoc extension are
// normalized as described above. For all other files, only the trailing end of
// line character is removed. Include files can also have a different encoding,
// which is specified using the encoding attribute. If the encoding attribute is
// not specified, UTF-8 is assumed.
//
// When the AsciiDoc processor brings the lines back together to produce the
// rendered document (HTML, DocBook, etc), it joins the lines on the line feed
// character (`\n`).
