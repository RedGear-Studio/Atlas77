#![allow(unused)]
pub mod real_compiler;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use reg_byte::OpCode;

#[derive(Parser)]
#[grammar = "real_compiler/grammar.pest"]
pub(crate) struct FromRegLangParser;

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
}
