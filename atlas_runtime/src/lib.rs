pub mod visitor;
pub mod node;
pub mod value;
pub mod vm_state;

use std::collections::HashMap;

use crate::visitor::{Visitor, Program};
use crate::value::Value;
use atlas_frontend::parser::ast::*;
use atlas_memory::object_map::{Memory, Object};
use atlas_memory::vm_data::{RawVMData, VMData};
use internment::Intern;

type VarMap = HashMap<Intern<String>, VMData>;
type Stack = Vec<VMData>;
pub type CallBack = fn(vm_state::VMState) -> Result<VMData, ()>;


pub struct Runtime {
    pub varmap: Vec<VarMap>, //usize is the parent scope
    pub stack: Stack,
    pub func_map: Vec<(String, FunctionExpression)>,
    pub extern_fn: Vec<CallBack>,
    pub consts: HashMap<Intern<String>, VMData>,
    pub object_map: Memory,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            varmap: Vec::new(),
            stack: Vec::new(),
            func_map: Vec::new(),
            extern_fn: Vec::new(),
            consts: {
                let mut h = HashMap::new();
                h.insert(Intern::new("pi".to_string()), VMData::new_f64(std::f64::consts::PI));
                h.insert(Intern::new("e".to_string()), VMData::new_f64(std::f64::consts::E));
                h
            },
            object_map: Memory::new(4096),
        }
    }
    pub fn add_extern_fn(&mut self, f: CallBack) {
        self.extern_fn.push(f);
    }

    fn find_variable(&self, name: Intern<String>) -> Option<&VMData> {
        if let Some(v) = self.consts.get(&Intern::new(name.to_string())) {
            return Some(v);
        }
        if let Some(v) = self.varmap.last().unwrap().get(&name) {
            Some(v)
        } else {
            if let Some(v) = self.varmap[0].get(&name) {
                Some(v)
            } else {
                None
            }
        }
    }
}

impl Visitor for Runtime {
    fn visit(&mut self, program: &Program) -> VMData {
        self.varmap.push(HashMap::new());
        for expression in program {
            if let Expression::VariableDeclaration(v) = *expression.clone() {
                self.visit_variable_declaration(&v);
            }
        }
        //This should return the VMData::FN_PTR pointer to the main function:

        if let Some(f) = self.func_map.iter().enumerate().find(|(_,(k, _))| k == "main") {
            return VMData::new_fn_ptr(f.0);
        } else {
            eprintln!("Runtime Error: Could not find main function");
            VMData::new_unit()
        }
    }

    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> VMData {
        let left = self.visit_expression(&expression.left);
        let right = self.visit_expression(&expression.right);
        match expression.operator {
            BinaryOperator::OpAdd => left + right,
            BinaryOperator::OpSub => left - right,
            BinaryOperator::OpMul => left * right,
            BinaryOperator::OpDiv => left / right,
            BinaryOperator::OpMod => left % right,
            BinaryOperator::OpEq => VMData::new_bool(left == right),
            BinaryOperator::OpNe => VMData::new_bool(left != right),
            BinaryOperator::OpLt => VMData::new_bool(left < right),
            BinaryOperator::OpLe => VMData::new_bool(left <= right),
            BinaryOperator::OpGt => VMData::new_bool(left > right),
            BinaryOperator::OpGe => VMData::new_bool(left >= right),

            _ => unimplemented!("Binary operator not implemented")
        }
    }

