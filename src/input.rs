//! Data types that describe the location of a syntactic element.

use std::fmt::Display;

use nom_span::Spanned;

pub(crate) type Input<'a> = Spanned<&'a str>;

/// Location within an input stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
    line: usize,
    col: usize,
}

impl Location {
    /// Return the line number (1-based).
    pub fn line(&self) -> usize {
        self.line
    }

    /// Return the column number (1-based).
    pub fn col(&self) -> usize {
        self.col
    }
}

impl<'a> From<&Input<'a>> for Location {
    fn from(i: &Input<'a>) -> Self {
        Self {
            line: i.line(),
            col: i.col(),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{col}", line = self.line, col = self.col)
    }
}

/// Span (start and end location) within an input stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    start: Location,
    after_end: Location,
}

impl Span {
    #[allow(dead_code)] // TEMPORARY
    pub(crate) fn from_start_and_after_end(start: &'_ Input<'_>, after_end: &'_ Input<'_>) -> Self {
        Self {
            start: start.into(),
            after_end: after_end.into(),
        }
    }

    /// Returns the starting location of this span.
    pub fn start(&self) -> Location {
        self.start
    }

    /// Returns the location immediately after the end of this span.
    pub fn after_end(&self) -> Location {
        self.after_end
    }

    /// Returns true if the `start` and `after_end` locations are the same.
    pub fn is_empty(&self) -> bool {
        self.start == self.after_end
    }
}
