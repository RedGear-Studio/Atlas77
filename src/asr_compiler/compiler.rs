use std::vec;

use pest::{iterators::Pair, error::Error};
use colored::Colorize;
use crate::Rule;
use crate::vm::virtual_cpu::VirtualCPU;
use super::intermediate_instruction::{IntermediateInstruction, Type, CmpFlag};
#[derive(Default)]
struct Message {
    ///Used to report potential issues
    warnings: Vec<Error<Rule>>,
    ///Used to report errors
    errors: Vec<Error<Rule>>,
    ///Used to report compiler modification of the code
    notices: Vec<Error<Rule>>,
}
impl Message {
    fn new() -> Self {
        Message { warnings: vec![], errors: vec![], notices: vec![] }
    }
}
#[derive(Debug, Clone)]
enum Value {
    I32(i32),
    I16(i16),
    I8(i8),
    F32(f32),
    U32(u32),
    U16(u16),
    U8(u8),
    Str(String),
    None,
}
#[derive(Debug)]
struct Function {
    name: String,
    address: usize,
    content: Vec<(String, Vec<IntermediateInstruction>)>,
    current_label: usize,
}
#[derive(Debug)]
struct Data {
    name: String,
    address: usize,
    value: Value,
}
#[derive(Debug, Default)]
pub struct ASMContext {
    functions: Vec<Function>,
    entry_point: Option<String>, //Function define in the .global directive
    data: Vec<Data>,
    data_segment: Vec<u8>,
}
impl ASMContext {
    pub fn new() -> Self {
        ASMContext {
            functions: vec![],
            entry_point: None,
            data: vec![],
            data_segment: vec![],
        }
    }
    pub fn compile(&mut self, program: Pair<Rule>) -> VirtualCPU {
        let mut messages = Message::new();
        let mut vm: VirtualCPU = VirtualCPU::default();
        for directive in program.into_inner() {
            match directive.as_rule() {
                Rule::include_directive => {
                    let result = self.compile_include_directive(directive);
                    messages.warnings.extend(result.warnings);
                    messages.errors.extend(result.errors);
                },
                Rule::data_directive => {
                    let result = self.compile_data_segment(directive);
                    messages.warnings.extend(result.warnings);
                    messages.errors.extend(result.errors);
                },
                Rule::text_directive => {
                    let result = self.compile_program(directive);
                    messages.warnings.extend(result.warnings);
                    messages.errors.extend(result.errors);
                },
                Rule::end_directive => {
                    //TODO!: Modify all the label in code by their respective address
                },
                Rule::EOI => {
                    vm.memory = self.data_segment.clone();
                    vm.base_pointer = self.data_segment.len();
                    vm.frame_pointer = self.data_segment.len();
                    vm.stack_pointer = self.data_segment.len();
                    break;
                }
                _ => unreachable!()
            }
        }
        println!("{:?}", self);
        for warning in messages.warnings {
            println!("{}:", "Warning".bold().yellow());
            println!("{}", warning);
        }
        for notice in messages.notices {
            println!("{}:", "Notice".bold().blue());
            println!("{}", notice);
        }
        if !messages.errors.is_empty() {
            for error in messages.errors {
                println!("{}:", "Error".bold().red());
                println!("{}", error);
            }
            std::process::exit(0);
        }
        vm
    }
    
    fn compile_include_directive(&mut self, include: Pair<Rule>) -> Message {
        let mut messages: Message = Message::new();
        for file in include.into_inner() {
            match file.as_rule() {
                Rule::file => {
                    let path = file.as_str().to_string();
                    let war = Error::new_from_span(
                        pest::error::ErrorVariant::<crate::Rule>::CustomError {
                            message: format!("Ignoring {}", path) 
                        },
                        file.as_span()
                    );
                    messages.warnings.push(war);
                }
                _ => unreachable!()
            }
            
        }
        messages
    }

