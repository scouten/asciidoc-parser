# AsciiDoc parser for Rust

[![CI](https://github.com/scouten/asciidoc-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/scouten/asciidoc-parser/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/asciidoc-parser.svg)](https://crates.io/crates/asciidoc-parser) [![docs.rs](https://img.shields.io/docsrs/asciidoc-parser)](https://docs.rs/asciidoc-parser/) [![Codecov](https://codecov.io/gh/scouten/asciidoc-parser/graph/badge.svg?token=ULDZN1IUR9)](https://codecov.io/gh/scouten/asciidoc-parser) [![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/scouten/asciidoc-parser)

This is an effort to write a semantic parser for the [AsciiDoc language](https://docs.asciidoctor.org/asciidoc/latest/) in the [Rust](https://rust-lang.org) language.

The project is in its infancy as of March 2025 and in **no way ready to use.**

You're welcome to follow along and contribute with the understanding that I may or may not drive this project a mature (1.0) release.

## Known limitations

* Parsing UTF-16 content is not supported. (UTF-16 documents must be re-encoded to UTF-8 prior to parsing with this crate.) I have no plans to support UTF-16 content.

## Licenses

The `asciidoc-parser` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates may be licensed under different terms; please check the license terms for each crate and component for details.

### License for AsciiDoc language materials

IMPORTANT: This repository contains a snapshot of the AsciiDoc language description which comes with its own license terms. It is not the purpose of _this_ repository to supplant or replace that description; these documents are here as part of tooling to ensure that this crate follows the language description as closely as possible. Please consult [AsciiDoc Language @ Eclipse GitLab](https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang) for the official language description.

The following applies to content in the `asg` and `spec` folders:

> The AsciiDoc Language and the accompanying materials are made available under the terms of the Eclipse Public License v 2.0 (EPL-2.0). See [LICENSE](https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang/-/blob/main/LICENSE) to find the full license text.

The following applies to content in the `docs` folder:

> The user documentation for the AsciiDoc Language, located in the docs/ folder, is made available under the terms of a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/) (CC-BY-4.0).
