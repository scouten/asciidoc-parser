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

use std::{
    convert::AsRef,
    ops::{Deref, RangeTo},
};

use bytecount::num_chars;
use memchr::Memchr;
use nom::{
    AsBytes, Compare, Err, FindSubstring, InputIter, InputLength, InputTake, InputTakeAtPosition,
    Offset, Slice,
};

/// Represents a subset of the overall UTF-8 input stream.
///
/// Annotated with 1-based line and column numbers relative to the
/// beginning of the overall input stream.
///
/// Called `Span` because its `data` member can be consumed
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

/// You can wrap your input in this struct with [`Span::new`].
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Span<'a> {
    data: &'a str,
    line: usize,
    col: usize,
    offset: usize,
}

impl<'a> Span<'a> {
    /// Create a new `Span` that describes an entire UTF-8 input stream.
    pub fn new(data: &'a str) -> Self {
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
    pub fn data(&self) -> &'a str {
        self.data
    }

    /// Splits the current input stream into a [`ParseResult<Span>`] at the
    /// given position.
    #[allow(dead_code)] // TEMPORARY while refactoring
    pub(crate) fn into_parse_result(&self, at_index: usize) -> ParseResult<Self> {
        ParseResult {
            t: self.slice(..at_index),
            rem: self.slice(at_index..),
        }
    }
}

impl<'a> AsBytes for Span<'a> {
    fn as_bytes(&self) -> &'a [u8] {
        self.data.as_bytes()
    }
}

impl<'a> AsRef<str> for Span<'a> {
    fn as_ref(&self) -> &str {
        self.data
    }
}

impl<'a> Compare<&str> for Span<'a> {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.data.compare(t)
    }

    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.data.compare_no_case(t)
    }
}

impl<'a> Deref for Span<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/* maybe this isn't needed either?
impl<'a> ExtendInto for Span<'a> {
    type Extender = &'a <str as ExtendInto>::Extender;
    type Item = &'a <str as ExtendInto>::Item;

    fn new_builder(&self) -> Self::Extender {
        self.data.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.data.extend_into(acc);
    }
}
*/

impl<'a> FindSubstring<&str> for Span<'a> {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.data.find_substring(substr)
    }
}

/* Maybe we don't need this?
impl<'a, Token> FindToken<Token> for Span<'a>
where
    &'a str: FindToken<Token>,
{
    fn find_token(&self, token: Token) -> bool {
        self.data.find_token(token)
    }
}
*/

impl<'a> InputIter for Span<'a>
where
    &'a str: InputIter,
{
    type Item = <&'a str as InputIter>::Item;
    type Iter = <&'a str as InputIter>::Iter;
    type IterElem = <&'a str as InputIter>::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.data.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.data.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.data.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.data.slice_index(count)
    }
}

impl<'a> InputLength for Span<'a> {
    fn input_len(&self) -> usize {
        self.data.input_len()
    }
}

impl<'a> InputTake for Span<'a> {
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<'a> InputTakeAtPosition for Span<'a> {
    type Item = <&'a str as InputIter>::Item;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        _e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.data.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(*self, e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.data.input_len() == 0 {
                    Err(Err::Error(E::from_error_kind(*self, e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl<'a> Offset for Span<'a> {
    fn offset(&self, second: &Self) -> usize {
        self.data.offset(second.data)
    }
}

/* Maybe we don't need this?
impl<'a, R: FromStr> ParseTo<R> for Span<'a> {
    fn parse_to(&self) -> Option<R> {
        self.data.parse_to()
    }
}
*/

impl<'a, R> Slice<R> for Span<'a>
where
    &'a str: Slice<R> + Offset + AsBytes + Slice<RangeTo<usize>>,
{
    fn slice(&self, range: R) -> Self {
        let next_data = self.data.slice(range);
        let offset = self.data.offset(next_data);

        if offset == 0 {
            return Self {
                data: next_data,
                line: self.line,
                col: self.col,
                offset: self.offset,
            };
        }

        let old_data = self.data.slice(..offset);
        let new_line_iter = Memchr::new(b'\n', old_data.as_bytes());

        let mut lines_to_add = 0;
        let mut last_index = None;
        for i in new_line_iter {
            lines_to_add += 1;
            last_index = Some(i);
        }
        let last_index = last_index.map_or(0, |v| v + 1);

        let col = num_chars(old_data.as_bytes().slice(last_index..));

        Self {
            data: next_data,
            line: self.line + lines_to_add,
            col: if lines_to_add == 0 {
                self.col + col
            } else {
                // When going to a new line, char starts at 1
                col + 1
            },
            offset: self.offset + offset,
        }
    }
}

/// Any syntactic element can describe its location
/// within the source material using this trait.
pub trait HasSpan<'a> {
    /// Return a [`Span`] describing the syntactic element's
    /// location within the source string/file.
    fn span(&'a self) -> &'a Span<'a>;
}

/// Represents a successful parse result and subsequent remainder of the input
/// stream.
pub(crate) struct ParseResult<'a, T> {
    pub(crate) t: T,
    pub(crate) rem: Span<'a>,
}
