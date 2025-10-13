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

    todo!("May contain preprocessor directives -- look more closely");
}
