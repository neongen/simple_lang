// Declare modules explicitly
pub mod ast {
    pub mod program_struct;
    pub mod function_struct;
    pub mod binary_operator_struct;
    pub mod expression_struct;
    pub mod parameter_struct;
    pub mod statement_struct;
    pub mod type_struct;
}

pub mod parser {
    pub mod parse_program;
    pub mod parse_function;
    pub mod parse_statement;
    pub mod parse_expression;
}

pub mod evaluator {
    pub mod evaluate_program;
    pub mod evaluate_function;
}

pub mod source {
    pub mod read_source_file;
}