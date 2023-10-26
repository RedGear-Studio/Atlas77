use crate::utils::span::WithSpan;
use crate::interfaces::lexer::token;

/// Contains all the ast nodes
pub mod ast;
/// Contains all the potential Parser errors
pub mod errors;
/// Contains all the definition of the Node trait
pub mod node;

/// The `Parser` trait defines the interface for parsing source code and generating an abstract syntax tree (AST).
/// 
/// There should be at least 2 field on your parser:
/// * `tokens` - A Vec of `WithSpan` tokens, each representing a token along with its span.
/// * `file_path` - The path to the root source file to be processed. (This is used for error reporting.)
pub trait Parser {
    /// Creates a new empty instance of a parser for a given file (the file is only used for error reporting and should be the root file).
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the source file to be processed.
    ///
    /// # Returns
    ///
    /// A `Result` that contains the parser instance if successful, or an `std::io::Error` if there's an issue
    /// with file I/O (e.g., file not found, permission issues).
    fn with_file_path(&mut self, file_path: String) -> Result<(), std::io::Error>;
    /// Adds the given tokens to the parser.
    /// 
    /// # Arguments
    ///
    /// * `tokens` - A Vec of `WithSpan` objects, each representing a token along with its span.
    fn with_tokens(&mut self, tokens: Vec<WithSpan<token::Token>>);
    /// Parses a sequence of tokens, generating an abstract syntax tree (AST).
    ///
    /// # Returns
    ///
    /// An `AbstractSyntaxTree` that represents the parsed code's hierarchical structure and semantics.
    fn parse(&mut self) -> ast::AbstractSyntaxTree;
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