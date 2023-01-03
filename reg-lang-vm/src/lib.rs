#![allow(unused, dead_code)]
use reg_byte::OpCode;

#[derive(Debug)]
pub struct RegLangVM {
    /// Array that simulates having hardware registers
    registers: [i32; 32],
    /// Program counter that tracks which byte is being executed
    program_counter: usize,
    /// The bytecode of the program being run
    program: Vec<u8>,
    /// Contains the remainder of modulo division ops
    remainder: u32,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
}
impl RegLangVM {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            registers: [0; 32],
            program_counter: 0,
            program,
            remainder: 0,
            equal_flag: false,
        }
    }
    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        println!("Running program: {:?} ({} bytes)", self.program, self.program.len());
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }
    /// 
    fn execute_instruction(&mut self) -> bool {
        println!("Executing instruction at {}: ", self.program_counter);
        if self.program_counter >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            OpCode::STORE => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
                println!("STORE: {} -> {}", number, register);
            },
            OpCode::HLT => {
                println!("HLT encountered");
                return true;
            },
            OpCode::ADD => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                self.registers[self.next_8_bits() as usize] = self.registers[register1] + self.registers[register2];
                println!("ADD: {} + {} -> {}", self.registers[register1], self.registers[register2], register1);
            },
            OpCode::MUL => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                self.registers[self.next_8_bits() as usize] = self.registers[register1] * self.registers[register2];
                println!("MUL: {} * {} -> {}", self.registers[register1], self.registers[register2], register1);
            },
            OpCode::SUB => {
                let register1 = self.next_8_bits() as usize;
                let register2 = self.next_8_bits() as usize;
                self.registers[self.next_8_bits() as usize] = self.registers[register1] - self.registers[register2];
                println!("SUB: {} - {} -> {}", self.registers[register1], self.registers[register2], register1);
            },
            OpCode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                println!("DIV: {} / {} -> {}", register1, register2, register1);
            },
            OpCode::JMP => {
                let value = self.registers[self.next_8_bits() as usize];
                self.program_counter = value as usize;
                println!("JMP: {}", value);
            },
            OpCode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.program_counter += value as usize;
                println!("JMPF: {}", value);
            },
            OpCode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.program_counter -= value as usize;
                println!("JMPB: {}", value);
            },
            OpCode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
                println!("EQ: {} == {} -> {}", register1, register2, self.equal_flag);
            },
            OpCode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 != register2;
                self.next_8_bits();
                println!("NEQ: {} != {} -> {}", register1, register2, self.equal_flag);
            },
            OpCode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 > register2;
                self.next_8_bits();
                println!("GT: {} > {} -> {}", register1, register2, self.equal_flag);
            },
            OpCode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 < register2;
                self.next_8_bits();
                println!("LT: {} < {} -> {}", register1, register2, self.equal_flag);
            },
            OpCode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 >= register2;
                self.next_8_bits();
                println!("GTE: {} >= {} -> {}", register1, register2, self.equal_flag);
            },
            OpCode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
                println!("LTE: {} <= {} -> {}", register1, register2, self.equal_flag);
            },
            OpCode::JMPE => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.program_counter = target as usize;
                }
                println!("JMPE: {} -> {}", register, target);
            }
            _ => {
                println!("Unknown opcode encountered");
                return true;
            }
        }
        return false;
    }
    /// Decodes the current byte and return the corresponding OpCode
    fn decode_opcode(&mut self) -> OpCode {
        let opcode = self.program[self.program_counter];
        self.program_counter += 1;
        OpCode::from(opcode)
    }
    /// Returns the next 8 bits of the program
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.program_counter];
        self.program_counter += 1;
        result
    }
    /// Returns the next 16 bits of the program
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.program_counter] as u16) << 8) | self.program[self.program_counter + 1] as u16;
        self.program_counter += 2;
        result
    }
}