pub mod simple_lexer;
pub mod simple_parser;
pub mod simple_visitor;
pub mod simple_cli;

use clap::Parser as ClapParser;

use codespan_reporting::files::{SimpleFile, SimpleFiles};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use atlas_span::Spanned;
//use atlas_parser_interpreter::ASTInterpreter;
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

    let number_of_try = 500;

    let start = Instant::now();

    let path = "C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\atlas-test\\test.atlas";
    for _ in 0..number_of_try {
        atlas_parser::parse(path);
    }

    let elapsed = start.elapsed();
    let average_time = elapsed/number_of_try;
    println!("Average time: {:.2?}", average_time);

}
