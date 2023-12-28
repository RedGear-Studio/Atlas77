use std::fmt::Display;

use atlas_core::{lexer_errors::LexerError, interfaces::error::Error, utils::span::{Spanned, Span}};

#[derive(Debug, Clone)]
pub enum LexError {
    UnknownCharacter {
        ch: char,
        code: u64,
        span: Span,
        recoverable: bool,
    }
}

impl LexerError for LexError {}

impl Error for LexError {
    fn code(&self) -> u64 {
        match self {
            LexError::UnknownCharacter { code, .. } => *code,
        }
    }
    fn recoverable(&self) -> bool {
        match self {
            LexError::UnknownCharacter { recoverable, .. } => *recoverable,
        }
    }
    ///Todo
    fn help(&self) -> Option<String> {
        None
    }
    fn message(&self) -> String {
        match self {
            LexError::UnknownCharacter { ch, .. } => format!("Unknown character: {}", ch),
        }
    }
}

impl Spanned for LexError {
    fn span(&self) -> Span {
        match self {
            LexError::UnknownCharacter { span, .. } => *span,
        }
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //will change this later to use ariadne maybe for better error messages
        write!(f, "{}", self.message())
    }
}