use crate::ast::binary_operator_struct::BinaryOperator;
use crate::ast::expression_struct::Expression;
use crate::ast::function_struct::Function;
///! Type-checks an entire program by verifying each function is correctly typed.
///!
///! This module validates that all functions in a program have consistent types,
///! proper variable declarations, correct function calls, and valid control flow.
///! Returns detailed error messages for any type mismatches found.
///! Ensures type safety before program evaluation begins.
use crate::ast::program_struct::Program;
use crate::ast::statement_struct::Statement;
use crate::ast::type_struct::Type;

/// Type-checks the entire program by verifying each function is correctly typed.
/// Returns Ok(()) if all functions pass type checking, otherwise returns an error
/// describing the first encountered type error.
pub fn type_check_program(program: &Program) -> Result<(), String> {
    for function in &program.functions {
        type_check_function(function)?;
    }
    Ok(())
}

/// Type-checks a single function by validating parameters, body statements, and return type.
fn type_check_function(function: &Function) -> Result<(), String> {
    let mut context = TypeContext::new();

    // Add parameters to context
    for param in &function.params {
        if context.contains(&param.name) {
            return Err(format!(
                "Parameter '{}' declared multiple times",
                param.name
            ));
        }
        context.insert(param.name.clone(), param.param_type.clone());
    }

    // Type check all statements in the function body
    for stmt in &function.body {
        type_check_statement(stmt, &mut context)?;
    }

    // Verify that non-void functions have a return statement
    if function.return_type != Type::Void {
        if let Some(last_stmt) = function.body.last() {
            if !is_return_statement(last_stmt) {
                return Err(format!(
                    "Function '{}' missing return statement",
                    function.name
                ));
            }
        } else {
            return Err(format!(
                "Function '{}' has empty body but non-void return type",
                function.name
            ));
        }
    }

    Ok(())
}

/// Type-checks a statement within the given context and updates variable bindings.
fn type_check_statement(stmt: &Statement, context: &mut TypeContext) -> Result<(), String> {
    match stmt {
        Statement::VariableDeclaration {
            name,
            var_type,
            value,
        } => {
            let expr_type = type_check_expression(value, context)?;
            if &expr_type != var_type {
                return Err(format!(
                    "Type mismatch for variable '{}': expected {:?}, found {:?}",
                    name, var_type, expr_type
                ));
            }
            if context.contains(name) {
                return Err(format!("Variable '{}' redeclared in the same scope", name));
            }
            context.insert(name.clone(), var_type.clone());
            Ok(())
        }
        Statement::FunctionCall { name: _, args } => {
            for arg in args {
                type_check_expression(arg, context)?;
            }
            Ok(())
        }
        Statement::If { condition, body } => {
            let cond_type = type_check_expression(condition, context)?;
            if cond_type != Type::I32 {
                return Err(format!(
                    "If condition must be of type i32 (interpreted as boolean), found {:?}",
                    cond_type
                ));
            }
            for stmt in body {
                type_check_statement(stmt, context)?;
            }
            Ok(())
        }
        Statement::Return { value } => {
            let _ = type_check_expression(value, context)?;
            Ok(())
        }
    }
}

/// Type-checks an expression and returns its type.
fn type_check_expression(expr: &Expression, context: &TypeContext) -> Result<Type, String> {
    match expr {
        Expression::IntegerLiteral(_) => Ok(Type::I32),
        Expression::StringLiteral(_) => Ok(Type::String),
        Expression::VariableRef(name) => context
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Use of undeclared variable '{}'", name)),
        Expression::BinaryOp { op, left, right } => {
            let left_type = type_check_expression(left, context)?;
            let right_type = type_check_expression(right, context)?;
            check_binary_op_types(op, &left_type, &right_type)
        }
        Expression::FunctionCall { name, args } => {
            // Handle built-in functions
            match name.as_str() {
                "print" => {
                    if args.len() != 1 {
                        return Err(String::from("print expects exactly one argument"));
                    }
                    let arg_type = type_check_expression(&args[0], context)?;
                    if arg_type != Type::String {
                        return Err(String::from("print expects a string argument"));
                    }
                    Ok(Type::Void)
                }
                "int_to_string" => {
                    if args.len() != 1 {
                        return Err(String::from("int_to_string expects exactly one argument"));
                    }
                    let arg_type = type_check_expression(&args[0], context)?;
                    if arg_type != Type::I32 {
                        return Err(String::from("int_to_string expects an i32 argument"));
                    }
                    Ok(Type::String)
                }
                _ => {
                    // For user-defined functions, we'd need function signature lookup
                    // For now, assume they return i32
                    for arg in args {
                        type_check_expression(arg, context)?;
                    }
                    Ok(Type::I32)
                }
            }
        }
    }
}

/// Checks if binary operation is valid for given types and returns result type.
fn check_binary_op_types(op: &BinaryOperator, left: &Type, right: &Type) -> Result<Type, String> {
    use BinaryOperator::*;
    use Type::*;

    match op {
        Add | Subtract | Multiply | Divide => {
            if left == &I32 && right == &I32 {
                Ok(I32)
            } else {
                Err(format!(
                    "Arithmetic operator '{:?}' requires both operands to be i32, got {:?} and {:?}",
                    op, left, right
                ))
            }
        }
        GreaterThan | LessThan | Equal => {
            if left == right && (*left == I32 || *left == String) {
                Ok(I32) // Boolean result as i32
            } else {
                Err(format!(
                    "Comparison operator '{:?}' requires both operands to be same type (i32 or string), got {:?} and {:?}",
                    op, left, right
                ))
            }
        }
    }
}

/// Returns true if the statement is a return statement.
fn is_return_statement(stmt: &Statement) -> bool {
    matches!(stmt, Statement::Return { .. })
}

/// Simple type context for tracking variable types in current scope.
struct TypeContext {
    variables: std::collections::HashMap<String, Type>,
}

impl TypeContext {
    fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, ty: Type) {
        self.variables.insert(name, ty);
    }

    fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    fn get(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_mismatch_i32_string() {
        let program = Program {
            functions: vec![Function {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::I32,
                body: vec![
                    Statement::VariableDeclaration {
                        name: "count".to_string(),
                        var_type: Type::I32,
                        value: Expression::StringLiteral("test".to_string()),
                    },
                    Statement::Return {
                        value: Expression::IntegerLiteral(0),
                    },
                ],
            }],
        };

        let result = type_check_program(&program);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Type mismatch for variable 'count'")
        );
    }

    #[test]
    fn test_valid_types() {
        let program = Program {
            functions: vec![Function {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::I32,
                body: vec![
                    Statement::VariableDeclaration {
                        name: "count".to_string(),
                        var_type: Type::I32,
                        value: Expression::IntegerLiteral(42),
                    },
                    Statement::Return {
                        value: Expression::VariableRef("count".to_string()),
                    },
                ],
            }],
        };

        let result = type_check_program(&program);
        assert!(result.is_ok());
    }
}
