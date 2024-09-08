use super::{ParseResult, Span};

impl<'src> Span<'src> {
    /// Split the span, consuming a single line from the source.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    pub(crate) fn take_line(self) -> ParseResult<'src, Self> {
        let line = match self.find('\n') {
            Some(index) => self.into_parse_result(index),
            None => self.into_parse_result(self.len()),
        };

        line.trim_rem_start_matches('\n').trim_t_end_matches('\r')
    }

    /// Split the span, consuming a single line and normalizing it.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    ///
    /// All trailing spaces are removed from the line.
    pub(crate) fn take_normalized_line(self) -> ParseResult<'src, Self> {
        self.take_line().trim_t_trailing_spaces()
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
    pub(crate) fn take_non_empty_line(self) -> Option<ParseResult<'src, Self>> {
        self.split_at_match_non_empty(|c| c == '\n')
            .map(|pr| {
                pr.trim_rem_start_matches('\n')
                    .trim_t_end_matches('\r')
                    .trim_t_trailing_spaces()
            })
            .filter(|line| !line.t.is_empty())
    }

    /// Split the span, assuming the span begins with an empty line.
    ///
    /// An empty line may contain any number of white space characters.
    ///
    /// Returns `None` if the first line of the span contains any
    /// non-white-space characters.
    pub(crate) fn take_empty_line(self) -> Option<ParseResult<'src, Self>> {
        let l = self.take_line();

        if l.t.data().bytes().all(|b| b == b' ' || b == b'\t') {
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
                Some(line) => i = line.rem,
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
    pub(crate) fn take_line_with_continuation(self) -> Option<ParseResult<'src, Self>> {
        // Consume any number of lines terminated by '\\'.
        let mut pr = self.into_parse_result(0);
        while let Some(new_pr) = pr.rem.one_line_with_continuation() {
            pr = new_pr;
        }

        // Consume at most one line without a `\\` terminator.
        pr = pr.rem.take_line();

        let pr = self
            .into_parse_result(pr.rem.byte_offset() - self.byte_offset())
            .trim_t_end_matches('\n')
            .trim_t_end_matches('\r')
            .trim_t_trailing_spaces();

        if pr.t.is_empty() {
            None
        } else {
            Some(pr)
        }
    }

    fn one_line_with_continuation(self) -> Option<ParseResult<'src, Self>> {
        let line = self.take_normalized_line();
        if line.t.ends_with('\\') {
            Some(line)
        } else {
            None
        }
    }
}
