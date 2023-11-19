use std::{iter::Peekable, vec::IntoIter, collections::HashMap};

use atlas_lexer_token::{Token, TokenKind, Literal, Keyword, PrimitiveType};
use atlas_span::{Span, BytePos, LineInformation};

pub struct Lexer {
    path: &'static str,
    source: Peekable<IntoIter<char>>,
    current_pos: BytePos,
    keywords: HashMap<String, TokenKind>
}

impl Lexer {
    pub fn new_with_path(path: &'static str) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        Lexer {
            path,
            source: content.chars().collect::<Vec<_>>().into_iter().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new()
        }
    }

    pub fn new_with_content(content: &str) -> Self {
        Lexer {
            //This may induce errors
            path: "<stdin>",
            source: content.chars().collect::<Vec<_>>().into_iter().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new()
        }
    }
}