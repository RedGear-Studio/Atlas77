pub mod parser {
    pub mod ast;
    pub mod parser;
}

use crate::{pest::Parser, parser::parser::generate_ast};
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

fn main() {
    let input = " print -(5 + 5);
                        let salut: string = \"YO\";
                        salut = \"Hello\";
                        if (salut == \"Hello\") then
                            print \"Hello\";
                        else
                            print \"World\";
                        end;
                      ";
    let program = TestParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    for programs in program.into_iter() {
        match programs.as_rule() {
            Rule::program => {
                generate_ast(programs);
            },
            _ => unreachable!(),
        }
    }
}