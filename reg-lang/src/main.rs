#![allow(unused)]

use std::vec;

use reg_lang_parser::parse;
use reg_lang_compiler::compile;
fn main() {
    let ast = parse("1 + 2 * 5");
    println!("{:?}", ast);
    let mut instructions: Vec<u8> = vec![];
    compile(&mut instructions, &ast.ast[0]);
    println!("{:?}", instructions);
}