// Enhanced evaluate_function.rs with proper if statement evaluation

use crate::data_struct::function_struct::Function;
use crate::data_struct::expression_struct::Expression;
use crate::data_struct::statement_struct::Statement;
use crate::data_struct::binary_operator_struct::BinaryOperator;
use crate::data_struct::environment_struct::Environment;

/// Evaluates a function given the function definition and argument expressions.
/// Returns the resulting Expression or an error string.
/// Enhanced with proper if statement evaluation.
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

    // Bind parameters to arguments
    for (param, arg) in function.params.iter().zip(args.into_iter()) {
        env.insert_variable(param.name.clone(), arg);
    }

    evaluate_statements(&function.body, &mut env)
}

/// Evaluate a list of statements in order, returning the final expression if a return is encountered.
/// Enhanced to handle if statements properly.
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
/// Enhanced with proper if statement handling.
fn evaluate_statement<'a>(
    stmt: &Statement,
    env: &mut Environment<'a>,
) -> Result<Option<Expression>, String> {
    match stmt {
        Statement::VariableDeclaration { name, value, .. } => {
            let val = evaluate_expression(value, env)?;
            env.insert_variable(name.clone(), val);
            Ok(None)
        }

        Statement::FunctionCall { name, args } => {
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

                        _ => Err("print only supports strings".to_string()),
                    }
                }

                _ => {
                    let evaluated_args = evaluate_arguments(args, env)?;
                    let result = evaluate_function_by_name(name, evaluated_args, env)?;
                    Ok(Some(result))
                }
            }
        }

        Statement::If { condition, body } => {
            let cond_val = evaluate_expression(condition, env)?;
            if is_truthy(&cond_val)? {
                // Execute if body statements
                for stmt in body {
                    if let Some(ret_val) = evaluate_statement(stmt, env)? {
                        return Ok(Some(ret_val)); // Early return from if block
                    }
                }
            }
            Ok(None)
        }

        Statement::Return { value } => {
            let val = evaluate_expression(value, env)?;
            Ok(Some(val))
        }
    }
}

/// Evaluate an expression in the given environment.
/// Enhanced with better error handling and support for all binary operations.
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
            // Handle built-in functions
            match name.as_str() {
                "int_to_string" => {
                    if args.len() != 1 {
                        return Err("int_to_string expects exactly one argument".to_string());
                    }
                    let val = evaluate_expression(&args[0], env)?;
                    match val {
                        Expression::IntegerLiteral(i) => {
                            Ok(Expression::StringLiteral(i.to_string()))
                        }
                        _ => Err("int_to_string expects an integer argument".to_string()),
                    }
                }
                _ => {
                    let evaluated_args = evaluate_arguments(args, env)?;
                    evaluate_function_by_name(name, evaluated_args, env)
                }
            }
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
/// In simple_lang, only i32 values are considered for truthiness:
/// - 0 is false
/// - Any non-zero value is true
fn is_truthy(expr: &Expression) -> Result<bool, String> {
    match expr {
        Expression::IntegerLiteral(i) => Ok(*i != 0),
        _ => Err("Invalid type for condition expression; expected i32".to_string()),
    }
}

/// Enhanced binary operation evaluation with proper overflow checking.
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
                GreaterThan => return Ok(IntegerLiteral(if l > r { 1 } else { 0 })),
                LessThan => return Ok(IntegerLiteral(if l < r { 1 } else { 0 })),
                Equal => return Ok(IntegerLiteral(if l == r { 1 } else { 0 })),
            };
            Ok(IntegerLiteral(result))
        }
        // Support string equality comparison
        (Expression::StringLiteral(l), Expression::StringLiteral(r)) => {
            match op {
                Equal => Ok(IntegerLiteral(if l == r { 1 } else { 0 })),
                _ => Err("Only equality comparison is supported for strings".to_string()),
            }
        }
        _ => Err("Binary operations require compatible types".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_struct::type_struct::Type;
    use std::collections::HashMap;

    #[test]
    fn test_evaluate_if_statement_true() {
        let mut env = Environment::new(HashMap::new());
        env.insert_variable("x".to_string(), Expression::IntegerLiteral(5));

        let if_stmt = Statement::If {
            condition: Expression::BinaryOp {
                op: BinaryOperator::GreaterThan,
                left: Box::new(Expression::VariableRef("x".to_string())),
                right: Box::new(Expression::IntegerLiteral(0)),
            },
            body: vec![
                Statement::VariableDeclaration {
                    name: "result".to_string(),
                    var_type: Type::I32,
                    value: Expression::IntegerLiteral(42),
                }
            ],
        };

        let result = evaluate_statement(&if_stmt, &mut env);
        assert!(result.is_ok());
        assert!(env.get("result").is_some());
    }

    #[test]
    fn test_evaluate_if_statement_false() {
        let mut env = Environment::new(HashMap::new());
        env.insert_variable("x".to_string(), Expression::IntegerLiteral(-5));

        let if_stmt = Statement::If {
            condition: Expression::BinaryOp {
                op: BinaryOperator::GreaterThan,
                left: Box::new(Expression::VariableRef("x".to_string())),
                right: Box::new(Expression::IntegerLiteral(0)),
            },
            body: vec![
                Statement::VariableDeclaration {
                    name: "result".to_string(),
                    var_type: Type::I32,
                    value: Expression::IntegerLiteral(42),
                }
            ],
        };

        let result = evaluate_statement(&if_stmt, &mut env);
        assert!(result.is_ok());
        assert!(env.get("result").is_none()); // Should not be executed
    }

    #[test]
    fn test_is_truthy() {
        assert_eq!(is_truthy(&Expression::IntegerLiteral(0)).unwrap(), false);
        assert_eq!(is_truthy(&Expression::IntegerLiteral(1)).unwrap(), true);
        assert_eq!(is_truthy(&Expression::IntegerLiteral(-1)).unwrap(), true);
        assert_eq!(is_truthy(&Expression::IntegerLiteral(42)).unwrap(), true);
    }

    #[test]
    fn test_evaluate_comparison_operators() {
        let greater = evaluate_binary_op(
            &BinaryOperator::GreaterThan,
            &Expression::IntegerLiteral(5),
            &Expression::IntegerLiteral(3)
        ).unwrap();
        assert_eq!(greater, Expression::IntegerLiteral(1));

        let less = evaluate_binary_op(
            &BinaryOperator::LessThan,
            &Expression::IntegerLiteral(3),
            &Expression::IntegerLiteral(5)
        ).unwrap();
        assert_eq!(less, Expression::IntegerLiteral(1));

        let equal = evaluate_binary_op(
            &BinaryOperator::Equal,
            &Expression::IntegerLiteral(5),
            &Expression::IntegerLiteral(5)
        ).unwrap();
        assert_eq!(equal, Expression::IntegerLiteral(1));
    }
}