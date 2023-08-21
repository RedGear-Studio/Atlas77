use atlas_misc::span::WithSpan;

use crate::ast::{Ident, Type};
use crate::parser::Parser;
use crate::token::{Token, TokenKind};

pub fn expect_identifier(p: &mut Parser) -> Result<WithSpan<Ident>, ()> {
    let token = p.advance();
    match &token.value {
        Token::Identifier(ident) => Ok(WithSpan::new(ident.clone(), token.span)),
        _ => {
            p.error(format!("Expected {} got {}", TokenKind::Identifier, token.value), token.span, 0, "Not the expected token".to_string());
            Err(())
        },
    }
}

pub fn expect_string(p: &mut Parser) -> Result<WithSpan<String>, ()> {
    let token = p.advance();
    match &token.value {
        Token::String(ident) => Ok(WithSpan::new(ident.clone(), token.span)),
        _ => {
            p.error(format!("Expected {} got {}", TokenKind::String, token.value), token.span, 0, "Not the expected token".to_string());
            Err(())
        },
    }
}

pub fn expect_type(p: &mut Parser) -> Result<WithSpan<Type>, ()> {
    let token = p.advance();
    match &token.value {
        Token::TBool => Ok(WithSpan::new(Type::Bool, token.span)),
        Token::TInt => Ok(WithSpan::new(Type::Int, token.span)),
        Token::TFloat => Ok(WithSpan::new(Type::Float, token.span)),
        Token::TChar => Ok(WithSpan::new(Type::Char, token.span)),
        Token::TVoid => Ok(WithSpan::new(Type::Void, token.span)),
        Token::Identifier(ident) => Ok(WithSpan::new(Type::Custom(ident.clone()), token.span)),
        _ => {
            p.error(format!("Expected {} got {}", TokenKind::TBool, token.value), token.span, 0, "Not the expected token".to_string());
            Err(())
        }
    }
}