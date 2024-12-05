use vm::instruction::compiler::parser::Parser;

use vm::memory::vm_data::VMData;
use vm::runtime::vm_state::VMState;
use vm::runtime::VM;

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
    let tmp = std::time::Instant::now();
    if let Ok(content) = std::fs::read_to_string("./vm/src/extern_call.txt") {
        let mut lexer = vm::instruction::compiler::lexer::AtlasLexer::default();
        lexer.set_path("src/extern_call.txt");
        lexer.set_source(content);
        lexer.add_system(vm::instruction::compiler::lexer::identifier_system);
        lexer.add_system(vm::instruction::compiler::lexer::comment_system);
        let res = lexer.tokenize();
        match res {
            Ok(t) => {
                println!("Ok Lexer: {:?}", tmp.elapsed());
                let tmp = std::time::Instant::now();
                //t.clone().into_iter().for_each(|ins| println!("{:?}, ", ins.kind()));
                let parser = Parser::parse(t);
                match parser {
                    Ok(code) => {
                        println!("Ok Parser: {:?}", tmp.elapsed());
                        let tmp = std::time::Instant::now();
                        //code.clone().into_iter().for_each(|ins| println!("{:?}", ins));
                        let mut vm = VM::new();
                        #[cfg(debug_assertions)]
                        println!("{:?}", code.fn_name);
                        #[cfg(debug_assertions)]
                        vm.set_fn_name(code.fn_name);
                        vm.add_extern_call(fib_extern)
                            .execute(code.ins, code.constants);
                        println!("Ok Excution: {:?}", tmp.elapsed())
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

pub fn fib_extern(vm_state: VMState) -> Result<VMData, ()> {
    fn fib(n: i64) -> i64 {
        if n < 2 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }
    let res = fib(vm_state.stack.pop().expect("Stack Underflow").as_i64());
    Ok(VMData::new_i64(res))
}
