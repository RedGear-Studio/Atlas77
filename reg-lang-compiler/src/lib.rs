extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use reg_byte::OpCode;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RegParser;
#[derive(Debug)]
pub struct RegCompiler {
    pub program: Vec<u8>,
    pub program_counter: usize,
    pub data: Vec<u64>,
}

impl RegCompiler {
    pub fn new() -> Self {
        RegCompiler {
            program: Vec::new(),
            program_counter: 0,
            data: Vec::new(),
        }
    }
    pub fn compile(&mut self, input: &str) {
        let program = RegParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
        for expr in program.into_iter() {
            match expr.as_rule() {
                Rule::program => {
                    for instruction in expr.into_inner() {
                        match instruction.as_rule() {
                            Rule::MOV => {
                                self.program.push(OpCode::MOV as u8);
                                let mut inner = instruction.into_inner();
                                let register1 = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register1);
                                let value = inner.next().unwrap().as_str().replace("#", "").parse::<u64>().unwrap();
                                self.data.push(value);
                                let reference = (self.data.len() - 1) as u16;
                                self.program.push((reference >> 8) as u8);
                                self.program.push(reference as u8);
                            },
                            Rule::ADD => {
                                self.program.push(OpCode::ADD as u8);
                                let mut inner = instruction.into_inner();
                                let register1 = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register1);
                                let register2 = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register2);
                                let register3 = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register3);
                            },
                            Rule::JMP => {
                                self.program.push(OpCode::JMP as u8);
                                let mut inner = instruction.into_inner();
                                let register = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register);
                                self.program.push(0);
                                self.program.push(0);
                            },
                            Rule::JMC => {
                                self.program.push(OpCode::JMC as u8);
                                let mut inner = instruction.into_inner();
                                let register = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register);
                                let flag = inner.next().unwrap().as_str();
                                match flag {
                                    "EQ" => {
                                        self.program.push(0);
                                    },
                                    "GT" => {
                                        self.program.push(1);
                                    },
                                    "LT" => {
                                        self.program.push(2);
                                    }
                                    _ => {
                                        panic!("Unknown flag for the JMC instruction");
                                    }
                                }
                                self.program.push(0);
                            },
                            Rule::COMPARISON => {
                                self.program.push(OpCode::CMP as u8);
                                let mut inner = instruction.into_inner();
                                let register = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register);
                                self.program.push(0);
                            },
                            Rule::PRINT => {
                                self.program.push(OpCode::PRT as u8);
                                let mut inner = instruction.into_inner();
                                let register = inner.next().unwrap().as_str().replace("$", "").parse::<u8>().unwrap();
                                self.program.push(register);
                                self.program.push(0);
                                self.program.push(0);
                            },
                            Rule::HLT => {
                                self.program.push(OpCode::HLT as u8);
                                self.program.push(0);
                                self.program.push(0);
                                self.program.push(0);
                            }
                            _ => (),
                        }

                    }
                }
                _ => ()
            }
        }
    }
}
