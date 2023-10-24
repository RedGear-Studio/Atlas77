pub mod interfaces;
pub mod simple_lexer;
pub mod simple_parser;
pub mod utils;
pub mod language;

use simple_lexer::SimpleLexerV1;
use interfaces::lexer::Lexer;
use interfaces::parser::Parser;

use crate::language::Language;
use crate::simple_parser::SimpleParserV1;


fn main() {
    let path = String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas\\test.atlas");
    let mut language = Language::new(
        Box::new(SimpleLexerV1::new(path.to_owned()).expect("Failed to create the lexer")),
        Box::new(SimpleParserV1::new(path).expect("Failed to create the parser"))
    );
    let tokens = language.lexer.tokenize();
    println!("{:?}", tokens);
    println!("{:?}", language.lexer.check(&tokens))
}