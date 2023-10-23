pub mod interfaces;
pub mod simple_lexer;
pub mod simple_checker;
pub mod utils;

use simple_lexer::SimpleLexerV1;
use interfaces::lexer::Lexer;


fn main() {
    let mut lexer: Box<dyn Lexer> = Box::new(SimpleLexerV1::new(String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas\\test.atlas")).expect("Failed to create the lexer"));
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
    println!("{:?}", lexer.check(&tokens))
}