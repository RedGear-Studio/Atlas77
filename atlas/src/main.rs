use atlas_core::simple_lexer::SimpleLexerV1;
use atlas_core::interfaces::lexer::Lexer;
use atlas_core::interfaces::parser::Parser;

use atlas_core::language::Language;
use atlas_core::simple_parser::SimpleParserV1;


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