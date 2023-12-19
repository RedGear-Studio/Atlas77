use crate::exit_err;
use std::collections::HashMap;

use crate::ast::{self, Function, Declaration, Statement, DoStatement, Expression, BinaryExpression, MatchExpression, FunctionCall};
use common::value::{Value, Type};

#[derive(Debug, Clone)]
pub enum Contract {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Not
}

type TypeHashMap = HashMap<TypeId, Vec<Contract>>; //Contract as in what does this types can do
type FunctionHashMap = HashMap<FuncId, (Vec<TypeId>, TypeId)>; //as in (Input, Output)
//Each id being unique and depends on the scope, I don't need multiple VariableHashMap. (You can easily reconstruct the id of a variable based on the scope you're in)
type VariableHashMap = HashMap<VarId, TypeId>;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct TypeId {
    id: u32,
    name: String
}

impl From<String> for TypeId {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "i64" => TypeId { id: 0, name: String::from("i64") },
            "f64" => TypeId { id: 1, name: String::from("f64") },
            "bool" => TypeId { id: 2, name: String::from("bool") },
            "string" => TypeId { id: 3, name: String::from("string") },
            _ => exit_err!("Unknown type: {}", value),
        }
    }
}

impl From<Type> for TypeId {
    fn from(value: Type) -> Self {
        match value {
            Type::Int64 => TypeId { id: 0, name: String::from("i64") },
            Type::Float64 => TypeId { id: 1, name: String::from("f64") },
            Type::Bool => TypeId { id: 2, name: String::from("bool") },
            Type::StringType => TypeId { id: 3, name: String::from("string") },
            _ => exit_err!("Unknown type: {:?}", value),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct FuncId {
    //id: u32,
    //scope: u32,
    name: String
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct VarId {
    name: String,
}

#[derive(Debug, Clone)]
pub struct IRContext {
    types: TypeHashMap,
    functions: FunctionHashMap,
    pub variables: VariableHashMap,
    ast: Vec<ast::Declaration>,
    depth: usize
}

impl IRContext {
    pub fn new(ast: Vec<ast::Declaration>) -> Self {
        let mut primitive_type = TypeHashMap::new();
        primitive_type.insert(TypeId {id: 0, name: String::from("i64")}, vec![Contract::Add, Contract::Sub, Contract::Mul, Contract::Div, Contract::Mod, Contract::Eq, Contract::Neq, Contract::Lt, Contract::Lte, Contract::Gt, Contract::Gte, Contract::And, Contract::Or, Contract::Not]);
        primitive_type.insert(TypeId { id: 1, name: String::from("f64") }, vec![Contract::Add, Contract::Sub, Contract::Mul, Contract::Div, Contract::Eq, Contract::Neq, Contract::Lt, Contract::Lte, Contract::Gt, Contract::Gte]);
        primitive_type.insert(TypeId { id: 2, name: String::from("bool") }, vec![Contract::Eq, Contract::Neq, Contract::And, Contract::Or, Contract::Not]);
        primitive_type.insert(TypeId { id: 3, name: String::from("string") }, vec![Contract::Eq, Contract::Neq, Contract::Add]);

        let mut ir = IRContext {
            types: primitive_type,
            functions: FunctionHashMap::new(),
            variables: VariableHashMap::new(),
            ast,
            depth: 0
        };
        ir.register_functions();
        ir
    }

    fn add_type(&mut self, id: TypeId, contracts: Vec<Contract>) {
        self.types.insert(id, contracts);
    }

    fn add_function(&mut self, id: FuncId, input: Vec<TypeId>, output: TypeId) {
        self.functions.insert(id, (input, output));
    }

    fn add_variable(&mut self, id: VarId, type_id: TypeId) {
        self.variables.insert(id, type_id);
    }

    fn register_functions(&mut self) {
        self.ast.clone().iter().for_each(|decl| {
            match decl {
                Declaration::Function(Function { name, args, return_type, ..}) => {
                    let mut input = Vec::new();
                    for arg in args {
                        let ty = TypeId::from(arg.ty.clone());
                        self.add_variable(VarId {
                            //id: self.variables.len() as u32,
                            //scope: self.depth as u32,
                            name: arg.name.clone()
                        }, ty.clone());
                        input.push(ty);
                    }
                    let rt_type = TypeId::from(return_type.clone());
                    self.add_function(FuncId {
                        //id: self.depth as u32,
                        //scope: self.depth as u32,
                        name: name.clone() 
                    }, input, rt_type.clone());
                }
            }
        });
    }

    pub fn type_check(&mut self) -> Result<(), String> {
        //Visit Declarations in the AST to save the functions
        self.ast.clone().iter().for_each(|decl| {
            match decl {
                Declaration::Function(Function { args, return_type, body, ..}) => {
                    let mut input = Vec::new();
                    for arg in args {
                        input.push(TypeId::from(arg.ty.clone()));
                    }
                    let rt_type = TypeId::from(return_type.clone());
                    //self.add_function(FuncId { id: self.depth as u32, scope: self.depth as u32, name: name.clone() }, input, rt_type.clone());
                    match self.type_check_fn_body(body, rt_type) {
                        Ok(_) => {},
                        Err(err) => {
                            exit_err!("{}", err);
                        }
                    }
                }
            }
        });

        Ok(())
    }
    fn type_check_fn_body(&mut self, statement: &Statement, rt_type: TypeId) -> Result<(), String> {
        match statement {
            Statement::DoStatement(d) => {
                self.do_statement(d)?;
            },
            Statement::Expression(e) => {
                self.get_type_from_expr(e);
            }
        }
        Ok(())
    }

    fn do_statement(&mut self, do_expr: &DoStatement) -> Result<(), String> {
        for expr in do_expr.exprs.iter() {
            match &expr {
                Expression::VariableDeclaration(v) => {
                    self.add_variable(VarId { name: v.name.clone() }, TypeId::from(v.ty.clone()));
                }
                _ => {}
            }
            self.get_type_from_expr(&expr);
        }
        Ok(())
    }

    fn get_type_from_expr(&self, expr: &Expression) -> TypeId {
        match expr.clone() {
            Expression::BinaryExpression(BinaryExpression {left, right, ..}) => {
                let left_type = self.get_type_from_expr(&left);
                let right_type = self.get_type_from_expr(&right);
                self.precedence_type(&left_type, &right_type)
            },
            Expression::MatchExpression(MatchExpression {expr, arms, .. }) => {
                let expr_type = self.get_type_from_expr(&expr);
                let mut arm_types = Vec::new();
                for arm in arms {
                    arm_types.push(self.get_type_from_expr(&arm.expr));
                }
                for arm in arm_types {
                    if arm != expr_type {
                        exit_err!("Match arms have different types {:?} and {:?}", expr_type, arm);
                    }
                }
                expr_type
            },
            Expression::Literal(lit) => {
                match lit {
                    Value::Int64(_) => TypeId::from(Type::Int64),
                    Value::Float64(_) => TypeId::from(Type::Float64),
                    Value::Bool(_) => TypeId::from(Type::Bool),
                    Value::StringValue(_) => TypeId::from(Type::StringType),
                    Value::Identifier(s) => {
                        match self.variables.get(&VarId { name: s.clone() }) {
                            Some(ty) => ty.clone(),
                            None => exit_err!("Variable {} not found", s)
                        }
                    },
                    _ => exit_err!("Unsupported literal type {:?}", lit)
                }
            },
            Expression::FunctionCall(f) => {
                match self.function_call(&f) {
                    Ok(ty) => ty,
                    Err(err) => exit_err!("{}", err)
                }
            },
            Expression::VariableDeclaration(v) => {
                TypeId::from(v.ty.clone())
            },
            Expression::CastingExpression(c) => {
                TypeId::from(c.ty.clone())
            },
            Expression::UnaryExpression(u) => {
                self.get_type_from_expr(&u.operand)
            }
        }
    }

    fn function_call(&self, f: &FunctionCall) -> Result<TypeId, String> {
        let func = self.functions.get(&FuncId { name: f.name.clone()});
        match func { 
            Some((input, output)) => {
                f.args.iter().enumerate().for_each(|(i, args)| {
                    let ty = self.get_type_from_expr(&args);
                    match input.get(i) {
                        Some(t) => {
                            if !(ty == *t) {
                                exit_err!("Expected {:?}, got {:?}", t, ty);
                            }
                        },
                        None => exit_err!("Too many arguments for function {}", f.name)
                    }
                });
                if f.args.len() != input.len() {
                    exit_err!("Too few arguments for function {}", f.name);
                }
                Ok(output.clone())
            },
            None => exit_err!("Function {} not found", f.name)
        }
    }

    fn precedence_type(&self, t1: &TypeId, t2: &TypeId) -> TypeId {
        match (t1, t2) {
            (TypeId { id: 0, .. }, TypeId { id: 0, .. }) => TypeId::from(Type::Int64),
            (TypeId { id: 0, .. }, TypeId { id: 1, .. }) => TypeId::from(Type::Float64),
            (TypeId { id: 1, .. }, TypeId { id: 0, .. }) => TypeId::from(Type::Float64),
            (TypeId { id: 1, .. }, TypeId { id: 1, .. }) => TypeId::from(Type::Float64),
            (TypeId { id: 2, .. }, TypeId { id: 2, .. }) => TypeId::from(Type::Bool),
            (TypeId { id: 3, .. }, TypeId { id: 3, .. }) => TypeId::from(Type::StringType),
            //String concatenation
            (TypeId { id: 3, .. }, TypeId { id: 0, .. }) => TypeId::from(Type::StringType),
            (TypeId { id: 0, .. }, TypeId { id: 3, .. }) => TypeId::from(Type::StringType),
            (TypeId { id: 3, .. }, TypeId { id: 1, .. }) => TypeId::from(Type::StringType),
            (TypeId { id: 1, .. }, TypeId { id: 3, .. }) => TypeId::from(Type::StringType),
            (TypeId { id: 3, .. }, TypeId { id: 2, .. }) => TypeId::from(Type::StringType),
            (TypeId { id: 2, .. }, TypeId { id: 3, .. }) => TypeId::from(Type::StringType),
            _ => exit_err!("Unsupported type precedence: {:?} and {:?}", t1, t2)
        }
    }

}
