#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone, PartialEq)]
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
    WhileStatement {
        condition: Expression,
        body: Vec<Statement>,
    },
    Assignment {
        name: String,
        value: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    NumberLiteral(i32),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
}
