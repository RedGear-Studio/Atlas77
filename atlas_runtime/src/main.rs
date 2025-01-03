use std::{path::PathBuf, time::Instant};

use atlas_memory::vm_data::VMData;
use atlas_runtime::{visitor::Visitor, vm_state::VMState, Runtime};
use atlas_frontend::parse;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author = "Gipson62", version, about = "Programming language made in Rust", long_about = None)]
struct Args {
    /// Run you program at the given <PATH>
    #[arg(short, long, value_name = "PATH")]
    run: String,
}

fn main() {
    let args = Args::parse();

    run(args.run);
}
fn print(state: VMState) -> Result<VMData, ()> {
    println!("Stack Last: {}", state.stack.last().unwrap());
    Ok(VMData::new_unit())
}

fn read_int(state: VMState) -> Result<VMData, ()> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i64>().unwrap();
    Ok(VMData::new_i64(input))
}

pub(crate) fn run(path: String) {
    let mut path_buf = PathBuf::from(path.clone());

    if let Ok(current_dir) = std::env::current_dir() {
        if !path_buf.is_absolute() {
            path_buf = current_dir.join(path_buf);
        }
    } else{
        println!("Failed to get current directory");
    }

    let start = Instant::now();

    let program = parse(path_buf.to_str().unwrap())
        .expect("Failed to open the file");
   
    let mut runtime = Runtime::new();
    runtime.add_extern_fn("print", print);
    runtime.add_extern_fn("read_int", read_int);

    println!("{:?}", runtime.visit(&program));

    let end = Instant::now();
    println!("Elapsed time: {:?}", (end - start)/10000);
}