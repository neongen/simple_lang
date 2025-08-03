/// Converts an Expression into its string representation.
/// Returns Err if the expression contains unsupported constructs.
pub fn stringify_expression(expr: &Expression) -> Result<String, String> {
    match expr {
        Expression::IntegerLiteral(value) => Ok(value.to_string()),

        Expression::StringLiteral(text) => {
            // Return quoted string literal
            Ok(format!("\"{}\"", text))
        }

        Expression::VariableRef(name) => Ok(name.clone()),

        Expression::BinaryOp { op, left, right } => {
            let left_str = stringify_expression(left)?;
            let right_str = stringify_expression(right)?;
            let op_str = stringify_operator(op)?;
            Ok(format!("({} {} {})", left_str, op_str, right_str))
        }

        Expression::FunctionCall { name, args } => {
            let mut arg_strs = Vec::with_capacity(args.len());
            for arg in args {
                arg_strs.push(stringify_expression(arg)?);
            }
            Ok(format!("{}({})", name, arg_strs.join(", ")))
        }
    }
}

/// Converts a BinaryOperator into its string symbol.
fn stringify_operator(op: &BinaryOperator) -> Result<&'static str, String> {
    match op {
        BinaryOperator::Add => Ok("+"),
        BinaryOperator::Subtract => Ok("-"),
        BinaryOperator::Multiply => Ok("*"),
        BinaryOperator::Divide => Ok("/"),
        BinaryOperator::GreaterThan => Ok(">"),
        BinaryOperator::LessThan => Ok("<"),
        BinaryOperator::Equal => Ok("=="),
    }
}
