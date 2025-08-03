/// Type context holds variable and function type information for type checking.
pub struct TypeContext {
    pub variables: std::collections::HashMap<String, Type>,
    pub functions: std::collections::HashMap<String, FunctionSignature>,
}

/// Represents a function's parameter types and return type.
pub struct FunctionSignature {
    pub param_types: Vec<Type>,
    pub return_type: Type,
}

/// Checks the type of an expression within a given type context.
/// Returns the expression's type on success, or an error message on failure.
pub fn type_check_expression(expr: &Expression, context: &TypeContext) -> Result<Type, String> {
    match expr {
        Expression::IntegerLiteral(_) => Ok(Type::I32),

        Expression::StringLiteral(_) => Ok(Type::String),

        Expression::VariableRef(name) => context
            .variables
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable '{}'", name)),

        Expression::BinaryOp { op, left, right } => {
            let left_type = type_check_expression(left, context)?;
            let right_type = type_check_expression(right, context)?;
            check_binary_op_types(op, &left_type, &right_type)
        }

        Expression::FunctionCall { name, args } => {
            let func_sig = context
                .functions
                .get(name)
                .ok_or_else(|| format!("Undefined function '{}'", name))?;

            if func_sig.param_types.len() != args.len() {
                return Err(format!(
                    "Function '{}' expects {} arguments, got {}",
                    name,
                    func_sig.param_types.len(),
                    args.len()
                ));
            }

            for (i, (arg_expr, expected_type)) in args.iter().zip(&func_sig.param_types).enumerate() {
                let arg_type = type_check_expression(arg_expr, context)?;
                if &arg_type != expected_type {
                    return Err(format!(
                        "Argument {} of function '{}' expected type {:?}, got {:?}",
                        i + 1,
                        name,
                        expected_type,
                        arg_type
                    ));
                }
            }

            Ok(func_sig.return_type.clone())
        }
    }
}

/// Helper function to check types for binary operators.
/// Returns resulting type if valid, otherwise an error message.
fn check_binary_op_types(op: &BinaryOperator, left: &Type, right: &Type) -> Result<Type, String> {
    use BinaryOperator::*;
    use Type::*;

    match op {
        Add | Subtract | Multiply | Divide => {
            if left == &I32 && right == &I32 {
                Ok(I32)
            } else {
                Err(format!(
                    "Operator '{:?}' requires both operands to be i32, got {:?} and {:?}",
                    op, left, right
                ))
            }
        }
        GreaterThan | LessThan | Equal => {
            // For simplicity, only allow comparisons on i32 and strings (both operands same type)
            if left == right && (*left == I32 || *left == String) {
                Ok(I32) // Boolean as i32 (e.g., 0 or 1)
            } else {
                Err(format!(
                    "Operator '{:?}' requires both operands to be same type i32 or string, got {:?} and {:?}",
                    op, left, right
                ))
            }
        }
    }
}
