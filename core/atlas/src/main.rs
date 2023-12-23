use compiler::parse;
use runtime::Runtime;
use common::visitor::{Visitor, Expression};

fn main() {
    /*let res = parse("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\examples\\fib.atlas");
    println!("{:?}", res);
    let runtime = Runtime::new();
    println!("{:?}", runtime);*/
    //let hehe = runtime.evaluate(res as Vec<&dyn Expression>);
    use std::time::Instant;
    let now = Instant::now();
    println!("Result: {}", ackermann(10, 5));
    println!("Time: {}ms", now.elapsed().as_millis());
}
fn fib(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fac(n: i128) -> i128 {
    if n <= 1 {
        1
    } else {
        n * fac(n - 1)
    }
}

fn ackermann(m: i32, n: i32) -> i32 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}