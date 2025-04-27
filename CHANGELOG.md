# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

The format of this changelog is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.4.0](https://github.com/scouten/asciidoc-parser/compare/v0.3.1...v0.4.0)
_27 April 2025_

### Major change

In this release, I replaced the previous "inline content" model with a new `Content` model which more accurately matches the manner in which Asciidoc handles [content substitutions](https://docs.asciidoctor.org/asciidoc/latest/subs/).

### Added

* Change `RawDelimitedBlock` to use `Content` for its inner body ([#238](https://github.com/scouten/asciidoc-parser/pull/238))
* Introduce `Content` model for rendered block content ([#236](https://github.com/scouten/asciidoc-parser/pull/236))
* Plumb `&mut Parser` through to the block-level parsers ([#235](https://github.com/scouten/asciidoc-parser/pull/235))
* Add internal `AttributeValue` struct for a single document attribute value ([#232](https://github.com/scouten/asciidoc-parser/pull/232))
* [**breaking**] Introduce new `Parser` struct which can configure and initiate parsing ([#233](https://github.com/scouten/asciidoc-parser/pull/233))
* [**breaking**] Rename `AttributeValue` to `InterpretedValue` ([#229](https://github.com/scouten/asciidoc-parser/pull/229))
* [**breaking**] Remove inline content model ([#228](https://github.com/scouten/asciidoc-parser/pull/228))
* Add new fn `Span::take_non_empty_lines` ([#225](https://github.com/scouten/asciidoc-parser/pull/225))

## [0.3.1](https://github.com/scouten/asciidoc-parser/compare/v0.3.0...v0.3.1)
_14 April 2025_

### Fixed

* Document attribute values that continue with `+` should include a line-end ([#221](https://github.com/scouten/asciidoc-parser/pull/221))
* User-defined attribute names may start with a digit ([#220](https://github.com/scouten/asciidoc-parser/pull/220))
* Enforce document attribute name restrictions (revert most of #215) ([#218](https://github.com/scouten/asciidoc-parser/pull/218))
* Document attribute names are free-form ([#215](https://github.com/scouten/asciidoc-parser/pull/215))

## [0.3.0](https://github.com/scouten/asciidoc-parser/compare/v0.2.0...v0.3.0)
_11 April 2025_

### Added

* Check block anchor name for valid XML name characters ([#208](https://github.com/scouten/asciidoc-parser/pull/208))
* Add support for block anchor syntax ([#205](https://github.com/scouten/asciidoc-parser/pull/205))
* Add `options` accessor to `IsBlock` trait ([#198](https://github.com/scouten/asciidoc-parser/pull/198))
* Add `options` accessor to `Attrlist` ([#197](https://github.com/scouten/asciidoc-parser/pull/197))
* Add `roles` accessor to `IsBlock` trait ([#195](https://github.com/scouten/asciidoc-parser/pull/195))
* Add `roles` accessor to `Attrlist` ([#193](https://github.com/scouten/asciidoc-parser/pull/193))
* Bump MSRV to 1.81.0 ([#194](https://github.com/scouten/asciidoc-parser/pull/194))
* Add method `IsBlock::id()` ([#184](https://github.com/scouten/asciidoc-parser/pull/184))
* Add new trait function `IsBlock::resolved_style` ([#182](https://github.com/scouten/asciidoc-parser/pull/182))
* Add new method `IsBlock::declared_style` ([#179](https://github.com/scouten/asciidoc-parser/pull/179))
* Rename `IsBlock::context` to `raw_context` ([#178](https://github.com/scouten/asciidoc-parser/pull/178))

### Fixed

* Add coverage for positional attributes in spec ([#202](https://github.com/scouten/asciidoc-parser/pull/202))

### Other

* Fix link to AsciiDoc repo
* Add license info for AsciiDoc language snapshot

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
