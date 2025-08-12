use crate::data_struct::parameter_struct::Parameter;
use crate::data_struct::statement_struct::Statement;
use crate::data_struct::type_struct::Type;

pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}