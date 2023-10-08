pub mod interfaces;
pub mod span;
pub mod simple_lexer;

use simple_lexer::SimpleLexer;
use interfaces::lexer::Lexer;


fn main() {
    let mut lex = SimpleLexer::new(String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas\\src\\simple_lexer.rs"));
    println!("{:?}", lex.expect("Failed to create lexer").tokenize());
}