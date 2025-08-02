// Adapted from nom-span, which comes with the following license:

// Copyright 2023 Jules Guesnon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the “Software”), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::ops::Deref;

/// Represents a subset of the overall UTF-8 input stream.
///
/// Annotated with 1-based line and column numbers relative to the
/// beginning of the overall input stream.
///
/// Called `Span` because its [`data()`] member can be consumed
/// to yield another `Span` with annotations for the end of the
/// syntactic element in question.
///
/// ## How to use it?
///
/// Here is a basic example of how to create the input and how to retrieve all
/// the informations you need.
///
/// ```
/// # use asciidoc_parser::Span;
/// #
/// fn main() {
///     let span = Span::new(r#"{"hello": "world 🙌"}"#);
///
///     assert_eq!(span.line(), 1);
///     assert_eq!(span.col(), 1);
///     assert_eq!(span.byte_offset(), 0);
/// }
/// ```
///
/// [`data()`]: Self::data
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span<'src> {
    data: &'src str,
    line: usize,
    col: usize,
    offset: usize,
}

impl<'src> Span<'src> {
    /// Create a new `Span` that describes an entire UTF-8 input stream.
    pub fn new(data: &'src str) -> Self {
        Self {
            data,
            line: 1,
            col: 1,
            offset: 0,
        }
    }

    /// Get the current line number.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get the current column number.
    pub fn col(&self) -> usize {
        self.col
    }

    /// Get the current byte offset.
    pub fn byte_offset(&self) -> usize {
        self.offset
    }

    /// Get the current data in the span.
    pub fn data(&self) -> &'src str {
        self.data
    }
}

impl AsRef<str> for Span<'_> {
    fn as_ref(&self) -> &str {
        self.data
    }
}

impl<'src> Deref for Span<'src> {
    type Target = &'src str;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// NOTE: The `Span` API is large. Only the public interface is implemented here.
// The other modules referenced below implement additional APIs that are
// available inside this crate only. (Exception: `Content` is defined here and
// exported publicly.)

mod discard;
mod line;
mod matched_item;
mod primitives;
mod r#slice;
mod split;
mod take;
mod trim;

pub(crate) use matched_item::MatchedItem;

/// Any syntactic element can describe its location
/// within the source material using this trait.
pub trait HasSpan<'src> {
    /// Return a [`Span`] describing the syntactic element's
    /// location within the source string/file.
    fn span(&self) -> Span<'src>;
}
