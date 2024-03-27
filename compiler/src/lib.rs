use atlas_core::{Lexer, Token, utils::span::Spanned};
use lexer::AtlasLexer;

pub mod lexer;
pub mod parser;

pub fn compile(path: &'static str) {
    let contents = std::fs::read_to_string(path).unwrap();
    let tokens = AtlasLexer::tokenize(path, &contents).unwrap();
    tokens.iter().for_each(|t| print!("{:?}", t));
}

//I forgot to implement `Display` in `Token` lmao
impl Hehe for Token {
    fn to_string(&self) -> String {
        format!("[\"{}\"-{}:{}]", self.kind(), self.span().start(), self.span().end())
    }
}

pub trait Hehe {
    fn to_string(&self) -> String;
}

#[macro_export]
macro_rules! map {
    (&key: ty, &val: ty) => {
        {
            let map: HashMap<&key, &val> = HashMap::new();
            map
        }
    };
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}

#[macro_export]
macro_rules! exit_err {
    ($($arg:tt)*) => {
        {
            println!($($arg)*);
            std::process::exit(1);
        }
    }
}