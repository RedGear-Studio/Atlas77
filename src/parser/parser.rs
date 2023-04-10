use pest::iterators::Pair;
use crate::Rule;
use super::ast::{Statement, Expression, BinaryOperator, DataType, Program, UnaryOperator, Literal};

pub fn generate_ast( program: Pair<Rule>) -> Program {
    let mut ast = Program {
        statements: vec![],
    };
    for programs in program.into_inner() {
        match programs.as_rule() {
            Rule::statement => {
                ast.statements.push(make_statement(programs.into_inner().next().unwrap()));
            },
            _ => unreachable!(),
        }
    }
    return ast;
}
fn make_statement( statement: Pair<Rule>) -> Statement {
    match statement.as_rule() {
        Rule::print_statement => {
            return Statement::PrintStatement(make_expression(statement.into_inner().next().unwrap().into_inner().next().unwrap()));
        },
        Rule::variable_declaration => {
            let mut inner_pairs = statement.into_inner();
            let identifier = make_identifier(inner_pairs.next().unwrap());
            let data_type = inner_pairs.next().unwrap().as_str().parse::<DataType>().unwrap();
            let something = inner_pairs.next();
            let value = if something.is_some() {
                Some(make_expression(something.unwrap().into_inner().next().unwrap()))
            } else {
                None
            };
            return Statement::VariableDeclaration {
                identifier,
                data_type,
                value,
            };
        },
        Rule::assignment => {
            let mut inner_pairs = statement.into_inner();
            return Statement::Assignment {
                identifier: make_identifier(inner_pairs.next().unwrap()),
                value: make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap()),
            };
        },
        Rule::if_statement => {
            let mut inner_pairs = statement.into_inner();
            let condition = make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap());
            let mut body = vec![];
            for statements in inner_pairs.next().unwrap().into_inner() {
                body.push(make_statement(statements));
            }
            let mut is_else = false;
            let mut else_body = vec![];
            let something = inner_pairs.next();
            if something.is_some() {
                is_else = true;
                for statements in something.unwrap().into_inner() {
                    else_body.push(make_statement(statements));
                }
            }
            return Statement::IfStatement {
                cond_expr: condition,
                body_expr: body,
                else_expr: if is_else {
                    Some(else_body)
                } else {
                    None
                }
            }
        }
        _ => unreachable!()
    }
}

fn make_expression( expression: Pair<Rule>) -> Expression {
    match expression.as_rule() {
        Rule::literal => {
            return Expression::Literal(make_literal(expression.into_inner().next().unwrap()));
        },
        Rule::identifier => {
            return Expression::Identifier(expression.as_span().as_str().to_string());
        },
        Rule::binary_expression => {
            let mut inner_pairs = expression.into_inner();
            return Expression::BinaryOp(
                Box::new(make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap())),
                inner_pairs.next().unwrap().as_str().parse::<BinaryOperator>().unwrap(),
                Box::new(make_expression(inner_pairs.next().unwrap().into_inner().next().unwrap())),
            )
        },
        Rule::unary_expression => {
            let mut inner_pairs = expression.into_inner();
            return Expression::UnaryOp(
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
        Rule::number => {
            return Literal::Number(literal.as_span().as_str().to_string().parse::<f64>().unwrap());
        },
        Rule::string_value => {
            return Literal::String(literal.as_span().as_str().to_string());
        },
        Rule::boolean => {
            return Literal::Boolean(literal.as_span().as_str().parse::<bool>().unwrap());
        }
        _ => unreachable!(),
    }
}
