use std::cmp::Ordering;

use super::{events::VMEvent, bytecode::OpCode};

/// Virtual CPU implementation, only support int 32 bits.
#[derive(Default)]
pub struct VirtualCPU {
    pub pc: usize, // program counter, holds the address of the next instruction to be executed
    pub registers: [u32; 32], // 32 registers (the first register is the zero register /!\ read-only)
    pub t_register: u32, // Temporary register used for swapping
    pub cmp_register: u8, // Store the result of the lost comparison
    pub memory: Vec<u8>, // Memory that behave as a stack and a heap for the virtual machine the first part of it is the data section and the second part is the dynamic memory (stack/heap)
    pub program: Vec<u32>, // Representation of the program instructions
    pub base_pointer: usize, // Base pointer for the current stack frame
    pub frame_pointer: usize, // Frame pointer for the current stack frame
    pub stack_pointer: usize, // Stack pointer for the current stack frame
}

impl VirtualCPU {
    pub fn new() -> VirtualCPU {
        VirtualCPU {
            pc: 0,
            registers: [0; 32],
            t_register: 0,
            cmp_register: 0,
            memory: vec![],
            program: vec![0b0001_0111_1001_0000],
            base_pointer: 0,
            frame_pointer: 0,
            stack_pointer: 0,
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.execute_instructions() {
                Some(VMEvent::IllegalOpCode) => {
                    println!("Error: Illegal opcode");    
                    break
                },
                Some(VMEvent::DivideByZero) => {
                    println!("Error: Division by zero");    
                    break
                },
                Some(VMEvent::OutOfBound) => {
                    println!("Error: Program Out of bound");    
                    break
                },
                Some(VMEvent::StackOverflow) => {
                    println!("Error: Stack Overflow");
                    break
                },
                Some(VMEvent::StackUnderflow) => {
                    println!("Error: Stack underflow");
                    break
                },
                Some(VMEvent::ExitSyscall) => {
                    println!("Exit program with code {}", self.registers[1]);
                    break;
                },
                _ => (),
            }
        }
    } 
    pub fn execute_instructions(&mut self) -> Option<VMEvent> {
        if self.pc >= self.program.len() {
            return Some(VMEvent::OutOfBound);
        }
        match self.decode_instruction() {
            OpCode::Nop => {
                self.pc += 1;
            },
            OpCode::Add => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                let reg3: usize = ((self.program[self.pc] >> 6) & 0b0001_1111) as usize;
                self.registers[reg3] = self.registers[reg1] + self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Sub => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                let reg3: usize = ((self.program[self.pc] >> 6) & 0b0001_1111) as usize;
                self.registers[reg3] = self.registers[reg1] - self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Mul => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                let reg3: usize = ((self.program[self.pc] >> 6) & 0b0001_1111) as usize;
                self.registers[reg3] = self.registers[reg1] * self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Div => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                let reg3: usize = ((self.program[self.pc] >> 6) & 0b0001_1111) as usize;
                if self.registers[reg2] == 0 {
                    return Some(VMEvent::DivideByZero);
                }
                self.registers[reg3] = self.registers[reg1] / self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Inc => {
                let reg1: usize = ((self.program[self.pc] >> 19) & 0b0001_1111) as usize;
                self.registers[reg1] += 1;
                self.pc += 1;
            },
            OpCode::Dec => {
                let reg1: usize = ((self.program[self.pc] >> 19) & 0b0001_1111) as usize;
                self.registers[reg1] -= 1;
                self.pc += 1;
            },
            OpCode::Mov => {
                let reg1: usize = ((self.program[self.pc] >> 19) & 0b0001_1111) as usize;
                let im: u16 = self.program[self.pc] as u16;
                self.registers[reg1] = im as u32;
                self.pc += 1;
            },
            OpCode::Nxt => {
                let reg1: usize = ((self.program[self.pc] >> 19) as u8 & 0b0001_1111) as usize;
                let im: u16 = self.program[self.pc] as u16;
                self.registers[reg1] = (self.registers[reg1] << 16) + im as u32;
                self.pc += 1;
            },
            OpCode::Swp => {
                let reg1: usize = ((self.program[self.pc] >> 19) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 14) & 0b0001_1111) as usize;
                self.t_register = self.registers[reg1];
                self.registers[reg1] = self.registers[reg2];
                self.registers[reg2] = self.t_register;
                self.pc += 1;
            },
            OpCode::Lod => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                match t {
                    //u8/i8 load
                    0 | 3 => {
                        self.registers[reg1] = (self.memory[self.registers[reg2] as usize]) as u32;
                    }
                    //u16/i16 load
                    1 | 4 => {
                        self.registers[reg1] = (self.memory[self.registers[reg2] as usize]) as u32;
                        self. registers[reg1] += (self.memory[(self.registers[reg2] + 1) as usize] as u32) << 8;
                    }
                    //u32/i32/f32 load
                    2 | 5 | 6 => {
                        self.registers[reg1] = (self.memory[self.registers[reg2] as usize]) as u32;
                        self.registers[reg1] = (self.registers[reg1] << 8) + (self.memory[(self.registers[reg2] + 1) as usize]) as u32;
                        self.registers[reg1] = (self.registers[reg1] << 16) + (self.memory[(self.registers[reg2] + 2) as usize]) as u32;
                        self.registers[reg1] = (self.registers[reg1] << 24) + (self.memory[(self.registers[reg2] + 3) as usize]) as u32;
                    }
                    //char
                    7 => {
                        let first_byte: u8 = self.memory[self.registers[reg2] as usize];
                        if first_byte <= 0x7F {
                            self.registers[reg1] = first_byte as u32;
                        } else if first_byte <= 0xDF {
                            self.registers[reg1] = first_byte as u32;
                            self.registers[reg1] = (self.registers[reg1] << 8)  + (self.memory[(self.registers[reg2] + 1) as usize]) as u32;
                        } else if first_byte <= 0xEF {
                            self.registers[reg1] = (self.memory[self.registers[reg2] as usize]) as u32;
                            self.registers[reg1] = (self.registers[reg1] << 8)  + (self.memory[(self.registers[reg2] + 1) as usize]) as u32;
                            self.registers[reg1] = (self.registers[reg1] << 16) + (self.memory[(self.registers[reg2] + 2) as usize]) as u32;
                        } else if first_byte <= 0xF4 {
                            self.registers[reg1] = (self.memory[self.registers[reg2] as usize]) as u32;
                            self.registers[reg1] = (self.registers[reg1] << 8)  + (self.memory[(self.registers[reg2] + 1) as usize]) as u32;
                            self.registers[reg1] = (self.registers[reg1] << 16) + (self.memory[(self.registers[reg2] + 2) as usize]) as u32;
                            self.registers[reg1] = (self.registers[reg1] << 24) + (self.memory[(self.registers[reg2] + 3) as usize]) as u32;
                        }
                    }
                    _ => unreachable!()
                }
                self.pc += 1;
            },
            OpCode::Str => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                match t {
                    //u8/i8 store
                    0 | 3 => {
                        self.memory[self.registers[reg2] as usize] = self.registers[reg1] as u8;
                    }
                    //u16/i16 store
                    1 | 4 => {
                        self.memory[self.registers[reg2] as usize] = self.registers[reg1] as u8;
                        self.memory[(self.registers[reg2] + 1) as usize] = (self.registers[reg1] >> 8) as u8;
                    }
                    //u32/i32/f32 store
                    2 | 5 | 6 => {
                        self.memory[self.registers[reg2] as usize] = self.registers[reg1] as u8;
                        self.memory[(self.registers[reg2] + 1) as usize] = (self.registers[reg1] >> 8) as u8;
                        self.memory[(self.registers[reg2] + 2) as usize] = (self.registers[reg1] >> 16) as u8;
                        self.memory[(self.registers[reg2] + 3) as usize] = (self.registers[reg1] >> 24) as u8;
                    }
                    //char
                    7 => {
                        let mut first_byte: u8 = (self.registers[reg1] >> 24) as u8;
                        let mut i: usize = 0;
                        for j in 0..3 {
                            if first_byte != 0 {
                                break;
                            }
                            first_byte = (self.registers[reg1] >> (j * 4)) as u8;
                            i = j;
                        }
                        if first_byte <= 0x7F {
                            self.memory[self.registers[reg2] as usize] = first_byte;
                        } else if first_byte <= 0xDF {
                            self.memory[self.registers[reg2] as usize] = first_byte;
                            self.memory[(self.registers[reg2] + 1) as usize] = (self.registers[reg1] >> (i * 4)) as u8;
                        } else if first_byte <= 0xEF {
                            self.memory[self.registers[reg2] as usize] = first_byte;
                            self.memory[(self.registers[reg2] + 1) as usize] = (self.registers[reg1] >> (i * 4)) as u8;
                            self.memory[(self.registers[reg2] + 2) as usize] = (self.registers[reg1] >> (i * 4)) as u8;
                        } else if first_byte <= 0xF4 {
                            self.memory[self.registers[reg2] as usize] = first_byte;
                            self.memory[(self.registers[reg2] + 1) as usize] = (self.registers[reg1] >> (i * 4)) as u8;
                            self.memory[(self.registers[reg2] + 2) as usize] = (self.registers[reg1] >> (i * 4)) as u8;
                            self.memory[(self.registers[reg2] + 3) as usize] = (self.registers[reg1] >> (i * 4)) as u8;
                        }
                    }
                    _ => unreachable!()
                }
                self.pc += 1;
            },
            OpCode::And => {
                let reg1: usize = ((self.program[self.pc] >> 19) as u8 & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 14) as u8 & 0b0001_1111) as usize;
                let reg3: usize = ((self.program[self.pc] >> 9) as u8 & 0b0001_1111) as usize;
                self.registers[reg3] = self.registers[reg1] & self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Psh => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 11) & 0b0001_1111) as usize;
                self.registers[reg2] = self.memory.len() as u32;
                match t {
                    //u8/i8 store
                    0 | 3 => {
                        self.memory.push(self.registers[reg1] as u8);
                    }
                    //u16/i16 store
                    1 | 4 => {
                        self.memory.push(self.registers[reg1] as u8);
                        self.memory.push((self.registers[reg1] >> 8) as u8);
                    }
                    //u32/i32/f32 store
                    2 | 5 | 6 => {
                        self.memory.push(self.registers[reg1] as u8);
                        self.memory.push((self.registers[reg1] >> 8) as u8);
                        self.memory.push((self.registers[reg1] >> 16) as u8);
                        self.memory.push((self.registers[reg1] >> 24) as u8);
                    }
                    //char
                    7 => {
                        let mut first_byte: u8 = (self.registers[reg1] >> 24) as u8;
                        let mut i: usize = 0;
                        for j in 0..3 {
                            if first_byte != 0 {
                                break;
                            }
                            first_byte = (self.registers[reg1] >> (j * 4)) as u8;
                            i = j;
                        }
                        if first_byte <= 0x7F {
                            self.memory.push(first_byte);
                        } else if first_byte <= 0xDF {
                            self.memory.push(first_byte);
                            self.memory.push((self.registers[reg1] >> (i * 4)) as u8);
                        } else if first_byte <= 0xEF {
                            self.memory.push(first_byte);
                            self.memory.push((self.registers[reg1] >> (i * 4)) as u8);
                            self.memory.push((self.registers[reg1] >> (i * 4)) as u8);
                        } else if first_byte <= 0xF4 {
                            self.memory.push(first_byte);
                            self.memory.push((self.registers[reg1] >> (i * 4)) as u8);
                            self.memory.push((self.registers[reg1] >> (i * 4)) as u8);
                            self.memory.push((self.registers[reg1] >> (i * 4)) as u8);
                        }
                    }
                    _ => unreachable!()
                }

                self.stack_pointer = self.memory.len();
                self.pc += 1;
            },
            OpCode::Pop => {
                let t: usize = ((self.program[self.pc] >> 22) & 0b0000_0111) as usize;
                let reg1: usize = ((self.program[self.pc] >> 16) & 0b0001_1111) as usize;
                match t {
                    //u8/i8 pop
                    0 | 3 => {
                        self.registers[reg1] = self.memory.pop().unwrap() as u32
                    },
                    //u16/i16 pop
                    1 | 4 => {
                        self.registers[reg1] = self.memory.pop().unwrap() as u32;
                        self.registers[reg1] += (self.memory.pop().unwrap() as u32) << 8;
                    }
                    //u32/i32/f32 pop
                    2 | 5 | 6 => {
                        self.registers[reg1] = self.memory.pop().unwrap() as u32;
                        self.registers[reg1] += (self.memory.pop().unwrap() as u32) << 8;
                        self.registers[reg1] += (self.memory.pop().unwrap() as u32) << 16;
                        self.registers[reg1] += (self.memory.pop().unwrap() as u32) << 24;
                    }
                    //char
                    7 => {
                        unimplemented!();
                    }
                    _ => unreachable!()
                }
                self.stack_pointer = self.memory.len();
                self.pc += 1;
            },
            OpCode::Cal => {
                let reg1 = ((self.program[self.pc] >> 19) & 0b0001_1111) as usize;
                self.memory.push(((self.pc as u32) >> 24) as u8);
                self.memory.push(((self.pc as u32) >> 16) as u8);
                self.memory.push(((self.pc as u32) >> 8) as u8);
                self.memory.push(self.pc as u8);
                self.memory.push(((self.frame_pointer as u32) >> 24) as u8);
                self.memory.push(((self.frame_pointer as u32) >> 16) as u8);
                self.memory.push(((self.frame_pointer as u32) >> 8) as u8);
                self.memory.push(self.frame_pointer as u8);
                self.frame_pointer = self.memory.len();
                self.stack_pointer = self.memory.len();
                self.pc = self.registers[reg1] as usize;
            },
            OpCode::Ret => {
                for i in self.frame_pointer..self.stack_pointer {
                    self.memory.pop();
                }
                for i in 0..3 {
                    self.frame_pointer += ((self.memory.pop().unwrap() as u32) << (i * 4)) as usize;
                }
                for i in 0..3 {
                    self.pc += ((self.memory.pop().unwrap() as u32) << (i * 4)) as usize;
                }
            },
            OpCode::Cmp => {
                let reg1: usize = ((self.program[self.pc] >> 19) & 0b0001_1111) as usize;
                let reg2: usize = ((self.program[self.pc] >> 14) & 0b0001_1111) as usize;
                match self.registers[reg1].cmp(&self.registers[reg2]) {
                    Ordering::Less => self.cmp_register = 0b0010,
                    Ordering::Greater => self.cmp_register = 0b0100,
                    Ordering::Equal => self.cmp_register = 0b0001
                }
                self.pc += 1;
            },
            OpCode::Cst => {
                let (t1, t2) = (((self.program[self.pc] >> 22) & 0b0000_0111) as usize,
                    ((self.program[self.pc] >> 19) & 0b0000_0111) as usize);
                let reg: usize = ((self.program[self.pc] >> 14) & 0b0001_1111) as usize;
                //0 = u8, 1 = u16, 2 = u32, 3 = i8, 4 = i16, 5 = i32, 6 = f32, 7 = char
                //Can't cast to a char
                unimplemented!("Cst opcode not implemented yet");
                match (t1, t2) {
                    (0, 0) | (1, 1) | (2, 2) | (3, 3) | (4, 4) | (5, 5) | (6, 6) => (), //Cast of the same type
                    (0, 1) | (0, 2) => (), //Cast of a more precise Uint type
                    (3, 4) => {

                    },
                    _ => unreachable!()
                }
            }
            OpCode::Jmp => {
                let reg1 = ((self.program[self.pc] >> 19) as u8 & 0b0001_1111) as usize;
                self.pc = self.registers[reg1] as usize;
            },
            OpCode::Jmc => {
                let reg1 = ((self.program[self.pc] >> 19) as u8 & 0b0001_1111) as usize;
                let flag = ((self.program[self.pc] >> 15) as u8 & 0b0000_1111);
                if self.cmp_register & flag > 0 {
                    self.pc = self.registers[reg1] as usize;
                }
                else {
                    self.pc += 1;
                }
            },
            OpCode::Sys => {
                let value = self.program[self.pc] as u8;
                match value {
                    0 => print!("{}", self.registers[1]), // Print integer, found the integer in the register 1
                    1 => print!("{}", self.registers[1] as f32), // Print float, found the float in the register 1 (currently non usable because float support isn't implemented)
                    2 => {
                        let mut i = self.registers[1] as usize;
                        while self.memory[i] != 0 {
                            print!("{}", self.memory[i]);
                            i += 1;
                        }
                    }, // Print string, found the address of the string in register 1
                    3 => todo!(), // Read integer, store the integer in the register 1
                    4 => todo!(), // Read float, store the float in the register 1 (currently non usable because float support isn't implemented)
                    5 => todo!(), // Read string, store the string on top of the memory and its address in register 1
                    6 => return Some(VMEvent::ExitSyscall), // Exit the program with the error code found in the register 1
                    _ => unreachable!(),
                }
            },
            _ => return Some(VMEvent::IllegalOpCode),
        }
        Some(VMEvent::AllGood)
    }

    fn decode_instruction(&mut self) -> OpCode {
        let instruction = self.program[self.pc];
        //first 6-bit are for the current opcode, and then based on the opcode, the args may differ
        OpCode::from((instruction >> 24) as u8)
    }
}
