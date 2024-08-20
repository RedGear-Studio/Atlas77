use std::{thread, time::Duration};

use crate::{
    instruction::Instruction,
    memory::{object_map::Memory, stack::Stack, vm_data::VMData},
};

pub struct VM {
    pub stack: Stack,
    pub object_map: Memory,
    constants: Vec<VMData>,
    call_stack: Vec<usize>,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            object_map: Memory::new(16),
            constants: vec![],
            call_stack: vec![],
            pc: usize::default(),
        }
    }
    #[inline(always)]
    pub fn clean(&mut self) {
        self.stack.top = 1;
        self.call_stack = vec![];
        self.pc = usize::default();
    }
    pub fn execute(&mut self, ins: Vec<Instruction>) {
        while self.pc < ins.len() {
            let ins = &ins[self.pc];
            #[cfg(debug_assertions)]
            println!("{:?}", ins);
            match ins {
                Instruction::HLT => break,
                _ => {
                    self.execute_instruction(ins);
                }
            }
            #[cfg(debug_assertions)]
            println!("{}", self.stack);

            #[cfg(debug_assertions)]
            thread::sleep(Duration::from_millis(250));
        }
        self.clean();
    }
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        use Instruction::*;
        match ins {
            PushI(i) => self.stack.push(VMData::new_i64(*i)),
            PushF(f) => self.stack.push(VMData::new_f64(*f)),
            PushU(u) => self.stack.push(VMData::new_u64(*u)),
            LoadConst(u) => {
                //constants aren't loaded as is, but are fetched from constants: Vec<VMData>
                #[cfg(debug_assertions)]
                if self.constants.len() < *u {
                    panic!("Can't load that constant, it doesn't exist");
                }
                self.stack.push(self.constants[*u]);
            }
            Pop => {
                self.stack.pop().expect("Stack Underflow");
            }
            Print => {
                let value = self.stack.last().expect("Stack Underflow");
                println!("val: {}", value)
            }
            AddI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a + b));
            }
            AddF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a + b));
            }
            AddU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a + b));
            }
            MulI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a * b));
            }
            MulF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a * b));
            }
            MulU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a + b));
            }
            DivI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                if b == 0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a / b));
            }
            DivF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                if b == 0.0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a / b));
            }
            DivU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                if b == 0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a / b));
            }
            SubI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a - b));
            }
            SubF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a - b));
            }
            SubU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a - b));
            }
            Dup => {
                let last = self.stack.last().expect("Stack Underflow");
                self.stack.push(*last);
            }
            Swap => {
                let a = self.stack.pop().expect("Stack Underflow");
                let b = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a);
                self.stack.push(b);
            }
            Rot => {
                let a = self.stack.pop().expect("Stack Underflow");
                let b = self.stack.pop().expect("Stack Underflow");
                let c = self.stack.pop().expect("Stack Underflow");
                self.stack.push(c);
                self.stack.push(b);
                self.stack.push(a);
            }
            Jmp(address) => {
                self.pc = address.into();
                return;
            }
            JumpIfTrue(address) => {
                let val = self.stack.pop().expect("Stack Underflow").as_bool();
                if val {
                    self.pc = address.into();
                    return;
                }
            }
            JumpIfFalse(address) => {
                let val = self.stack.pop().expect("Stack Underflow").as_bool();
                if !val {
                    self.pc = address.into();
                    return;
                }
            }
            Call(address) => {
                self.call_stack.push(self.pc + 1);
                self.pc = address.into();
                return;
            }
            Ret => {
                self.pc = self.call_stack.pop().expect("Call Stack Underflow");
                return;
            }
            Read => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let val = String::from(input.trim());
                match self.object_map.put(val.into()) {
                    Ok(i) => {
                        self.stack.push(VMData::new_string(i));
                    }
                    Err(o) => {
                        println!("Memory full, can't insert: [{:?}]", o);
                    }
                }
            }
            Instruction::Eq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a == b));
            }
            Instruction::Neq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a != b));
            }
            Instruction::Lt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a < b));
            }
            Instruction::Gt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a > b));
            }
            Instruction::Lte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a <= b));
            }
            Instruction::Gte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a >= b));
            }
            Instruction::And => {
                let b = self.stack.pop().expect("Stack underflow").as_bool();
                let a = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(a && b));
            }
            Instruction::Or => {
                let b = self.stack.pop().expect("Stack underflow").as_bool();
                let a = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(a || b));
            }
            Instruction::Not => {
                let value = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(!value));
            }
            Nop => {}
            _ => unimplemented!(),
        }
        self.pc += 1;
    }
}
