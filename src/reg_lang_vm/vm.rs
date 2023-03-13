use crate::instructions::OpCode;

use super::register::Register;

/// Virtual machine struct that will execute bytecode
pub struct VM {
    /// An array of 64 registers, each capable of storing 64 bits of binary data.
    pub registers: [Register; 64],
    /// A flag that is set to `true` if the most recent comparison was equal.
    equal_flag: bool,
    /// A flag to print or not print the value in a register in a more uwu way
    uwu_flag: bool,
    /// The program's bytecode instructions.
    pub program: Vec<u8>,
    /// An alias that can be specified by the user and used to refer to the Node
    pub alias: Option<String>,
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// Keeps track of the current frame pointer
    bp: usize,
    /// Loop counter field, used with the `LOOP` instruction
    loop_counter: usize,
    /// Contains the read-only section data
    ro_data: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [Register::new(); 64],
            equal_flag: false,
            uwu_flag: false,
            program: Vec::new(),
            alias: None,
            pc: 0,
            bp: 0,
            loop_counter: 0,
            ro_data: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {}
    }

    pub fn execute_instruction(&mut self) -> Option<u8> {
        if self.pc >= self.program.len() {
            return Some(1);
        }
        match self.decode_opcode() {
            OpCode::MOV => {
                // TODO: Implement the data section
            },
            OpCode::HLT => {
                println!("HLT instruction encountered, exiting...");
                return Some(0);
            }
            OpCode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
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