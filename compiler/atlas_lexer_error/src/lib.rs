use std::{error::Error, fmt};
use atlas_span::{Span, Spanned};


#[derive(Debug)]
pub enum AtlasLexerError {
    UnexpectedCharacter{
        val:char,
        span: Span,
        recoverable: bool,
    },
    InvalidNumber {
        span: Span,
        recoverable: bool,
    },
    UnterminatedStringLiteral{
        span: Span,
        recoverable: bool,
    },
    UnterminatedComment {
        span: Span,
        recoverable: bool,
    },
}

impl AtlasLexerError {
    pub fn change_span(&mut self, span: Span) -> Self {
        match self {
            AtlasLexerError::UnexpectedCharacter { span: _, recoverable, val, .. } => AtlasLexerError::UnexpectedCharacter { span, recoverable: *recoverable,  val: *val },
            AtlasLexerError::InvalidNumber { span: _, recoverable, .. } => AtlasLexerError::InvalidNumber { span, recoverable: *recoverable },
            AtlasLexerError::UnterminatedStringLiteral { span: _, recoverable, .. } => AtlasLexerError::UnterminatedStringLiteral { span, recoverable: *recoverable },
            AtlasLexerError::UnterminatedComment { span: _, recoverable, .. } => AtlasLexerError::UnterminatedComment { span, recoverable: *recoverable },
        }
    }
    pub fn is_recoverable(&self) -> bool {
        match self {
            AtlasLexerError::UnexpectedCharacter { recoverable, .. } => *recoverable,
            AtlasLexerError::InvalidNumber { recoverable, .. } => *recoverable,
            AtlasLexerError::UnterminatedStringLiteral { recoverable, .. } => *recoverable,
            AtlasLexerError::UnterminatedComment { recoverable, .. } => *recoverable,
        }
    }
}

impl Spanned for AtlasLexerError {
    fn span(&self) -> Span {
        match self {
            AtlasLexerError::UnexpectedCharacter { span, .. } => *span,
            AtlasLexerError::InvalidNumber { span, .. } => *span,
            AtlasLexerError::UnterminatedStringLiteral { span, .. } => *span,
            AtlasLexerError::UnterminatedComment { span, .. } => *span,
        }
    }
}

impl fmt::Display for AtlasLexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtlasLexerError::UnexpectedCharacter{val, ..} => write!(f, "Unexpected character: {}", val),
            AtlasLexerError::InvalidNumber{..} => write!(f, "Invalid number"),
            AtlasLexerError::UnterminatedStringLiteral{..} => write!(f, "Unterminated string literal"),
            AtlasLexerError::UnterminatedComment{..} => write!(f, "Unterminated comment"),
        }
    }
}

impl Error for AtlasLexerError {}

