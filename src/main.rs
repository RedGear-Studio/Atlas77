pub mod parser {
    pub mod ast;
    pub mod parser;
    pub mod eval;
}

use crate::{pest::Parser, parser::{eval::{SymbolTable},parser::generate_ast}};
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

fn main() {
    let input: &str = "let x: int = 1;
    let hehe: boolean = true;
    while hehe do
        x = (x + 1);
        print x;
    end;";
    let program = TestParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    for programs in program.into_iter() {
        match programs.as_rule() {
            Rule::program => {
                let ast = generate_ast(programs);
                let mut symbol_table = SymbolTable::new();
                let result = symbol_table.eval(ast.statements, 1);
                match result {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            },
            _ => unreachable!(),
        }
    }
}