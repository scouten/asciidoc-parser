#![allow(unused)] // TEMPORARY while building

use std::sync::LazyLock;

use regex::Regex;

use crate::{
    Parser, Span,
    attributes::{Attrlist, AttrlistContext},
    parser::{SourceLine, SourceMap},
    span::MatchedItem,
};

/// Given a root file (initial input to `Parser::parse`), convert this into a
/// `String` suitable for regular parsing and a `SourceMap` that maps line
/// numbers in the parse-ready text back to original input file and line
/// numbers.
///
/// This function handles [include file] and [conditional] processing.
///
/// [include file]: https://docs.asciidoctor.org/asciidoc/latest/directives/include/
/// [conditional]: https://docs.asciidoctor.org/asciidoc/latest/directives/conditionals/
pub(crate) fn preprocess(source: &str, parser: &Parser) -> (String, SourceMap) {
    // Short-circuit if the original source document has no pre-processor
    // directives.
    if !source.starts_with("include::")
        && !source.starts_with("if")
        && !source.contains("\ninclude::")
        && !source.contains("\nif")
    {
        let mut source_map = SourceMap::default();
        source_map.append(1, SourceLine(parser.primary_file_name.clone(), 1));

        return (source.to_owned(), source_map);
    }

    // We use a temporary clone of the parser to track document attribute values
    // while parsing. These get recalculated again later when doing the full
    // document parsing.
    let mut temp_parser = parser.clone();
    let mut state = PreprocessorState::new(&mut temp_parser);
    state.process_adoc_include(source, parser.primary_file_name.as_deref());

    (state.output, state.source_map)
}

#[derive(Debug)]
struct PreprocessorState<'p> {
    parser: &'p mut Parser,
    in_document_header: bool,
    include_depth: usize,
    output_line_number: usize,
    output: String,
    source_map: SourceMap,
}

impl<'p> PreprocessorState<'p> {
    fn new(parser: &'p mut Parser) -> Self {
        Self {
            parser,
            in_document_header: true,
            include_depth: 0,
            output_line_number: 1,
            output: String::new(),
            source_map: SourceMap::default(),
        }
    }

    fn process_adoc_include(&mut self, source: &str, file_name: Option<&str>) {
        self.include_depth += 1;

        let mut has_reported_file = false;
        let mut source_span = Span::new(source);

        while !source_span.is_empty() {
            let original_source = source_span;

            let MatchedItem { item: line, after } = source_span.take_line();
            source_span = after;

            let source_line_number = line.line();

            if line.starts_with("include::")
                && let Some(caps) = INCLUDE_DIRECTIVE.captures(line.data())
            {
                let target = &caps[1];

                let attrlist = caps
                    .get(2)
                    .map(|attrlist| {
                        let span = Span::new(attrlist.as_str());
                        Attrlist::parse(span, self.parser, AttrlistContext::Inline)
                            .item
                            .item
                    })
                    .unwrap_or_default();

                if let Some(include_text) =
                    self.parser.include_file_handler.as_ref().and_then(|ifh| {
                        ifh.resolve_target(file_name, target, &attrlist, self.parser)
                    })
                {
                    // TO DO: Use process_adoc_include or (TBD) depending on
                    // whether it's an Asciidoc file type.
                    self.process_adoc_include(&include_text, Some(target));

                    // Re-report the including file if there's more content.
                    has_reported_file = false;
                } else {
                    self.output_line_number += 1;
                    self.output.push_str(&format!(
                        "Unresolved directive in {file_name} - {line}\n",
                        file_name = file_name.unwrap_or("(root file)",),
                        line = line.data(),
                    ));
                }
            } else {
                // If none of the above apply, add the line to output.
                if !has_reported_file {
                    has_reported_file = true;
                    self.source_map.append(
                        self.output_line_number,
                        SourceLine(to_owned(file_name), source_line_number),
                    );
                }

                self.output_line_number += 1;
                self.output.push_str(line.data());
                self.output.push('\n');
            }
        }

        self.include_depth -= 1;
    }
}

fn to_owned(maybe_file_name: Option<&str>) -> Option<String> {
    maybe_file_name.map(|n| n.to_string())
}

static INCLUDE_DIRECTIVE: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)                      # Extended (verbose) mode

        ^                           # Start of string

        include::                   # Literal 'include::' macro prefix

        (                           # (1) Target path
            [^\s\[]                   #   First char: not space or '['
            (?: [^\[]* [^\s\[] )?     #   Optional middle part ending with non-space/non-'['
        )                           # end capture group 1

        \[                          # Literal '[' starting the attributes block

        ([^\]].+)?                  # (2) Optional contents inside brackets (lazy by default)

        \]                          # Literal closing bracket

        $                           # End of line
        "#,
    )
    .unwrap()
});
