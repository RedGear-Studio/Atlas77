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
    program: Vec<u32>,
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
                    self.program.extend(Self::process_functions(function_slice));
                    vm.program = self.program.to_owned();
                },
                Rule::EOI => {
                    vm.memory = self.data_segment.to_owned();
                    vm.base_pointer = self.data_segment.len();
                    vm.frame_pointer = self.data_segment.len();
                    vm.stack_pointer = self.data_segment.len();
                    break;
                }
                _ => unreachable!()
            }
        }
        for instruction in vm.program.iter() {
            println!("{:#08x}", instruction);
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

    fn movl_to_movi(instruction: &mut IntermediateInstruction, data: &[Data]) {
        if let IntermediateInstruction::Movl(reg1, l) = instruction {
            if let Some(later) = Self::find_data_address(data, l) {
                *instruction = IntermediateInstruction::Movi(*reg1, later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
    }

    fn jmcl_to_jmci(instruction: &mut IntermediateInstruction, functions: &[Function]) {
        if let IntermediateInstruction::Jmcl(c, l) = instruction {
            if let Some(later) = Self::find_label_address(functions, l) {
                *instruction = IntermediateInstruction::Jmci(c.clone(), later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
    }

    fn jmpl_to_jmpi(instruction: &mut IntermediateInstruction, functions: &[Function]) {
        if let IntermediateInstruction::Jmpl(l) = instruction {
            if let Some(later) = Self::find_label_address(functions, l) {
                *instruction = IntermediateInstruction::Jmpi(later as u16);
            } else {
                panic!("{}:\n  |\n  = Label {} not found", "Error".bold().red(), l);
            }
        }
    }

    fn clean_functions(&mut self, data: &[Data], function_slice: &[Function]) {
        for function in self.functions.iter_mut() {
            for (_label, instructions) in function.content.iter_mut() {
                for instruction in instructions {
                    match instruction {
                        IntermediateInstruction::Movl(_, _) => {
                            Self::movl_to_movi(instruction, data);
                        },
                        IntermediateInstruction::Jmpl(_) => {
                            Self::jmpl_to_jmpi(instruction, function_slice);
                        },
                        IntermediateInstruction::Jmcl(_, _) => {
                            Self::jmcl_to_jmci(instruction, function_slice)
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn process_functions(functions: &[Function]) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        for function in functions {
            for (_label, instructions) in function.content.iter() {
                for instruction in instructions {
                    result.push(u32::from(instruction.clone()));
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
                let reg = inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap();
                let ir_value = inner.next().unwrap();
                if let Ok(value) = ir_value.as_str().parse::<usize>() {
                    if value > u16::MAX.into() {
                        messages.notices.push(Error::new_from_span(
                            pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                message: format!("The mov instruction has been split in 2 separate instructions, mov reg{}, {} and nxt reg{}, {}", reg, ((value as u32) >> 16) as u16, reg, value as u16),
                            },
                            ir_value.as_span()
                        ));
                        current_function.content[current_function.current_label].1.extend(
                            [IntermediateInstruction::Movi(reg, ((value as u32) >> 16) as u16),
                            IntermediateInstruction::Nxt(reg, value as u16)]
                        )
                    } else {
                        current_function.content[current_function.current_label].1.push(
                            IntermediateInstruction::Movi(reg, value as u16)
                       )
                    }
                } else {
                    current_function.content[current_function.current_label].1.push(
                        IntermediateInstruction::Movl(
                            reg,
                            ir_value.as_str().to_owned(),
                        )
                    )
                }
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
                        instruction.into_inner().next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::dec_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Dec(
                        instruction.into_inner().next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::swp_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Swp(
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::lod_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Lod(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                )
            },
            Rule::str_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Str(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                )
            },
            Rule::jmp_ins => {
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Jmpl(
                        instruction.into_inner().next().unwrap().as_str().to_owned()
                    )
                );
            },
            Rule::jmc_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Jmcl(
                        inner.next().unwrap().as_str().parse::<CmpFlag>().unwrap(), 
                        inner.next().unwrap().as_str().to_owned()
                    )
                );
            },
            Rule::psh_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Psh(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                    )
                );
            },
            Rule::pop_ins => {
                let mut inner = instruction.into_inner();
                current_function.content[current_function.current_label].1.push(
                    IntermediateInstruction::Pop(
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(),
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
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
                        inner.next().unwrap().as_str().parse::<Type>().unwrap(), 
                        inner.next().unwrap().into_inner().next().unwrap().as_str().parse::<usize>().unwrap()
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
