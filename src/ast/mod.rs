#[derive(Debug)]
pub enum Type {
    String,
    Number,
    Boolean,
}

#[derive(Debug)]
pub enum Statement {
    ConsoleLog(Expression),
    VariableDeclaration {
        name: String,
        type_annotation: Type,
        value: Option<Expression>,
    },
    IfStatement {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(String),
    NumberLiteral(i32),
    Identifier(String),
}
