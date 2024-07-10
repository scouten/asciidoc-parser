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
    ops::{RangeFrom, RangeTo},
    str::FromStr,
};

/// Represents a subset of the overall input stream.
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
/// ```ignore
/// use crate::Span;
///
/// fn main() {
///     let span = Span::new(
///       r#"{"hello": "world 🙌"}"#,
///     );
///
///     assert_eq!(span.line(), 1);
///     assert_eq!(span.col(), 1);
///     assert_eq!(span.byte_offset(), 0);
/// }
/// ```
use bytecount::num_chars;
use memchr::Memchr;
use nom::{
    AsBytes, Compare, Err, ExtendInto, FindSubstring, FindToken, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, ParseTo, Slice,
};

/// You can wrap your input in this struct with [`Spanned::new`]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Span {
    data: &str,
    line: usize,
    col: usize,
    offset: usize,
}

impl Span {
    /// Create a new `Span` that describes an entire UTF-8 input stream.
    pub fn new(data: &str) -> Self {
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
    pub fn data(&self) -> &&str {
        &self.data
    }
}

impl std::ops::Deref<&str> for Span {
    type Target = &str;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::convert::AsRef<&str> for Span {
    fn as_ref(&self) -> &U {
        self.data.as_ref()
    }
}

impl AsBytes for Span {
    fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl Compare<&str> for Span {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.data.compare(t)
    }

    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.data.compare_no_case(t)
    }
}

impl ExtendInto for Span {
    type Extender = &str::Extender;
    type Item = &str::Item;

    fn new_builder(&self) -> Self::Extender {
        self.data.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.data.extend_into(acc);
    }
}

impl FindSubstring<&str> for Span {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.data.find_substring(substr)
    }
}

impl<Token> FindToken<Token> for Span {
    fn find_token(&self, token: Token) -> bool {
        self.data.find_token(token)
    }
}

impl InputIter for Span {
    type Item = &str::Item;
    type Iter = &str::Iter;
    type IterElem = &str::IterElem;

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

impl InputLength for Span {
    fn input_len(&self) -> usize {
        self.data.input_len()
    }
}

impl InputTake for Span {
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl InputTakeAtPosition for Span {
    type Item = <&str as InputIter>::Item;

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
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.data.input_len() == 0 {
                    Err(Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl Offset for Span {
    fn offset(&self, second: &Self) -> usize {
        self.data.offset(&second.data)
    }
}

impl<R: FromStr> ParseTo<R> for Span {
    fn parse_to(&self) -> Option<R> {
        self.data.parse_to()
    }
}

impl<R> Slice<R> for Span {
    fn slice(&self, range: R) -> Self {
        let next_data = self.data.slice(range);

        let offset = self.data.offset(&next_data);

        let old_data = self.data.slice(..offset);

        if offset == 0 {
            return Self {
                data: next_data,
                line: self.line,
                col: self.col,
                offset: self.offset,
            };
        }

        let new_line_iter = Memchr::new(b'\n', old_data.as_bytes());

        let mut lines_to_add = 0;
        let mut last_index = None;
        for i in new_line_iter {
            lines_to_add += 1;
            last_index = Some(i);
        }
        let last_index = last_index.map_or(0, |v| v + 1);

        let col = if self.handle_utf8 {
            num_chars(old_data.as_bytes().slice(last_index..))
        } else {
            old_data.as_bytes().len() - last_index
        };

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
