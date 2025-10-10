use pretty_assertions_sorted::assert_eq;

use crate::{Parser, Span};

#[test]
fn v_prefix_standalone() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("v1.2.3"), &mut parser);

    assert_eq!(result.revnumber(), Some("1.2.3"));
    assert_eq!(result.revdate(), "");
    assert_eq!(result.revremark(), None);
}

#[test]
fn standalone_number_without_v_prefix() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("1.2.3"), &mut parser);

    // According to Asciidoctor behavior, standalone numbers without "v" are not
    // revision numbers
    assert_eq!(result.revnumber(), None);
    assert_eq!(result.revdate(), "1.2.3");
    assert_eq!(result.revremark(), None);
}

#[test]
fn other_prefix_standalone() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("LPR1.2.3"), &mut parser);

    // Other prefixes don't have special standalone treatment
    assert_eq!(result.revnumber(), None);
    assert_eq!(result.revdate(), "LPR1.2.3");
    assert_eq!(result.revremark(), None);
}

#[test]
fn v_prefix_with_comma_and_date() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("v1.2.3, 2023-01-15"), &mut parser);

    assert_eq!(result.revnumber(), Some("1.2.3"));
    assert_eq!(result.revdate(), "2023-01-15");
    assert_eq!(result.revremark(), None);
}

#[test]
fn other_prefix_with_comma_and_date() {
    let mut parser = Parser::default();
    let result =
        crate::document::RevisionLine::parse(Span::new("LPR1.2.3, 2023-01-15"), &mut parser);

    // With comma, other prefixes should be stripped from revision number
    assert_eq!(result.revnumber(), Some("1.2.3"));
    assert_eq!(result.revdate(), "2023-01-15");
    assert_eq!(result.revremark(), None);
}

#[test]
fn revision_with_colon_and_remark() {
    let mut parser = Parser::default();
    let result =
        crate::document::RevisionLine::parse(Span::new("v1.2.3: A great release"), &mut parser);

    assert_eq!(result.revnumber(), Some("1.2.3"));
    assert_eq!(result.revdate(), "");
    assert_eq!(result.revremark(), Some("A great release"));
}

#[test]
fn full_revision_line() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(
        Span::new("v2.1.0, 2023-12-25: Christmas release"),
        &mut parser,
    );

    assert_eq!(result.revnumber(), Some("2.1.0"));
    assert_eq!(result.revdate(), "2023-12-25");
    assert_eq!(result.revremark(), Some("Christmas release"));
}

#[test]
fn only_date() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("2023-01-15"), &mut parser);

    // Just a date, no revision number
    assert_eq!(result.revnumber(), None);
    assert_eq!(result.revdate(), "2023-01-15");
    assert_eq!(result.revremark(), None);
}

#[test]
fn date_with_remark() {
    let mut parser = Parser::default();
    let result =
        crate::document::RevisionLine::parse(Span::new("2023-01-15: New year update"), &mut parser);

    assert_eq!(result.revnumber(), None);
    assert_eq!(result.revdate(), "2023-01-15");
    assert_eq!(result.revremark(), Some("New year update"));
}

#[test]
fn whitespace_handling() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(
        Span::new("  v1.0.0  ,   Jan 1, 2023   :   Initial release  "),
        &mut parser,
    );

    assert_eq!(result.revnumber(), Some("1.0.0"));
    assert_eq!(result.revdate(), "Jan 1, 2023");
    assert_eq!(result.revremark(), Some("Initial release"));
}

#[test]
fn v_only_no_digits() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("v"), &mut parser);

    // "v" without digits should not be treated as a standalone revision
    assert_eq!(result.revnumber(), None);
    assert_eq!(result.revdate(), "v");
    assert_eq!(result.revremark(), None);
}

#[test]
fn complex_version_with_v() {
    let mut parser = Parser::default();
    let result = crate::document::RevisionLine::parse(Span::new("v1.2.3-beta.1"), &mut parser);

    assert_eq!(result.revnumber(), Some("1.2.3-beta.1"));
    assert_eq!(result.revdate(), "");
    assert_eq!(result.revremark(), None);
}

#[test]
fn numeric_prefix_stripped() {
    let mut parser = Parser::default();
    let result =
        crate::document::RevisionLine::parse(Span::new("abc123def, 2023-01-01"), &mut parser);

    // Non-numeric prefix should be stripped, leaving "123def"
    assert_eq!(result.revnumber(), Some("123def"));
    assert_eq!(result.revdate(), "2023-01-01");
    assert_eq!(result.revremark(), None);
}

