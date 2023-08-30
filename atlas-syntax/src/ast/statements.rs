pub enum Statement {
    AssignmentStmt(Assignment),
    IfStmt(If),
}

pub struct Assignment {
    var_name: String,
    value: String, //todo, should be Expression
}

pub struct If {
    cond: String, //todo, should be Expression
    body: String, //todo, should be Vec<Statement>
    else_body: Option<String>, //todo, should be Vec<Statement>
}