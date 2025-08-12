use crate::data_struct::binary_operator_struct::BinaryOperator;

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    IntegerLiteral(i32),
    StringLiteral(String),
    VariableRef(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}