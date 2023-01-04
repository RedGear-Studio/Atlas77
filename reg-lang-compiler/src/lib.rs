#![allow(unused)]

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
}

pub fn compile(input :&str) -> RegCompiler {
    let mut compiler = RegCompiler {
        program: vec![],
        program_counter: 0,
    };
    let mut program = RegParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    for expr in program.into_iter() {
        match expr.as_rule() {
            Rule::program => {
                for instruction in expr.into_inner() {
                    match instruction.as_rule() {
                        Rule::STORE => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::STORE as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    Rule::NUMBERS => {
                                        let number = args.as_str().replace("#", "").parse::<u16>().unwrap();
                                        compiler.program.push((number >> 8) as u8);
                                        compiler.program.push((number & 0xFF) as u8);
                                    }
                                    _ => {
                                        panic!("Invalid rule (STORE)");
                                    }
                                }
                            }
                        },
                        Rule::ADD => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::ADD as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (ADD)");
                                    }
                                }
                            }
                        },
                        Rule::MUL => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::MUL as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (MUL)");
                                    }
                                }
                            }
                        },
                        Rule::DIV => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::DIV as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (DIV)");
                                    }
                                }
                            }
                        },
                        Rule::SUB => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::SUB as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (SUB)");
                                    }
                                }
                            }
                        },
                        Rule::JMP => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::JMP as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (JMP)");
                                    }
                                }
                            }
                        },
                        Rule::JMPB => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::JMPB as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (JMPB)");
                                    }
                                }
                            }
                        },
                        Rule::JMPF => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::JMPF as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (JMPF)");
                                    }
                                }
                            }
                        },
                        Rule::EQ => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::EQ as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (EQ)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::NEQ => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::NEQ as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (NEQ)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::GT => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::GT as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (GT)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::LT => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::LT as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (LT)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::GTE => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::GTE as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (GTE)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::LTE => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::LTE as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (LTE)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::LTE => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::LTE as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (LTE)");
                                    }
                                }
                            }
                            compiler.program.push(0);
                        },
                        Rule::JMPE => {
                           compiler.program_counter += 1;
                            compiler.program.push(OpCode::JMPE as u8);
                            for args in instruction.into_inner() {
                                match args.as_rule() {
                                    Rule::REGISTER => {
                                        let register = args.as_str().replace("$", "");
                                        compiler.program.push(register.parse::<u8>().unwrap());
                                    },
                                    _ => {
                                        panic!("Invalid rule (JMPE)");
                                    }
                                }
                            }
                        },
                        Rule::HLT => {
                            compiler.program_counter += 1;
                            compiler.program.push(OpCode::HLT as u8);
                        },
                        Rule::EOI => {
                        } 
                        _ => {
                            panic!("Invalid rule (instruction)");
                        }
                    }
                }
            }
            _ => {
                panic!("Invalid rule (program)");
            }
        }
    }
    return compiler;
}