use atlas_core::prelude::{LexerState, Span};
use internment::Intern;
use lexer::{AtlasLexer, Token};

pub mod lexer;

pub fn parse(path: &'static str) -> Result<Vec<Token>, ()> {
    //"default()" setup all the systems you asked for, tho you could use "new()" to add them manually
    let mut lex: AtlasLexer = lexer::AtlasLexer::default();
    lex.set_path(path)
        .set_source(String::from(
            r#"
        import std::IO;

        interface Prototype {
            public:
                function clone(&self) -> Self;
        }
        class Person extends Prototype {
            public:
                string first_name; 
                string name;
                
            private:
                u32 iq;

            public:
                function get_full_name(&self) -> string {
                    return self.first_name + self.name;
                }
                function clone(&self) -> Self {
                    Person {
                        first_name: self.first_name,
                        name: self.name,
                        iq: self.iq
                    }
                }
        }
        struct Position {
            x: f32,
            y: f32,
            z: f32,
        }

        function main() {
            print("Hello World!");
        }"#,
        ))
        .add_system(identifier_system)
        .add_system(string_system)
        .tokenize()
}

fn string_system(c: char, state: &mut LexerState) -> Option<Token> {
    let start = state.current_pos;
    let mut s = String::new();
    if c == '"' {
        println!("string in the making");
        state.next();
        loop {
            if let Some(ch) = state.peek() {
                if *ch == '"' {
                    state.next();
                    break;
                }
                s.push(*ch);
                state.next();
            }
        }
        return Some(Token::new(
            Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            lexer::TokenKind::Literal(lexer::Literal::StringLiteral(Intern::new(s))),
        ));
    } else {
        None
    }
}

fn identifier_system(c: char, state: &mut LexerState) -> Option<Token> {
    let start = state.current_pos;
    let mut s = String::new();
    if c.is_alphabetic() || c == '_' {
        s.push(c);
        state.next();
        loop {
            if let Some(c) = state.peek() {
                if c.is_alphanumeric() || *c == '_' {
                    s.push(*c);
                    state.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return Some(Token::new(
            Span {
                start,
                end: state.current_pos,
                path: state.path,
            },
            lexer::TokenKind::Literal(lexer::Literal::Identifier(Intern::new(s))),
        ));
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn hehe() {
        let res = parse("0/85/9").unwrap();
        println!("{:?}", res);
    }
}
