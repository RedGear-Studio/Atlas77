use std::fmt::Display;

use atlas_core::{parser_errors::ParseError, interfaces::error::Error, utils::span::{Spanned, Span}, TokenKind};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ParseErr {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
        code: u64,
        recoverable: bool,
    }
}

impl Error for ParseErr {
    fn code(&self) -> u64 {
        match self {
            ParseErr::UnexpectedToken { code, .. } => *code,
        }
    }
    fn help(&self) -> Option<String> {
        None
    }
    fn recoverable(&self) -> bool {
        match self {
            ParseErr::UnexpectedToken { recoverable, .. } => *recoverable,
        }
    }
    fn message(&self) -> String {
        match self {
            ParseErr::UnexpectedToken { expected, found, .. } => {
                format!("Unexpected Token Error:\nExpected {}, found {}", expected, found)
            }
        }
    }
}

impl Spanned for ParseErr {
    #[inline(always)]
    fn span(&self) -> Span {
        match self {
            ParseErr::UnexpectedToken { span, .. } => *span,
        }
    }
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl ParseError for ParseErr {}
