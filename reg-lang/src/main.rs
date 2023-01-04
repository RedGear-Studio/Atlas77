#![allow(unused)]

use std::vec;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;

use reg_lang_compiler::compile;
use reg_lang_vm::RegLangVM;
fn main() {
    run_console();
}

fn run_console() {
    println!("Welcome to the RegLang console!");
    println!("Enter your instructions or type exit to leave.");
    println!("(Note: this is a work in progress, so not all instructions are implemented yet.");
    loop {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        print!("RegLang > ");
        std::io::stdout().flush().expect("Unable to flush stdout.");

        stdin.read_line(&mut buffer).expect("Unable to read line.");
        let buffer = buffer.trim();

        match buffer {
            "exit" => {
                println!("Exiting...");
                break;
            },
            _ => {
                let mut vm = RegLangVM::new(compile(buffer).program);
                vm.run();
            }
        }
    }
}