use lexer::{AtlasLexer, Token};

pub mod lexer;

pub fn parse(path: &'static str) -> Result<Vec<Token>, ()> {
    //"default()" setup all the systems you asked for, tho you could use "new()" to add them manually
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    lex.set_path(path)
        .set_source(String::from("241.25"))
        .tokenize()
}
