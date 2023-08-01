use core::panic;
use std::collections::HashMap;

use pest::{iterators::Pair, error::Error, Span};
use colored::Colorize;
use crate::{Rule, vm::bytecode::Instruction};
use crate::vm::virtual_cpu::VirtualCPU;
use super::intermediate_instruction::{IntermediateInstruction, CmpFlag};
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
    F32(f32),
    U32(u32),
    Str(String),
}
#[derive(Debug, Clone)]
struct Function {
    name: String,
    address: usize,
    content: Vec<(Label, Vec<IntermediateInstruction>)>,
    current_label: usize,
}
#[derive(Debug, Clone)]
struct Label {
    name: String,
    address: usize,
}
#[derive(Debug, Clone)]
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
    program: Vec<Instruction>,
}
impl ASMContext {
    pub fn new() -> Self {
        ASMContext {
            functions: vec![],
            entry_point: None,
            data: vec![],
            data_segment: vec![],
            program: vec![],
        }
    }
    pub fn compile(&mut self, program: Pair<Rule>) -> VirtualCPU {
        let mut messages = Message::new();
        let mut vm: VirtualCPU = VirtualCPU::new(false);
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
                    if !self.functions.is_empty() {
                        let mut address = 0;
                        for function in self.functions.iter_mut() {
                            function.address = address;
                            for (label, v) in function.content.iter_mut() {
                                label.address = address;
                                address += v.len();
                            }
                        }
                    }
                },
                Rule::end_directive => {
                    let data = self.data.clone();
                    let data_slice = data.as_slice();
                    let function = self.functions.clone();
                    let function_slice = function.as_slice();
                    self.clean_functions(data_slice, function_slice);
                    self.program.extend(Self::process_functions(&self.functions));

                    vm.program = self.program.to_owned();
                },
                Rule::EOI => {
                    vm.memory = self.data_segment.to_owned();
                    vm.base_pointer = self.data_segment.len() as u32;
                    vm.frame_pointer = self.data_segment.len() as u32;
                    vm.stack_pointer = self.data_segment.len() as u32;
                    break;
                }
                _ => unreachable!()
            }
        }
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
    //return an Option that contains the address of the label
    fn find_data_address(data: &[Data], label: &str) -> Option<usize> {
        if let Some(data) = data.iter().find(|d| d.name == label) {
            return Some(data.address.clone());
        }
        None
    }
    fn find_label_address(functions: &[Function], label: &str) -> Option<usize> {
        for function in functions.iter() {
            if let Some(data) = function.content.iter().find(|d| d.0.name == label) {
                return Some(data.0.address);
            }
        }
        None
    }

    fn movl_to_movi(instruction: &IntermediateInstruction, data: &[Data]) -> IntermediateInstruction{
        if let IntermediateInstruction::MovL(reg1, l) = instruction {
            if let Some(later) = Self::find_data_address(data, l) {
                return IntermediateInstruction::MovI(*reg1, later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
        panic!("Wtf Bro ?")
    }

    fn jmcl_to_jmci(instruction: &IntermediateInstruction, functions: &[Function]) -> IntermediateInstruction {
        if let IntermediateInstruction::JmcL(c, l) = instruction {
            if let Some(later) = Self::find_label_address(functions, l) {
                return IntermediateInstruction::JmcI(c.clone(), later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
        panic!("Wtf Bro ?")
    }

    fn jmpl_to_jmpi(instruction: &IntermediateInstruction, functions: &[Function]) -> IntermediateInstruction {
        if let IntermediateInstruction::JmpL(l) = instruction {
            if let Some(later) = Self::find_label_address(functions, l) {
                return IntermediateInstruction::JmpI(later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
        panic!("Wtf Bro ?")
    }

    fn call_to_cali(instruction: &IntermediateInstruction, functions: &[Function]) -> IntermediateInstruction {
        if let IntermediateInstruction::CalL(l) = instruction {
            if let Some(later) = Self::find_label_address(functions, l) {
                return IntermediateInstruction::CalI(later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
        panic!("Wtf Bro ?")
    }

    fn clean_functions(&mut self, data: &[Data], function_slice: &[Function]) {
        for (i, function) in function_slice.iter().enumerate() {
            for (j, (_label, instructions)) in function.content.iter().enumerate() {
                for (k, instruction) in instructions.iter().enumerate() {
                    match *instruction {
                        IntermediateInstruction::MovL(_, _) => {
                            self.functions[i].content[j].1[k] = Self::movl_to_movi(instruction, data);
                        },
                        IntermediateInstruction::JmpL(_) => {
                            self.functions[i].content[j].1[k] = Self::jmpl_to_jmpi(instruction, function_slice);
                        },
                        IntermediateInstruction::JmcL(_, _) => {
                            self.functions[i].content[j].1[k] = Self::jmcl_to_jmci(instruction, function_slice);
                        },
                        IntermediateInstruction::CalL(_) => {
                            self.functions[i].content[j].1[k] = Self::call_to_cali(instruction, function_slice);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn process_functions(functions: &[Function]) -> Vec<Instruction> {
        let mut result: Vec<Instruction> = Vec::new();
        for function in functions {
            for (_label, instructions) in function.content.iter() {
                for instruction in instructions {
                    result.push(Instruction::from(instruction.clone()));
                }
            }
        }
        result
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
                Value::U32(u) => {
                    self.data.push(Data {
                        name: label.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = u.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::Str(s) => {
                    self.data_segment.extend(s.as_bytes());
                    self.data_segment.push(0);
                },
                _ => ()
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
            content: vec![(Label {name, address: 0}, vec![])],
            current_label: 0,
        };
        for something in inner_pairs {
            match something.as_rule() {
                Rule::definition => {
                    current_function.content.push((
                        Label {
                            name: something.into_inner().next().unwrap().as_str().to_owned(),
                            address: current_function.current_label
                        },
                        vec![]
                    ));
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
        (messages, current_function)
    }
    fn compile_instruction(&mut self, current_function: &mut Function, instruction: Pair<Rule>) -> Message {
        let mut messages = Message::new();

        match instruction.as_rule() {
            Rule::add_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2), Ok(r3)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Add(r1, r2, r3)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::sub_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2), Ok(r3)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Sub(r1, r2, r3)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::mul_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2), Ok(r3)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Mul(r1, r2, r3)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::div_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2), Ok(r3)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Div(r1, r2, r3)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::mov_ins => {
                let mut inner = instruction.clone().into_inner();
                let reg = inner.next().unwrap().into_inner().next().unwrap();
                let ir_value = inner.next().unwrap();
                match ir_value.as_rule() {
                    Rule::int => {
                        current_function.content[current_function.current_label].1.push(
                            if let Ok(r) = Self::get_register(reg.clone()) {
                                IntermediateInstruction::MovI(r, ir_value.as_str().parse::<i16>().unwrap() as u16)    
                            } else {
                                messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "The register doesn't exist.".to_owned() }, reg.as_span()));
                                IntermediateInstruction::Ilg
                            }
                        )
                    },
                    Rule::label => {
                        current_function.content[current_function.current_label].1.push(
                            if let Ok(r) = Self::get_register(reg.clone()) {
                                IntermediateInstruction::MovL(r, ir_value.as_str().to_owned())    
                            } else {    
                                messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "The register doesn't exist.".to_owned() }, reg.as_span()));
                                IntermediateInstruction::Ilg
                            }
                        )
                    },
                    Rule::register => {
                        if let (Ok(r1), Ok(r2)) = (
                            Self::get_register(reg),
                            Self::get_register(ir_value)
                        ) {
                            current_function.content[current_function.current_label].1.push(
                                IntermediateInstruction::MovR(r1, r2)
                            )
                        } else {
                            current_function.content[current_function.current_label].1.push(
                                {
                                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                                    IntermediateInstruction::Ilg
                                }
                            )
                        }
                        
                    }
                    _ => unreachable!()
                }
            },
            Rule::sys_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Sys(
                        instruction.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                )
            },
            Rule::lor_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2), Ok(r3)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::LOr(r1, r2, r3)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::and_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2), Ok(r3)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::And(r1, r2, r3)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::cmp_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Cmp(r1, r2)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::inc_ins => {
                let mut inner = instruction.clone().into_inner();
                if let Ok(r1) = Self::get_register(inner.next().unwrap().into_inner().next().unwrap()) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Inc(r1)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::dec_ins => {
                let mut inner = instruction.clone().into_inner();
                if let Ok(r1) = Self::get_register(inner.next().unwrap().into_inner().next().unwrap()) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Dec(r1)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::swp_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Swp(r1, r2)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::lod_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Lod(r1, r2)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::str_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Str(r1, r2)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::jmp_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::JmpL(
                        instruction.into_inner().next().unwrap().as_str().to_owned()
                    )
                );
            },
            Rule::jmc_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::JmcL(
                        inner.next().unwrap().as_str().parse::<CmpFlag>().unwrap(), 
                        inner.next().unwrap().as_str().to_owned()
                    )
                );
            },
            Rule::psh_ins => {
                let mut inner = instruction.clone().into_inner();
                if let (Ok(r1), Ok(r2)) = (
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap()),
                    Self::get_register(inner.next().unwrap().into_inner().next().unwrap())
                ) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Psh(r1, r2)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::pop_ins => {
                let mut inner = instruction.clone().into_inner();
                if let Ok(r1) = Self::get_register(inner.next().unwrap().into_inner().next().unwrap()) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Pop(r1)
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::cal_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::CalL(
                        instruction.into_inner().next().unwrap().as_str().to_owned()
                    )
                )
            },
            Rule::ret_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Ret
                );
            },
            Rule::nop_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Nop
                );
            },
            Rule::shl_ins => {
                let mut inner = instruction.clone().into_inner();
                if let Ok(r1) = Self::get_register(inner.next().unwrap().into_inner().next().unwrap()) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Shl(r1, inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap())
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
            },
            Rule::shr_ins => {
                let mut inner = instruction.clone().into_inner();
                if let Ok(r1) = Self::get_register(inner.next().unwrap().into_inner().next().unwrap()) {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Shr(r1, inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap())
                    ) 
                } else {
                    messages.errors.push(Error::new_from_span(pest::error::ErrorVariant::CustomError { message: "One of these registers don't exist.".to_owned() }, instruction.as_span()));
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Ilg
                    ) 
                }
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
    fn get_register(register: Pair<Rule>) -> Result<usize, Span> {
        let str_register = register.as_str();
        let register_names = [
            "zero", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "s0", "s1","s2", "s3", "s4", "s5", "s6",
            "fa0", "fa1", "fa2", "fa3", "ft0", "ft1", "ft2", "ft3", "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7"
        ];
        let registers: HashMap<&str, u8>  = register_names.iter().enumerate().map(|(i, r)| (*r, i as u8)).collect();
        if let Some(value) = registers.get(&str_register) {
            return Ok(*value as usize);
        } else if let Ok(r) = str_register.parse::<usize>() {
            return Ok(r)
        }
        Err(register.as_span())
    }
}
