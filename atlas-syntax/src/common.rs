use atlas_misc::span::WithSpan;

use crate::{ast::{expr::Identifier, core::CoreType}, parser::Parser, token::Token};

pub fn expect_identifier(p: &mut Parser) -> Result<WithSpan<Identifier>, String> {
    let token = p.next().unwrap();
    match &token.value {
        Token::Ident(ident) => Ok(WithSpan::new(ident.clone(), token.span)),
        _ => {
            p.error(format!("Expected Identifier, got {:?}", token.value), token.span, 0, "Not the expected token".to_string());
            Err(format!("Expected Identifier, got {:?}", token.value))
        },
    }
}

pub fn expect_type(p: &mut Parser) -> Result<WithSpan<CoreType>, String> {
    let token = p.next().unwrap();
    match &token.value {
        Token::KwInt => Ok(WithSpan::new(CoreType::CTInt, token.span)),
        Token::KwFloat => Ok(WithSpan::new(CoreType::CTFloat, token.span)),
        Token::KwChar => Ok(WithSpan::new(CoreType::CTChar, token.span)),
        Token::KwBool => Ok(WithSpan::new(CoreType::CTBool, token.span)),
        Token::KwString => Ok(WithSpan::new(CoreType::CTString, token.span)),
        _ => {
            p.error(format!("Expected Type, got {:?}", token.value), token.span, 0, "Not the expected token".to_string());
            Err(format!("Expected Type, got {:?}", token.value))
        }
    }
}
