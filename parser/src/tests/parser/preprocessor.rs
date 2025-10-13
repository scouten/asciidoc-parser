use crate::{
    Parser,
    parser::{SourceLine, preprocessor::preprocess},
    tests::fixtures::inline_file_handler::InlineFileHandler,
};

#[test]
fn no_preprocessor_directives() {
    let source = "= Document Title\n\nThis is a simple document with no includes or conditionals.";
    let parser = Parser::default().with_primary_file_name("test.adoc");

    let (processed_source, source_map) = preprocess(source, &parser);

    // Should return the source unchanged.
    assert_eq!(processed_source, source);

    // Should have a source map entry pointing to the original file.
    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("test.adoc".to_owned()), 1))
    );
}

#[test]
fn simple_include_directive() {
    let source = "= Document Title\n\ninclude::shared.adoc[]\n\nMore content.";

    // Create an inline file handler with test content.
    let handler = InlineFileHandler::from_pairs([(
        "shared.adoc",
        "This is shared content.\n\nWith multiple lines.",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // This test will currently panic due to the TODO, but it demonstrates
    // the expected structure and will help guide implementation.
    // When implemented, we should see the included content expanded.
}

#[test]
fn include_directive_at_start() {
    let source = "include::header.adoc[]\n\n= Document Title\n\nContent here.";

    let handler =
        InlineFileHandler::from_pairs([("header.adoc", ":author: John Doe\n:version: 1.0")]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // This should expand the header include at the beginning.
}

#[test]
fn nested_includes() {
    let source = "= Document Title\n\ninclude::chapter1.adoc[]";

    let handler = InlineFileHandler::from_pairs([
        ("chapter1.adoc", "== Chapter 1\n\ninclude::section1.adoc[]"),
        ("section1.adoc", "=== Section 1\n\nContent here."),
    ]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // This should handle nested includes properly.
}

#[test]
fn include_with_missing_file() {
    let source = "= Document Title\n\ninclude::missing.adoc[]\n\nMore content.";

    // Handler doesn't provide missing.adoc.
    let handler = InlineFileHandler::from_pairs([("other.adoc", "Other content")]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // Should handle missing files gracefully, probably by leaving
    // the include directive as-is or removing it.
}

#[test]
fn conditional_if_directive() {
    let source =
        "= Document Title\n\nifdef::debug[]\nThis is debug content.\nendif::[]\n\nRegular content.";

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_intrinsic_attribute("debug", "", crate::parser::ModificationContext::ApiOnly);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // Should process conditional directives based on attribute values.
}

#[test]
fn mixed_includes_and_conditionals() {
    let source = "= Document Title\n\nifdef::include-examples[]\ninclude::examples.adoc[]\nendif::[]\n\nMore content.";

    let handler =
        InlineFileHandler::from_pairs([("examples.adoc", "== Examples\n\nExample content here.")]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler)
        .with_intrinsic_attribute(
            "include-examples",
            "",
            crate::parser::ModificationContext::ApiOnly,
        );

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // Should handle both includes and conditionals together.
}

#[test]
fn source_map_line_tracking() {
    let source = "Line 1\nLine 2\ninclude::insert.adoc[]\nLine 4\nLine 5";

    let handler = InlineFileHandler::from_pairs([(
        "insert.adoc",
        "Inserted line 1\nInserted line 2\nInserted line 3",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // The source map should correctly track which lines come from which files.
    // Lines 1-2 should map to main.adoc:1-2
    // The inserted lines should map to insert.adoc:1-3
    // Lines 4-5 should map to main.adoc:4-5 (adjusted for the expansion)
}

#[test]
fn empty_file_with_include() {
    let source = "include::entire-doc.adoc[]";

    let handler = InlineFileHandler::from_pairs([(
        "entire-doc.adoc",
        "= Full Document\n\n== Chapter 1\n\nContent here.",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // Should handle the case where the entire main file is just an include.
}

#[test]
fn no_include_handler() {
    let source = "= Document Title\n\ninclude::missing.adoc[]\n\nMore content.";

    // No include file handler provided.
    let parser = Parser::default().with_primary_file_name("main.adoc");

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // Should handle the case where no include handler is provided.
    // Likely should leave include directives as-is or skip them.
}

#[test]
fn multiple_includes_same_line() {
    let source = "include::part1.adoc[] include::part2.adoc[]";

    let handler = InlineFileHandler::from_pairs([
        ("part1.adoc", "First part"),
        ("part2.adoc", "Second part"),
    ]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (_processed_source, _source_map) = preprocess(source, &parser);

    // Should handle multiple includes on the same line.
}
