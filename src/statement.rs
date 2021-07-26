use crate::types::Type;

struct SubroutineDeclaration {
    name: String,
    parameter_names: Vec<String>,
    parameter_types: Vec<Type>,
    statements: Vec<Expression>,
    return_type: Type,
}

struct AssignmentExpression {
    left: String,
    right: Box<Expression>,
}

enum Expression {
    Assignment(AssignmentExpression),
    Integer(i64),
    String(String),
}

enum Declaration {
    Subroutine(SubroutineDeclaration),
    Statement(Expression),
}