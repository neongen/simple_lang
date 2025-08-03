/// Performs the specified binary operation on two expressions.
/// Supports integer arithmetic (+, -, *, /) and comparisons (>, <, ==).
/// Returns an Expression with the result or an error string if operation is invalid.
pub fn binary_op(
    op: &BinaryOperator,
    left: &Expression,
    right: &Expression,
) -> Result<Expression, String> {
    // Helper to extract i32 value from Expression::IntegerLiteral or return error
    fn extract_i32(expr: &Expression) -> Result<i32, String> {
        if let Expression::IntegerLiteral(value) = expr {
            Ok(*value)
        } else {
            Err(String::from("Expected integer literal in binary operation"))
        }
    }

    // Helper to perform integer arithmetic
    fn int_arithmetic(op: &BinaryOperator, left: i32, right: i32) -> Result<Expression, String> {
        match op {
            BinaryOperator::Add => Ok(Expression::IntegerLiteral(left + right)),
            BinaryOperator::Subtract => Ok(Expression::IntegerLiteral(left - right)),
            BinaryOperator::Multiply => Ok(Expression::IntegerLiteral(left * right)),
            BinaryOperator::Divide => {
                if right == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(Expression::IntegerLiteral(left / right))
                }
            }
            _ => Err(String::from("Unsupported operator for arithmetic")),
        }
    }

    // Helper to perform comparison, returning i32 1 for true, 0 for false
    fn compare(op: &BinaryOperator, left: i32, right: i32) -> Result<Expression, String> {
        let result = match op {
            BinaryOperator::GreaterThan => left > right,
            BinaryOperator::LessThan => left < right,
            BinaryOperator::Equal => left == right,
            _ => return Err(String::from("Unsupported operator for comparison")),
        };
        Ok(Expression::IntegerLiteral(if result { 1 } else { 0 }))
    }

    match op {
        BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide => {
            let l = extract_i32(left)?;
            let r = extract_i32(right)?;
            int_arithmetic(op, l, r)
        }
        BinaryOperator::GreaterThan | BinaryOperator::LessThan | BinaryOperator::Equal => {
            let l = extract_i32(left)?;
            let r = extract_i32(right)?;
            compare(op, l, r)
        }
    }
}
