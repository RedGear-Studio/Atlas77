use super::{assign::IRAssignement, variable::IRVariable, loops::{IRWhile, IRFor}};

pub enum IRStatement {
    Assignement(IRAssignement),
    Variable(IRVariable),
    While(IRWhile),
    For(IRFor),
}