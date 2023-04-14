use super::{expr::IRExpression, data_type::IRDataType};

pub struct IRAssignement {
    identifier: String, // To be able to retrive the id of the variable and for debugging purposes too.
    id: usize,
    value: Box<IRExpression>,
    data_type: IRDataType
}