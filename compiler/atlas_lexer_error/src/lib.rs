use std::{error::Error, fmt};
use atlas_span::{Span, Spanned};


#[derive(Debug)]
pub enum AtlasLexerError {
    UnexpectedCharacter{
        val:char,
        span: Span,
    },
    InvalidNumber {
        span: Span,
    },
    UnterminatedStringLiteral{
        span: Span,
    },
    UnterminatedComment {
        span: Span,
    },
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

