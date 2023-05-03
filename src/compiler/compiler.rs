use pest::{iterators::Pair, error::Error};
use colored::Colorize;
use crate::Rule;
use crate::vm::virtual_cpu::VirtualCPU;
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
    Bool(bool),
    None,
}
#[derive(Debug)]
struct Function {
    name: String,
    address: usize,
}
#[derive(Debug)]
struct Data {
    name: String,
    address: usize,
    value: Value,
}
#[derive(Debug)]
struct ASMContext {
    functions: Vec<Function>,
    entry_point: String, //Function define in the .global directive
    data: Vec<Data>,
    data_segment: Vec<u8>,
}

pub fn compile(program: Pair<Rule>) -> VirtualCPU {
    let mut context: ASMContext = ASMContext {
        functions: Vec::new(),
        entry_point: String::new(),
        data: Vec::new(),
        data_segment: Vec::new(),
    };
    let mut warnings: Vec<Error<Rule>> = vec![];
    let mut errors: Vec<Error<Rule>> = vec![];
    let mut vm: VirtualCPU = VirtualCPU::default();
    for directive in program.into_inner() {
        match directive.as_rule() {
            Rule::include_directive => {
                for file in directive.into_inner() {
                    match file.as_rule() {
                        Rule::file => {
                            let path = file.as_str().to_string();
                            let war = Error::new_from_span(
                                pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                    message: format!("Ignoring {}...", path) 
                                },
                                file.as_span()
                            );
                            warnings.push(war);
                        }
                        _ => unreachable!()
                    }
                    
                }
            },
            Rule::data_directive => {
                for variable in directive.into_inner() {
                    let mut inner_pairs = variable.into_inner();
                    let definition = inner_pairs.next().unwrap();
                    let type_directive = inner_pairs.next().unwrap();
                    let value = type_directive.into_inner().next().unwrap();
                    let mut data;
                    match value.as_rule() {
                        Rule::string_directive => {
                            let intermediate = Value::Str(value.into_inner().next().unwrap().as_str().to_string());
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: intermediate.clone()
                            });
                            data = intermediate;
                        },
                        Rule::i32_directive => {
                            data = Value::I32(value.into_inner().next().unwrap().as_str().parse::<i32>().unwrap());
                        },
                        Rule::i16_directive => {
                            data = Value::I16(value.into_inner().next().unwrap().as_str().parse::<i16>().unwrap());
                        },
                        Rule::i8_directive => {
                            data = Value::I8(value.into_inner().next().unwrap().as_str().parse::<i8>().unwrap());
                        },
                        Rule::f32_directive => {
                            data = Value::F32(value.into_inner().next().unwrap().as_str().parse::<f32>().unwrap());
                        },
                        Rule::u32_directive => {
                            data = Value::U32(value.into_inner().next().unwrap().as_str().parse::<u32>().unwrap());
                        },
                        Rule::u16_directive => {
                            data = Value::U16(value.into_inner().next().unwrap().as_str().parse::<u16>().unwrap());
                        },
                        Rule::u8_directive => {
                            data = Value::U8(value.into_inner().next().unwrap().as_str().parse::<u8>().unwrap());
                        },
                        Rule::array_u32_directive => {
                            let war = Error::new_from_span(
                                pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                    message: format!("array_u32 aren't supported yet")
                                },
                                value.as_span()
                            );
                            warnings.push(war);
                            data = Value::None;
                        },
                        Rule::array_f32_directive => {
                            let war = Error::new_from_span(
                                pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                    message: format!("array_f32 aren't supported yet")
                                },
                                value.as_span()
                            );
                            warnings.push(war);
                            data = Value::None;
                        },
                        Rule::array_i32_directive => {
                            let war = Error::new_from_span(
                                pest::error::ErrorVariant::<crate::Rule>::CustomError {
                                    message: format!("array_i32 aren't supported yet")
                                },
                                value.as_span()
                            );
                            warnings.push(war);
                            data = Value::None;
                        },
                        _ => unreachable!()
                    }
                    match data {
                        Value::F32(f) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            let byte = f.to_be_bytes();
                            context.data_segment.extend(byte);
                        },
                        Value::I32(i) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            let byte = i.to_be_bytes();
                            context.data_segment.extend(byte);
                        },
                        Value::I16(i) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            let byte = i.to_be_bytes();
                            context.data_segment.extend(byte);
                        },
                        Value::I8(i) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            context.data_segment.push(i as u8);
                        },
                        Value::U32(u) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            let byte = u.to_be_bytes();
                            context.data_segment.extend(byte);
                        },
                        Value::U16(u) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            let byte = u.to_be_bytes();
                            context.data_segment.extend(byte);
                        },
                        Value::U8(u) => {
                            context.data.push(Data {
                                name: definition.as_str().to_string(),
                                address: context.data_segment.len(),
                                value: data
                            });
                            context.data_segment.push(u);
                        },
                        Value::Str(s) => {
                            context.data_segment.extend(s.as_bytes());
                            context.data_segment.push(0);
                        },
                        Value::None => (),
                        _ => unimplemented!()
                    }
                    
                }
            },
            Rule::text_directive => {

            },
            Rule::end_directive => {
                
            },
            _ => ()
        }
    }
    
    for warning in warnings {
        println!("{}:", "Warning".bold().yellow());
        println!("{}", warning);
    }
    if errors.len() > 0 {
        for error in errors {
            println!("{}:", "Error".bold().red());
            println!("{}", error);
        }
        std::process::exit(1);
    }
    todo!("Return the VirtualCPU");
}
