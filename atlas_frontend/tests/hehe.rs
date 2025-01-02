use atlas_frontend::{lexer::*, parser::Parser};

pub fn test() {
    let mut lex: AtlasLexer = atlas_frontend::lexer::AtlasLexer::default();
    let tokens = lex
        .set_path("0/58/21")
        .set_source(String::from(
            r#"
let fib: (a: i64) -> i64 = 
    match a 
    | 0 ~> 0,
    | 1 ~> 1,
    \ _ ~> fib(a - 1) + fib(a - 2)

let main: () -> i64 = do
    print([
        "Hello",
        "World"
    ]);
    let b: List[i64] = [1, 2, 3];
    print(b[0]);
    print("How many fib numbers do you want?");
    let a: i64 = read_i64();
    fib(a);
end"#,
        ))
        .tokenize();
    let mut parser = atlas_frontend::parser::SimpleParserV1::new();
    parser.with_tokens(tokens.unwrap());
    let ast = parser.parse();
    println!("{:?}", ast);
}