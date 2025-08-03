use crate::ast::parameter_struct::Parameter;
use crate::ast::statement_struct::Statement;
use crate::ast::type_struct::Type;

pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}