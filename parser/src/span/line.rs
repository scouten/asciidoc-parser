use super::{MatchedItem, Span};

impl<'src> Span<'src> {
    /// Split the span, consuming a single line from the source.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    pub(crate) fn take_line(self) -> MatchedItem<'src, Self> {
        let line = match self.find('\n') {
            Some(index) => self.into_parse_result(index),
            None => self.into_parse_result(self.len()),
        };

        line.trim_after_start_matches('\n')
            .trim_item_end_matches('\r')
    }

    /// Split the span, consuming a single line and normalizing it.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    ///
    /// All trailing spaces are removed from the line.
    pub(crate) fn take_normalized_line(self) -> MatchedItem<'src, Self> {
        self.take_line().trim_item_trailing_spaces()
    }

    /// Split the span, consuming a single _normalized, non-empty_ line from the
    /// source if one exists.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    ///
    /// All trailing spaces are removed from the line.
    ///
    /// Returns `None` if the line becomes empty after trailing spaces have been
    /// removed.
    pub(crate) fn take_non_empty_line(self) -> Option<MatchedItem<'src, Self>> {
        self.split_at_match_non_empty(|c| c == '\n')
            .map(|mi| {
                mi.trim_after_start_matches('\n')
                    .trim_item_end_matches('\r')
                    .trim_item_trailing_spaces()
            })
            .filter(|line| !line.item.is_empty())
    }

    /// Split the span, assuming the span begins with an empty line.
    ///
    /// An empty line may contain any number of white space characters.
    ///
    /// Returns `None` if the first line of the span contains any
    /// non-white-space characters.
    pub(crate) fn take_empty_line(self) -> Option<MatchedItem<'src, Self>> {
        let l = self.take_line();

        if l.item.data().bytes().all(|b| b == b' ' || b == b'\t') {
            Some(l)
        } else {
            None
        }
    }

    /// Discard zero or more empty lines.
    ///
    /// Return the original span if no empty lines are found.
    pub(crate) fn discard_empty_lines(self) -> Span<'src> {
        let mut i = self;

        while !i.data().is_empty() {
            match i.take_empty_line() {
                Some(line) => i = line.after,
                None => break,
            }
        }

        i
    }

    /// Split the span, consuming one normalized, non-empty line that may be
    /// continued onto subsequent lines with an explicit continuation marker.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    ///
    /// A line is _not_ terminated by `\n` when preceded by a `\` character.
    ///
    /// SPEC QUESTION: Is it allowed to have a `+` character followed by
    /// white space? For now, I'm saying yes.
    ///
    /// Trailing white spaces are removed from the final line and are not
    /// removed from any lines with continuations.
    ///
    /// Returns `None` if the line becomes empty after trailing spaces have been
    /// removed.
    pub(crate) fn take_line_with_continuation(self) -> Option<MatchedItem<'src, Self>> {
        // Consume any number of lines terminated by '\\'.
        let mut mi = self.into_parse_result(0);
        while let Some(new_pr) = mi.after.one_line_with_continuation() {
            mi = new_pr;
        }

        // Consume at most one line without a `\\` terminator.
        mi = mi.after.take_line();

        let mi = self
            .into_parse_result(mi.after.byte_offset() - self.byte_offset())
            .trim_item_end_matches('\n')
            .trim_item_end_matches('\r')
            .trim_item_trailing_spaces();

        if mi.item.is_empty() {
            None
        } else {
            Some(mi)
        }
    }

    fn one_line_with_continuation(self) -> Option<MatchedItem<'src, Self>> {
        let line = self.take_normalized_line();
        if line.item.ends_with('\\') {
            Some(line)
        } else {
            None
        }
    }
}
