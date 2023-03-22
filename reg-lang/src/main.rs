pub mod repl;

use reg_lang_vm::VM;
//use reg_lang_compiler::RegCompiler;
use repl::REPL;
use rasm_compiler::RASMCompiler;

fn main() {
    let mut repl = REPL::new();
    repl.run_console();
}