    fn visit_expression(&mut self, expression: &Expression)  -> VMData {
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
                        VMData::new_i64(*i)
                    }
                    Literal::Float(f) => {
                        VMData::new_f64(*f)
                    },
                    Literal::Bool(b) => {
                        VMData::new_bool(*b)
                    },
                    Literal::String(s) => {
                        let res = self.object_map.put(Object::String(s.to_string()));
                        match res {
                            Ok(i) => VMData::new_string(i),
                            Err(_) => VMData::new_unit()
                        }
                    }
                    /* TO BE IMPLEMENTED
                    Literal::List(l) => {
                        let mut list = vec![];
                        for e in l {
                            list.push(self.visit_expression(e));
                        }
                        Value::List(list)
                    }*/
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
            Expression::MatchExpression(m) => {
                self.visit_match_expression(m)
            }
            Expression::IndexExpression(i) => {
                self.visit_index_expression(i)
            }
            _ => unimplemented!("Expression not implemented\n\t{}", expression)
        }
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) -> VMData {
        match function_call.name.as_str() {
            "print" => {
                for arg in &function_call.args {
                    let val = self.visit_expression(&arg);
                    println!("{:?}", val);
                }
                VMData::new_unit()
            },
            "read_int" => {
                //read input from the console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if let Ok(i) = input.trim().parse::<i64>() {
                    VMData::new_i64(i)
                } else {
                    VMData::new_unit()
                }
            },
            "read_f64" => {
                //read input from the console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if let Ok(i) = input.trim().parse::<f64>() {
                    VMData::new_f64(i)
                } else {
                    VMData::new_unit()
                }
            },
            "read_str" => {
                //read input from the console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                let res = self.object_map.put(Object::String(input));
                match res {
                    Ok(i) => VMData::new_string(i),
                    Err(_) => VMData::new_unit()
                }
            }
            _ => {
                for arg in &function_call.args {
                    let val = self.visit_expression(&arg);
                    self.stack.push(val);
                }
                if let Some(f) = self.varmap.last().unwrap().get(&function_call.name) {
                    match f.tag {
                        VMData::TAG_FN_PTR => {
                            let f = f.as_fn_ptr();
                            let func = self.func_map[f].1.clone();
                            let res = self.visit_function_expression(&func);
                            res
                        },
                        _ => unimplemented!("Main function is not a function body")
                    }
                } else {
                    unreachable!("Function {} not found", function_call.name)
                }
            }
        }
    }

    fn visit_identifier(&mut self, identifier: &IdentifierNode)  -> VMData {
        if let Some(v) = self.find_variable(identifier.name) {
            *v
        } else {
            eprintln!("Variable {} not found", identifier.name);
            std::process::exit(1);
        }
    }

    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> VMData {
        match &expression.operator {
            Some(op) => {
                match op {
                    UnaryOperator::OpAdd => {
                        self.visit_expression(&expression.expression)
                    },
                    UnaryOperator::OpSub => {
                        let res = self.visit_expression(&expression.expression);
                        match res.tag {
                            VMData::TAG_I64 => VMData::new_i64(-res.as_i64()),
                            VMData::TAG_FLOAT => VMData::new_f64(-res.as_f64()),
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
            None => {
                self.visit_expression(&expression.expression)
            }
        }
    }

    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> VMData {
        match &variable_declaration.value {
            Some(v) => {
                let val = *v.clone();
                match val {
                    Expression::FunctionExpression(f) => {
                        self.func_map.push((variable_declaration.name.to_string(), f));
                    },
                    _ => {
                        let value = self.visit_expression(&variable_declaration.value.clone().unwrap());
                        self.varmap.last_mut().unwrap().insert(variable_declaration.name, value);
                    }
                }
            }
            None => {
                self.varmap.last_mut().unwrap().insert(variable_declaration.name, VMData::new_unit());
            }
        }
        
        VMData::new_unit()
    }

    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> VMData {
        let condition = self.visit_expression(&if_else_node.condition);
        match condition.tag {
            VMData::TAG_I64 => {
                if condition.as_i64() != 0 {
                    self.visit_expression(&if_else_node.if_body)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap())
                }
            },
            VMData::TAG_BOOL => {
                if condition.as_bool() {
                    self.visit_expression(&if_else_node.if_body)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap())
                }
            },
            _ => {
                eprintln!("Unsupported condition: {}", if_else_node.condition);
                std::process::exit(1);
            }
        }
    }

    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> VMData {
        self.varmap.push(HashMap::new());
        let mut args = Vec::new();
        for _ in &function_expression.args {
            args.push(self.stack.pop().unwrap());
        }
        for arg in &function_expression.args {
            self.varmap.last_mut().unwrap().insert(arg.0, args.pop().unwrap());
        }

        let val = self.visit_expression(&function_expression.body);
        self.varmap.pop();
        val
    }

    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> VMData {
        let mut last_evaluated_expr = VMData::new_unit();
        for expression in &do_expression.body {
            last_evaluated_expr = self.visit_expression(&expression);
        }

        last_evaluated_expr
    }

    fn visit_match_expression(&mut self, match_expression: &MatchExpression) -> VMData {
        let val = self.visit_expression(&match_expression.expr);

        for case in &match_expression.arms {
            if self.visit_expression(case.pattern.as_ref()) == val {
                return self.visit_expression(&case.body);
            }
        }
        if let Some(e) = &match_expression.default {
            return self.visit_expression(&e);
        }
        VMData::new_unit()
    }
    
    fn visit_index_expression(&mut self, index_expression: &IndexExpression) -> VMData {
        /*let index = self.visit_expression(&index_expression.index);
        if let Some(arr) = self.find_variable(index_expression.name) {
            match arr {
                Value::List(a) => {
                    if let Value::Integer(i) = index {
                        return a[i as usize].clone();
                    }
                    unimplemented!("Unsupported index type: {:?}", index);
                },
                _ => unimplemented!("You can only index lists")
            }
        } else {
            unreachable!("Variable {} not found", index_expression.name)
        };*/
        VMData::new_unit()
    }

}