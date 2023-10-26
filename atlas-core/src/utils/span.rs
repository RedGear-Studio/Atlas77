use std::fmt::Display;

/// Represents a position in bytes within a source file.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct BytePos(usize);
impl BytePos {
    /// Shifts the byte position by the length of a character.
    pub fn shift(self, ch: char) -> Self {
        BytePos(self.0 + ch.len_utf8())
    }
    /// Shifts the byte position by a specified number of bytes.
    pub fn shift_by(self, n: usize) -> Self {
        BytePos(self.0 + n)
    }
}

impl Display for BytePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a span in a source file, defined by a start and end byte position.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Span {
    /// The position of character at the start of the span
    pub start: BytePos,
    /// The position of character at the end of the span
    pub end: BytePos,
}

impl Span {
    /// Creates a new `Span` without bounds checking.
    pub unsafe fn new_unchecked(start: usize, end:  usize) -> Self {
        Span {
            start: BytePos(start),
            end: BytePos(end),
        }
    }
    /// Creates an empty `Span` with both start and end positions at zero.
    pub const fn empty() -> Self {
        Span {
            start: BytePos(0),
            end: BytePos(0),
        }
    }
    /// Combines two spans to create a new span that encompasses both.
    pub fn union_span(a: Self, other: Self) -> Self {
        use std::cmp;

        Span {
            start: cmp::min(a.start, other.start),
            end: cmp::max(a.end, other.end),
        }        
    }
    /// Retrieves line information (line number, column number, and line text) associated with the span.
    pub fn get_line_info<'a>(&'a self, file: &'a str) -> (usize, usize, &str) {
        let start_byte = self.start.0;
        let end_byte = self.end.0;

        // Find the start and end of the line containing the span's start position
        let line_start = file[..start_byte].rfind('\n').map(|idx| idx + 1).unwrap_or(0);
        let line_end = file[end_byte..].find('\n').map(|idx| end_byte + idx).unwrap_or(end_byte);

        let line_text = &file[line_start..=(line_end - 1)];
        let line_number = file[..start_byte].chars().filter(|&c| c == '\n').count() + 1;
        let column_number = start_byte - line_start + 1;

        (line_number, column_number, line_text)
    }
}

impl<T> From<WithSpan<T>> for Span {
    fn from(with_span: WithSpan<T>) -> Span {
        with_span.span
    }
}

impl<T> From<&WithSpan<T>> for Span {
    fn from(with_span: &WithSpan<T>) -> Span {
        with_span.span
    }
}

/// Represents a value associated with a span in a source file.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct WithSpan<T> {
    /// The value within the span 
    /// 
    /// NB: Often use for Nodes and Tokens
    pub value: T,
    /// The span of the value
    pub span: Span,
}

impl<T> WithSpan<T> {
    /// Creates a new `WithSpan` with a value and a specified span.
    pub const fn new(value: T, span: Span) -> Self {
        WithSpan { value, span }
    }
    /// Creates an empty `WithSpan` with a value and an empty span.
    pub const fn empty(value: T) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(0),
                end: BytePos(0),
            },
        }
    }
    /// Creates a new `WithSpan` without bounds checking.
    pub const unsafe fn new_unchecked(value: T, start: usize, end: usize) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(start),
                end: BytePos(end),
            },
        }
    }
    /// Converts a `WithSpan` into a reference to a `WithSpan` with a reference to the value.
    pub const fn as_ref(&self) -> WithSpan<&T> {
        WithSpan {
            span: self.span,
            value: &self.value,
        }
    }
}
