pub mod asr_compiler;
pub mod vm;

use std::fs;
use colored::Colorize;
use vm::bytecode::InstructionTest;
use crate::asr_compiler::compiler::ASMContext;

use crate::pest::Parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TestParser;

fn main(){
    let command = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Reg-Lang CLI v0.1.0
  USAGE: asl <command> [args]
  Commands:
    - compile <path>: Compiles the given file.

NB: Please note that you cannot include external files using the `.include` directive at this time. We apologize for any inconvenience this may cause. If you need to use external files, you can try using a different method of including them, such as copying and pasting the contents of the file into your source code.

We are working to add support for the `.include` directive in a future version of the assembler. Thank you for your understanding.");
        std::process::exit(0);
    });

    if command == "compile" {
        let Some(path) = std::env::args().nth(2) else {
            eprintln!("{}: You must specify a path to a file", "Error".bold().red());
            std::process::exit(1);
        };
        if !path[..].ends_with(".asr") {
            eprintln!("{}: You must specify a file with the {} extension", "Error".bold().red(), ".asr".bold());
            std::process::exit(1);
        }
        let content = fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read input file {}: {}", path, e);
            std::process::exit(1);
        });
        println!("{}: {}", "Parsing".bold().blue(), path);
        let result = TestParser::parse(Rule::program, &content).unwrap_or_else(|e| {
            println!("{}:\n{}", "Error".bold().red(), e);
            std::process::exit(0);
        });
        println!("{}", "Finished".bold().green());
        println!("{} parsed result", "Compiling".bold().blue());
        let mut context = ASMContext::new();
        let mut vm = context.compile(result.into_iter().next().unwrap());
        println!("{}", "Finished".bold().green());
        println!("{}", "Running".bold().green());
        vm.run();
    }
}
