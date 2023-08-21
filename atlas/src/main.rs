use atlas_misc::{file::FilePath, report::Report};
use atlas_syntax::parse;


use std::{io::{self, Write}, fs};

fn evaluate_input(input: &str, path: &str) -> Result<String, Vec<Report>> {

    let res = parse(input, path)?;

    let mut result = String::new();
    for res in res {
        result.push_str( format!("{:?}", res.value).as_str());
    }

    return Ok(result);
}

fn _repl() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let _result = evaluate_input(input, "stdin");
        //println!("{}", result);
    }
}

fn main() {
    let path = "atlas/main.atl";
    let code = fs::read_to_string(path).unwrap();
    
    let res = evaluate_input(
        &code,
        path
    );
    if res.is_ok() {
        println!("{}", res.unwrap());
    } else {
        println!("{}", res.unwrap_err()[0]);
    }
}