use std::ops::{Range, RangeFrom, RangeTo};

use bytecount::num_chars;
use memchr::Memchr;

use super::Span;

impl<'a> Span<'a> {
    /// Returns the requested subrange of this input span.
    pub fn slice(&self, range: Range<usize>) -> Self {
        self.slice_internal(&self.data[range])
    }

    /// Returns the requested subrange of this input span.
    pub fn slice_from(&self, range: RangeFrom<usize>) -> Self {
        self.slice_internal(&self.data[range])
    }

    /// Returns the requested subrange of this input span.
    pub fn slice_to(&self, range: RangeTo<usize>) -> Self {
        self.slice_internal(&self.data[range])
    }

    /// Returns the first position where `predicate` returns `true`.
    pub fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(char) -> bool,
    {
        for (o, c) in self.data.char_indices() {
            if predicate(c) {
                return Some(o);
            }
        }
        None
    }

    fn slice_internal(&self, slice_data: &'a str) -> Self {
        let offset = offset(self.data, slice_data);

        if offset == 0 {
            return Self {
                data: slice_data,
                line: self.line,
                col: self.col,
                offset: self.offset,
            };
        }

        let old_data = &self.data[..offset];
        let new_line_iter = Memchr::new(b'\n', old_data.as_bytes());

        let mut lines_to_add = 0;
        let mut last_index = None;
        for i in new_line_iter {
            lines_to_add += 1;
            last_index = Some(i);
        }
        let last_index = last_index.map_or(0, |v| v + 1);

        let col = num_chars(&old_data.as_bytes()[last_index..]);

        Self {
            data: slice_data,
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

fn offset(first: &str, second: &str) -> usize {
    let p1 = first.as_ptr();
    let p2 = second.as_ptr();
    p2 as usize - p1 as usize
}
