use std::time::Duration;

use vm::instruction::compiler::parser::Parser;
use vm::instruction::{Address, Instruction::*};

use vm::memory::object_map::Structure;
use vm::memory::vm_data::VMData;
use vm::runtime::VM;

const TEST_AMOUNT: usize = 10;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    /*let mut tmp = Duration::new(0, 0);
    let mut stack_machine = VM::new();
    let ins = vec![];
    for _ in 0..TEST_AMOUNT {
        use std::time;
        let ins = ins.clone();
        let instant = time::Instant::now();
        stack_machine.execute(ins);
        tmp += instant.elapsed();
        stack_machine.clean();
    }
    println!("tmp1: {:?}", tmp.div_f32(TEST_AMOUNT as f32));*/

    if let Ok(content) = std::fs::read_to_string("./vm/src/example.txt") {
        let mut lexer = vm::instruction::compiler::lexer::AtlasLexer::default();
        lexer.set_path("src/example.txt");
        lexer.set_source(content);
        lexer.add_system(vm::instruction::compiler::lexer::identifier_system);
        lexer.add_system(vm::instruction::compiler::lexer::comment_system);
        let res = lexer.tokenize();
        match res {
            Ok(t) => {
                println!("ok lexer");
                //t.clone().into_iter().for_each(|ins| println!("{:?}, ", ins.kind()));
                let parser = Parser::parse(t);
                match parser {
                    Ok(code) => {
                        println!("ok parser");
                        //code.clone().into_iter().for_each(|ins| println!("{:?}", ins));
                        let mut vm = VM::new();
                        #[cfg(debug_assertions)]
                        vm.set_fn_name(code.fn_name);
                        vm.execute(code.ins, code.constants);
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                };
            }
            Err(_e) => {
                println!("Error1");
            }
        }
    } else {
        println!("Error2")
    }
}
