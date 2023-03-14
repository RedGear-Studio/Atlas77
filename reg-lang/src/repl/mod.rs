use std::io::{self, Write};

use super::*;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}


impl REPL {
    pub fn new() -> Self {
        REPL {
            command_buffer: Vec::new(),
            vm: VM::new(),
        }
    }

    pub fn run_console(&mut self) {
        println!("Reg-Lang REPL (v0.0.1) RedGear-Studio");
        println!("Type .quit or .exit to exit the REPL");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!("Reg-Lang > ");
            io::stdout().flush().expect("ERROR: Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("ERROR: Unable to read line from user");
            let buffer = buffer.trim();
            let mut compiler = RegCompiler::new();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" | ".exit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                },
                _ => {
                    compiler.compile(buffer);
                    let bytecode = compiler.program;
                    self.vm.program = bytecode;
                    self.vm.data = compiler.data;
                    self.vm.run();
                }
            }
        }
    }
}