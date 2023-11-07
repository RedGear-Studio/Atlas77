pub mod simple_lexer;
pub mod simple_parser;
pub mod simple_visitor;

use crate::simple_lexer::SimpleLexerV1;
use atlas_core::language::Language;
use crate::simple_parser::SimpleParserV1;
use crate::simple_visitor::SimpleVisitorV1;

use std::time::Instant;


fn main() {
    let start = Instant::now();

    for _ in 0..10000 {
        let path = String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas-test\\test.atlas");
        let mut language = Language::new(
            Box::new(SimpleLexerV1::new()),
            Box::new(SimpleParserV1::new()),
            Box::new(SimpleVisitorV1{}),
        );
        language.lexer.with_file_path(path.clone()).expect("Failed to open the file");
        let tokens = language.lexer.tokenize();
        //language.lexer.check(&tokens).expect("Failed to tokenize the file");
        language.parser.with_file_path(path).expect("Failed to open the file");
        language.parser.with_tokens(tokens);
        let ast = language.parser.parse().expect("Failed to parse the file");
        let _result = language.visitor.visit_expression(&ast[0].value);
        //println!("Result: {}", result);
    }

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start)/10000);
}