pub mod simple_lexer;
pub mod simple_parser;
pub mod simple_visitor;

use crate::simple_lexer::SimpleLexerV1;
use atlas_core::Parser;
use atlas_core::language::Language;
use crate::simple_parser::SimpleParserV1;
use crate::simple_visitor::SimpleVisitorV1;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let path = String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas-test\\test.atlas");
    let mut language = Language::new(
        Box::new(SimpleLexerV1::new()),
        Box::new(SimpleParserV1::new()),
        Box::new(SimpleVisitorV1::new())
    );
    language.lexer.with_file_path(path.clone()).expect("Failed to open the file");
    let tokens = language.lexer.tokenize();
    //println!("Tokens: {:?} ", tokens);
    //language.lexer.check(&tokens).expect("Failed to tokenize the file");
    let mut parser = SimpleParserV1::new();
    parser.with_file_path(path).expect("Failed to open the file");
    parser.with_tokens(tokens);
    let res = parser.parse().expect("Failed to parse the file");
    println!("{:?}", language.visitor.visit(&res));

    /*for _ in 0..10000 {
        let path = String::from("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas-test\\test.atlas");
        let mut language = Language::new(
            Box::new(SimpleLexerV1::new()),
            Box::new(SimpleParserV1::new()),
            Box::new(SimpleVisitorV1::new())
        );
        language.lexer.with_file_path(path.clone()).expect("Failed to open the file");
        let tokens = language.lexer.tokenize();
        //language.lexer.check(&tokens).expect("Failed to tokenize the file");
        language.parser.with_file_path(path).expect("Failed to open the file");
        language.parser.with_tokens(tokens);
        let ast = language.parser.parse().expect("Failed to parse the file");
        for i in 0..ast.len() {
            //println!("{}: {}", i, ast[i].value);
            language.visitor.visit_statement(&ast[i].value);
        }
    }*/

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start)/10000);
}