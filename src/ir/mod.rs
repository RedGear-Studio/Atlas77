use self::ir_nodes::func::IRFunction;
pub mod ir_nodes;
pub mod print;
pub mod analysis;
pub mod builder;
pub mod context;
pub mod errors;
pub mod transform;
#[derive(Default)]
pub struct IRProgram {
    pub functions:Vec<IRFunction>
}