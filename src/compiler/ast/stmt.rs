use crate::compiler::ir::data_type::IRDataType;

use super::expr::Expression;
#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        identifier: String,
        value: Option<Expression>,
        data_type: IRDataType,
    },
    Assignment {
        identifier: String,
        value: Expression,
    },
    PrintStatement(Expression),
    IfStatement {
        cond_expr: Expression,
        body_expr: Vec<Statement>,
        else_expr: Option<Vec<Statement>>,
    }
}