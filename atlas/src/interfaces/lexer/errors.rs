use crate::utils::span;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnknownToken(char, span::Span)
}