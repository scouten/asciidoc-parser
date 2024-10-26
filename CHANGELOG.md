# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

The format of this changelog is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0](https://github.com/scouten/asciidoc-parser/releases/tag/v0.1.0)
_26 October 2024_

### Fixed

* Don't skip CI for owner-submitted PRs
* Bump actions/checkout from 3 to 4 ([#137](https://github.com/scouten/asciidoc-parser/pull/137))

### Other

* Raw delimited block ([#135](https://github.com/scouten/asciidoc-parser/pull/135))
* Refactor tests::blocks::block into multiple files ([#136](https://github.com/scouten/asciidoc-parser/pull/136))
* Re-check tracking refs to AsciiDoc language documentation ([#134](https://github.com/scouten/asciidoc-parser/pull/134))
* Define `Warning` type and start annotating possible parse errors ([#131](https://github.com/scouten/asciidoc-parser/pull/131))
* Support document header without title ([#133](https://github.com/scouten/asciidoc-parser/pull/133))
* Add new benchmark for parsing element attributes ([#132](https://github.com/scouten/asciidoc-parser/pull/132))
* Rename `ParseResult` to `MatchedItem` ([#130](https://github.com/scouten/asciidoc-parser/pull/130))
* Move `trim_source_for_rem` to `Span::trim_remainder` ([#129](https://github.com/scouten/asciidoc-parser/pull/129))
* Fix one remaining instance of `inp` ([#128](https://github.com/scouten/asciidoc-parser/pull/128))
* Code clarity: Use `source` for input text consistently ([#127](https://github.com/scouten/asciidoc-parser/pull/127))
* Code clarity: Use lifetime `'src` for all data structures that are tied to the original input source ([#126](https://github.com/scouten/asciidoc-parser/pull/126))
* Block style ([#124](https://github.com/scouten/asciidoc-parser/pull/124))
* Fix elided lifetime warning in recent nightly builds ([#125](https://github.com/scouten/asciidoc-parser/pull/125))
* Implement `Attrlist::id()` accessor ([#123](https://github.com/scouten/asciidoc-parser/pull/123))
* Add support for parsing shorthand items ([#117](https://github.com/scouten/asciidoc-parser/pull/117))
* Add Span::discard() and discard_all() ([#119](https://github.com/scouten/asciidoc-parser/pull/119))
* Add safety check to Span::into_parse_result ([#118](https://github.com/scouten/asciidoc-parser/pull/118))
* Remove `#[allow(dead_code)]` exceptions ([#116](https://github.com/scouten/asciidoc-parser/pull/116))
* Rename `temp_xxx` to `xxx` now that nom traits are gone ([#115](https://github.com/scouten/asciidoc-parser/pull/115))
* Remove nom dependency ([#114](https://github.com/scouten/asciidoc-parser/pull/114))
* Remove `Error` type ([#113](https://github.com/scouten/asciidoc-parser/pull/113))
* Remove nom trait implementations for Span ([#112](https://github.com/scouten/asciidoc-parser/pull/112))
* Refactor `Inline::parse` to no longer use `nom::InputTake` ([#111](https://github.com/scouten/asciidoc-parser/pull/111))
* Remove use of nom::InputIter trait ([#110](https://github.com/scouten/asciidoc-parser/pull/110))
* Replace `nom::Slice` with `Span::temp_slice` and `Span::temp_slice_from` ([#109](https://github.com/scouten/asciidoc-parser/pull/109))
* Retrofit `SimpleBlock::parse` to use new API style ([#108](https://github.com/scouten/asciidoc-parser/pull/108))
* Retrofit `Block::parse` to use new API style ([#107](https://github.com/scouten/asciidoc-parser/pull/107))
* Retrofit `SectionBlock::parse` to use new API style ([#106](https://github.com/scouten/asciidoc-parser/pull/106))
* Retrofit `ElementAttribute::parse` to use new API style ([#105](https://github.com/scouten/asciidoc-parser/pull/105))
* Retrofit `Attrlist::parse` to use new API style ([#104](https://github.com/scouten/asciidoc-parser/pull/104))
* Add new API `Span::take_ident` which replaces `primitives::ident` ([#103](https://github.com/scouten/asciidoc-parser/pull/103))
* Retrofit `Header::parse` to use new API style ([#102](https://github.com/scouten/asciidoc-parser/pull/102))
* Retrofit `Document::parse` to return an `Option` ([#101](https://github.com/scouten/asciidoc-parser/pull/101))
* Add `Span::take_required_whitespace` which will replace nom's `space1` ([#100](https://github.com/scouten/asciidoc-parser/pull/100))
* Add new API function `Span::take_whitespace` which will replace `space0` ([#99](https://github.com/scouten/asciidoc-parser/pull/99))
* Add `Span::take_prefix` which will replace nom's `tag` ([#98](https://github.com/scouten/asciidoc-parser/pull/98))
* Remove last remaining reference to `primitives::attr_name` ([#97](https://github.com/scouten/asciidoc-parser/pull/97))
* Replace `primitives::attr_name` with `Span::take_attr_name` ([#96](https://github.com/scouten/asciidoc-parser/pull/96))
* Replace `primitives::quoted_string` with `Span::take_quoted_string` ([#95](https://github.com/scouten/asciidoc-parser/pull/95))
* Add new method `Span::take_while` ([#94](https://github.com/scouten/asciidoc-parser/pull/94))
* Fix `TSpan impl Debug` to match new (internal) name for `Span` ([#93](https://github.com/scouten/asciidoc-parser/pull/93))
* Move `primitives::line_with_continuation` to `Span::take_line_with_continuation` ([#92](https://github.com/scouten/asciidoc-parser/pull/92))
* Move `primitives::non_empty_line` to `Span::take_non_empty_line` ([#91](https://github.com/scouten/asciidoc-parser/pull/91))
* Fix a bug in `Span::split_at_match_non_empty` ([#90](https://github.com/scouten/asciidoc-parser/pull/90))
* :split_at_match_non_empty` takes ownership (copies) self ([#89](https://github.com/scouten/asciidoc-parser/pull/89))
* Move `primitives::normalized_line` to `Span::take_normalized_line` ([#88](https://github.com/scouten/asciidoc-parser/pull/88))
* Move `primitives::line` to `Span::take_line` ([#87](https://github.com/scouten/asciidoc-parser/pull/87))
* Move `take_empty_line` and `discard_empty_lines` into `Span` ([#86](https://github.com/scouten/asciidoc-parser/pull/86))
* Move trim operations into `ParseResult` ([#85](https://github.com/scouten/asciidoc-parser/pull/85))
* Refactor the `Span` implementation ([#84](https://github.com/scouten/asciidoc-parser/pull/84))
* Add `Span::split_at_match_non_empty` ([#83](https://github.com/scouten/asciidoc-parser/pull/83))
* Add `Span::into_parse_result` ([#82](https://github.com/scouten/asciidoc-parser/pull/82))
* Bring `Span` type into this crate ([#81](https://github.com/scouten/asciidoc-parser/pull/81))
* Introduce new (internal) ParseResult type ([#80](https://github.com/scouten/asciidoc-parser/pull/80))
* Remove ParseResult type since we don't expose Error types. ([#77](https://github.com/scouten/asciidoc-parser/pull/77))
* Remove serde dependency ([#79](https://github.com/scouten/asciidoc-parser/pull/79))
* Change Inline::parse and ::parse_lines to return an Option ([#76](https://github.com/scouten/asciidoc-parser/pull/76))
* Change primitive non_empty_line parser to return an Option ([#75](https://github.com/scouten/asciidoc-parser/pull/75))
* Refactor handling of Inline parsing to reduce need for error handling ([#74](https://github.com/scouten/asciidoc-parser/pull/74))
* Make primitives::normalized_line infallible ([#73](https://github.com/scouten/asciidoc-parser/pull/73))
* Make primitives::line infallible ([#72](https://github.com/scouten/asciidoc-parser/pull/72))
* Add test for positional and named attribute equivalency ([#71](https://github.com/scouten/asciidoc-parser/pull/71))
* MacroBlock now parses Attrlist instead of a Span ([#70](https://github.com/scouten/asciidoc-parser/pull/70))
* Add Attrlist::named_or_positional_attribute ([#69](https://github.com/scouten/asciidoc-parser/pull/69))
* Add test cases for Element Attributes page ([#68](https://github.com/scouten/asciidoc-parser/pull/68))
* Attrlist::nth_attribute(0) should always return None ([#67](https://github.com/scouten/asciidoc-parser/pull/67))
* Add parser for attrlist ([#66](https://github.com/scouten/asciidoc-parser/pull/66))
* Add parser for element attribute ([#65](https://github.com/scouten/asciidoc-parser/pull/65))
* Add primitive parser for attribute name ([#64](https://github.com/scouten/asciidoc-parser/pull/64))
* Rename variable from line to qstr in quoted_string test suite ([#63](https://github.com/scouten/asciidoc-parser/pull/63))
* Add a primitive parser for quoted string as defined in element attributes ([#61](https://github.com/scouten/asciidoc-parser/pull/61))
* Remove redundant imports ([#62](https://github.com/scouten/asciidoc-parser/pull/62))
* Share implementation of parsing multiple blocks ([#59](https://github.com/scouten/asciidoc-parser/pull/59))
* Section should stop parsing at peer or ancestor level ([#60](https://github.com/scouten/asciidoc-parser/pull/60))
* Blocks main page ([#58](https://github.com/scouten/asciidoc-parser/pull/58))
* Add support for context to IsBlock trait ([#57](https://github.com/scouten/asciidoc-parser/pull/57))
* Add benchmark left out by mistake ([#56](https://github.com/scouten/asciidoc-parser/pull/56))
* Add SectionBlock to Block enum ([#55](https://github.com/scouten/asciidoc-parser/pull/55))
* Add preliminary implementation of section block parser ([#54](https://github.com/scouten/asciidoc-parser/pull/54))
* Debug formatting should use tuple syntax ([#53](https://github.com/scouten/asciidoc-parser/pull/53))
* Ignore extra spaces between = and document title ([#52](https://github.com/scouten/asciidoc-parser/pull/52))
* Document now implements IsBlock ([#51](https://github.com/scouten/asciidoc-parser/pull/51))
* Add new `IsBlock` trait ([#50](https://github.com/scouten/asciidoc-parser/pull/50))
* Describe content model for blocks ([#49](https://github.com/scouten/asciidoc-parser/pull/49))
* Normalization ([#47](https://github.com/scouten/asciidoc-parser/pull/47))
* Bump MSRV to 1.74.0 to resolve a problem with cargo llvm-cov ([#48](https://github.com/scouten/asciidoc-parser/pull/48))
* Document processing ([#46](https://github.com/scouten/asciidoc-parser/pull/46))
* Add test for inline macro example from spec ([#45](https://github.com/scouten/asciidoc-parser/pull/45))
* Parse inline block macro syntax as part of an overall inline block ([#44](https://github.com/scouten/asciidoc-parser/pull/44))
* Macro name must be an identifier ([#43](https://github.com/scouten/asciidoc-parser/pull/43))
* Factor Attribute's "name" parser into a new "ident" primitive ([#42](https://github.com/scouten/asciidoc-parser/pull/42))
* Add parser for inline macro ([#40](https://github.com/scouten/asciidoc-parser/pull/40))
* Add inline content model to SimpleBlock ([#39](https://github.com/scouten/asciidoc-parser/pull/39))
* Initial data model for inline elements ([#38](https://github.com/scouten/asciidoc-parser/pull/38))
* Update criterion requirement from 0.3 to 0.5.1 ([#37](https://github.com/scouten/asciidoc-parser/pull/37))
* Add CI status, code coverage, and benchmark badges ([#36](https://github.com/scouten/asciidoc-parser/pull/36))
* Benchmarks via criterion and codspeed ([#35](https://github.com/scouten/asciidoc-parser/pull/35))
* Add macro block to Block enum ([#34](https://github.com/scouten/asciidoc-parser/pull/34))
* Initial parser for block macros ([#33](https://github.com/scouten/asciidoc-parser/pull/33))
* Key concepts ([#32](https://github.com/scouten/asciidoc-parser/pull/32))
* Finish spec-driven-development of the document_structure page ([#31](https://github.com/scouten/asciidoc-parser/pull/31))
* Parse attribute lines with soft wrap continuation ([#30](https://github.com/scouten/asciidoc-parser/pull/30))
* Line continuation is marked with `\` not `+` ([#29](https://github.com/scouten/asciidoc-parser/pull/29))
* Add primitive parser for line with continuation marker (trailing `+`) ([#28](https://github.com/scouten/asciidoc-parser/pull/28))
* Spec-driven development: Lines ([#27](https://github.com/scouten/asciidoc-parser/pull/27))
* Test various invalid attribute name cases ([#26](https://github.com/scouten/asciidoc-parser/pull/26))
* Improve code coverage ([#25](https://github.com/scouten/asciidoc-parser/pull/25))
* Add Vec<Attribute> to Header ([#24](https://github.com/scouten/asciidoc-parser/pull/24))
* Add document::Attribute struct ([#23](https://github.com/scouten/asciidoc-parser/pull/23))
* Add Header struct ([#22](https://github.com/scouten/asciidoc-parser/pull/22))
* Restructure document module ([#21](https://github.com/scouten/asciidoc-parser/pull/21))
* Introduce HasSpan trait ([#20](https://github.com/scouten/asciidoc-parser/pull/20))
* Add internal fn trim_input_for_rem ([#19](https://github.com/scouten/asciidoc-parser/pull/19))
* Move struct Document inside mod document ([#18](https://github.com/scouten/asciidoc-parser/pull/18))
* Spec-driven development ([#17](https://github.com/scouten/asciidoc-parser/pull/17))
* Use pretty_assertions_sorted ([#16](https://github.com/scouten/asciidoc-parser/pull/16))
* Create infrastructure for easier test-result comparisons ([#15](https://github.com/scouten/asciidoc-parser/pull/15))
* Introduce Document struct, representing the parse result for a single AsciiDoc string/file ([#14](https://github.com/scouten/asciidoc-parser/pull/14))
* Add a Block enum type which will eventually represent all possible block types in AsciiDoc ([#13](https://github.com/scouten/asciidoc-parser/pull/13))
* :parse consumes blank lines that follow the block ([#12](https://github.com/scouten/asciidoc-parser/pull/12))
* New primitive to consume empty lines ([#11](https://github.com/scouten/asciidoc-parser/pull/11))
* Add primitive for consuming an empty line ([#10](https://github.com/scouten/asciidoc-parser/pull/10))
* Use nom-span for input types ([#9](https://github.com/scouten/asciidoc-parser/pull/9))
* Add parser for simple block ([#8](https://github.com/scouten/asciidoc-parser/pull/8))
* Add Error and ParseResult types ([#7](https://github.com/scouten/asciidoc-parser/pull/7))
* Add primitive parser for normalized, non-empty line ([#6](https://github.com/scouten/asciidoc-parser/pull/6))
* Add primitive parser for normalized line ([#5](https://github.com/scouten/asciidoc-parser/pull/5))
* Add a primitive to parse a \n- or \r\n-terminated line ([#4](https://github.com/scouten/asciidoc-parser/pull/4))
* Remove temporary placeholder function ([#3](https://github.com/scouten/asciidoc-parser/pull/3))
* Boost coverage for struct CowStr ([#2](https://github.com/scouten/asciidoc-parser/pull/2))
* Reorg strings tests a bit more
* Boost coverage for struct InlineStr ([#1](https://github.com/scouten/asciidoc-parser/pull/1))
* Fix codecov config to properly use token
* Never mind; remove abandoned clippy-check action
* Add GitHub token for Cargo clippy action
* Fix Clippy errors
* Use hyphenated crate name `asciidoc-parser`
* Add Codecov token
* Add initial CI build automation
* Quick README comments
* Import string types from pulldown-cmark
* Set up test infrastructure
* Tighten warnings
* Add Cargo.toml link to README.md
* Initial commit

## [0.1.0](https://github.com/scouten/asciidoc-parser/releases/tag/v0.1.0)
_26 October 2024_

* Initial public release of this crate. (Still very much a work-in-progress.)
