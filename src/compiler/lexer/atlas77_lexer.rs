use super::{tokens::Tokens, position::Position};

pub struct Atlas77Lexer {
    pub tokens: Vec<Tokens>,
    pub pos: Position,
}