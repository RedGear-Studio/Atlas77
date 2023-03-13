use crate::instructions::OpCode;

use super::register::Register;

/// Virtual machine struct that will execute bytecode
pub struct VM {
    /// An array of 64 registers, each capable of storing 64 bits of binary data.
    pub registers: [Register; 64],
    /// A flag that is set when a comparison instruction is executed 
    /// 0 for equal, 1 for greater than, 2 for less than
    cmp_flag: u8,
    /// A flag to print or not print the value in a register in a more uwu way
    uwu_flag: bool,
    /// The program's bytecode instructions.
    pub program: Vec<u8>,
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// Contains the read-only section data
    data: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [Register::new(); 64],
            cmp_flag: 0,
            uwu_flag: false,
            program: Vec::new(),
            pc: 0,
            data: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.execute_instruction() {
                Some(0) => {
                    println!("Illegal opcode");
                    break;
                },
                Some(1) => {
                    println!("Program counter out of bounds");
                    break;
                },
                Some(2) => {
                    println!("Invalid register");
                    break;
                },
                None => (),
                _ => (),
                
            }
        }
    }
    /// Executes the next instruction in the program
    /// Returns an error code if there is a problem
    /// - `0` for illegal opcode
    /// - `1` for program counter out of bounds
    /// - `2` for invalid register
    pub fn execute_instruction(&mut self) -> Option<u8> {
        if self.pc >= self.program.len() {
            return Some(1);
        }
        match self.decode_opcode() {
            OpCode::MOV => {
                let register = self.next_8_bits() as usize;
                let reference = self.next_16_bits() as usize;
                self.registers[register] = Register { value: self.data[reference] as u64 };
            },
            OpCode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            OpCode::JMP => {
                let register = self.registers[self.next_8_bits() as usize];
                self.pc = register.value as usize;
            },
            OpCode::CMP => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.cmp_flag = 0;
                } else if register1 > register2 {
                    self.cmp_flag = 1;
                } else {
                    self.cmp_flag = 2;
                }
            },
            OpCode::JMC => {
                let register = self.registers[self.next_8_bits() as usize];
                let flag = self.next_8_bits();
                if flag == self.cmp_flag {
                    self.pc = register.value as usize;
                }
            },
            OpCode::PRT => {
                let register = self.registers[self.next_8_bits() as usize];
                if self.uwu_flag {
                    println!("UwU {}", register.value);
                } else {
                    println!("{}", register.value);
                }
            },
            OpCode::UWU => {
                self.uwu_flag = !self.uwu_flag;
            },
            OpCode::HLT => {
                println!("HLT instruction encountered, exiting...");
                return Some(0);
            },
            _ => {
                panic!("Unknown OpCode");
            }
        }
        return None;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((u16::from(self.program[self.pc])) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
        result
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

}