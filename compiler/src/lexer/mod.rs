mod lex_error;

use atlas_core::{utils::span::{BytePos, Span}, interfaces::{lexer::{Lexer, token::Token}, error::Error}, Literal, TokenKind, lexer_errors::LexerError};
use std::{iter::Peekable, collections::HashMap, str::Chars};
use internment::Intern;

use crate::map;
use self::lex_error::LexError;

#[derive(Debug, Clone)]
/// Default Lexer and base one for the Atlas77 language
pub(crate) struct AtlasLexer<'a> {
    path: &'static str,
    current_pos: BytePos,
    it: Peekable<Chars<'a>>,
    keywords: HashMap<Intern<String>, TokenKind>,
}

impl<'a> Lexer for AtlasLexer<'_> {
    //Guess it'll be better to use this like that: `AtlasLexer::tokenize(path, contents)` and everything lives and dies in it.
    fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn LexerError>> {
        let mut tokens = vec![
            Token::new(Span {
                start: BytePos::default(),
                end: BytePos::default(),
                path: self.path,
            },
            TokenKind::SoI)
        ];

        loop {
            let start_pos = self.current_pos;
            let ch = match self.next() {
                None => break,
                Some(c) => c,
            };

            match self.lex(ch) {
                Ok(kind) => {
                    tokens.push(Token::new(
                        Span {
                            start: start_pos,
                            end: self.current_pos,
                            path: self.path,
                        },
                        kind
                    ));
                    if kind == TokenKind::EoI {
                        break;
                    }
                },
                Err(err) => {
                    println!("Error: {}", err);
                    if !err.recoverable() {
                        break
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
    pub(crate) fn new(path: &'static str, contents: &'a str) -> Self {
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

    fn lex(&mut self, ch: char) -> Result<TokenKind, LexError> {
        use TokenKind::*;
        match ch {
            '\n' | '\t' | ' ' | '\r' => {
                if !self.peek().is_none() {
                    let ch = self.next().unwrap();
                    self.lex(ch)
                } else {
                    Err(LexError::UnexpectedEndOfInput { 
                        span: Span {
                            start: self.current_pos,
                            end: self.current_pos,
                            path: self.path,
                        },
                        recoverable: false,
                        code: 2
                    })
                }
            },
            '(' => Ok(LParen),
            ')' => Ok(RParen),
            '{' => Ok(LBrace),
            '}' => Ok(RBrace),
            '[' => Ok(LBracket),
            ']' => Ok(RBracket),
            '+' => Ok(Plus),
            '_' => Ok(Underscore),
            '-' => Ok(self.either('>', RArrow, Minus)),
            '*' => Ok(Star),
            //TODO: Add support for multiline comments
            '/' => {
                if self.consume_if(|c| c == '/') {
                    self.consume_while(|c| c != '\n');
                    if !self.peek().is_none() {
                        let ch = self.next().unwrap();
                        self.lex(ch)
                    } else {
                        Err(LexError::UnexpectedEndOfInput { 
                            span: Span {
                                start: self.current_pos,
                                end: self.current_pos,
                                path: self.path,
                            },
                            recoverable: false,
                            code: 2
                        })
                    }
                } else {
                    Ok(Slash)
                }
            },
            '\\' => {
                //Add support for escaping characters
                Ok(Backslash)
            }
            '%' => Ok(Percent),
            '^' => Ok(Caret),
            '<' => {
                if self.consume_if(|c| c == '=') {
                    Ok(LtEq)
                } else {
                    Ok(self.either('-', LArrow, LAngle))
                }
            },
            '>' => Ok(self.either('=', GtEq, RAngle)),
            '=' => {
                if self.consume_if(|ch| ch == '>') {
                    Ok(FatArrow)
                } else {
                    Ok(self.either('=', DoubleEq, Eq))
                }
            },
            '&' => Ok(Ampersand),
            '|' => Ok(Pipe),
            '!' => Ok(self.either('=', NEq, Bang)),
            //Logical
            ':' => Ok(self.either(':', DoubleColon, Colon)),
            ';' => Ok(SemiColon),
            ',' => Ok(Comma),
            '.' => Ok(self.either('.', DoubleDot, Dot)),
            '@' => Ok(At),
            '#' => Ok(HashTag),
            '~' => Ok(Tilde),
            '?' => Ok(Question),
            '$' => Ok(Dollar),
            //Identifiers
            ch if ch.is_alphabetic() || ch == '_' => {
                Ok(self.identifier(ch).unwrap())
            },
            x if x.is_numeric() => {
                Ok(self.number(x).unwrap())
            },
            '"' => {
                let mut string = String::new();
                string.push_str(self.consume_while(|ch| ch != '"').iter().collect::<String>().as_ref());
                self.next().unwrap();
                Ok(TokenKind::Literal(atlas_core::Literal::StringLiteral(Intern::new(string))))
            },
            c => Err(LexError::UnknownCharacter {
                ch: c,
                code: 0,
                span: Span {
                    start: self.current_pos,
                    end: self.current_pos.shift(c),
                    path: self.path,
                },
                recoverable: true
            })
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
            Intern::new(String::from("match")) => TokenKind::Keyword(Intern::new(String::from("match"))),
            Intern::new(String::from("as")) => TokenKind::Keyword(Intern::new(String::from("as"))),
            Intern::new(String::from("enum")) => TokenKind::Keyword(Intern::new(String::from("enum"))),
            Intern::new(String::from("do")) => TokenKind::Keyword(Intern::new(String::from("do"))),
            Intern::new(String::from("with")) => TokenKind::Keyword(Intern::new(String::from("with"))),
            Intern::new(String::from("or")) => TokenKind::Keyword(Intern::new(String::from("or"))),
            Intern::new(String::from("And")) => TokenKind::Keyword(Intern::new(String::from("and"))),
            Intern::new(String::from("struct")) => TokenKind::Keyword(Intern::new(String::from("struct"))),
            Intern::new(String::from("let")) => TokenKind::Keyword(Intern::new(String::from("let"))),
            Intern::new(String::from("fn")) => TokenKind::Keyword(Intern::new(String::from("fn"))),
            Intern::new(String::from("in")) => TokenKind::Keyword(Intern::new(String::from("in")))

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
        assert_eq!(tokens[1].kind(), TokenKind::Keyword(Intern::new(String::from("let"))));
        println!("Tokens: {:?}", tokens);
    }
}