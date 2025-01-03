use atlas_runtime::{visitor::Visitor, SimpleVisitorV1};
use atlas_frontend::parse;

fn main() {
    let res = parse();
    let mut runtime = SimpleVisitorV1::new();
    match res {
        Ok(ast) => {
            println!("{:?}", runtime.visit(&ast));
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}