# AsciiDoc parser for Rust

[![CI](https://github.com/scouten/asciidoc-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/scouten/asciidoc-parser/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/asciidoc-parser.svg)](https://crates.io/crates/asciidoc-parser) [![docs.rs](https://img.shields.io/docsrs/asciidoc-parser)](https://docs.rs/asciidoc-parser/) [![Codecov](https://codecov.io/gh/scouten/asciidoc-parser/graph/badge.svg?token=ULDZN1IUR9)](https://codecov.io/gh/scouten/asciidoc-parser) [![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/scouten/asciidoc-parser)

This is an effort to write a semantic parser for the [AsciiDoc language](https://docs.asciidoctor.org/asciidoc/latest/) in the [Rust](https://rust-lang.org) language.

As of October 2025 this crate is ready for experimental projects, but far from a complete parser and not yet suitable for production applications. I don't list work items that I've not yet started; work items that are _partially_ implemented are documented with the [**to do** tag](https://github.com/scouten/asciidoc-parser/issues?q=is%3Aissue%20state%3Aopen%20label%3Ato-do) in GitHub. Follow that for the most current status for planned development.

You're welcome to follow along and contribute with the understanding that I may or may not drive this project a mature (1.0) release.

## Why is the code coverage so low?

If you know me from other projects, you know that I value high code coverage and that isn't changed here, despite what the badge above might say.

With this project, I'm doing an experiment in what I call “spec-driven development.” That means not only am I monitoring [coverage of the _code_](https://app.codecov.io/gh/scouten/asciidoc-parser/tree/main/parser%2Fsrc) (which is typically above 99%) but also [coverage of the _spec_](https://app.codecov.io/gh/scouten/asciidoc-parser/tree/main/docs%2Fmodules).

I'm reading page-by-page, line-by-line, and writing tests to verify that the implementation matches the specification(*). This slows progress considerably, but I expect it to result in an implementation that is very solid once complete. I've started tracking bugs found via this approach with the tag [#sddbugfind](https://github.com/scouten/asciidoc-parser/pulls?q=is%3Apr+label%3Asddbugfind+is%3Aclosed). There are many already since I started tracking in August 2025.

(*) Yes, I'm aware that the Asciidoc language authors consider this a "language description," not a specification. Since I'm experimenting with and potentially introducing the term spec-driven development as a broader term, I'm splitting the difference here.

## No planned support for some AsciiDoc features

The following features are supported in the [Ruby implementation of Asciidoctor](https://github.com/asciidoctor/asciidoctor), on which this project is based, but are not supported -- and will likely never be supported -- in this crate:

* Parsing UTF-16 content is not supported. (UTF-16 documents must be re-encoded to UTF-8 prior to parsing with this crate.)
* The document attribute [`compat-mode`](https://docs.asciidoctor.org/asciidoctor/latest/migrate/asciidoc-py/#compatibility-mode) is not supported.
* The parser has built-in support for HTML5 rendering similar to what is provided in Asciidoctor. Other back ends could be supported by other crates by implementing the `InlineSubstitutionRenderer` trait. They will not be directly supported in this crate.
* Setting document attributes via the [inline attribute entry syntax](https://docs.asciidoctor.org/asciidoc/latest/attributes/inline-attribute-entries/) is not supported. (Note that this syntax is discouraged in the AsciiDoc language documentation.)

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