#[test]
fn no_numeric_content() {
    let mut parser = Parser::default();
    let result =
        crate::document::RevisionLine::parse(Span::new("nodigits, 2023-01-01"), &mut parser);

    // When there are no digits, the prefix stripping should leave empty string
    assert_eq!(result.revnumber(), Some(""));
    assert_eq!(result.revdate(), "2023-01-01");
    assert_eq!(result.revremark(), None);
}

#[test]
fn sets_document_attributes_with_all_components() {
    let mut parser = Parser::default();
    let _result = crate::document::RevisionLine::parse(
        Span::new("v2.1.0, 2023-12-25: Christmas release"),
        &mut parser,
    );

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("2.1.0")
    );

    assert_eq!(
        parser.attribute_value("revdate").as_maybe_str(),
        Some("2023-12-25")
    );

    assert_eq!(
        parser.attribute_value("revremark").as_maybe_str(),
        Some("Christmas release")
    );
}

#[test]
fn sets_document_attributes_revision_number_only() {
    let mut parser = Parser::default();
    let _result = crate::document::RevisionLine::parse(Span::new("v1.2.3"), &mut parser);

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("1.2.3")
    );

    assert_eq!(parser.attribute_value("revdate").as_maybe_str(), Some(""));
    assert_eq!(parser.attribute_value("revremark").as_maybe_str(), None);
}

#[test]
fn sets_document_attributes_date_only() {
    let mut parser = Parser::default();
    let _result = crate::document::RevisionLine::parse(Span::new("2023-01-15"), &mut parser);

    assert_eq!(parser.attribute_value("revnumber").as_maybe_str(), None);

    assert_eq!(
        parser.attribute_value("revdate").as_maybe_str(),
        Some("2023-01-15")
    );

    assert_eq!(parser.attribute_value("revremark").as_maybe_str(), None);
}

#[test]
fn sets_document_attributes_date_with_remark() {
    let mut parser = Parser::default();
    let _result =
        crate::document::RevisionLine::parse(Span::new("2023-01-15: New year update"), &mut parser);

    assert_eq!(parser.attribute_value("revnumber").as_maybe_str(), None);

    assert_eq!(
        parser.attribute_value("revdate").as_maybe_str(),
        Some("2023-01-15")
    );

    assert_eq!(
        parser.attribute_value("revremark").as_maybe_str(),
        Some("New year update")
    );
}

#[test]
fn sets_document_attributes_revision_with_date() {
    let mut parser = Parser::default();
    let _result =
        crate::document::RevisionLine::parse(Span::new("v1.2.3, 2023-01-15"), &mut parser);

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("1.2.3")
    );

    assert_eq!(
        parser.attribute_value("revdate").as_maybe_str(),
        Some("2023-01-15")
    );

    assert_eq!(parser.attribute_value("revremark").as_maybe_str(), None);
}

#[test]
fn sets_document_attributes_revision_with_remark_only() {
    let mut parser = Parser::default();
    let _result =
        crate::document::RevisionLine::parse(Span::new("v1.2.3: A great release"), &mut parser);

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("1.2.3")
    );

    assert_eq!(parser.attribute_value("revdate").as_maybe_str(), Some(""));

    assert_eq!(
        parser.attribute_value("revremark").as_maybe_str(),
        Some("A great release")
    );
}

#[test]
fn sets_document_attributes_with_whitespace_handling() {
    let mut parser = Parser::default();
    let _result = crate::document::RevisionLine::parse(
        Span::new("  v1.0.0  ,   Jan 1, 2023   :   Initial release  "),
        &mut parser,
    );

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("1.0.0")
    );

    assert_eq!(
        parser.attribute_value("revdate").as_maybe_str(),
        Some("Jan 1, 2023")
    );

    assert_eq!(
        parser.attribute_value("revremark").as_maybe_str(),
        Some("Initial release")
    );
}

#[test]
fn sets_document_attributes_with_prefix_stripping() {
    let mut parser = Parser::default();
    let _result =
        crate::document::RevisionLine::parse(Span::new("abc123def, 2023-01-01"), &mut parser);

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("123def")
    );

    assert_eq!(
        parser.attribute_value("revdate").as_maybe_str(),
        Some("2023-01-01")
    );

    assert_eq!(parser.attribute_value("revremark").as_maybe_str(), None);
}

#[test]
fn sets_document_attributes_complex_version() {
    let mut parser = Parser::default();
    let _result = crate::document::RevisionLine::parse(Span::new("v1.2.3-beta.1"), &mut parser);

    assert_eq!(
        parser.attribute_value("revnumber").as_maybe_str(),
        Some("1.2.3-beta.1")
    );

    assert_eq!(parser.attribute_value("revdate").as_maybe_str(), Some(""));
    assert_eq!(parser.attribute_value("revremark").as_maybe_str(), None);
}
