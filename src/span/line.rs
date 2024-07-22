#![allow(dead_code)] // TEMPORARY while refactoring

use super::{ParseResult, Span};
use crate::primitives::line;

impl<'a> Span<'a> {
    /// Split the span, assuming the span begins with an empty line.
    ///
    /// An empty line may contain any number of white space characters.
    ///
    /// Returns `None` if the first line of the span contains any
    /// non-white-space characters.
    pub(crate) fn take_empty_line(self) -> Option<ParseResult<'a, Self>> {
        let l = line(self);

        if l.t.data().bytes().all(nom::character::is_space) {
            Some(l)
        } else {
            None
        }
    }

    /// Discard zero or more empty lines.
    ///
    /// Return the original span if no empty lines are found.
    pub(crate) fn discard_empty_lines(self) -> Span<'a> {
        let mut i = self;

        while !i.data().is_empty() {
            match i.take_empty_line() {
                Some(line) => i = line.rem,
                None => break,
            }
        }

        i
    }
}
