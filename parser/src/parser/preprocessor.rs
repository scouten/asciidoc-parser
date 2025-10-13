#![allow(unused)] // TEMPORARY while building
use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, Replacer};

use crate::{
    HasSpan, Parser, Span,
    attributes::{Attrlist, AttrlistContext},
    document::{Attribute, InterpretedValue},
    parser::{SourceLine, SourceMap},
    span::MatchedItem,
    warnings::Warning,
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
    can_have_attribute: bool,
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
            can_have_attribute: true,
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

            if self.can_have_attribute
                && line.starts_with(':')
                && (line.ends_with(':') || line.contains(": "))
                && let Some(attr) = Attribute::parse(original_source, self.parser)
            {
                // Process attribute entries so they're available for include directives. NOTE:
                // We ignore warnings here since this is a quick pass through the content.
                // Later, `Block::parse` will see the same warnings, if they occur, and will
                // actually record them.
                if !has_reported_file {
                    has_reported_file = true;
                    self.source_map.append(
                        self.output_line_number,
                        SourceLine(to_owned(file_name), source_line_number),
                    );
                }

                let mut warnings: Vec<Warning> = vec![];
                self.parser
                    .set_attribute_from_body(&attr.item, &mut warnings);

                self.output.push_str(attr.item.span().data());
                self.output.push('\n');

                self.output_line_number += attr
                    .item
                    .span()
                    .data()
                    .as_bytes()
                    .iter()
                    .filter(|&&b| b == b'\n')
                    .count()
                    + 1;

                source_span = attr.after;
            } else if line.starts_with("include::")
                && let Some(caps) = INCLUDE_DIRECTIVE.captures(line.data())
            {
                let target = self.substitute_attributes(&caps[1]);

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
                        ifh.resolve_target(file_name, &target, &attrlist, self.parser)
                    })
                {
                    // TODO: Use process_adoc_include or (TBD) depending on
                    // whether it's an Asciidoc file type.
                    self.process_adoc_include(&include_text, Some(&target));

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

                if line.is_empty() {
                    self.in_document_header = false;
                    self.can_have_attribute = true;
                } else if !self.in_document_header {
                    self.can_have_attribute = false;
                }

                self.output_line_number += 1;
                self.output.push_str(line.data());
                self.output.push('\n');
            }
        }

        self.include_depth -= 1;
    }

    /// Apply attribute substitution to a string, replacing {attribute-name}
    /// patterns with their corresponding values from the parser.
    fn substitute_attributes(&self, input: &str) -> String {
        if !input.contains('{') {
            return input.to_string();
        }

        #[derive(Debug)]
        struct AttributeReplacer<'p>(&'p Parser);

        impl Replacer for AttributeReplacer<'_> {
            fn replace_append(&mut self, caps: &regex::Captures<'_>, dest: &mut String) {
                let attr_name = &caps[1];

                if !self.0.has_attribute(attr_name) {
                    dest.push_str(&caps[0]);
                    return;
                }

                if caps[0].starts_with('\\') {
                    dest.push_str(&caps[0][1..]);
                    return;
                }

                if let InterpretedValue::Value(value) = self.0.attribute_value(attr_name) {
                    dest.push_str(value.as_ref());
                }
            }
        }

        let result: Cow<'_, str> = input.into();

        if let Cow::Owned(new_result) =
            ATTRIBUTE_REFERENCE.replace_all(&result, AttributeReplacer(self.parser))
        {
            new_result
        } else {
            input.to_string()
        }
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

static ATTRIBUTE_REFERENCE: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(r#"\\?\{([A-Za-z0-9_][A-Za-z0-9_-]*)\}"#).unwrap()
});
