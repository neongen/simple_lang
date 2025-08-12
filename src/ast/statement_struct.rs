use crate::ast::type_struct::Type;
use crate::ast::expression_struct::Expression;

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
    },
    Return {
        value: Expression,
    },
}