    fn compile_data_segment(&mut self, data: Pair<Rule>) -> Message {
        let mut messages: Message = Message::new();
        for variable in data.into_inner() {
            let mut inner_pairs = variable.into_inner();
            let label = inner_pairs.next().unwrap().into_inner().next().unwrap();
            let type_directive = inner_pairs.next().unwrap();
            let value = type_directive.into_inner().next().unwrap();
            let data;
            match value.as_rule() {
                Rule::string_directive => {
                    let intermediate = Value::Str(value.into_inner().next().unwrap().as_str().to_string());
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: intermediate.clone()
                    });
                    data = intermediate;
                },
                Rule::i32_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<i32>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::I32(number);
                },
                Rule::i16_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<i16>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::I16(number);
                },
                Rule::i8_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<i8>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::I8(number);
                },
                Rule::f32_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<f32>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::F32(number);
                },
                Rule::u32_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<u32>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::U32(number);
                },
                Rule::u16_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<u16>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::U16(number);
                },
                Rule::u8_directive => {
                    let ir = value.clone().into_inner().next().unwrap();
                    let Ok(number) = ir.as_str().parse::<u8>() else {
                        let war = Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: "Value out of range".to_owned()
                            },
                            value.as_span()
                        );
                        messages.errors.push(war);
                        continue;
                    };
                    data = Value::U8(number);
                },
                Rule::array_u32_directive => {
                    let war = Error::new_from_span(
                        pest::error::ErrorVariant::<crate::Rule>::CustomError {
                            message: "array_u32 aren't supported yet".to_owned()
                        },
                        value.as_span()
                    );
                    messages.warnings.push(war);
                    data = Value::None;
                },
                Rule::array_f32_directive => {
                    let war = Error::new_from_span(
                        pest::error::ErrorVariant::<crate::Rule>::CustomError {
                            message: "array_f32 aren't supported yet".to_owned()
                        },
                        value.as_span()
                    );
                    messages.warnings.push(war);
                    data = Value::None;
                },
                Rule::array_i32_directive => {
                    let war = Error::new_from_span(
                        pest::error::ErrorVariant::<crate::Rule>::CustomError {
                            message: "array_i32 aren't supported yet".to_owned()
                        },
                        value.as_span()
                    );
                    messages.warnings.push(war);
                    data = Value::None;
                },
                _ => unreachable!()
            }
            match data {
                Value::F32(f) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = f.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::I32(i) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = i.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::I16(i) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = i.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::I8(i) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    self.data_segment.push(i as u8);
                },
                Value::U32(u) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = u.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::U16(u) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = u.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::U8(u) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    self.data_segment.push(u);
                },
                Value::Str(s) => {
                    self.data_segment.extend(s.as_bytes());
                    self.data_segment.push(0);
                },
                Value::None => (),
            }
        }
        messages
    }
    fn compile_program(&mut self, program: Pair<Rule>) -> Message {
        let mut messages = Message::new();
        let mut inner_pairs = program.into_inner();
        //Define the entry point of the program.
        let directive = inner_pairs.next().unwrap();
        self.entry_point = match directive.as_rule() {
            Rule::global_directive => {
                Some(directive.into_inner().next().unwrap().as_str().to_owned())
            },
            Rule::library_directive => {
                None
            }
            _ => {
                let err = Error::new_from_span(
                    pest::error::ErrorVariant::<crate::Rule>::CustomError {
                        message: "Unknown directive".to_owned()
                    },
                    directive.as_span()
                );
                messages.errors.push(err);
                None
            }
        };
        for function in inner_pairs {
            let (result, result_function) = self.compile_function(function);
            self.functions.push(result_function);
            messages.notices.extend(result.notices);
            messages.errors.extend(result.errors);
            messages.warnings.extend(result.warnings);
        }
        messages
    }
    fn compile_function(&mut self, function: Pair<Rule>) -> (Message, Function) {
        let mut messages = Message::new();
        let mut inner_pairs = function.into_inner();
        let name = inner_pairs.next().unwrap().as_str().to_owned();
        let mut current_function = Function {
            name: name.clone(),
            address: 0,
            content: vec![(name, vec![])],
            current_label: 0,
        };
        for something in inner_pairs.next().into_iter() {
            match something.as_rule() {
                Rule::definition => {
                    current_function.content.push((something.into_inner().next().unwrap().as_str().to_owned(), vec![]));
                    current_function.current_label += 1;
                },
                Rule::instruction => {
                    for instruction in something.into_inner() {
                        let result = self.compile_instruction(&mut current_function, instruction);
                        messages.notices.extend(result.notices);
                        messages.warnings.extend(result.warnings);
                        messages.errors.extend(result.errors);
                    }
                }
                _ => unreachable!()
            }
        }
        current_function.address = if self.functions.len() >= 1 {
            let mut address = 0;
            for function in self.functions.iter() {
                for label in function.content.iter() {
                    address += label.1.len();
                }
            }
            address
        } else {
            0
        };
        (messages, current_function)
    }
    fn compile_instruction(&mut self, current_function: &mut Function, instruction: Pair<Rule>) -> Message {
        let mut messages = Message::new();

        match instruction.as_rule() {
            Rule::add_ins => {
                let mut inner = instruction.into_inner();
                let _type = inner.next().unwrap().as_str().parse::<Type>().unwrap();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Add(
                        _type,
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::sub_ins => {
                let mut inner = instruction.into_inner();
                let _type = inner.next().unwrap().as_str().parse::<Type>().unwrap();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Sub(
                        _type,
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::mul_ins => {
                let mut inner = instruction.into_inner();
                let _type = inner.next().unwrap().as_str().parse::<Type>().unwrap();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Mul(
                        _type,
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::div_ins => {
                let mut inner = instruction.into_inner();
                let _type = inner.next().unwrap().as_str().parse::<Type>().unwrap();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Div(
                        _type,
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::mov_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Mov(
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().as_str().to_owned(),
                    )
                )
            },
            Rule::sys_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Sys(
                        instruction.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                )
            },
            Rule::or_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Or(
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::and_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::And(
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::cmp_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Cmp(
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::inc_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Inc(
                        instruction.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::dec_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Dec(
                        instruction.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::swp_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Swp(
                        inner.next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::lod_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Lod(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(), 
                        inner.next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                )
            },
            Rule::str_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Str(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(), 
                        inner.next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                )
            },
            Rule::jmp_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Jmp(
                        instruction.into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::jmc_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Jmc(
                        inner.next().unwrap().as_str().parse::<CmpFlag>().unwrap(), 
                        inner.next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            Rule::psh_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Psh(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(),
                        inner.next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::pop_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Pop(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(),
                        inner.next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::cal_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Cal(
                        instruction.into_inner().next().unwrap().as_str().to_owned()
                    )
                )
            },
            Rule::ret_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Ret
                );
            },
            Rule::cst_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Cst(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(), 
                        inner.next().unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            },
            _ => {
                let err = Error::new_from_span(
                    pest::error::ErrorVariant::<crate::Rule>::CustomError {
                        message: "Unknown instruction. The instruction may not be implemented yet".to_owned()
                    },
                    instruction.as_span()
                );
                messages.errors.push(err);
            },
        }
        messages
    }
}
