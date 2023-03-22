pub mod register;
pub mod event;

use reg_byte::OpCode;

use crate::{
    register::Register,
    event::VMEvent
};

use std::f64;

/// Virtual machine struct that will execute bytecode
#[derive(Debug)]
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
    /// The stack that is used for the `PUSH` and `POP` instructions
    /// 
    pub stack: Vec<u64>,
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// Contains the read-only section data
    pub data: Vec<u64>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [Register::new(); 64],
            cmp_flag: 0,
            uwu_flag: false,
            program: Vec::new(),
            stack: Vec::new(),
            pc: 0,
            data: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.execute_instruction() {
                Some(VMEvent::IllegalOpCode) => {
                    println!("Illegal opcode");
                    break;
                },
                Some(VMEvent::OutOfBound) => {
                    println!("Program counter out of bounds");
                    break;
                },
                Some(VMEvent::InvalidRegister) => {
                    println!("Invalid register");
                    break;
                },
                Some(VMEvent::HaltEncountered) => {
                    break;
                }
                None => (),
            }
        }
    }
    /// Executes the next instruction in the program
    /// Returns an error code if there is a problem
    /// - `0` for illegal opcode
    /// - `1` for program counter out of bounds
    /// - `2` for invalid register
    pub fn execute_instruction(&mut self) -> Option<VMEvent> {
        if self.pc >= self.program.len() {
            return Some(VMEvent::OutOfBound);
        }
        match self.decode_opcode() {
            OpCode::MOV => {
                let register = self.next_8_bits() as usize;
                let reference = self.next_16_bits() as usize;
                self.registers[register] = Register { value: self.data[reference] as u64 };
            },
            OpCode::ADD => {
                let value1 = self.registers[self.next_8_bits() as usize].value as i64;
                let value2 = self.registers[self.next_8_bits() as usize].value as i64;
                let result = value1 + value2;
                self.registers[self.next_8_bits() as usize] = Register{value: result as u64};
            },
            OpCode::SUB => {
                let value1 = self.registers[self.next_8_bits() as usize].value as i64;
                let value2 = self.registers[self.next_8_bits() as usize].value as i64;
                let result = value1 - value2;
                self.registers[self.next_8_bits() as usize] = Register{value: result as u64};
            },
            OpCode::MUL => {
                let value1 = self.registers[self.next_8_bits() as usize].value as i64;
                let value2 = self.registers[self.next_8_bits() as usize].value as i64;
                let result = value1 * value2;
                self.registers[self.next_8_bits() as usize] = Register{value: result as u64};
            },
            OpCode::DIV => {
                let value1 = self.registers[self.next_8_bits() as usize].value as i64;
                let value2 = self.registers[self.next_8_bits() as usize].value as i64;
                let result = value1 / value2;
                self.registers[self.next_8_bits() as usize] = Register{value: result as u64};
            },
            OpCode::ADF => {
                let value1 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let value2 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let result = value1 + value2;
                self.registers[self.next_8_bits() as usize] = Register { value: result.to_bits() };
            },
            OpCode::SBF => {
                let value1 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let value2 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let result = value1 - value2;
                self.registers[self.next_8_bits() as usize] = Register { value: result.to_bits() };
            },
            OpCode::MLF => {
                let value1 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let value2 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let result = value1 * value2;
                self.registers[self.next_8_bits() as usize] = Register { value: result.to_bits() };
            },
            OpCode::DVF => {
                let value1 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let value2 = f64::from_bits(self.registers[self.next_8_bits() as usize].value);
                let result = value1 / value2;
                self.registers[self.next_8_bits() as usize] = Register { value: result.to_bits() };
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
                let types = self.next_8_bits();
                self.next_8_bits();
                if types == 0 {
                    println!("{}", register.value);
                } else if types == 1 {
                    println!("{}", f64::from_bits(register.value));
                } else if types == 2 {
                    println!("{}", register.value as i64);
                } else {
                    return Some(VMEvent::InvalidRegister);
                }
            },
            // Cast the value in a register to a type from a type `CST REGISTER TYPE_FROM TYPE_TO`
            // 0 for u64, 1 for f64 and 2 for i64
            OpCode::CST => {
                let register = self.next_8_bits() as usize;
                let type_from = self.next_8_bits();
                let type_to = self.next_8_bits();
                match type_from {
                    0 => {
                        match type_to {
                            1 => {
                                let value = self.registers[register].value as f64;
                                self.registers[register] = Register { value: value.to_bits() };
                            },
                            2 => {
                                let value = self.registers[register].value as i64;
                                self.registers[register] = Register { value: value as u64 };
                            },
                            _ => {
                                return Some(VMEvent::InvalidRegister);
                            }
                        }
                    },
                    1 => {
                        match type_to {
                            0 => {
                                let value = f64::from_bits(self.registers[register].value);
                                self.registers[register] = Register { value: value as u64 };
                            },
                            2 => {
                                let value = f64::from_bits(self.registers[register].value) as i64;
                                self.registers[register] = Register { value: value as u64 };
                            },
                            _ => {
                                return Some(VMEvent::InvalidRegister);
                            }
                        }
                    },
                    2 => {
                        match type_to {
                            0 => {
                                let value = self.registers[register].value as i64;
                                self.registers[register] = Register { value: value as u64 };
                            },
                            1 => {
                                let value = self.registers[register].value as i64;
                                self.registers[register] = Register { value: (value as f64).to_bits()};
                            },
                            _ => {
                                return Some(VMEvent::InvalidRegister);
                            }
                        }
                    },
                    _ => {
                        return Some(VMEvent::InvalidRegister);
                    }
                }
            },
            OpCode::UWU => {
                self.uwu_flag = !self.uwu_flag;
            },
            OpCode::HLT => {
                println!("HLT instruction encountered, exiting...");
                return Some(VMEvent::HaltEncountered);
            },
            _ => {
                return Some(VMEvent::IllegalOpCode);
            },
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