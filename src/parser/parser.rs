use pest::iterators::Pair;
use crate::Rule;
use super::ast::{Statement, Expression, BinaryOperator, DataType, Program, UnaryOperator, Literal, ForLoopDirection};

pub fn generate_ast( program: Pair<Rule>) -> Program {
    let mut ast = Program {
        statements: vec![],
    };
    for programs in program.into_inner() {
        match programs.as_rule() {
            Rule::statement => {
                ast.statements.push(make_statement(programs.into_inner().next().unwrap()));
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    ast
}
fn make_statement( statement: Pair<Rule>) -> Statement {
    match statement.as_rule() {
        Rule::print_statement => Statement::PrintStatement(make_expression(statement.into_inner().next().unwrap().into_inner().next().unwrap())),
        Rule::variable_declaration => {
            let mut inner_pairs = statement.into_inner();
            let identifier = make_identifier(inner_pairs.next().unwrap());
            let data_type = inner_pairs.next().unwrap().as_str().parse::<DataType>().unwrap();
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
                step: if step.is_some() {
                    step.unwrap()
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
