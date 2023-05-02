#![allow(unused)]

pub mod compiler;
pub mod vm;

use std::fs;

use crate::pest::Parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

fn main(){
    //Bro WTF?
    let command = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Reg-Lang CLI v0.1.0
  USAGE: reglang <command> [args]
  Commands:
    - compile <path>
    - run <path>
Nota Bene: The compile command doesn't work as of now.");
        std::process::exit(0);
    });

    if command == "compile" {
        let Some(path) = std::env::args().nth(2) else {
            eprintln!("You must specify a path to a file");
            std::process::exit(1);
        };
        let content = fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read input file {}: {}", path, e);
            std::process::exit(1);
        });
        let result = TestParser::parse(Rule::program, &content).unwrap_or_else(|e| panic!("{}", e));
    } else if command == "compile" {
        let Some(path) = std::env::args().nth(2) else {
            eprintln!("You must specify a path to a file");
            std::process::exit(1);
        };
    }
    println!("Hello, world!");
}