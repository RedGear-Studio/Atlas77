pub mod opcode;
use opcode::*;

#[allow(unused)]
pub struct RegLangVM {
    registers: [i32; 32],
    program_counter: usize,
    program: Vec<u8>,
    remainder: u32,
}
impl RegLangVM {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            registers: [0; 32],
            program_counter: 0,
            program,
            remainder: 0,
        }
    }
    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }


    fn execute_instruction(&mut self) -> bool {
        if self.program_counter >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            OpCode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            },
            OpCode::HLT => {
                println!("HLT encountered");
                return false
            },
            OpCode::ADD => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                self.registers[self.next_8_bits() as usize] = self.registers[register1] + self.registers[register2];
            },
            OpCode::MUL => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                self.registers[self.next_8_bits() as usize] = self.registers[register1] * self.registers[register2];
            },
            OpCode::SUB => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                self.registers[self.next_8_bits() as usize] = self.registers[register1] - self.registers[register2];
            },
            OpCode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            _ => {
                println!("Unknown opcode encountered");
                return false
            }
        }
        true
    }
    fn decode_opcode(&mut self) -> OpCode {
        let opcode = self.program[self.program_counter];
        self.program_counter += 1;
        OpCode::from(opcode)
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.program_counter];
        self.program_counter += 1;
        return result;
    }
    
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.program_counter] as u16) << 8) | self.program[self.program_counter + 1] as u16;
        self.program_counter += 2;
        return result;
    }
}