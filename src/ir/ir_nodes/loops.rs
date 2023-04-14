use super::{expr::IRExpression, block::IRBlock, variable::IRVariable};

pub struct IRWhile {
    condition: IRExpression,
    block: IRBlock,
}
//If there is no variable with the same name, a Variable is create on top of the IRFor IRBlock
pub struct IRFor {
    variable_id: usize,
    identifier: String,
    block: IRBlock,
    step: IRExpression,
    behaviour: IRForBehaviour,
}
pub enum IRForBehaviour {
    Increment,
    Decrement,
    Both,
}