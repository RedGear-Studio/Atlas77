use atlas_frontend::lexer::*;

pub fn test() {
    let mut lex: AtlasLexer = atlas_frontend::lexer::AtlasLexer::default();
    let _ = lex
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
}