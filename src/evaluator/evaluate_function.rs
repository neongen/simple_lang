use crate::ast::function_struct::Function;
use crate::ast::expression_struct::Expression;
use crate::ast::statement_struct::Statement;
use crate::ast::binary_operator_struct::BinaryOperator;
use crate::ast::environment_struct::Environment;

/// Evaluates a function given the function definition and argument expressions.
/// Returns the resulting Expression or an error string.
/// Assumes arguments are already evaluated expressions.
pub fn evaluate_function<'a>(
    function: &'a Function,
    args: Vec<Expression>,
    outer_env: &Environment<'a>,
) -> Result<Expression, String> {
    if args.len() != function.params.len() {
        return Err(format!(
            "Expected {} arguments but got {}",
            function.params.len(),
            args.len()
        ));
    }

    // Create new environment inheriting the functions from the outer environment
    let mut env = Environment::new(outer_env.functions.clone());

    for (param, arg) in function.params.iter().zip(args.into_iter()) {
        env.insert_variable(param.name.clone(), arg);
    }

    evaluate_statements(&function.body, &mut env)
}

/// Evaluate a list of statements in order, returning the final expression if a return is encountered.
/// Returns error if no return statement is found for non-void functions.
fn evaluate_statements<'a>(
    statements: &[Statement],
    env: &mut Environment<'a>,
) -> Result<Expression, String> {
    for stmt in statements {
        match evaluate_statement(stmt, env)? {
            Some(ret_val) => return Ok(ret_val), // Return statement encountered
            None => continue,
        }
    }
    Err("Function did not return a value".to_string())
}

/// Evaluate a single statement.
fn evaluate_statement<'a>(
    stmt: &Statement,
    env: &mut Environment<'a>,
) -> Result<Option<Expression>, String> {
    use crate::ast::statement_struct::Statement::*;
    use crate::ast::expression_struct::Expression;

    match stmt {
        VariableDeclaration { name, value, .. } => {
            let val = evaluate_expression(value, env)?;
            env.insert_variable(name.clone(), val);
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

                _ => {
                    let evaluated_args = evaluate_arguments(args, env)?;
                    let result = evaluate_function_by_name(name, evaluated_args, env)?;
                    Ok(Some(result))
                }
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
pub fn evaluate_expression<'a>(
    expr: &Expression,
    env: &Environment<'a>,
) -> Result<Expression, String> {
    match expr {
        Expression::IntegerLiteral(_) | Expression::StringLiteral(_) => Ok(expr.clone()),

        Expression::VariableRef(name) => {
            env.get(name)
                .cloned()
                .ok_or_else(|| format!("Variable '{}' not found", name))
        }

        Expression::BinaryOp { op, left, right } => {
            let l_val = evaluate_expression(left, env)?;
            let r_val = evaluate_expression(right, env)?;
            evaluate_binary_op(op, &l_val, &r_val)
        }

        Expression::FunctionCall { name, args } => {
            let evaluated_args = evaluate_arguments(args, env)?;
            evaluate_function_by_name(name, evaluated_args, env)
        }
    }
}

/// Evaluates all argument expressions in order.
fn evaluate_arguments<'a>(
    args: &[Expression],
    env: &Environment<'a>,
) -> Result<Vec<Expression>, String> {
    args.iter().map(|arg| evaluate_expression(arg, env)).collect()
}

/// Looks up a function by name and evaluates it with args.
fn evaluate_function_by_name<'a>(
    name: &str,
    args: Vec<Expression>,
    env: &Environment<'a>,
) -> Result<Expression, String> {
    let func = env
        .get_function(name)
        .ok_or_else(|| format!("Function '{}' not found", name))?;

    evaluate_function(func, args, env)
}

/// Helper for truthiness of condition expressions.
fn is_truthy(expr: &Expression) -> Result<bool, String> {
    match expr {
        Expression::IntegerLiteral(i) => Ok(*i != 0),
        _ => Err("Invalid type for condition expression; expected i32".to_string()),
    }
}

/// Evaluate binary operation.
fn evaluate_binary_op(
    op: &BinaryOperator,
    left: &Expression,
    right: &Expression,
) -> Result<Expression, String> {
    use BinaryOperator::*;
    use Expression::IntegerLiteral;

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
                GreaterThan => return Ok(IntegerLiteral((l > r) as i32)),
                LessThan => return Ok(IntegerLiteral((l < r) as i32)),
                Equal => return Ok(IntegerLiteral((l == r) as i32)),
            };
            Ok(IntegerLiteral(result))
        }
        _ => Err("Binary operations only supported on i32 literals".to_string()),
    }
}
