use compiler::lexer::atlas77_lexer::Atlas77Lexer;

pub mod compiler;
pub mod tree_walker;

use std::io::{self, Write};

fn evaluate_input(input: &str) -> String {
    let mut lexer = Atlas77Lexer::new(input.to_string(), "stdin".to_string());
    let tokens = lexer.make_tokens();

    let mut result = String::new();
    for token in tokens {
        result.push_str(format!("{}", token).as_str());
    }
    
    return result;
}

fn repl() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let result = evaluate_input(input);
        println!("{}", result);
    }
}

fn main() {
    repl();
}