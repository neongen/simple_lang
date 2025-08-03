use crate::ast::program_struct::Program;
use crate::ast::type_struct::Type;
use crate::ast::expression_struct::Expression;
use crate::ast::function_struct::Function;
use crate::evaluator::evaluate_function::evaluate_function;

///// Evaluates the program starting from the `main` function.
///// Returns the final i32 return value of `main`, or an error if evaluation fails.
pub fn evaluate_program(program: &Program) -> Result<i32, String> {
    // Find the main function
    let main_fn = find_main_function(program)?;

    // Ensure main has no parameters and returns i32
    if !main_fn.params.is_empty() {
        return Err("main function must not take any parameters.".to_string());
    }

    match main_fn.return_type {
        Type::I32 => {
            let result = evaluate_function(main_fn, Vec::new())?;

            match result {
                Expression::IntegerLiteral(value) => Ok(value),
                _ => Err("main function did not return an i32 value.".to_string()),
            }
        }
        _ => Err("main function must return i32.".to_string()),
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

