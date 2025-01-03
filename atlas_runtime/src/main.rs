use atlas_runtime::{visitor::Visitor, Runtime};
use atlas_frontend::parse;

fn main() {
    let res = parse();
    let mut runtime = Runtime::new();
    match res {
        Ok(ast) => {
            println!("{:?}", runtime.visit(&ast));
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}