use std::str::{CharIndices, Chars};

use nom::{
    Compare, FindSubstring, InputIter, InputLength, InputTake, InputTakeAtPosition, Offset, Slice,
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
/// TO DO: Planning to remove dependency on `nom_span` crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Span<'a> {
    s: nom_span::Spanned<&'a str>,
}

impl<'a> Span<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            s: nom_span::Spanned::new(data, true),
        }
    }

    /// Get the current line number
    pub fn line(&self) -> usize {
        self.s.line()
    }

    /// Get the current column number
    pub fn col(&self) -> usize {
        self.s.col()
    }

    /// Get the current byte offset
    pub fn byte_offset(&self) -> usize {
        self.s.byte_offset()
    }

    /// Get the current data in the span
    pub fn data(&self) -> &'a str {
        &self.s.data()
    }
}

impl<'a> core::convert::AsRef<str> for Span<'a> {
    fn as_ref(&self) -> &str {
        self.s.as_ref()
    }
}

impl<'a> core::ops::Deref for Span<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        self.s.deref()
    }
}

impl<'a> Compare<&'a str> for Span<'a> {
    fn compare(&self, t: &'a str) -> nom::CompareResult {
        self.s.compare(t)
    }

    fn compare_no_case(&self, t: &'a str) -> nom::CompareResult {
        self.s.compare_no_case(t)
    }
}

impl<'a> FindSubstring<&str> for Span<'a> {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.s.find_substring(substr)
    }
}

impl<'a> InputIter for Span<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    fn iter_indices(&self) -> Self::Iter {
        self.s.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.s.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.s.slice_index(count)
    }
}

impl<'a> InputLength for Span<'a> {
    fn input_len(&self) -> usize {
        self.s.input_len()
    }
}

impl<'a> InputTake for Span<'a> {
    fn take(&self, count: usize) -> Self {
        Self {
            s: self.s.take(count),
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (a, b) = self.s.take_split(count);
        (Self { s: a }, Self { s: b })
    }
}

impl<'a> InputTakeAtPosition for Span<'a> {
    type Item = char;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s
            .split_at_position(predicate)
            .map(|(s1, s2)| (Self { s: s1 }, Self { s: s2 }))
            .map_err(span_err)
    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s
            .split_at_position1(predicate, e)
            .map(|(s1, s2)| (Self { s: s1 }, Self { s: s2 }))
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s
            .split_at_position_complete(predicate)
            .map(|s| Self { s })
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s
            .split_at_position1_complete(predicate, e)
            .map(|s| Self { s })
    }
}

impl<'a> Offset for Span<'a> {
    fn offset(&self, second: &Self) -> usize {
        self.s.offset(&second.s)
    }
}

impl<'a, R> Slice<R> for Span<'a> {
    fn slice(&self, range: R) -> Self {
        Self {
            s: self.s.slice(range),
        }
    }
}

fn map_span_err(e: nom::error::ParseError<nom_span::Spanned<&str>>) -> nom::error::ParseError<Span> {
}
