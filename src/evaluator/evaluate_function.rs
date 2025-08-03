use crate::ast::function_struct::Function;
use crate::ast::expression_struct::Expression;
use crate::ast::statement_struct::Statement;
use crate::ast::binary_operator_struct::BinaryOperator;

/// Evaluates a function given the function definition and argument expressions.
/// Returns the resulting Expression or an error string.
/// Assumes arguments are already evaluated expressions.
pub fn evaluate_function(function: &Function, args: Vec<Expression>) -> Result<Expression, String> {
    if args.len() != function.params.len() {
        return Err(format!(
            "Expected {} arguments but got {}",
            function.params.len(),
            args.len()
        ));
    }

    // Create initial environment mapping parameter names to argument values.
    let mut env = Environment::new();
    for (param, arg) in function.params.iter().zip(args.into_iter()) {
        env.insert(param.name.clone(), arg);
    }

    evaluate_statements(&function.body, &mut env)
}

/// Environment stores variable bindings during evaluation.
type Environment = std::collections::HashMap<String, Expression>;

/// Evaluate a list of statements in order, returning the final expression if a return is encountered.
/// Returns error if no return statement is found for non-void functions.
fn evaluate_statements(statements: &[Statement], env: &mut Environment) -> Result<Expression, String> {
    for stmt in statements {
        match evaluate_statement(stmt, env)? {
            Some(ret_val) => return Ok(ret_val), // Return statement encountered
            None => continue,
        }
    }
    // If no return statement found, for non-void functions, this is an error.
    Err("Function did not return a value".to_string())
}

/// Evaluate a single statement. Returns:
/// - Ok(Some(Expression)) if a return statement with value was executed.
/// - Ok(None) otherwise.
/// - Err on error.
fn evaluate_statement(stmt: &Statement, env: &mut Environment) -> Result<Option<Expression>, String> {
    use crate::ast::statement_struct::Statement::*;
    use crate::ast::expression_struct::Expression;

    match stmt {
        VariableDeclaration { name, value, .. } => {
            let val = evaluate_expression(value, env)?;
            env.insert(name.clone(), val);
            Ok(None)
        }

        FunctionCall { name, args } => {
            match name.as_str() {
                "print" => {
                    if args.len() != 1 {
                        return Err("print expects exactly one argument".to_string());
                    }
                    let value = evaluate_expression(&args[0], env)?;
                    match value {
                        Expression::StringLiteral(s) => {
                            println!("{}", s);
                            Ok(None)
                        }
                        Expression::IntegerLiteral(i) => {
                            println!("{}", i);
                            Ok(None)
                        }
                        _ => Err("print only supports strings and integers".to_string()),
                    }
                }

                "print_number" => {
                    if args.len() != 1 {
                        return Err("print_number expects one i32 argument".to_string());
                    }
                    let val = evaluate_expression(&args[0], env)?;
                    match val {
                        Expression::IntegerLiteral(i) => {
                            let text = i.to_string();
                            println!("{}", text);
                            Ok(None)
                        }
                        _ => Err("print_number expects an integer".to_string()),
                    }
                }

                _ => Err(format!("Function call to '{}' not supported in evaluate_function", name)),
            }
        }

        If { condition, body } => {
            let cond_val = evaluate_expression(condition, env)?;
            if is_truthy(&cond_val)? {
                for stmt in body {
                    if let Some(ret_val) = evaluate_statement(stmt, env)? {
                        return Ok(Some(ret_val));
                    }
                }
            }
            Ok(None)
        }

        Return { value } => {
            let val = evaluate_expression(value, env)?;
            Ok(Some(val))
        }
    }
}


/// Evaluate an expression in the given environment.
/// Returns the evaluated expression or error.
fn evaluate_expression(expr: &Expression, env: &Environment) -> Result<Expression, String> {
    use crate::ast::expression_struct::Expression::*;

    match expr {
        IntegerLiteral(_) | StringLiteral(_) => Ok(expr.clone()),
        VariableRef(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable '{}'", name)),
        BinaryOp { op, left, right } => {
            let lval = evaluate_expression(left, env)?;
            let rval = evaluate_expression(right, env)?;
            evaluate_binary_op(op, &lval, &rval)
        }
        FunctionCall { .. } => {
            // No support for nested function calls in this context
            Err("Nested function calls are not supported in evaluate_function".to_string())
        }
    }
}

/// Helper to determine if an expression is truthy for control flow.
/// For integers, 0 is false, non-zero true.
/// Strings are considered error in condition context.
fn is_truthy(expr: &Expression) -> Result<bool, String> {
    match expr {
        Expression::IntegerLiteral(i) => Ok(*i != 0),
        _ => Err("Invalid type for condition expression; expected i32".to_string()),
    }
}

/// Evaluate a binary operation on two evaluated expressions.
/// Supports i32 operations only.
fn evaluate_binary_op(op: &BinaryOperator, left: &Expression, right: &Expression) -> Result<Expression, String> {
    use crate::ast::binary_operator_struct::BinaryOperator::*;
    use crate::ast::expression_struct::Expression::IntegerLiteral;

    match (left, right) {
        (IntegerLiteral(l), IntegerLiteral(r)) => {
            let result = match op {
                Add => l.checked_add(*r).ok_or("Integer overflow on addition")?,
                Subtract => l.checked_sub(*r).ok_or("Integer overflow on subtraction")?,
                Multiply => l.checked_mul(*r).ok_or("Integer overflow on multiplication")?,
                Divide => {
                    if *r == 0 {
                        return Err("Division by zero".to_string());
                    }
                    l.checked_div(*r).ok_or("Integer overflow on division")?
                }
                GreaterThan => return Ok(Expression::IntegerLiteral((l > r) as i32)),
                LessThan => return Ok(Expression::IntegerLiteral((l < r) as i32)),
                Equal => return Ok(Expression::IntegerLiteral((l == r) as i32)),
            };
            Ok(IntegerLiteral(result))
        }
        _ => Err("Binary operations only supported on i32 literals".to_string()),
    }
}
