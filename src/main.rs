#![allow(unused)]

pub mod compiler;
pub mod vm;
//use std::path::PathBuf;

//use clap::{arg, Command};

use crate::pest::Parser;
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
    let program: pest::iterators::Pairs<Rule> = TestParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
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