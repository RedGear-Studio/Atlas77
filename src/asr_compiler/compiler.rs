use pest::{iterators::Pair, error::Error};
use colored::Colorize;
use crate::Rule;
use crate::vm::virtual_cpu::VirtualCPU;
use super::intermediate_instruction::IntermediateInstruction;

struct ErrorsAndWarnings {
    warnings: Vec<Error<Rule>>,
    errors: Vec<Error<Rule>>,
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
    Bool(bool),
    None,
}
#[derive(Debug)]
struct Function {
    name: String,
    address: usize,
    content: Vec<(String, Vec<IntermediateInstruction>)>
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
        let mut errors_and_warnings = ErrorsAndWarnings { errors:vec![], warnings:vec![] };
        let mut vm: VirtualCPU = VirtualCPU::default();
        for directive in program.into_inner() {
            match directive.as_rule() {
                Rule::include_directive => {
                    let result = self.compile_include_directive(directive);
                    errors_and_warnings.warnings.extend(result.warnings);
                    errors_and_warnings.errors.extend(result.errors);
                },
                Rule::data_directive => {
                    let result = self.compile_data_segment(directive);
                    errors_and_warnings.warnings.extend(result.warnings);
                    errors_and_warnings.errors.extend(result.errors);
                },
                Rule::text_directive => {
                    //warnings.extend(self.compile_program(directive));
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
        
        for warning in errors_and_warnings.warnings {
            println!("{}:", "Warning".bold().yellow());
            println!("{}", warning);
        }
        if !errors_and_warnings.errors.is_empty() {
            for error in errors_and_warnings.errors {
                println!("{}:", "Error".bold().red());
                println!("{}", error);
            }
            std::process::exit(0);
        }
        vm
    }
    
    fn compile_include_directive(&mut self, include: Pair<Rule>) -> ErrorsAndWarnings {
        let mut errors_and_warnings: ErrorsAndWarnings = ErrorsAndWarnings { errors:vec![], warnings:vec![] };
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
                    errors_and_warnings.warnings.push(war);
                }
                _ => unreachable!()
            }
            
        }
        errors_and_warnings
    }

    fn compile_data_segment(&mut self, data: Pair<Rule>) -> ErrorsAndWarnings {
        let mut errors_and_warnings: ErrorsAndWarnings = ErrorsAndWarnings { errors:vec![], warnings:vec![] };
        for variable in data.into_inner() {
            let mut inner_pairs = variable.into_inner();
            let definition = inner_pairs.next().unwrap();
            let type_directive = inner_pairs.next().unwrap();
            let value = type_directive.into_inner().next().unwrap();
            let mut data;
            match value.as_rule() {
                Rule::string_directive => {
                    let intermediate = Value::Str(value.into_inner().next().unwrap().as_str().to_string());
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
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
                        errors_and_warnings.errors.push(war);
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
                        errors_and_warnings.errors.push(war);
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
                        errors_and_warnings.errors.push(war);
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
                        errors_and_warnings.errors.push(war);
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
                        errors_and_warnings.errors.push(war);
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
                        errors_and_warnings.errors.push(war);
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
                        errors_and_warnings.errors.push(war);
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
                    errors_and_warnings.warnings.push(war);
                    data = Value::None;
                },
                Rule::array_f32_directive => {
                    let war = Error::new_from_span(
                        pest::error::ErrorVariant::<crate::Rule>::CustomError {
                            message: "array_f32 aren't supported yet".to_owned()
                        },
                        value.as_span()
                    );
                    errors_and_warnings.warnings.push(war);
                    data = Value::None;
                },
                Rule::array_i32_directive => {
                    let war = Error::new_from_span(
                        pest::error::ErrorVariant::<crate::Rule>::CustomError {
                            message: "array_i32 aren't supported yet".to_owned()
                        },
                        value.as_span()
                    );
                    errors_and_warnings.warnings.push(war);
                    data = Value::None;
                },
                _ => unreachable!()
            }
            match data {
                Value::F32(f) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = f.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::I32(i) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = i.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::I16(i) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = i.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::I8(i) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    self.data_segment.push(i as u8);
                },
                Value::U32(u) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = u.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::U16(u) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
                        address: self.data_segment.len(),
                        value: data
                    });
                    let byte = u.to_be_bytes();
                    self.data_segment.extend(byte);
                },
                Value::U8(u) => {
                    self.data.push(Data {
                        name: definition.as_str().to_string(),
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
                _ => unimplemented!()
            }
        }
        errors_and_warnings
    }
    fn compile_program(&mut self, program: Pair<Rule>) -> Vec<Error<Rule>> {
        todo!()
    }
    fn compile_function(&mut self, function: Pair<Rule>) -> Vec<Error<Rule>> {
        todo!()
    }
}