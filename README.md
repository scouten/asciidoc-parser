# AsciiDoc parser for Rust

[![CI](https://github.com/scouten/asciidoc-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/scouten/asciidoc-parser/actions/workflows/ci.yml) [![Codecov](https://codecov.io/gh/scouten/asciidoc-parser/graph/badge.svg?token=ULDZN1IUR9)](https://codecov.io/gh/scouten/asciidoc-parser) [![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/scouten/asciidoc-parser)

This is an effort to write a semantic parser for the [AsciiDoc language](https://docs.asciidoctor.org/asciidoc/latest/). 

The project is in its infancy as of October 2024 and in **no way ready to use.**

You're welcome to follow along and contribute with the understanding that I may or may not drive this project a mature (1.0) release.

## Known limitations

* Parsing UTF-16 content is not supported. (UTF-16 documents must be re-encoded to UTF-8 prior to parsing with this crate.) I have no plans to support UTF-16 content.

## License

The `asciidoc-parser` crate is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).

Note that some components and dependent crates may be licensed under different terms; please check the license terms for each crate and component for details.
