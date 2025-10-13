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
    // all content comes from the included file.
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

#[test]
fn attribute_substitution_in_include_target() {
    let source =
        ":fixturesdir: fixtures\n:ext: adoc\n\ninclude::{fixturesdir}/include-file.{ext}[]";

    let handler = InlineFileHandler::from_pairs([(
        "fixtures/include-file.adoc",
        "This is included content.",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":fixturesdir: fixtures\n:ext: adoc\n\nThis is included content.\n"
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
        Some(SourceLine(Some("fixtures/include-file.adoc".to_owned()), 1))
    );
}

#[test]
fn multiple_attribute_substitution_in_include_target() {
    let source = ":dir: chapters\n:filename: intro\n:extension: adoc\n\ninclude::{dir}/{filename}.{extension}[]";

    let handler = InlineFileHandler::from_pairs([(
        "chapters/intro.adoc",
        "= Introduction\n\nWelcome to the guide.",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":dir: chapters\n:filename: intro\n:extension: adoc\n\n= Introduction\n\nWelcome to the guide.\n"
    );

    assert_eq!(
        source_map.original_file_and_line(5),
        Some(SourceLine(Some("chapters/intro.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(6),
        Some(SourceLine(Some("chapters/intro.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(7),
        Some(SourceLine(Some("chapters/intro.adoc".to_owned()), 3))
    );
}

#[test]
fn missing_attribute_in_include_target() {
    let source = ":fixturesdir: fixtures\n\ninclude::{fixturesdir}/include-file.{missingext}[]";

    let handler = InlineFileHandler::from_pairs([
        (
            "fixtures/include-file.adoc",
            "This content won't be included.",
        ),
        ("fixtures/include-file.", "This shouldn't match either."),
    ]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":fixturesdir: fixtures\n\nUnresolved directive in main.adoc - include::{fixturesdir}/include-file.{missingext}[]\n"
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
}

#[test]
fn attribute_substitution_with_nested_includes() {
    let source = ":basedir: content\n:format: adoc\n\ninclude::{basedir}/main.{format}[]";

    let handler = InlineFileHandler::from_pairs([
        (
            "content/main.adoc",
            ":partdir: parts\n\n== Main Chapter\n\ninclude::{partdir}/section1.{format}[]",
        ),
        (
            "parts/section1.adoc",
            "=== Section 1\n\nSection content here.",
        ),
    ]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":basedir: content\n:format: adoc\n\n:partdir: parts\n\n== Main Chapter\n\n=== Section 1\n\nSection content here.\n"
    );

    assert_eq!(
        source_map.original_file_and_line(8),
        Some(SourceLine(Some("parts/section1.adoc".to_owned()), 1))
    );
    assert_eq!(
        source_map.original_file_and_line(9),
        Some(SourceLine(Some("parts/section1.adoc".to_owned()), 2))
    );
    assert_eq!(
        source_map.original_file_and_line(10),
        Some(SourceLine(Some("parts/section1.adoc".to_owned()), 3))
    );
}

#[test]
#[ignore]
fn attribute_substitution_in_target_with_attrlist() {
    // TODO: Implement tag handling.
    let source = ":srcdir: examples\n:lang: java\n\ninclude::{srcdir}/hello.{lang}[tag=main]";

    let handler = InlineFileHandler::from_pairs([(
        "examples/hello.java",
        "// tag::main\npublic class Hello {}\n// end::main",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":srcdir: examples\n:lang: java\n\n// tag::main\npublic class Hello {}\n// end::main\n"
    );

    assert_eq!(
        source_map.original_file_and_line(4),
        Some(SourceLine(Some("examples/hello.java".to_owned()), 1))
    );
}

#[test]
fn attribute_substitution_with_multiline_attribute() {
    let source = ":longpath: very/long/path/to/some/ \\\nsubdirectory\n:ext: adoc\n\ninclude::{longpath}/file.{ext}[]";

    // TODO: This should be "very/long/path/to/some/subdirectory/file.adoc" (without
    // space) but the current Attribute::parse() incorrectly preserves the space
    // before the backslash in multi-line attribute continuation. This is a bug
    // that should be fixed.
    let handler = InlineFileHandler::from_pairs([(
        "very/long/path/to/some/ subdirectory/file.adoc",
        "Multi-line attribute worked!",
    )]);

    let parser = Parser::default()
        .with_primary_file_name("main.adoc")
        .with_include_file_handler(handler);

    let (processed_source, source_map) = preprocess(source, &parser);

    assert_eq!(
        processed_source,
        ":longpath: very/long/path/to/some/ \\\nsubdirectory\n:ext: adoc\n\nMulti-line attribute worked!\n"
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
    assert_eq!(
        source_map.original_file_and_line(5),
        Some(SourceLine(
            Some("very/long/path/to/some/ subdirectory/file.adoc".to_owned()),
            1
        ))
    );
}
