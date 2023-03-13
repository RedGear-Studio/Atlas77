#![allow(unused)]


use reg_lang_vm::vm;
pub mod reg_lang_vm {
    pub mod register;
    pub mod vm;
}
pub mod reg_lang_compiler {
    pub mod compiler;
}
pub mod reg_lang_repl {
    pub mod repl;
}
pub mod instructions;

fn main() {
    vm::VM::new().run();
}