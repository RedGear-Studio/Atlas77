use std::collections::HashMap;

use atlas_core::prelude::visitor::Visitor;
use atlas_core::ast::*;
use atlas_core::prelude::visitor::value::Value;

pub struct SimpleVisitorV1 {
    varmap: HashMap<String, Value>
}

impl SimpleVisitorV1 {
    pub fn new() -> Self {
        SimpleVisitorV1 {
            varmap: HashMap::new()
        }
    }

    pub fn visit(&mut self, expression: &Expression) -> Value {
        self.visit_expression(expression)
    }
}

impl Visitor for SimpleVisitorV1 {
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> Value{
        let left = self.visit_expression(&expression.left.value);
        let right = self.visit_expression(&expression.right.value);
        match expression.operator {
            BinaryOperator::OpAdd => left + right,
            BinaryOperator::OpSub => left - right,
            BinaryOperator::OpMul => left * right,
            BinaryOperator::OpDiv => left / right,
            BinaryOperator::OpMod => left.modulo(right),
            BinaryOperator::OpPow => left.power(right),
            
            _ => unimplemented!("Binary operator not implemented")
        }
        
    }
    fn visit_expression(&mut self, expression: &Expression)  -> Value {
        match expression {
            Expression::BinaryExpression(bin) => {
                self.visit_binary_expression(bin)
            }
            Expression::UnaryExpression(un) => {
                self.visit_unary_expression(un)
            }
            Expression::Literal(l) => {
                match l {
                    Literal::Integer(i) => {
                        Value::Integer(*i)
                    }
                    Literal::Float(f) => {
                        Value::Float(*f)
                    },
                    _ => unimplemented!()
                }
            }
            Expression::Identifier(i) => {
                self.visit_identifier(i)
            }
            Expression::IfElseNode(i) => {
                self.visit_if_else_node(i)
            }
        }
    }
    fn visit_identifier(&mut self, identifier: &IdentifierNode)  -> Value {
        if let Some(f) = self.varmap.get(&identifier.name) {
            f.clone()
        } else {
            unreachable!("Variable {} not found", identifier.name)
        }
    }
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> Value {
        match &expression.operator {
            Some(op) => {
                match op {
                    UnaryOperator::OpAdd => {
                        self.visit_expression(&expression.expression.value)
                    },
                    UnaryOperator::OpSub => {
                        match self.visit_expression(&expression.expression.value) {
                            Value::Integer(i) => Value::Integer(-i),
                            Value::Float(f) => Value::Float(-f),
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
            None => {
                self.visit_expression(&expression.expression.value)
            }
        }
    }
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Expression(e) => {
                self.visit_expression(&e);
            }
            Statement::VariableDeclaration(v) => {
                self.visit_variable_declaration(v)
            }
        }
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) {
        let value = self.visit_expression(&variable_declaration.value.clone().unwrap().value);
        if self.varmap.get(&variable_declaration.name).is_some() {
            eprintln!("Variable {} already declared", variable_declaration.name);
            let yo = self.varmap.get_mut(&variable_declaration.name).unwrap();
            *yo = value;
        } else {
            self.varmap.insert(variable_declaration.name.clone(), value);
        }
    }
    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value {
        todo!("visit_if_else_node")
    }
}