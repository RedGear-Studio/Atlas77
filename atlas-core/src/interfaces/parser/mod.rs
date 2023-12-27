use std::path::PathBuf;

use crate::Token;

use self::{parse_errors::ParseError, nodes::Program};

/// Contains all the potential Parser errors
pub mod parse_errors;

pub mod nodes;
pub mod data_types;

/// The `Parser` trait defines the interface for parsing source code and generating an abstract syntax tree (AST).
/// 
/// There should be at least 2 field on your parser:
/// * `tokens` - A Vec of `WithSpan` tokens, each representing a token along with its span.
/// * `file_path` - The path to the root source file to be processed. (This is used for error reporting.)
pub trait Parser {
    /// Parses a sequence of tokens, generating an abstract syntax tree (AST).
    ///
    /// # Returns
    ///
    /// An `AbstractSyntaxTree` that represents the parsed code's hierarchical structure and semantics.
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Program, ParseError>;
}