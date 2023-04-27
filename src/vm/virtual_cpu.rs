use super::{events::VMEvent, bytecode::OpCode};

/// Virtual CPU implementation based on MIPS architecture.
pub struct VirtualCPU {
    pub pc: usize, // program counter, holds the address of the next instruction to be executed
    pub registers: [u32; 16], // 32 registers
    pub t_register: u32, // Temporary register used for swapping
    pub cmp_register: u8, // Store the result of the lost comparison
    pub memory: Vec<u8>, // Memory that behave as a stack and a heap for the virtual machine the first part of it is the data section and the second part is the dynamic memory (stack/heap)
    pub program: Vec<u16>, // Representation of the program instructions
    pub base_pointer: usize, // Base pointer for the current stack frame
    pub frame_pointer: usize, // Frame pointer for the current stack frame
    pub stack_pointer: usize, // Stack pointer for the current stack frame
}

impl VirtualCPU {
    pub fn new() -> VirtualCPU {
        VirtualCPU {
            pc: 0,
            registers: [0; 16],
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
                Some(VMEvent::IllegalOpCode) => break,
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
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.registers[reg2] = self.registers[reg1] + self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Sub => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.registers[reg2] = self.registers[reg1] - self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Mul => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.registers[reg2] = self.registers[reg1] * self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Div => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                if self.registers[reg2] == 0 {
                    return Some(VMEvent::DivideByZero);
                }
                self.registers[reg2] = self.registers[reg1] / self.registers[reg2];
                self.pc += 1;
            },
            OpCode::Mov => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let im = self.program[self.pc] as u8 & 0b0111_1111;
                self.registers[reg1] = im as u32;
                self.pc += 1;
            },
            OpCode::Nxt => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let im = self.program[self.pc] as u8 & 0b0111_1111;
                self.registers[reg1] << 7;
                self.registers[reg1] += im as u32;
                self.pc += 1;
            },
            OpCode::Swp => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.t_register = self.registers[reg1];
                self.registers[reg1] = self.registers[reg2];
                self.registers[reg2] = self.t_register;
                self.pc += 1;
            },
            OpCode::Lod => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.registers[reg1] = (self.memory[self.registers[reg2] as usize] << 24 | self.memory[(self.registers[reg2] + 1) as usize] << 16 | self.memory[(self.registers[reg2] + 2) as usize] << 8 | self.memory[(self.registers[reg2] + 3) as usize]) as u32;
                self.pc += 1;
            },
            OpCode::Str => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.memory[self.registers[reg1] as usize] = self.registers[reg2] as u8;
                self.memory[(self.registers[reg1] + 1) as usize] = (self.registers[reg2] >> 8) as u8;
                self.memory[(self.registers[reg1] + 2) as usize] = (self.registers[reg2] >> 16) as u8;
                self.memory[(self.registers[reg1] + 3) as usize] = (self.registers[reg2] >> 24) as u8;
                self.pc += 1;
            },
            OpCode::And => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.registers[reg2] &= self.registers[reg1];
                self.pc += 1;
            },
            OpCode::Psh => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                self.registers[reg2] = self.memory.len() as u32;
                self.memory.push(self.registers[reg1] as u8);
                self.memory.push((self.registers[reg1] >> 8) as u8);
                self.memory.push((self.registers[reg1] >> 16) as u8);
                self.memory.push((self.registers[reg1] >> 24) as u8);
                self.pc += 1;
            },
            OpCode::Pop => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                //pop the value from the stack and store it in reg1
                self.registers[reg1] = ((self.memory.pop().unwrap() as u32) << 24 | (self.memory.pop().unwrap() as u32) << 16 | (self.memory.pop().unwrap() as u32) << 8 | self.memory.pop().unwrap() as u32);
                self.pc += 1;
            },
            OpCode::Cal => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
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
                    self.frame_pointer += ((self.memory.pop().unwrap() as u32) << i*8) as usize;
                }
                for i in 0..3 {
                    self.pc += ((self.memory.pop().unwrap() as u32) << i*8) as usize;
                }
            },
            OpCode::Cmp => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                let reg2 = ((self.program[self.pc] >> 4) as u8 & 0b0000_0111) as usize;
                if self.registers[reg1] == self.registers[reg2] {
                    self.cmp_register = 0b0001;
                }
                else if self.registers[reg1] < self.registers[reg2] {
                    self.cmp_register = 0b0010;                    
                }
                else if self.registers[reg1] > self.registers[reg2] {
                    self.cmp_register = 0b0100;
                }
                self.pc += 1;
            },
            OpCode::Jmp => {
                let reg1 = ((self.program[self.pc] >> 7) as u8 & 0b0000_0111) as usize;
                self.pc = self.registers[reg1] as usize;
            },
            OpCode::Jmc => {
                let flag = ((self.program[self.pc] >> 6) as u8 & 0b0000_1111);
                let reg1 = ((self.program[self.pc] >> 3) as u8 & 0b0000_0111) as usize;
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
                        let mut i = 0;
                        while self.memory[i] != 0 {
                            print!("{}", self.memory[i]);
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
        OpCode::from((instruction >> 8) as u8 & 0b1111_1100)
    }
}
    