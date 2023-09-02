use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct BytePos(usize);
impl BytePos {
    pub fn shift(self, ch: char) -> Self {
        BytePos(self.0 + ch.len_utf8())
    }
}

impl Display for BytePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Span {
    pub start: BytePos,
    pub end: BytePos,
}

impl Span {
    pub unsafe fn new_unchecked(start: usize, end:  usize) -> Self {
        Span {
            start: BytePos(start),
            end: BytePos(end),
        }
    }

    pub const fn empty() -> Self {
        Span {
            start: BytePos(0),
            end: BytePos(0),
        }
    }

    pub fn union_span(a: Self, other: Self) -> Self {
        use std::cmp;

        Span {
            start: cmp::min(a.start, other.start),
            end: cmp::max(a.end, other.end),
        }        
    }

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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}

impl<T> WithSpan<T> {
    pub const fn new(value: T, span: Span) -> Self {
        WithSpan { value, span }
    }

    pub const fn empty(value: T) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(0),
                end: BytePos(0),
            },
        }
    }

    pub const unsafe fn new_unchecked(value: T, start: usize, end: usize) -> Self {
        Self {
            value,
            span: Span {
                start: BytePos(start),
                end: BytePos(end),
            },
        }
    }

    pub const fn as_ref(&self) -> WithSpan<&T> {
        WithSpan {
            span: self.span,
            value: &self.value,
        }
    }
}