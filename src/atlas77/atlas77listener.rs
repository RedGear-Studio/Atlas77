#![allow(nonstandard_style)]
// Generated from Atlas77.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::atlas77parser::*;

pub trait Atlas77Listener<'input> : ParseTreeListener<'input,Atlas77ParserContextType>{
/**
 * Enter a parse tree produced by {@link Atlas77Parser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#let_statement}.
 * @param ctx the parse tree
 */
fn enter_let_statement(&mut self, _ctx: &Let_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#let_statement}.
 * @param ctx the parse tree
 */
fn exit_let_statement(&mut self, _ctx: &Let_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#return_statement}.
 * @param ctx the parse tree
 */
fn enter_return_statement(&mut self, _ctx: &Return_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#return_statement}.
 * @param ctx the parse tree
 */
fn exit_return_statement(&mut self, _ctx: &Return_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#expression_statement}.
 * @param ctx the parse tree
 */
fn enter_expression_statement(&mut self, _ctx: &Expression_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#expression_statement}.
 * @param ctx the parse tree
 */
fn exit_expression_statement(&mut self, _ctx: &Expression_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#if_statement}.
 * @param ctx the parse tree
 */
fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#if_statement}.
 * @param ctx the parse tree
 */
fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#function_declaration}.
 * @param ctx the parse tree
 */
fn enter_function_declaration(&mut self, _ctx: &Function_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#function_declaration}.
 * @param ctx the parse tree
 */
fn exit_function_declaration(&mut self, _ctx: &Function_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#type}.
 * @param ctx the parse tree
 */
fn enter_type(&mut self, _ctx: &TypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#type}.
 * @param ctx the parse tree
 */
fn exit_type(&mut self, _ctx: &TypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#logical_or}.
 * @param ctx the parse tree
 */
fn enter_logical_or(&mut self, _ctx: &Logical_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#logical_or}.
 * @param ctx the parse tree
 */
fn exit_logical_or(&mut self, _ctx: &Logical_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#logical_and}.
 * @param ctx the parse tree
 */
fn enter_logical_and(&mut self, _ctx: &Logical_andContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#logical_and}.
 * @param ctx the parse tree
 */
fn exit_logical_and(&mut self, _ctx: &Logical_andContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#equality}.
 * @param ctx the parse tree
 */
fn enter_equality(&mut self, _ctx: &EqualityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#equality}.
 * @param ctx the parse tree
 */
fn exit_equality(&mut self, _ctx: &EqualityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#comparison}.
 * @param ctx the parse tree
 */
fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#comparison}.
 * @param ctx the parse tree
 */
fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#addition}.
 * @param ctx the parse tree
 */
fn enter_addition(&mut self, _ctx: &AdditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#addition}.
 * @param ctx the parse tree
 */
fn exit_addition(&mut self, _ctx: &AdditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#multiplication}.
 * @param ctx the parse tree
 */
fn enter_multiplication(&mut self, _ctx: &MultiplicationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#multiplication}.
 * @param ctx the parse tree
 */
fn exit_multiplication(&mut self, _ctx: &MultiplicationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#unary}.
 * @param ctx the parse tree
 */
fn enter_unary(&mut self, _ctx: &UnaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#unary}.
 * @param ctx the parse tree
 */
fn exit_unary(&mut self, _ctx: &UnaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#call}.
 * @param ctx the parse tree
 */
fn enter_call(&mut self, _ctx: &CallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#call}.
 * @param ctx the parse tree
 */
fn exit_call(&mut self, _ctx: &CallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Atlas77Parser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Atlas77Parser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : Atlas77Listener<'input> }


