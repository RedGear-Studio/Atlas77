use atlas_core::prelude::{LexerState, Span};
use internment::Intern;
use lexer::{AtlasLexer, Token};

pub mod lexer;
pub mod parser;
fn main() {
    let tokens = lex("src/main.atlas").unwrap();
    for token in &tokens {
        println!("{:?}", token);
    }
    parse(tokens);
}

pub fn parse(tokens: Vec<Token>) {
    let mut parser = parser::SimpleParserV1::new();
    parser.with_tokens(tokens);
    let program = parser.parse();
    println!("{:?}", program);
}

pub fn lex(path: &'static str) -> Result<Vec<Token>, ()> {
    //"default()" setup all the systems you asked for, tho you could use "new()" to add them manually
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    let tokens = lex
        .set_path(path)
        .set_source(String::from(
            r#"
let fib: (a: int) -> int = 
    match a 
    | 0 ~> 0,
    | 1 ~> 1,
    \ _ ~> fib(a - 1) + fib(a - 2)

let main: () -> int = do
    print([
        "Hello",
        "World"
    ]);
    let b: List[int] = [1, 2, 3];
    print(b[0]);
    print("How many fib numbers do you want?");
    let a: int = read_int();
    fib(a);
end"#,
        ))
        .tokenize();

    tokens
}

