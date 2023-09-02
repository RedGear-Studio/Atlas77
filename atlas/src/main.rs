use atlas_misc::report::Report;
use atlas_syntax::compile;

use clap::Command;

use std::fs;

fn evaluate_input(input: &str, path: &str) -> Result<String, Vec<Report>> {

    let res = compile(input, path);

    let mut result = String::new();
    for res in res {
        result.push_str( format!("[{:?}] ", res).as_str());
    }

    return Ok(result);
}

/*fn _repl() {
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
}*/

fn cli() -> Command {
    Command::new("atlas")
        .about("Compiler for Atlas")
        .version("0.0.1")
        .subcommand(
            Command::new("build")
                .about("Builds the project")
                .arg(
                    clap::Arg::new("path")
                        .short('p')
                        .long("path")
                        .value_name("PATH")
                        .help("Sets the path to the project")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("test")
                .about("Compiles the test/example project")
        )
}

fn main() {
    let args = cli().get_matches();

    match args.subcommand() {
        Some(("build", p)) => {
            let path = p.get_one::<String>("path").map(|s| s.as_str()).unwrap();
            let code = fs::read_to_string(path).unwrap();
            
            let res = evaluate_input(
                &code,
                path
            );
            if res.is_ok() {
                println!("{}", res.unwrap());
            } else {
                for report in res.unwrap_err() {
                    println!("{}", report);
                }
            }
        },
        Some(("test", _)) => {
            let path = "/home/gipson62/Atlas77/atlas/main.at";
            let code = fs::read_to_string(path).unwrap();
            
            let res = evaluate_input(
                &code,
                path
            );
            if res.is_ok() {
                println!("{}", res.unwrap());
            } else {
                for report in res.unwrap_err() {
                    println!("{}", report);
                }
            }
        }
        _ => {
            
        }
    }
}