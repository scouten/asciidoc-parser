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

    assert_eq!(processed_source, source);

    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("test.adoc".to_owned()), 1))
    );
}

#[test]
fn simple_include_directive() {
    let source = "= Document Title\n\ninclude::shared.adoc[]\n\nMore content.";

    let handler = InlineFileHandler::from_pairs([(
        "shared.adoc",
        "This is shared content.\n\nWith multiple lines.\n",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        "= Document Title\n\nThis is shared content.\n\nWith multiple lines.\n\nMore content.\n"
    );

    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("main.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(2),
        Some(SourceLine(Some("main.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(3),
        Some(SourceLine(Some("shared.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(4),
        Some(SourceLine(Some("shared.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(5),
        Some(SourceLine(Some("shared.adoc".to_owned()), 3))
    );
    assert_eq!(
        source_map.original_file_and_line(6),
        Some(SourceLine(Some("main.adoc".to_owned()), 4))
    );
    assert_eq!(
        source_map.original_file_and_line(7),
        Some(SourceLine(Some("main.adoc".to_owned()), 5))
    );
}

#[test]
fn include_directive_at_start() {
    let source = "include::header.adoc[]\n\n= Document Title\n\nContent here.";

    let handler =
        InlineFileHandler::from_pairs([("header.adoc", ":author: John Doe\n:version: 1.0")]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":author: John Doe\n:version: 1.0\n\n= Document Title\n\nContent here.\n"
    );

    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("header.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(2),
        Some(SourceLine(Some("header.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(3),
        Some(SourceLine(Some("main.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(4),
        Some(SourceLine(Some("main.adoc".to_owned()), 3))
    );
    assert_eq!(
        source_map.original_file_and_line(5),
        Some(SourceLine(Some("main.adoc".to_owned()), 4))
    );
    assert_eq!(
        source_map.original_file_and_line(6),
        Some(SourceLine(Some("main.adoc".to_owned()), 5))
    );
}

#[test]
fn nested_includes() {
    let source = "= Document Title\n\ninclude::chapter1.adoc[]\n\n(a little more of root document)";

    let handler = InlineFileHandler::from_pairs([
        (
            "chapter1.adoc",
            "== Chapter 1\n\ninclude::section1.adoc[]\n\n(a little more of chapter 1)",
        ),
        ("section1.adoc", "=== Section 1\n\nContent here."),
    ]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        "= Document Title\n\n== Chapter 1\n\n=== Section 1\n\nContent here.\n\n(a little more of chapter 1)\n\n(a little more of root document)\n"
    );

    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("main.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(2),
        Some(SourceLine(Some("main.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(3),
        Some(SourceLine(Some("chapter1.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(4),
        Some(SourceLine(Some("chapter1.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(5),
        Some(SourceLine(Some("section1.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(6),
        Some(SourceLine(Some("section1.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(7),
        Some(SourceLine(Some("section1.adoc".to_owned()), 3))
    );
    assert_eq!(
        source_map.original_file_and_line(8),
        Some(SourceLine(Some("chapter1.adoc".to_owned()), 4))
    );
    assert_eq!(
        source_map.original_file_and_line(9),
        Some(SourceLine(Some("chapter1.adoc".to_owned()), 5))
    );
    assert_eq!(
        source_map.original_file_and_line(10),
        Some(SourceLine(Some("main.adoc".to_owned()), 4))
    );
}

#[test]
fn include_with_missing_file() {
    let source = "= Document Title\n\ninclude::missing.adoc[]\n\nMore content.";

    // Handler doesn't provide missing.adoc.
    let handler = InlineFileHandler::from_pairs([("other.adoc", "Other content")]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        "= Document Title\n\nUnresolved directive in main.adoc - include::missing.adoc[]\n\nMore content.\n"
    );

    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("main.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(2),
        Some(SourceLine(Some("main.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(3),
        Some(SourceLine(Some("main.adoc".to_owned()), 3))
    );
    assert_eq!(
        source_map.original_file_and_line(4),
        Some(SourceLine(Some("main.adoc".to_owned()), 4))
    );
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

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        "= Full Document\n\n== Chapter 1\n\nContent here.\n"
    );

    // Since the main file only contains an include directive,
    // all content comes from the included file
    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("entire-doc.adoc".to_owned()), 1))
    );
}

#[test]
fn no_include_handler() {
    let source = "= Document Title\n\ninclude::missing.adoc[]\n\nMore content.";

    // NOTE: No include file handler provided.
    let parser = Parser::default().with_primary_file_name("main.adoc");

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        "= Document Title\n\nUnresolved directive in main.adoc - include::missing.adoc[]\n\nMore content.\n"
    );

    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("main.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(2),
        Some(SourceLine(Some("main.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(3),
        Some(SourceLine(Some("main.adoc".to_owned()), 3))
    );
    assert_eq!(
        source_map.original_file_and_line(4),
        Some(SourceLine(Some("main.adoc".to_owned()), 4))
    );
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

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        "include::part1.adoc[] include::part2.adoc[]\n"
    );
    assert_eq!(
        source_map.original_file_and_line(1),
        Some(SourceLine(Some("main.adoc".to_owned()), 1))
    );
}
