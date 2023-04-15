pub mod parser;
pub mod ast;
pub mod compiler;
pub mod vm;
pub mod ir;
pub mod tree_walker;
//use std::path::PathBuf;

//use clap::{arg, Command};

use crate::pest::Parser;
use crate::parser::parser::generate_ast;
use crate::tree_walker::eval::SymbolTable;
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

fn main() {
    /*let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let path = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Paths: {:?}", path);
        }
        _ => unreachable!()
    }*/
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

/*fn cli() -> Command {
    Command::new("reg-lang")
        .about(" A simple and in development programming language written in Rust.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("run")
                .about("Run a program.")
                .arg(arg!(-f --file <FILE> "File to run."))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("compile")
                .about("Compile a program. Not usable for now.")
                .arg(arg!(-f --file <FILE> "File to compile."))
                .arg_required_else_help(true)
        )
}*/