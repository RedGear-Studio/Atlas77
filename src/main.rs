use compiler::lexer::atlas77_lexer::Atlas77Lexer;

pub mod compiler;
pub mod tree_walker;

fn main() {
    //Read from console
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut lexer = Atlas77Lexer::new(buf, "stdin".to_string());
    let tokens = lexer.make_tokens();

    for token in tokens {
        println!("{}", token);
    }
}