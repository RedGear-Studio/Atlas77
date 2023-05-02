use pest::iterators::Pair;
use crate::Rule;
use crate::vm::virtual_cpu::VirtualCPU;
enum Value {
    I32(i32),
    I16(i16),
    I8(i8),
    F32(f32),
    U32(u32),
    U16(u16),
    U8(u8),
    Str(String),
    Bool(bool),
}
struct Function {
    name: String,
    address: usize,
}
struct Data {
    name: String,
    address: usize,
    value: Value,
}
struct ASMContext {
    functions: Vec<Function>,
    data_size: usize, //Current size of the data segment
    program_size: usize, //Current size of the program
    entry_point: String, //Function define in the .global directive
    data: Vec<Data>,
}

pub fn compile(program: Pair<Rule>) -> VirtualCPU {
    let mut context: ASMContext = ASMContext {
        functions: Vec::new(),
        data_size: 0,
        program_size: 0,
        entry_point: String::new(),
        data: Vec::new(),
    };
    let mut vm: VirtualCPU = VirtualCPU::default();
    for directive in program.into_inner() {
        match directive.as_rule() {
            Rule::include_directive => {

            },
            Rule::data_directive => {

            },
            Rule::text_directive => {

            },
            Rule::end_directive => {
                
            },
            _ => unreachable!()
        }
    }
    u16::MAX;
    todo!()
}