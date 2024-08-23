use criterion::{criterion_group, criterion_main, Criterion};
use vm::{instruction::compiler::parser::Parser, runtime::VM};

fn vm_test_benchmark(c: &mut Criterion) {
    c.bench_function("vm_instruction", |b| {
        b.iter(|| {
            //let tmp = std::time::Instant::now();
            if let Ok(content) = std::fs::read_to_string("./src/example.txt") {
                let mut lexer = vm::instruction::compiler::lexer::AtlasLexer::default();
                lexer.set_path("src/example.txt");
                lexer.set_source(content);
                lexer.add_system(vm::instruction::compiler::lexer::identifier_system);
                lexer.add_system(vm::instruction::compiler::lexer::comment_system);
                let res = lexer.tokenize();
                match res {
                    Ok(t) => {
                        //println!("Ok Lexer: {:?}", tmp.elapsed());
                        //let tmp = std::time::Instant::now();
                        //t.clone().into_iter().for_each(|ins| println!("{:?}, ", ins.kind()));
                        let parser = Parser::parse(t);
                        match parser {
                            Ok(code) => {
                                //println!("Ok Parser: {:?}", tmp.elapsed());
                                //let tmp = std::time::Instant::now();
                                //code.clone().into_iter().for_each(|ins| println!("{:?}", ins));
                                let mut vm = VM::new();
                                //#[cfg(debug_assertions)]
                                //vm.set_fn_name(code.fn_name);
                                vm.execute(code.ins, code.constants);
                                //println!("Ok Excution: {:?}", tmp.elapsed())
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
        })
    });
}

criterion_group!(benches, vm_test_benchmark);
criterion_main!(benches);
