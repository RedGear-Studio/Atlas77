use atlas_misc::span::{WithSpan, Span};

use crate::{parser::Parser, ast::{Statement, Expression}, token::TokenKind};


pub fn parse(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    parse_stmt(it)
}

fn parse_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    match it.peek() {
        TokenKind::Print => parse_print_stmt(it),
        TokenKind::If => parse_if_stmt(it),
        TokenKind::LeftBrace => parse_block_stmt(it),
        TokenKind::Return => parse_return_stmt(it),
        TokenKind::Include => parse_include_stmt(it),
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
    todo!()
}

fn parse_block_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    todo!()
}

fn parse_return_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    todo!()
}

fn parse_include_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    todo!()
}

fn parse_expr_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    todo!()
}

fn parse_expr(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    super::expr_parser::parse(it)
}
