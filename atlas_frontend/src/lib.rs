use atlas_core::prelude::{LexerState, Span};
use internment::Intern;
use lexer::{AtlasLexer, Token};

pub mod lexer;
pub mod parser;

pub fn parse(path: &'static str) -> Result<Vec<Token>, ()> {
    //"default()" setup all the systems you asked for, tho you could use "new()" to add them manually
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    let tokens = lex
        .set_path(path)
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

    tokens
}

#[cfg(test)]
mod test {
    use crate::lexer::*;
    #[test]
    fn hehe() {
        
    }
}
