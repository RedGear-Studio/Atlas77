use compiler::parse;
use runtime::Runtime;
use common::visitor::{Visitor, Expression};

fn main() {
    let res = parse("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\examples\\fib.atlas");
    println!("{:?}", res);
    let runtime = Runtime::new();
    
}