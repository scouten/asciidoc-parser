use std::fmt::Debug;

use crate::{Parser, attributes::Attrlist};

/// An `IncludeFileHandler` is responsible for providing the text content for an
/// `include::` directive when encountered.
///
/// A client of [`Parser`] may provide an `IncludeFileHandler` to customize how
/// include file resolution is handled.
///
/// [`Parser`]: crate::Parser
pub trait IncludeFileHandler: Debug {
    /// Provide the file content for an `include::` directive, if available.
    ///
    /// # Parameters
    /// - `source`: The path to the document that is including the file. A root
    ///   document may be signaled via `None` depending on how the parser was
    ///   invoked. This path should be considered when resolving relative paths.
    /// - `target`: The path to the document that was provided in the
    ///   `include::` directive.
    /// - `attrlist`: Any attributes specified on the include directive.
    /// - `parser`: An implementation may read document attribute values from
    ///   the [`Parser`] state.
    ///
    /// Return the string content of the include file if found. If no file is
    /// found, return `None` and an appropriate warning message will be
    /// generated.
    ///
    /// # Options
    /// With the exception of `encoding` (see below), the implementation should
    /// not attempt to interpret any of the built-in attributes (i.e.
    /// `leveloffset`, `lines`, `tags`, or `indent`). Correct handling of these
    /// attributes will be provided by the parser itself.
    ///
    /// # Encoding
    /// If a `Some` result is provided, it is a typical Rust [`String`] and
    /// therefore must be encoded as UTF-8. If the implementation is capable of
    /// transcoding from other formats, it may use the `encoding` attribute as a
    /// hint of the source format. If the implementation finds a file that is
    /// not encoded in UTF-8 and is incapable of translating, it should return
    /// `None`.
    fn resolve_target<'src>(
        &self,
        source: Option<&str>,
        target: &str,
        attrlist: &Attrlist<'src>,
        parser: &Parser,
    ) -> Option<String>;
}
