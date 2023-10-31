pub mod simple_lexer;
pub mod simple_parser;

use crate::simple_lexer::SimpleLexerV1;
use atlas_core::language::Language;
use crate::simple_parser::SimpleParserV1;


fn main() {
    let path = String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas-test\\test.atlas");
    let mut language = Language::new(
        Box::new(SimpleLexerV1::new()),
        Box::new(SimpleParserV1::new())
    );
    language.lexer.with_file_path(path.clone()).expect("Failed to open the file");
    let tokens = language.lexer.tokenize();
    println!("{:?}", tokens);
    println!("{:?}", language.lexer.check(&tokens));
    language.parser.with_file_path(path).expect("Failed to open the file");
    language.parser.with_tokens(tokens);
    language.parser.parse().expect("Failed to parse the file");
}