use std::{collections::HashMap, fmt::Display, iter::Peekable, str::Chars};
use common::{map, span::BytePos};
use token::{Token, TokenKind, Keyword};
pub mod token;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct StringID(u32);
impl Display for StringID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}", self.0)
    }
}

pub type StringMap = HashMap<String, StringID>;


pub fn lex(content: &str) -> Vec<Token> {
    let l = Lexer::new(StringMap::new(), content.chars().peekable());
    unimplemented!("lexer not yet implemented");
}

pub(crate) struct Lexer<'a> {
    string_map: StringMap,
    source: Peekable<Chars<'a>>,
    keywords: HashMap<String, TokenKind>,
    current_pos: BytePos,
}

impl<'a> Lexer<'a> {
    pub fn new(string_map: StringMap, chars: Peekable<Chars<'a>>) -> Self {
        let mut l = Self {
            string_map,
            source: chars,
            keywords: map!{},
            current_pos: BytePos::default(),
        };
        l.populate_keywords();
        l
    }

    fn tokenize(&mut self) -> Vec<Token> {
        loop {
            let start_pos = self.current_pos;
        }
        todo!("not yet implemented")
    }

    fn lex(&mut self, ch: char) -> Result<TokenKind, String> {
        match ch {
            _ => Err(format!("Unexpected character: {}", ch)),
        }
    }

    #[inline]
    fn next(&mut self) -> Option<char> {
        let next = self.source.next();
        if let Some(ch) = next {
            self.current_pos = self.current_pos.shift(ch);
        }
        next
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn either(&mut self, to_match: char, matched: TokenKind, unmatched: TokenKind) -> TokenKind {
        if self.consume_if(|ch| ch == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.source.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_if_next<F>(&mut self, f: F) -> bool 
    where
        F: Fn(char) -> bool
    {
        //This is quite inefficient... You clone the whole `Peekable<IntoIter<char>>` which is expensive
        let mut it = self.source.clone();
        match it.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = it.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if f(ch) {
                self.next().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

    fn populate_keywords(&mut self) {
        self.keywords = map!{
            String::from("let") => TokenKind::Keyword(Keyword::Let),
            String::from("if") => TokenKind::Keyword(Keyword::If),
            String::from("then") => TokenKind::Keyword(Keyword::Then),
            String::from("else") => TokenKind::Keyword(Keyword::Else),
            String::from("struct") => TokenKind::Keyword(Keyword::Struct),
            String::from("enum") => TokenKind::Keyword(Keyword::Enum),
            String::from("do") => TokenKind::Keyword(Keyword::Do),
            String::from("end") => TokenKind::Keyword(Keyword::End),
            String::from("match") => TokenKind::Keyword(Keyword::Match),
            String::from("with") => TokenKind::Keyword(Keyword::With),
            String::from("or") => TokenKind::Keyword(Keyword::Or),
            String::from("and") => TokenKind::Keyword(Keyword::And),
            String::from("as") => TokenKind::Keyword(Keyword::As)
        };
    }
}