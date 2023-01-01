#![allow(unused)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
pub mod ast;
use pest::Parser;   
use ast::{bin_op::*, expr::*};

use crate::ast::bin_op;

#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
struct RegLangParser;
#[derive(Debug, Clone)]
pub struct RegLangAST {
    //Store the 3 last expressions of the Parser and at the end, make the final BinOp (for now)
    pub ast: Vec<Expr>,
}
impl RegLangAST {
    fn make_bin_op(&mut self) -> BinOp {
        let right = self.ast.pop().unwrap();
        let op = self.ast.pop().unwrap();
        let left = self.ast.pop().unwrap();
        match op {
            Expr::Literal(oper) => {
                match oper.as_str() {
                    "+" => BinOp::new(Box::new(left), Operator::Add, Box::new(right)),
                    "-" => BinOp::new(Box::new(left), Operator::Sub, Box::new(right)),
                    "*" => BinOp::new(Box::new(left), Operator::Mul, Box::new(right)),
                    "/" => BinOp::new(Box::new(left), Operator::Div, Box::new(right)),
                    "%" => BinOp::new(Box::new(left), Operator::Mod, Box::new(right)),
                    "^" => BinOp::new(Box::new(left), Operator::Pow, Box::new(right)),
                    _ => panic!("Not a valid operator")
                }
            },
            _ => panic!("Not a valid operator")
        }
    }
}

pub fn parse(input: &str) -> RegLangAST {
    let mut reg_lang_stack = RegLangAST {
        ast: Vec::new(),
        //ast: Box::new(Expr::Literal("".to_string())),
    };
    let pairs = RegLangParser::parse(Rule::file, input).unwrap_or_else(|e| panic!("{}",  e));
    for pair in pairs.into_iter() {
        match pair.as_rule() {
            Rule::term => {
                let mut counter: usize = 0;
                for term_pair in pair.into_inner() {
                    //Use the stack to store the 3 last values and make the BinOp
                    match term_pair.as_rule() {
                        Rule::int => reg_lang_stack.ast.push(Expr::Integer(term_pair.as_str().parse::<i64>().unwrap().into())),
                        Rule::float => reg_lang_stack.ast.push(Expr::Float(term_pair.as_str().parse::<f64>().unwrap().into())),
                        Rule::add => reg_lang_stack.ast.push(Expr::Literal("+".to_string())),
                        Rule::sub => reg_lang_stack.ast.push(Expr::Literal("-".to_string())),
                        Rule::factor => {
                            let mut counter: usize = 0;
                            for factor_pair in term_pair.into_inner() {
                                match factor_pair.as_rule() {
                                    Rule::int => reg_lang_stack.ast.push(Expr::Integer(factor_pair.as_str().parse::<i64>().unwrap().into())),
                                    Rule::float => reg_lang_stack.ast.push(Expr::Float(factor_pair.as_str().parse::<f64>().unwrap().into())),
                                    Rule::mul => reg_lang_stack.ast.push(Expr::Literal("*".to_string())),
                                    Rule::div => reg_lang_stack.ast.push(Expr::Literal("/".to_string())),
                                    Rule::modulo => reg_lang_stack.ast.push(Expr::Literal("%".to_string())),
                                    _ => println!("Default")
                                }
                                counter += 1;
                                if counter == 3 {
                                    let bin_op = reg_lang_stack.make_bin_op();
                                    reg_lang_stack.ast.push(Expr::BinOpExpr(bin_op));
                                    counter = 1;
                                }
                            }
                            
                        }
                        _ => println!("default")
                    }
                    if reg_lang_stack.ast.len() == 3 {
                        let bin_op = reg_lang_stack.make_bin_op();
                        reg_lang_stack.ast.push(Expr::BinOpExpr(bin_op));
                    }
                }
            }
            Rule::EOI => break,
            _ => println!("Default")
        }
    }
    reg_lang_stack
}
