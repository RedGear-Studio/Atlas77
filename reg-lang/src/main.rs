#![allow(unused)]

use std::process::exit;
use reg_lang_parser::*;
use reg_lang_compiler::{
    generate_bytecode,
};
use reg_lang_runtime::{run,
    instructions::set::Instructions,
};

fn main() {
    let ask = "20.24242 % 2.231";
    println!("calc: {}", ask);
    let parser = parse(ask);
    println!("AST: {:?}", parser);

    let mut instructions: Vec<Instructions> = vec![];
    generate_bytecode(&parser.stack[0], &mut instructions);
    instructions.push(Instructions::Print);
    println!("Bytecode: {:?}", instructions);
    
    run(instructions);
}