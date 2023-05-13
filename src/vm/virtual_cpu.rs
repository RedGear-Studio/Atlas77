use crate::Colorize;
use super::{events::VMEvent, bytecode::Instruction, register::Register};

/// Virtual CPU implementation
#[derive(Default)]
pub struct VirtualCPU {
    pub pc: usize, // program counter, holds the address of the next instruction to be executed
    pub registers_i: [Register<i32>; 16], // 16 integer registers (the first register is the zero register /!\ read-only)
    pub registers_f: [Register<f32>; 16], // 16 float registers (the exact implementation of both type of registers can be found in the README)
    pub cmp_register: u8, // Store the result of the lost comparison
    pub memory: Vec<u8>, // Memory that behave as a stack and a heap for the virtual machine the first part of it is the data section and the second part is the dynamic memory (stack/heap)
    pub program: Vec<Instruction>, // Representation of the program instructions
    pub base_pointer: usize, // Base pointer for the current stack frame
    pub frame_pointer: usize, // Frame pointer for the current stack frame
    pub stack_pointer: usize, // Stack pointer for the current stack frame
}

impl VirtualCPU {
    pub fn new() -> VirtualCPU {
        VirtualCPU {
            pc: 0,
            registers_i: [Register(0); 16],
            registers_f: [Register(0.0); 16],
            cmp_register: 0,
            memory: vec![],
            program: vec![],
            base_pointer: 0,
            frame_pointer: 0,
            stack_pointer: 0,
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.execute_instruction() {
                VMEvent::IllegalOpCode => {
                    println!("{}:\n  |\n  = Illegal Opcode at address {:#06x}", "Runtime Error:".bold().red(), self.pc);
                    break
                },
                VMEvent::DivideByZero => {
                    println!("{}:\n  |\n  = Division by zero at address {:#06x}", "Runtime Error".bold().red(), self.pc);
                    break
                },
                VMEvent::OutOfBound => {
                    println!("{}:\n  |\n  = Program Out of bound, the program size is {:#06x} and the address is {:#06x}", "Runtime Error".bold().red(), self.program.len(), self.pc);    
                    break
                },
                VMEvent::StackOverflow => {
                    println!("{}:\n  |\n  = Stack overflow at address {:#06x}", "Runtime Error".bold().red(), self.pc);
                    break
                },
                VMEvent::StackUnderflow => {
                    println!("{}:\n  |\n  = Stack underflow at address {:#06x}", "Runtime Error".bold().red(), self.pc);
                    break
                },
                VMEvent::ExitSyscall => {
                    match self.registers_i[1].get() {
                        0 => {
                            println!("{}:\n  |\n  = Success, exit with code {}", "Finished:".bold().green(), self.registers_i[1].get());
                            break;
                        }
                        _ => {
                            //TODO!: Add more information about the error
                            println!("{}:\n  |\n  = Exit with code {}: Failure", "Runtime Error:".bold().red(), self.registers_i[1].get());
                            break;
                        }
                    }
                },
                _ => (),
            }
        }
    }
    pub fn execute_instruction(&mut self) -> VMEvent {
        if self.pc > self.program.len() {
            return VMEvent::OutOfBound;
        }
        match self.program[self.pc] {
            Instruction::Add(r1, r2, r3) => {
                if r1 < 16 {
                    // Integer register
                    let value1 = *self.registers_i[r2 as usize].get();
                    let value2 = *self.registers_i[r3 as usize].get();
                    let result = value1 + value2;
                    self.registers_i[r1 as usize].set(result);
                } else {
                    // Float register
                    let value1 = *self.registers_f[(r2 - 16) as usize].get();
                    let value2 = *self.registers_f[(r3 - 16) as usize].get();
                    let result = value1 + value2;
                    self.registers_f[(r1 - 16) as usize].set(result);
                }
                self.pc += 1;
            },
            Instruction::Sub(r1, r2, r3) => {
                if r1 < 16 {
                    // Integer register
                    let value1 = *self.registers_i[r2 as usize].get();
                    let value2 = *self.registers_i[r3 as usize].get();
                    let result = value1 - value2;
                    self.registers_i[r1 as usize].set(result);
                } else {
                    // Float register
                    let value1 = *self.registers_f[(r2 - 16) as usize].get();
                    let value2 = *self.registers_f[(r3 - 16) as usize].get();
                    let result = value1 - value2;
                    self.registers_f[(r1 - 16) as usize].set(result);
                }
                self.pc += 1;
            },
            Instruction::Mul(r1, r2, r3) => {
                if r1 < 16 {
                    // Integer register
                    let value1 = *self.registers_i[r2 as usize].get();
                    let value2 = *self.registers_i[r3 as usize].get();
                    let result = value1 * value2;
                    self.registers_i[r1 as usize].set(result);
                } else {
                    // Float register
                    let value1 = *self.registers_f[(r2 - 16) as usize].get();
                    let value2 = *self.registers_f[(r3 - 16) as usize].get();
                    let result = value1 * value2;
                    self.registers_f[(r1 - 16) as usize].set(result);
                }
                self.pc += 1;
            },
            Instruction::Div(r1, r2, r3) => {
                if r1 < 16 {
                    // Integer register
                    let value1 = *self.registers_i[r2 as usize].get();
                    let value2 = *self.registers_i[r3 as usize].get();
                    if value2 == 0 {
                        return VMEvent::DivideByZero;
                    }
                    let result = value1 / value2;
                    self.registers_i[r1 as usize].set(result);
                } else {
                    // Float register
                    let value1 = *self.registers_f[(r2 - 16) as usize].get();
                    let value2 = *self.registers_f[(r3 - 16) as usize].get();
                    if value2 == 0.0 {
                        return VMEvent::DivideByZero;
                    }
                    let result = value1 / value2;
                    self.registers_f[(r1 - 16) as usize].set(result);
                }
                self.pc += 1;
            },
            _ => return VMEvent::IllegalOpCode,
        }
        VMEvent::AllGood
    }
}
