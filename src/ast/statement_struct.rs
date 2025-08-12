use crate::ast::expression_struct::Expression;
use crate::ast::type_struct::Type;

pub enum Statement {
    VariableDeclaration {
        name: String,
        var_type: Type,
        value: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Return {
        value: Expression,
    },
}
