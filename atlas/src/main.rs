pub mod simple_parser;
pub mod simple_visitor;
use atlas_core::{Lexer, Token, utils::span::Spanned};
use compiler::lexer::AtlasLexer;

fn main() {
    let path: &'static str = "C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\examples\\test.atlas";
    let contents = std::fs::read_to_string(path).unwrap();
    let mut lexer = AtlasLexer::new(path, &contents);
    let tokens = lexer.tokenize().unwrap();
    tokens.iter().for_each(|t| print!("{}", t.to_string()));
}

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