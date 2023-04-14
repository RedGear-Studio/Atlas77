use super::expr::IRExpression;

pub struct IRUnaryOp  {
    op: UnaryOperator,
    expr: Box<IRExpression>,
}
pub enum UnaryOperator {
    Negate,
    Not,
}