use core::fmt;
use std::path::Path;

use ariadne::{Span as AriadneSpan, Line};

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

impl fmt::Display for BytePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    /// The source file path
    pub path: &'static str,
}

impl AriadneSpan for Span {
    type SourceId = Path;

    fn start(&self) -> usize {
        self.start.0
    }

    fn end(&self) -> usize {
        self.end.0
    }

    fn len(&self) -> usize {
        self.end.0 - self.start.0
    }

    fn contains(&self, offset: usize) -> bool {
        self.start.0 <= offset && offset <= self.end.0
    }

    fn source(&self) -> &Self::SourceId {
        Path::new(self.path)
    }
}

impl Span {
    /// Creates a new `Span` without bounds checking.
    pub unsafe fn new_unchecked(start: usize, end:  usize, path: &'static str) -> Self {
        Span {
            start: BytePos(start),
            end: BytePos(end),
            path,
        }
    }

    /// Creates an empty `Span` with both start and end positions at zero.
    pub const fn empty() -> Self {
        Span {
            start: BytePos(0),
            end: BytePos(0),
            path: "",
        }
    }

    /// Combines two spans to create a new span that encompasses both.
    pub fn union_span(self, other: Self) -> Self {
        use std::cmp;
        if self.path != other.path {
            panic!("Cannot union spans from different files: {} and {}", self.path, other.path);
        }
        Span {
            start: cmp::min(self.start, other.start),
            end: cmp::max(self.end, other.end),
            path: self.path,
        }        
    }

    /// Retrieves line information associated with the span.
    pub fn get_line_info(&self) -> LineInformation {
        let start_byte = self.start.0;
        let end_byte = self.end.0;

        // Find the start and end of the line containing the span's start position
        let line_start = self.path[..start_byte].rfind('\n').map(|idx| idx + 1).unwrap_or(0);
        let line_end = self.path[end_byte..].find('\n').map(|idx| end_byte + idx).unwrap_or(end_byte);

        let line_text = self.path[line_start..=(line_end - 1)].to_owned();
        let line_number = self.path[..start_byte].chars().filter(|&c| c == '\n').count() + 1;
        let column_number = start_byte - line_start + 1;

        LineInformation::new(line_number, column_number, line_text)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{} in \"{}\"]", self.start, self.end, self.path)
    }
}

pub struct LineInformation {
    pub line_number: usize,
    pub column_number: usize,
    pub line_text: String,
}

impl LineInformation {
    pub fn new(line_number: usize, column_number: usize, line_text: String) -> Self {
        LineInformation {
            line_number,
            column_number,
            line_text,
        }
    }
}

impl fmt::Display for LineInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:\n {}", self.line_number, self.column_number, self.line_text)
    }
}