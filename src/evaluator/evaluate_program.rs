use std::collections::HashMap;

use crate::ast::program_struct::Program;
use crate::ast::expression_struct::Expression;
use crate::ast::function_struct::Function;
use crate::ast::environment_struct::Environment;
use crate::evaluator::evaluate_function::evaluate_function;

///// Evaluates the program starting from the `main` function.
///// Returns the final i32 return value of `main`, or an error if evaluation fails.
pub fn evaluate_program(program: &Program) -> Result<i32, String> {
    let mut env = Environment {
        variables: HashMap::new(),
        functions: program
            .functions
            .iter()
            .map(|f| (f.name.clone(), f))
            .collect(),
    };

    let main_fn = env.get_function("main")
        .ok_or_else(|| "main function not found".to_string())?;

    let result = evaluate_function(main_fn, vec![], &mut env)?;
    if let Expression::IntegerLiteral(code) = result {
        Ok(code)
    } else {
        Err("main function did not return an integer".to_string())
    }
}

///// Finds the `main` function in the program.
fn find_main_function(program: &Program) -> Result<&Function, String> {
    program
        .functions
        .iter()
        .find(|f| f.name == "main")
        .ok_or_else(|| "main function not found.".to_string())
}
