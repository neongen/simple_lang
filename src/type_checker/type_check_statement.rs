/// Checks if a `Statement` is type-correct within the given `TypeContext`.
/// Returns `Ok(())` if valid, or an error message otherwise.
pub fn type_check_statement(statement: &Statement, context: &TypeContext) -> Result<(), String> {
    match statement {
        Statement::VariableDeclaration { name: _, var_type, value } => {
            let expr_type = type_check_expression(value, context)?;
            if &expr_type != var_type {
                return Err(format!(
                    "Type mismatch in variable declaration: expected {:?}, found {:?}",
                    var_type, expr_type
                ));
            }
            Ok(())
        }
        Statement::FunctionCall { name: _, args } => {
            // Function calls must match the signature; assume context can check function signature.
            // Here we validate argument types against function parameters.
            let (expected_param_types, _) = context
                .get_function_signature(statement)
                .ok_or_else(|| "Function not found in context".to_string())?;
            if expected_param_types.len() != args.len() {
                return Err(format!(
                    "Function call argument count mismatch: expected {}, found {}",
                    expected_param_types.len(),
                    args.len()
                ));
            }
            for (arg_expr, expected_type) in args.iter().zip(expected_param_types.iter()) {
                let arg_type = type_check_expression(arg_expr, context)?;
                if &arg_type != expected_type {
                    return Err(format!(
                        "Function call argument type mismatch: expected {:?}, found {:?}",
                        expected_type, arg_type
                    ));
                }
            }
            Ok(())
        }
        Statement::If { condition, body } => {
            let cond_type = type_check_expression(condition, context)?;
            if cond_type != Type::I32 {
                return Err(format!(
                    "If condition must be of type i32 (boolean), found {:?}",
                    cond_type
                ));
            }
            for stmt in body {
                type_check_statement(stmt, context)?
            }
            Ok(())
        }
        Statement::Return { value } => {
            let val_type = type_check_expression(value, context)?;
            if val_type != context.current_return_type() {
                return Err(format!(
                    "Return type mismatch: expected {:?}, found {:?}",
                    context.current_return_type(),
                    val_type
                ));
            }
            Ok(())
        }
    }
}

/// Helper function to type check an expression in given context.
/// Provided here assuming `type_check_expression` is available.
/// 
/// This is a placeholder to satisfy the compiler for the function above.
/// Replace with your actual implementation.
fn type_check_expression(expr: &Expression, context: &TypeContext) -> Result<Type, String> {
    // Stub for demonstration
    unimplemented!()
}

/// TypeContext trait or struct assumed to be defined with these methods:
/// - get_function_signature: returns (Vec<Type>, Type) for function param types and return type
/// - current_return_type: returns the expected return Type in current function scope
/// These are necessary for type checking function calls and return statements.
pub trait TypeContext {
    fn get_function_signature(&self, statement: &Statement) -> Option<(Vec<Type>, Type)>;
    fn current_return_type(&self) -> Type;
}

/// Dummy Type enum assumed for comparison in type checking
#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    I32,
    String,
    Void,
}

/// Dummy Statement enum variants used in this implementation for reference.
pub enum Statement {
    VariableDeclaration {
        name: String,
        var_type: Type,
        value: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
    },
    Return {
        value: Expression,
    },
}

/// Dummy Expression enum for completeness
pub enum Expression {
    // variants omitted
}
