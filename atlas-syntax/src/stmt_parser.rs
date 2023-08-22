use atlas_misc::span::{WithSpan, Span};

use crate::{parser::Parser, ast::{Statement, Expression}, token::TokenKind, common::{expect_identifier, expect_type}};


pub fn parse(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    parse_stmt(it)
}

fn parse_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    match it.peek() {
        TokenKind::Print => parse_print_stmt(it),
        TokenKind::If => parse_if_stmt(it),
        TokenKind::LeftBrace => parse_block_stmt(it),
        TokenKind::Return => parse_return_stmt(it),
        TokenKind::Let => parse_let_stmt(it),
        _ => parse_expr_stmt(it),
    }
}

fn parse_print_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    let start_tok = it.expect(TokenKind::Print)?;
    let expr = parse_expr(it)?;
    let end_tok = it.expect(TokenKind::Semicolon)?;
    Ok(WithSpan::new(Statement::Print(expr), Span::union_span(start_tok.into(), end_tok.into())))
}

fn parse_if_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    let start_tok = it.expect(TokenKind::If)?;
    let cond = parse_expr(it)?;
    let body = parse_block_stmt(it)?;
    let else_body = if it.check(TokenKind::Else) {
        Some(parse_block_stmt(it)?)
    } else {
        None
    };
    if else_body.is_some() {
        match else_body.clone().unwrap().value {
            Statement::Block(_) => {
                return Ok(WithSpan::new(Statement::If {
                    cond,
                    body: Box::new(body),
                    else_: Some(Box::new(else_body.clone().unwrap()))
                }, Span::union_span(start_tok.into(), else_body.unwrap().span.into())))
            },
            _ => Err(())
        }
    } else {
        return Ok(WithSpan::new(Statement::If {
            cond,
            body: Box::new(body.clone()),
            else_: None
        }, Span::union_span(start_tok.into(), body.span.into())))
    }
}

fn parse_block_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    let start_tok = it.expect(TokenKind::LeftBrace)?;
    let mut stmts = Vec::new();
    while !it.check(TokenKind::RightBrace) {
        stmts.push(parse_stmt(it)?);
    }
    let end_tok = it.expect(TokenKind::RightBrace)?;
    Ok(WithSpan::new(Statement::Block(stmts), Span::union_span(start_tok.into(), end_tok.into())))
}

fn parse_return_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    let start_tok = it.expect(TokenKind::Return)?;
    let expr = if it.check(TokenKind::Semicolon) {
        None
    } else {
        Some(parse_expr(it)?)
    };
    let end_tok = it.expect(TokenKind::Semicolon)?;
    Ok(WithSpan::new(Statement::Return(expr), Span::union_span(start_tok.into(), end_tok.into())))
}

fn parse_let_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    let start_tok = it.expect(TokenKind::Let)?;
    let name = expect_identifier(it)?;
    let mut t = None;
    if it.check(TokenKind::Colon) {
        it.expect(TokenKind::Colon)?;
        t = Some(expect_type(it)?);
    }
    let mut expr = None;
    if it.check(TokenKind::Equal) {
        it.expect(TokenKind::Equal)?;
        expr = Some(parse_expr(it)?);
        let end_tok = it.expect(TokenKind::Semicolon)?;
        Ok(WithSpan::new(Statement::VarDecl{
            ident: name, 
            type_: t,
            expr
        },
        Span::union_span(start_tok.into(), end_tok.into())))
    } else {
        let end_tok = it.expect(TokenKind::Semicolon)?;
        Ok(WithSpan::new(Statement::VarDecl{
            ident: name, 
            type_: t,
            expr
        }, Span::union_span(start_tok.into(), end_tok.into())))
    }
}

fn parse_expr_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    let expr = parse_expr(it)?;
    let end_tok = it.expect(TokenKind::Semicolon)?;
    Ok(WithSpan::new(Statement::Expr{ expr: expr.clone() }, Span::union_span(expr.span.into(), end_tok.into())))
}

fn parse_expr(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    super::expr_parser::parse(it)
}
