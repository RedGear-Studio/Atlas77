use std::vec;

use pest::iterators::Pair;
use crate::Rule;
use crate::compiler::ast::func::Function;
use crate::compiler::ast::{stmt::Statement, expr::Expression, expr::BinaryOperator,  Program, expr::UnaryOperator, literal::Literal, stmt::ForLoopDirection};
use crate::compiler::ir::data_type::IRDataType;

pub fn generate_ast( program: Pair<Rule>) -> Program {
    let mut ast = Program {
        functions: vec![],
    };
    for programs in program.into_inner() {
        match programs.as_rule() {
            Rule::function => {
                ast.functions.push(make_function(programs));
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    ast
}
fn make_function(function:Pair<Rule>) -> Function {
    let mut inner_pairs = function.into_inner();
    let name = make_identifier(inner_pairs.next().unwrap());
    let mut args = vec![];
    let mut current_identifier = String::new();
    let mut current_type = IRDataType::Boolean;
    let mut i = 0;
    for something in inner_pairs.next().unwrap().into_inner() {
        match something.as_rule() {
            Rule::identifier => current_identifier = make_identifier(something),
            Rule::data_type => current_type = something.as_str().parse::<IRDataType>().unwrap(),
            _ => unreachable!(),
        }
        if i%2 == 1 {
            args.push((current_identifier.clone(), current_type.clone()));
        }
        i += 1;
    }
    let mut body = vec![];
    let mut return_type = IRDataType::Void;
    for insides in inner_pairs.into_iter() {
        match insides.as_rule() {
            Rule::data_type => {
                return_type = insides.as_str().parse::<IRDataType>().unwrap();
            },
            Rule::block => {
                for statement in insides.into_inner().next().into_iter() {
                    body.push(make_statement(statement.into_inner().next().unwrap()));
                }
            }
            _ => unreachable!()
        }
    }
    Function::new(name, args, body, return_type)
}

fn make_statement( statement: Pair<Rule>) -> Statement {
    match statement.as_rule() {
        Rule::print_statement => Statement::PrintStatement(make_expression(statement.into_inner().next().unwrap().into_inner().next().unwrap())),
        Rule::variable_declaration => {
            let mut inner_pairs = statement.into_inner();
            let identifier = make_identifier(inner_pairs.next().unwrap());
            let data_type = inner_pairs.next().unwrap().as_str().parse::<IRDataType>().unwrap();
            let something = inner_pairs.next();
            let value = something.map(|inside| make_expression(inside.into_inner().next().unwrap()));
            Statement::VariableDeclaration {
                identifier,
                data_type,
                value,
            }
        },
        Rule::assignment => {
            let mut inner_pairs = statement.into_inner();
            Statement::Assignment {
                identifier: make_identifier(inner_pairs.next().unwrap()),
                value: make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap()),
            }
        },
        Rule::if_statement => {
            let mut inner_pairs = statement.into_inner();
            let condition = make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap());
            let mut body = vec![];
            for statements in inner_pairs.next().unwrap().into_inner() {
                body.push(make_statement(statements.into_inner().next().unwrap()));
            }
            let mut is_else = false;
            let mut else_body = vec![];
            let something = inner_pairs.next();
            if let Some(value) = something {
                is_else = true;
                for statements in value.into_inner() {
                    else_body.push(make_statement(statements.into_inner().next().unwrap()));
                }
            }
            Statement::IfStatement {
                cond_expr: condition,
                body_expr: body,
                else_expr: if is_else {
                    Some(else_body)
                } else {
                    None
                }
            }
        },
        Rule::while_loop => {
            let mut inner_pairs = statement.into_inner();
            let condition = make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap());
            let mut body = vec![];
            for statements in inner_pairs.next().unwrap().into_inner() {
                body.push(make_statement(statements.into_inner().next().unwrap()));
            }
            Statement::WhileLoop {
                cond_expr: condition,
                body_expr: body,
            }
        },
        Rule::for_loop => {
            let mut inner_pairs = statement.into_inner();
            let identifier = make_identifier(inner_pairs.next().unwrap());
            let expr = make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap());
            let mut step = None;
            let mut direction = ForLoopDirection::Increase;
            let mut body = vec![];
            for something in inner_pairs {
                match something.as_rule() {
                    Rule::step => {
                        step = Some(make_expression(something.into_inner().next().unwrap().into_inner().next().unwrap()));
                    },
                    Rule::direction => {
                        match something.into_inner().next().unwrap().as_rule() {
                            Rule::increase => direction = ForLoopDirection::Increase,
                            Rule::decrease => direction = ForLoopDirection::Decrease,
                            Rule::both => direction = ForLoopDirection::Both,
                            _ => unreachable!()
                        };
                    },
                    Rule::block => {
                        for statements in something.into_inner() {
                            body.push(make_statement(statements.into_inner().next().unwrap()));
                        }
                    },
                    _ =>  unreachable!()
                }
            }
            Statement::ForLoop {
                identifier,
                expr,
                step: if let Some(value) = step {
                    value
                } else {
                    Expression::Literal(Literal::Number(1.0))
                },
                direction,
                body_expr: body,
            }
        },
        _ => unreachable!()
    }
}

fn make_expression( expression: Pair<Rule>) -> Expression {
    match expression.as_rule() {
        Rule::literal => Expression::Literal(make_literal(expression.into_inner().next().unwrap())),
        Rule::identifier => Expression::Identifier(expression.as_span().as_str().to_string()),
        Rule::binary_expression => {
            let mut inner_pairs = expression.into_inner();
            Expression::BinaryOp(
                Box::new(make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap())),
                inner_pairs.next().unwrap().as_str().parse::<BinaryOperator>().unwrap(),
                Box::new(make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap())),
            )
        },
        Rule::unary_expression => {
            let mut inner_pairs = expression.into_inner();
            Expression::UnaryOp(
                inner_pairs.next().unwrap().as_str().parse::<UnaryOperator>().unwrap(),
                Box::new(make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap()))
            )  
        },
        _ => unreachable!(),
    }
}

fn make_identifier( identifier: Pair<Rule>) -> String {
    return identifier.as_span().as_str().to_string();
}
//Literals
fn make_literal( literal: Pair<Rule>) -> Literal {
    match literal.as_rule() {
        Rule::number => Literal::Number(literal.as_span().as_str().to_string().parse::<f64>().unwrap()),
        Rule::string_value => Literal::String(literal.as_span().as_str().to_string()),
        Rule::boolean => Literal::Boolean(literal.as_span().as_str().parse::<bool>().unwrap()),
        _ => unreachable!(),
    }
}
