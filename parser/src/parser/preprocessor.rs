#![allow(unused)] // TEMPORARY while building

use crate::{
    Parser,
    parser::{SourceLine, SourceMap},
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
        todo!();
    }
}
