# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

The format of this changelog is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.2.0](https://github.com/scouten/asciidoc-parser/compare/v0.1.1...v0.2.0)
_30 November 2024_

### Added

* Parse attribute lists for blocks ([#164](https://github.com/scouten/asciidoc-parser/pull/164))
* Add support for block titles using `.(title)` syntax ([#158](https://github.com/scouten/asciidoc-parser/pull/158))
* SDD: Delimited blocks ([#157](https://github.com/scouten/asciidoc-parser/pull/157))
* Add support for compound delimited blocks ([#150](https://github.com/scouten/asciidoc-parser/pull/150))

### Fixed

* Add coverage for a missing case of `TInline::Span`
* Resolve new Clippy warnings for Rust 1.83 ([#161](https://github.com/scouten/asciidoc-parser/pull/161))
* Do not treat triple-hyphen as a delimiter for open block ([#156](https://github.com/scouten/asciidoc-parser/pull/156))
* `Span.trim_remainder` gave incorrect result if `after` was incomplete subset of `self` ([#147](https://github.com/scouten/asciidoc-parser/pull/147))

### Updated dependencies

* Update thiserror requirement from 1.0.63 to 2.0.1 ([#152](https://github.com/scouten/asciidoc-parser/pull/152))

## [0.1.1](https://github.com/scouten/asciidoc-parser/compare/v0.1.0...v0.1.1)
_26 October 2024_

### Fixed

* Copy/paste error in crate description

## [0.1.0](https://github.com/scouten/asciidoc-parser/releases/tag/v0.1.0)
_26 October 2024_

* Initial public release of this crate. (Still very much a work-in-progress.)
