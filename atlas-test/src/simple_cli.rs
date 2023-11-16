use std::{time::Instant, path::PathBuf};

use atlas_core::{Language, Parser};

use crate::{simple_lexer::SimpleLexerV1, simple_parser::SimpleParserV1, simple_visitor::SimpleVisitorV1};

pub(crate) fn run(path: String) {
    let mut path_buf = PathBuf::from(path.clone());

    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else{
        println!("Failed to get current directory");
    }

    let start = Instant::now();

    let mut language = Language::new(
        Box::new(SimpleLexerV1::new()),
        Box::new(SimpleParserV1::new()),
        Box::new(SimpleVisitorV1::new())
    );

    language.lexer.with_file_path(path_buf.clone())
        .expect("Failed to open the file");
    let tokens = language.lexer.tokenize();
    //println!("Tokens: {:?} ", tokens);
    //language.lexer.check(&tokens).expect("Failed to tokenize the file");
    
    let mut parser = SimpleParserV1::new();
    parser.with_file_path(path_buf)
        .expect("Failed to open the file");
    parser.with_tokens(tokens);
    
    let res = parser.parse()
        .expect("Failed to parse the file");
    println!("{:?}", language.visitor.visit(&res));

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start)/10000);
}