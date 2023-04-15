pub mod expr;
pub mod stmt;
pub mod literal;
pub mod func;
use self::func::Function;
#[derive(Default)]
pub struct Program {
    pub functions: Vec<Function>,
}