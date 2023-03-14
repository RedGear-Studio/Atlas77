pub mod repl;

use reg_lang_vm::VM;
use reg_lang_compiler::RegCompiler;
use repl::REPL;

fn main() {
    let mut repl = REPL::new();
    repl.run_console();
}

