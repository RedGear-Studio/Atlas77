#![allow(unused)]
pub mod compiler;
pub mod tree_walker;

use compiler::ir::builder::IRBuilder;
use crate::pest::Parser;
use crate::compiler::parser::parser::generate_ast;
use crate::tree_walker::eval::SymbolTable;
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

fn main() {
    let input: &str = "
    function salut(x: int, y: boolean): int
    begin
        print x;
    end;";
    let program = TestParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    for programs in program.into_iter() {
        match programs.as_rule() {
            Rule::program => {
                let ast = generate_ast(programs);
                let mut ir = IRBuilder::new();
                ir.build(ast);
                let mut symbol_table = SymbolTable::default();
                /*let result = symbol_table.eval(ast.functions, 1);
                match result {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }*/
            },
            _ => unreachable!(),
        }
    }
}