mod lex_error;

use atlas_core::{utils::span::{BytePos, Span}, interfaces::lexer::{Lexer, token::Token}, Literal, TokenKind, Keyword, lexer_errors::LexerError};
use std::{iter::Peekable, collections::HashMap, str::Chars};
use internment::Intern;

use crate::map;
use self::lex_error::LexError;

#[derive(Debug, Clone)]
/// Default Lexer and base one for the Atlas77 language
pub struct AtlasLexer<'a> {
    path: &'static str,
    current_pos: BytePos,
    it: Peekable<Chars<'a>>,
    keywords: HashMap<Intern<String>, TokenKind>,
}

impl<'a> Lexer for AtlasLexer<'_> {
    fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn LexerError>> {
        let mut tokens = Vec::new();

        loop {
            let start_pos = self.current_pos;
            let ch = match self.next() {
                None => break,
                Some(c) => c,
            };
            
            if let Some(kind) = self.lex(ch) {
                match kind {
                    TokenKind::EOI => {
                        tokens.push(Token::new(
                            Span {
                                start: start_pos,
                                end: self.current_pos,
                                path: self.path,
                            },
                            TokenKind::EOI
                        ));
                        break
                    },
                    TokenKind::UnknownChar(c) => {
                        let err = LexError::UnknownCharacter {
                            ch: c,
                            code: 0,
                            span: Span {
                                start: start_pos,
                                end: self.current_pos,
                                path: self.path,
                            },
                            recoverable: true
                        };
                        return Err(Box::new(err));
                    },
                    _ => {
                        tokens.push(Token::new(
                            Span {
                                start: start_pos,
                                end: self.current_pos,
                                path: self.path,
                            },
                            kind
                        ));
                    }
                }
                
            }
        }
        return Ok(tokens);
    }
}

impl<'a> AtlasLexer<'a> {
    /// Create a new empty `AtlasLexer`
    /// Is it really how I should do it?
    pub fn new(path: &'static str, contents: &'a str) -> Self {
        let mut a = AtlasLexer {
            path,
            it: contents.chars().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new()
        };
        a.populate_keyword();
        a
    }

