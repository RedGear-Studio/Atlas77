pub mod simple_lexer;
pub mod simple_parser;
pub mod simple_visitor;
pub mod simple_cli;

use clap::Parser as ClapParser;
use atlas_lexer::{Lexer, AtlasLexer};

use codespan_reporting::files::{SimpleFile, SimpleFiles};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use atlas_span::Spanned;
use atlas_parser::{AtlasParser, Parser};
use atlas_parser_interpreter::ASTInterpreter;
use atlas_utils::{Visitor, Expression, Value};

use std::time::Instant;

#[derive(ClapParser, Debug)]
#[clap(author = "Gipson62", version, about = "Programming language made in Rust", long_about = None)]
struct Args {
    /// Run you program at the given <PATH>
    #[arg(short, long, value_name = "PATH")]
    run: String,
}

fn main() { 

    let number_of_try = 10000;

    let start = Instant::now();

    let path = "C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas-test\\test.atlas";
    for _ in 0..number_of_try {
        let mut lexer = AtlasLexer::new_with_path(path);
        let res = lexer.tokenize();
        //println!("{:?}", res);
        if res.err.len() > 0 {
            println!("{} errors", res.err.len());
                    
            let mut file = SimpleFiles::new();
            let file_id = file.add(path, std::fs::read_to_string(path).unwrap());

            for err in res.err.iter() {
                let diagnostic= Diagnostic::error()
                    .with_message(err.to_string())
                    .with_code("Something")
                    .with_labels(vec![
                        Label::primary(file_id, err.start()..err.end()).with_message(err.to_string()),
                    ])
                    .with_notes(vec!["Notes".to_string()]);
                
                let writer = StandardStream::stderr(ColorChoice::Always);
                let config = codespan_reporting::term::Config::default();
                

                if let Ok(_c) = codespan_reporting::term::emit(&mut writer.lock(), &config, &file, &diagnostic) {
                    
                };
            }   
        }

        let mut parser = AtlasParser::new_with_path(path, res.tokens);
        let res = parser.parse();
        //println!("{:?}", res);

        let mut visitor = ASTInterpreter::new();

        let mut exprs = Vec::new();
        for expr in res.iter() {
            exprs.push(expr as &dyn Expression);
        }

        let res = visitor.evaluate(exprs);
        //println!("{:?}", res);
    }

    let elapsed = start.elapsed();
    let average_time = elapsed/number_of_try;
    println!("Average time: {:.2?}", average_time);

}
