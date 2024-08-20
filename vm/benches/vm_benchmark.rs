use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vm::{
    instruction::{
        Address,
        Instruction::{self, *},
    },
    runtime::VM,
};

fn vm_test_benchmark(c: &mut Criterion) {
    let ins = black_box(vec![
        PushI(25),
        Call(Address::Val(4)),
        Nop,
        HLT,
        Dup,
        PushI(2),
        Lt,
        JumpIfFalse(Address::Val(9)),
        Ret,
        Dup,
        PushI(1),
        SubI,
        Call(Address::Val(4)),
        Swap,
        PushI(2),
        SubI,
        Call(Address::Val(4)),
        AddI,
        Ret,
    ]);
    let mut vm = VM::new();
    let mut counter = 0;
    c.bench_function("vm_instruction", |b| {
        b.iter(|| {
            vm.execute_instruction(&ins[0]);
            vm.stack.top = 1;
            counter += 1;
            if counter >= ins.len() {
                counter = 0
            }
        })
    });
}

criterion_group!(benches, vm_test_benchmark);
criterion_main!(benches);
