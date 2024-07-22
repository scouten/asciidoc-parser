use super::{ParseResult, Span};

impl<'a> Span<'a> {
    /// Split the span, consuming a single line from the source.
    ///
    /// A line is terminated by end-of-input or a single `\n` character
    /// or a single `\r\n` sequence. The end of line sequence is consumed
    /// but not included in the returned line.
    pub(crate) fn take_line(self) -> ParseResult<'a, Self> {
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
    pub(crate) fn take_normalized_line(self) -> ParseResult<'a, Self> {
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
    pub(crate) fn take_non_empty_line(self) -> Option<ParseResult<'a, Self>> {
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
    pub(crate) fn take_empty_line(self) -> Option<ParseResult<'a, Self>> {
        let l = self.take_line();

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
