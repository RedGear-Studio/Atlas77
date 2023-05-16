use crate::{Colorize, vm::bytecode::Syscall};
use super::{events::VMEvent, bytecode::Instruction, register::Register};

/// Virtual CPU implementation
#[derive(Default)]
pub struct VirtualCPU {
    pub pc: u32, // program counter, holds the address of the next instruction to be executed
    pub registers_i: [Register<i32>; 16], // 16 integer registers (the first register is the zero register /!\ read-only)
    pub registers_f: [Register<f32>; 16], // 16 float registers (the exact implementation of both type of registers can be found in the README)
    pub cmp_register: u8, // Store the result of the lost comparison
    pub memory: Vec<u8>, // Memory that behave as a stack and a heap for the virtual machine the first part of it is the data section and the second part is the dynamic memory (stack/heap)
    pub program: Vec<Instruction>, // Representation of the program instructions
    pub base_pointer: u32, // Base pointer for the current stack frame
    pub frame_pointer: u32, // Frame pointer for the current stack frame
    pub stack_pointer: u32, // Stack pointer for the current stack frame
    pub debug_mode: bool,
}

impl VirtualCPU {
    pub fn new(debug_mode: bool) -> VirtualCPU {
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
            debug_mode,
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
                            println!("{}:\n  |\n  = Success, exit with code {}", "Finished".bold().green(), self.registers_i[1].get());
                            break;
                        }
                        _ => {
                            //TODO!: Add more information about the error
                            println!("{}:\n  |\n  = Exit with code {}: Failure", "Runtime Error".bold().red(), self.registers_i[1].get());
                            break;
                        }
                    }
                },
                _ => (),
            }
        }
    }
    pub fn execute_instruction(&mut self) -> VMEvent {
        use Instruction::*;
        if self.pc > self.program.len() as u32 {
            return VMEvent::OutOfBound;
        }

if self.debug_mode {
    println!("{}: {}", "Executing Instruction".blue().bold(), self.program[self.pc as usize]);
}

        match self.program[self.pc as usize] {
            Nop => self.pc += 1,
            Add(r1, r2, r3) => {
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
            Sub(r1, r2, r3) => {
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
            Mul(r1, r2, r3) => {
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
            Div(r1, r2, r3) => {
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
            Mov(rt, value) => {
                //Remove the register part and get only the type of mov operation
                // 0 = immediate value to a register (if the user use `mov $1 string` it'll move its address => immediate value)
                // 1 = register to a register
                // 2 = from data segment to a register
                let t = rt >> 5;
                let r = rt & 0b00011111;
                self.pc += 1;
                //Mov immediate value to a register:
                if t == 0 {
                    if r < 16 {
                        // return bool if the mov was split in half by the compiler to be able to move 32bits of information.
                        let mut complete_value: u32 = value as u32;
                        if self.peek_nxt() {
                            match self.program[self.pc as usize] {
                                Nxt(_r1, value_nxt) => {
                                    complete_value = (complete_value << 16) + value_nxt as u32;
                                    self.pc += 1;
                                },
                                _ => unreachable!(),
                            }
                        }
                        self.registers_i[r as usize].set(complete_value as i32);
                    } else {
                        // If there's no nxt instruction after program can't run because f16 doesn't exit.
                        if !self.peek_nxt() {
                            return VMEvent::IllegalOpCode;
                        }
                        let mut complete_value: u32 = value as u32;
                        if self.peek_nxt() {
                            match self.program[self.pc as usize] {
                                Nxt(_r1, value_nxt) => {
                                    complete_value = (complete_value << 16) + value_nxt as u32;
                                    self.pc += 1;
                                },
                                _ => unreachable!(),
                            }
                        }
                        self.registers_f[(r - 16) as usize].set(f32::from_bits(complete_value));
                    }
                }
                //Mov register to another register (copy)
                else if t == 1 {
                    if r < 16 {
                        if value < 16 {
                            self.registers_i[r as usize].set(*self.registers_i[value as usize].get());
                        }
                        else {
                            self.registers_i[r as usize].set(*self.registers_f[(value - 16) as usize].get() as i32);
                        }
                    }
                    else {
                        if value < 16 {
                            self.registers_f[(r - 16) as usize].set(*self.registers_i[value as usize].get() as f32);
                        }
                        else {
                            self.registers_f[(r - 16) as usize].set(*self.registers_f[(value - 16) as usize].get());
                        }
                    }
                }
                //Mov a piece of data (32 bits) from the data segment
                else if t == 2 {
                    let mut complete_address: u32 = value as u32;
                    if self.peek_nxt() {
                        match self.program[self.pc as usize] {
                            Nxt(_r1, value_nxt) => {
                                complete_address = (complete_address << 16) + value_nxt as u32;
                                self.pc += 1;
                            },
                            _ => unreachable!(),
                        }
                    }
                    let mut data: u32 = self.memory[complete_address as usize] as u32;
                    data = (data << 8) + self.memory[complete_address as usize + 1] as u32;
                    data = (data << 8) + self.memory[complete_address as usize + 2] as u32;
                    data = (data << 8) + self.memory[complete_address as usize + 3] as u32;
                    //Mov an Int
                    if r < 16 {
                        self.registers_i[r as usize].set(data as i32);
                    }
                    //Mov a Float
                    else {
                        self.registers_f[(r - 16) as usize].set(f32::from_bits(data));
                    }
                }
            }
            Swp(r1, r2) => {
                if r1 < 16 {
                    if r2 < 16 {
                        // Swap integer registers
                        let value1: i32 = *self.registers_i[r1 as usize].get();
                        let value2: i32 = *self.registers_i[r2 as usize].get();
                        self.registers_i[r1 as usize].set(value2);
                        self.registers_i[r2 as usize].set(value1);
                    } else {
                        // Swap between integer and float registers
                        let value1: i32 = *self.registers_i[r1 as usize].get();
                        let value2: f32 = *self.registers_f[(r2 - 16) as usize].get();
                        self.registers_i[r1 as usize].set(value2 as i32);
                        self.registers_f[(r2 - 16) as usize].set(value1 as f32);
                    }
                } else {
                    if r2 < 16 {
                        // Swap between float and integer registers
                        let value1: f32 = *self.registers_f[(r1 - 16) as usize].get();
                        let value2: i32 = *self.registers_i[r2 as usize].get();
                        self.registers_f[(r1 - 16) as usize].set(value2 as f32);
                        self.registers_i[r2 as usize].set(value1 as i32);
                    } else {
                        // Swap float registers
                        let value1: f32 = *self.registers_f[(r1 - 16) as usize].get();
                        let value2: f32 = *self.registers_f[(r2 - 16) as usize].get();
                        self.registers_f[(r1 - 16) as usize].set(value2);
                        self.registers_f[(r2 - 16) as usize].set(value1);
                    }
                }
                self.pc += 1;
            },
            Lod(r1, r2) => {
                let address: usize = *self.registers_i[r1 as usize].get() as usize;
                let mut t_result: u32 = self.memory[address] as u32;
                t_result = (t_result << 8) + self.memory[address + 1] as u32;
                t_result = (t_result << 8) + self.memory[address + 2] as u32;
                t_result = (t_result << 8) + self.memory[address + 3] as u32;
                
                if r2 < 16 {
                    // Integer register
                    self.registers_i[r2 as usize].set(t_result as i32);
                } else {
                    // Float register
                    self.registers_f[(r2 - 16) as usize].set(f32::from_bits(t_result));
                }
                
                self.pc += 1;
            }
            Str(r1, r2) => {
                let address: usize = *self.registers_i[r1 as usize].get() as usize;
                if r2 < 16 {
                    let value: u32 = *self.registers_i[r2 as usize].get() as u32;
                    self.memory[address] = value as u8;
                    self.memory[address + 1] = (value >> 8) as u8;
                    self.memory[address + 2] = (value >> 16) as u8;
                    self.memory[address + 3] = (value >> 24) as u8;
                } else {
                    let value: u32 = f32::to_bits(*self.registers_f[(r2 - 16) as usize].get());
                    self.memory[address] = value as u8;
                    self.memory[address + 1] = (value >> 8) as u8;
                    self.memory[address + 2] = (value >> 16) as u8;
                    self.memory[address + 3] = (value >> 24) as u8;
                }
                self.pc += 1;
            },
            And(r1, r2, r3) => {
                let value1 = *self.registers_i[r2 as usize].get();
                let value2 = *self.registers_i[r3 as usize].get();
                let result = value1 * value2;
                self.registers_i[r1 as usize].set(result);
                self.pc += 1;
            },
            LOr(r1, r2, r3) => {
                let value1 = *self.registers_i[r2 as usize].get();
                let value2 = *self.registers_i[r3 as usize].get();
                let result = value1 * value2;
                self.registers_i[r1 as usize].set(result);
                self.pc += 1;
            },
            Psh(r1, r2) => {
                if r2 < 16 {
                    let value: u32 = *self.registers_i[r2 as usize].get() as u32;
                    let bytes = value.to_le_bytes();
                    self.registers_i[r1 as usize].set(self.stack_pointer as i32);
                    self.memory.extend(bytes);
                } else {
                    let value: u32 = f32::to_bits(*self.registers_f[(r2 - 16) as usize].get());
                    let bytes = value.to_le_bytes();
                    self.registers_i[r1 as usize].set(self.stack_pointer as i32);
                    self.memory.extend(bytes);
                }
                self.stack_pointer += 4;
                self.pc += 1;
            },
            Pop(r1) => {
                let mut t_result: u32 = self.memory.pop().unwrap() as u32;
                t_result += (self.memory.pop().unwrap() as u32) << 8;
                t_result += (self.memory.pop().unwrap() as u32) << 16;
                t_result += (self.memory.pop().unwrap() as u32) << 24;
                
                if r1 < 16 {
                    // Integer register
                    self.registers_i[r1 as usize].set(t_result as i32);
                } else {
                    // Float register
                    self.registers_f[(r1 - 16) as usize].set(f32::from_bits(t_result));
                }
                self.stack_pointer -= 4; 
                self.pc += 1;
            },
            Cal(add) => {
                //Store caller next instruction address
                self.pc += 1;
                self.memory.extend(self.pc.to_le_bytes());
                //Store previous base frame pointer (aka frame_pointer)
                self.memory.extend(self.frame_pointer.to_le_bytes());
                //Create new stack frame by moving the current base frame pointer
                self.stack_pointer += 8;
                self.frame_pointer = self.stack_pointer;
                
                self.pc = add as u32;
            },
            Ret => {
                //Remove all the data in the current stack frame:
                self.memory.truncate(self.frame_pointer as usize);

                let mut prev_frame_pointer_bytes: [u8; 4] = [0; 4];
                prev_frame_pointer_bytes[0] = self.memory.pop().unwrap();
                prev_frame_pointer_bytes[1] = self.memory.pop().unwrap();
                prev_frame_pointer_bytes[2] = self.memory.pop().unwrap();
                prev_frame_pointer_bytes[3] = self.memory.pop().unwrap();
                let prev_frame_pointer = u32::from_be_bytes(prev_frame_pointer_bytes);

                let mut prev_pc_bytes: [u8; 4] = [0; 4];
                prev_pc_bytes[0] = self.memory.pop().unwrap();
                prev_pc_bytes[1] = self.memory.pop().unwrap();
                prev_pc_bytes[2] = self.memory.pop().unwrap();
                prev_pc_bytes[3] = self.memory.pop().unwrap();
                let prev_pc = u32::from_be_bytes(prev_pc_bytes);
                self.frame_pointer = prev_frame_pointer;
                self.stack_pointer -= 8;
                self.pc = prev_pc;
            },
            Cmp(r1, r2) => {
                if r1 < 16 {
                    let value1: i32 = *self.registers_i[r1 as usize].get();
                    let value2: i32 = *self.registers_i[r2 as usize].get();
            
                    if value1 == value2 {
                        self.cmp_register = 0b001;  // Equal
                    } else if value1 > value2 {
                        self.cmp_register = 0b010;  // Greater than
                    } else {
                        self.cmp_register = 0b100;  // Less than
                    }
                } else {
                    let value1: f32 = *self.registers_f[(r1 - 16) as usize].get();
                    let value2: f32 = *self.registers_f[(r2 - 16) as usize].get();
            
                    if value1 == value2 {
                        self.cmp_register = 0b001;  // Equal
                    } else if value1 > value2 {
                        self.cmp_register = 0b010;  // Greater than
                    } else {
                        self.cmp_register = 0b100;  // Less than
                    }
                }
                
                self.pc += 1;
            },
            Jmp(add) => {
                self.pc = add as u32;
            },
            Jmc(flag, add) => {
                if (flag & self.cmp_register) > 0 {
                    self.pc = add as u32;
                } else {
                    self.pc += 1;
                }
            },
            Sys(call) => {
                use Syscall::*;
                match call {
                    PrintInt => print!("{}", *self.registers_i[1].get()),
                    PrintFloat => print!("{}", *self.registers_f[0].get()),
                    PrintString => {
                        let mut s: String = String::new();
                        let mut i = *self.registers_i[1].get() as usize;
                        while self.memory[i] != b'\0' {
                            s.push(self.memory[i] as char);
                            i += 1;
                        }
                        println!("{}", s);
                    },
                    ReadInt => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read input");
                        let value: i32 = input.trim().parse().expect("Invalid input");
                        self.registers_i[1].set(value);
                    },
                    ReadFloat => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read input");
                        let value: f32 = input.trim().parse().expect("Invalid input");
                        self.registers_f[0].set(value);
                    },
                    ReadString => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read input");
                        let start_address = self.stack_pointer;
                        self.memory.extend(input.trim().to_string().as_bytes());
                        self.registers_i[1].set(start_address as i32);
                        self.stack_pointer += input.len() as u32;
                    },
                    Exit => {
                        return VMEvent::ExitSyscall;
                    }
                }
                self.pc += 1;
            },
            Inc(r) => {
                if r < 16 {
                    self.registers_i[r as usize].set(*self.registers_i[r as usize].get() + 1);
                } else {
                    self.registers_f[(r - 16) as usize].set(*self.registers_f[(r - 16) as usize].get() + 1.0);
                }
                self.pc += 1;
            },
            Dec(r) => {
                if r < 16 {
                    self.registers_i[r as usize].set(*self.registers_i[r as usize].get() - 1);
                } else {
                    self.registers_f[(r - 16) as usize].set(*self.registers_f[(r - 16) as usize].get() - 1.0);
                }
                self.pc += 1;
            },
            Shl(r, im) => {
                self.registers_i[r as usize].set(*self.registers_i[r as usize].get() << im);
                self.pc += 1;
            },
            Shr(r, im) => {
                self.registers_i[r as usize].set(*self.registers_i[r as usize].get() >> im);
                self.pc += 1;
            }
            _ => return VMEvent::IllegalOpCode,
        }
        VMEvent::AllGood
    }

    fn peek_nxt(&mut self) -> bool {
        match self.program[self.pc as usize] {
            Instruction::Nxt(_, _) => true,
            _ => false
        }
    }
}
