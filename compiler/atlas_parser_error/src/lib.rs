use core::fmt;
use std::error::Error;

use atlas_span::{Span, Spanned};
use atlas_lexer_token::TokenKind;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedEndOfInput {
        span: Span,
        recoverable: bool,
    },
    UnexpectedToken {
        span: Span,
        expected: TokenKind,
        recoverable: bool,
        found: TokenKind,
    },
    InvalidExpression {
        span: Span,
        recoverable: bool,
    },
    MissingOperand {
        span: Span,
        recoverable: bool,
    },
    MissingClosingDelimiter {
        span: Span,
        recoverable: bool,
        // Can be either ')', ']', '}', '''
        expected: TokenKind,
    },
}

impl Spanned for ParseError {
    fn span(&self) -> Span {
        match self {
            ParseError::UnexpectedEndOfInput { span, .. } => *span,
            ParseError::UnexpectedToken { span, .. } => *span,
            ParseError::InvalidExpression { span, .. } => *span,
            ParseError::MissingOperand { span, .. } => *span,
            ParseError::MissingClosingDelimiter { span, .. } => *span,
        }
    }
}

impl ParseError {
    pub fn change_span(&mut self, span: Span) -> Self {
        match self {
            ParseError::UnexpectedEndOfInput { recoverable, .. } => ParseError::UnexpectedEndOfInput {
                span,
                recoverable: *recoverable,
            },
            ParseError::UnexpectedToken { expected, found, recoverable, .. } => ParseError::UnexpectedToken {
                span,
                recoverable: *recoverable,
                expected: expected.clone(),
                found: found.clone(),
            },
            ParseError::InvalidExpression { recoverable, .. } => ParseError::InvalidExpression {
                span,
                recoverable: *recoverable,
            },
            ParseError::MissingOperand { recoverable, .. } => ParseError::MissingOperand {
                span,
                recoverable: *recoverable,
            },
            ParseError::MissingClosingDelimiter { expected, recoverable, .. } => ParseError::MissingClosingDelimiter {
                span,
                recoverable: *recoverable,
                expected: expected.clone(),
            },
        }
    }

    pub fn is_recoverable(&self) -> bool {
        match self {
            ParseError::UnexpectedEndOfInput { recoverable, .. } => *recoverable,
            ParseError::UnexpectedToken { recoverable, .. } => *recoverable,
            ParseError::InvalidExpression { recoverable, .. } => *recoverable,
            ParseError::MissingOperand { recoverable, .. } => *recoverable,
            ParseError::MissingClosingDelimiter { recoverable, .. } => *recoverable,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEndOfInput { .. } => write!(f, "unexpected end of input"),
            ParseError::UnexpectedToken { expected, found, .. } => write!(f, "expected {}, found {}", expected, found),
            ParseError::InvalidExpression { .. } => write!(f, "invalid expression"),
            ParseError::MissingOperand { .. } => write!(f, "missing operand"),
            ParseError::MissingClosingDelimiter { expected, .. } => write!(f, "missing closing delimiter {}", expected),
        }
    }
}

impl Error for ParseError {}