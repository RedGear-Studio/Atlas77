use super::expr::Expression;
use super::data_type::DataType;
#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        identifier: String,
        value: Option<Expression>,
        data_type: DataType,
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
    },
    WhileLoop {
        cond_expr: Expression,
        body_expr: Vec<Statement>,
    },
    ForLoop {
        identifier: String,
        expr: Expression,
        step: Expression,
        direction: ForLoopDirection,
        body_expr: Vec<Statement>,

    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ForLoopDirection {
    Increase,
    Decrease,
    Both
}