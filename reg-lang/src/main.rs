#![allow(unused)]

use std::vec;

use reg_lang_compiler::compile;
use reg_lang_vm::RegLangVM;
fn main() {
    let compiler = compile();
    let bytecode = compiler.program;
    let pc = compiler.program_counter;
    let mut vm = RegLangVM::new(bytecode);
    vm.run();
    println!("VM {:?}", vm);
}