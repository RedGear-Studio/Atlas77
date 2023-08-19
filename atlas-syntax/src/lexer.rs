use std::{iter::Peekable, str::Chars, collections::HashMap};
use atlas_misc::span::*;

use crate::token::TokenKind;

pub struct Lexer<'a> {
    current_char: Peekable<Chars<'a>>,
    current_pos: BytePos,
    file_name: &'a str, //No need to always copy paste it everywhere
    keywords: HashMap<&'a str, TokenKind>,
}