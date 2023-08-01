pub mod ir_nodes;
pub mod print;
pub mod analysis;
pub mod builder;
pub mod context;
pub mod errors;
pub mod transform;
pub mod data_type;
#[derive(Default)]
pub struct IRProgram {
    pub functions:Vec<ir_nodes::IRFunction>
}