    fn next(&mut self) -> Option<char> {
        let next = self.it.next();
        if let Some(ch) = next {
            self.current_pos = self.current_pos.shift(ch);
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    fn either(&mut self, to_match: char, matched: TokenKind, unmatched: TokenKind) -> TokenKind {
        if self.consume_if(|c| c == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.it.peek() {
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
        let mut it = self.it.clone();
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

    fn lex(&mut self, ch: char) -> Option<TokenKind> {
        use TokenKind::*;
        match ch {
            '\n' | '\t' | ' ' | '\r' => {
                if !self.peek().is_none() {
                    let ch = self.next().unwrap();
                    self.lex(ch)
                } else {
                    None
                }
            },
            '(' => Some(LParen),
            ')' => Some(RParen),
            '{' => Some(LBrace),
            '}' => Some(RBrace),
            '[' => Some(LBracket),
            ']' => Some(RBracket),
            '+' => Some(Plus),
            '_' => Some(Underscore),
            '-' => Some(self.either('>', RArrow, Minus)),
            '*' => Some(Star),
            //TODO: Add support for multiline comments
            '/' => {
                if self.consume_if(|c| c == '/') {
                    self.consume_while(|c| c != '\n');
                    if !self.peek().is_none() {
                        let ch = self.next().unwrap();
                        self.lex(ch)
                    } else {
                        None
                    }
                } else {
                    Some(Slash)
                }
            },
            '\\' => {
                //Add support for escaping characters
                Some(Backslash)
            }
            '%' => Some(Percent),
            '^' => Some(Caret),
            '<' => {
                if self.consume_if(|c| c == '=') {
                    Some(LtEq)
                } else {
                    Some(self.either('-', LArrow, LAngle))
                }
            },
            '>' => Some(self.either('=', GtEq, RAngle)),
            '=' => {
                if self.consume_if(|ch| ch == '>') {
                    Some(FatArrow)
                } else {
                    Some(self.either('=', DoubleEq, Eq))
                }
            },
            '&' => Some(Ampersand),
            '|' => Some(Pipe),
            '!' => Some(self.either('=', NEq, Bang)),
            //Logical
            ':' => Some(self.either(':', DoubleColon, Colon)),
            ';' => Some(SemiColon),
            ',' => Some(Comma),
            '.' => Some(self.either('.', DoubleDot, Dot)),
            '@' => Some(At),
            '#' => Some(HashTag),
            '~' => Some(Tilde),
            '?' => Some(Question),
            '$' => Some(Dollar),
            //Identifiers
            ch if ch.is_alphabetic() || ch == '_' => {
                Some(self.identifier(ch).unwrap())
            },
            x if x.is_numeric() => {
                Some(self.number(x).unwrap())
            },
            '"' => {
                let mut string = String::new();
                string.push_str(self.consume_while(|ch| ch != '"').iter().collect::<String>().as_ref());
                self.next().unwrap();
                Some(TokenKind::Literal(atlas_core::Literal::StringLiteral(Intern::new(string))))
            },
            c => Some(UnknownChar(c))
        }
    }

    fn identifier(&mut self, c: char) -> Option<TokenKind> {
        let mut ident = String::new();
        ident.push(c);

        while let Some(&ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.next().unwrap());
            } else {
                break;
            }
        }
        let id = Intern::new(ident.to_owned());

        if let Some(k) = self.keywords.get(&id) {
            Some(k.clone())
        } else {
            Some(TokenKind::Literal(Literal::Identifier(id)))
        }        
    }

    fn number(&mut self, c: char) -> Option<TokenKind> {
        let mut number = String::new();
        number.push(c);

        let num: String = self
            .consume_while(|a| a.is_numeric())
            .into_iter()
            .collect();
        number.push_str(&num);

        if self.peek() == Some(&'.') && self.consume_if_next(|c| c.is_numeric()) {
            number.push('.');

            let num: String = self
                .consume_while(|a| a.is_numeric())
                .into_iter()
                .collect();
            number.push_str(&num);

            Some(TokenKind::Literal(Literal::Float(number.parse::<f64>().unwrap())))
        } else {
            Some(TokenKind::Literal(Literal::Int(number.parse::<i64>().unwrap())))
        }
    }
    
    fn populate_keyword(&mut self) {

        self.keywords = map!{
            Intern::new(String::from("struct")) => TokenKind::Keyword(Keyword::Struct),
            Intern::new(String::from("let")) => TokenKind::Keyword(Keyword::Let),
            Intern::new(String::from("i64")) => TokenKind::Literal(Literal::Identifier(Intern::new(String::from("i64")))),
            Intern::new(String::from("f64")) => TokenKind::Literal(Literal::Identifier(Intern::new(String::from("f64")))),
            Intern::new(String::from("bool")) => TokenKind::Literal(Literal::Identifier(Intern::new(String::from("bool")))),
            Intern::new(String::from("string")) => TokenKind::Literal(Literal::Identifier(Intern::new(String::from("string")))),
            Intern::new(String::from("match")) => TokenKind::Keyword(Keyword::Match),
            Intern::new(String::from("as")) => TokenKind::Keyword(Keyword::As),
            Intern::new(String::from("enum")) => TokenKind::Keyword(Keyword::Enum),
            Intern::new(String::from("do")) => TokenKind::Keyword(Keyword::Do),
            Intern::new(String::from("with")) => TokenKind::Keyword(Keyword::With),
            Intern::new(String::from("or")) => TokenKind::Keyword(Keyword::Or),
            Intern::new(String::from("And")) => TokenKind::Keyword(Keyword::And)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lexer() {
        let mut lexer = AtlasLexer::new("<stdin>", "let x: i64 = 5");
        println!("Lexer: {:?}", lexer);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind(), TokenKind::Keyword(Keyword::Let));
        println!("Tokens: {:?}", tokens);
    }
}