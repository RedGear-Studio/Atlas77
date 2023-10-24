use crate::utils::span::WithSpan;
use crate::interfaces::lexer::token;

pub mod ast;
pub mod errors;

/// The `Parser` trait defines the interface for parsing source code and generating an abstract syntax tree (AST).
pub trait Parser {
        /// Creates a new instance of a parser for a given file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the source file to be processed.
    ///
    /// # Returns
    ///
    /// A `Result` that contains the parser instance if successful, or an `std::io::Error` if there's an issue
    /// with file I/O (e.g., file not found, permission issues).
    fn new(file_path: String) -> Result<Self, std::io::Error>
        where Self: Sized;
    /// Parses a sequence of tokens, generating an abstract syntax tree (AST).
    ///
    /// # Arguments
    ///
    /// * `tokens` - A slice of `WithSpan` objects, each representing a token along with its span.
    ///
    /// # Returns
    ///
    /// An `ast::AbstractSyntaxTree` that represents the parsed code's hierarchical structure and semantics.
    fn parse(&mut self, tokens: &[WithSpan<token::Token>]) -> ast::AbstractSyntaxTree;
    /// Checks the parsed abstract syntax tree (AST) for correctness and adherence to the language's grammar rules.
    ///
    /// # Arguments
    ///
    /// * `_ast` - The abstract syntax tree (AST) generated during parsing.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok`). This method is a placeholder and is intended to be implemented in the future.
    ///
    /// # Example
    ///
    /// ```
    /// use my_parser::Parser;
    ///
    /// let mut my_parser = MyParser::new("source_file.txt".to_string()).unwrap();
    /// let tokens = /* Lex tokens from the source file */;
    /// let ast = my_parser.parse(&tokens);
    ///
    /// if let Err(err) = my_parser.check(ast) {
    ///     eprintln!("Parser error: {:?}", err);
    /// }
    /// ```
    fn check(&mut self, _ast: ast::AbstractSyntaxTree) -> Result<(), errors::ParseError> {
        println!("It'll be implemented later, dw it'll be... :)");
        Ok(())
    }
}