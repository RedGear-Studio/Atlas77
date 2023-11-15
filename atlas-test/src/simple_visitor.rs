use std::collections::HashMap;
use std::f32::consts::E;

use atlas_core::prelude::visitor::{Visitor, Program};
use atlas_core::ast::*;
use atlas_core::prelude::visitor::value::Value;

type VarMap = HashMap<String, Value>;
type Stack = Vec<Value>;

pub struct SimpleVisitorV1 {
    varmap: VarMap,
    stack: Stack,
}

impl SimpleVisitorV1 {
    pub fn new() -> Self {
        SimpleVisitorV1 {
            varmap: HashMap::new(), 
            stack: Vec::new()
        }
    }
}

impl Visitor for SimpleVisitorV1 {
    fn visit(&mut self, program: &Program) -> Value {
        for expression in program {
            if let Expression::VariableDeclaration(v) = *expression.value.clone() {
                self.visit_variable_declaration(&v);
            }
        }
        if let Some(f) = self.varmap.get("main") {
            match f {
                Value::FunctionBody(f) => {
                    return self.visit_function_expression(&f.clone());
                }
                _ => unimplemented!("Main function is not a function body")
            }
        }
        eprintln!("Runtime Error: Could not find main function");
        Value::Undefined
    }

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
            BinaryOperator::OpEq => Value::Bool(left == right),
            BinaryOperator::OpNe => Value::Bool(left != right),
            BinaryOperator::OpLt => left.cmp_lt(right),
            BinaryOperator::OpLe => left.cmp_le(right),
            BinaryOperator::OpGt => left.cmp_ge(right),
            BinaryOperator::OpGe => left.cmp_gt(right),

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
            Expression::VariableDeclaration(v) => {
                self.visit_variable_declaration(v)
            }
            Expression::FunctionCall(func) => {
                self.visit_function_call(func)
            }
            Expression::DoExpression(d) => {
                self.visit_do_expression(d)
            }
            _ => unimplemented!("Expression not implemented\n\t{}", expression)
        }
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) -> Value {
        for arg in &function_call.args {
            let val = self.visit_expression(&arg.value);
            self.stack.push(val);
        }
        if let Some(f) = self.varmap.get(&function_call.name) {
            match f {
                Value::FunctionBody(f) => {
                    return self.visit_function_expression(&f.clone());
                }
                _ => unimplemented!("Main function is not a function body")
            }
        } else {
            unreachable!("Function {} not found", function_call.name)
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

    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> Value {
        println!("{}", variable_declaration);
        match &variable_declaration.value {
            Some(v) => {
                let val = *v.value.clone();
                match val {
                    Expression::FunctionExpression(f) => {
                        if let Some(v) = self.varmap.get_mut(&variable_declaration.name) {
                            *v = Value::FunctionBody(f);
                            eprintln!("Variable {} already declared", variable_declaration.name);
                        } else {
                            self.varmap.insert(variable_declaration.name.clone(), Value::FunctionBody(f));
                        }
                    },
                    _ => {
                        let value = self.visit_expression(&variable_declaration.value.clone().unwrap().value);
                        if self.varmap.get(&variable_declaration.name).is_some() {
                            eprintln!("Variable {} already declared", variable_declaration.name);
                            let yo = self.varmap.get_mut(&variable_declaration.name).unwrap();
                            *yo = value;
                        } else {
                            self.varmap.insert(variable_declaration.name.clone(), value);
                        }
                    }
                }
            }
            None => {
                if let Some(v) = self.varmap.get_mut(&variable_declaration.name) {
                    *v = Value::Undefined;
                }
            }
        }
        
        Value::Undefined
    }

    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value {
        let condition = self.visit_expression(&if_else_node.condition.value);
        match condition {
            Value::Integer(i) => {
                if i != 0 {
                    self.visit_expression(&if_else_node.if_body.value)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap().value)
                }
            },
            Value::Bool(b) => {
                if b {
                    self.visit_expression(&if_else_node.if_body.value)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap().value)
                }
            }
            _ => {
                eprintln!("Unsupported condition: {}", if_else_node.condition.value);
                Value::Undefined
            }
        }
    }

    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> Value {
        let mut args = Vec::new();
        for _ in &function_expression.args {
            args.push(self.stack.pop().unwrap());
        }

        for arg in &function_expression.args {
            self.varmap.insert(arg.0.clone(), args.pop().unwrap());
        }

        self.visit_expression(&function_expression.body.value)
    }

    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> Value {
        let mut last_evaluated_expr = Value::Undefined;
        for expression in &do_expression.body {
            last_evaluated_expr = self.visit_expression(&expression.value);
        }
        last_evaluated_expr
    }
}