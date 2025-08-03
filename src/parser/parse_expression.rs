use crate::ast::expression_struct::Expression;
use crate::ast::binary_operator_struct::BinaryOperator;

///// Parses a string expression into an `Expression` AST node.
/////
///// Supports integer literals, string literals, variable references,
///// binary operations (+, -, *, /, >, <, ==), and simple function calls.
///// Returns a parse error string if the expression is invalid.

pub fn parse_expression(expr_str: &str) -> Result<Expression, String> {
    let expr_str = expr_str.trim();

    // Try parsing as integer literal
    if let Ok(num) = expr_str.parse::<i32>() {
        return Ok(Expression::IntegerLiteral(num));
    }

    // Try parsing as string literal
    if expr_str.starts_with('"') && expr_str.ends_with('"') && expr_str.len() >= 2 {
        let content = &expr_str[1..expr_str.len() - 1];
        return Ok(Expression::StringLiteral(content.to_string()));
    }

    // Try parsing as binary operation
    if let Some(expr) = try_parse_binary_op(expr_str)? {
        return Ok(expr);
    }

    // Try parsing as function call
    if let Some(expr) = try_parse_function_call(expr_str)? {
        return Ok(expr);
    }

    // Assume it's a variable reference (identifier)
    if is_valid_identifier(expr_str) {
        return Ok(Expression::VariableRef(expr_str.to_string()));
    }

    Err(format!("Unrecognized expression: {}", expr_str))
}

///// Attempts to parse a binary operation from a string expression.
fn try_parse_binary_op(expr_str: &str) -> Result<Option<Expression>, String> {
    let ops = [
        ("+", BinaryOperator::Add),
        ("-", BinaryOperator::Subtract),
        ("*", BinaryOperator::Multiply),
        ("/", BinaryOperator::Divide),
        (">", BinaryOperator::GreaterThan),
        ("<", BinaryOperator::LessThan),
        ("==", BinaryOperator::Equal),
    ];

    for (symbol, op_enum) in ops.iter() {
        if let Some(index) = expr_str.find(symbol) {
            let (left_str, right_str) = expr_str.split_at(index);
            let right_str = &right_str[symbol.len()..]; // skip the operator
            let left = parse_expression(left_str.trim())?;
            let right = parse_expression(right_str.trim())?;
            return Ok(Some(Expression::BinaryOp {
                op: op_enum.clone(),
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
    }

    Ok(None)
}

///// Attempts to parse a function call from a string expression.
fn try_parse_function_call(expr_str: &str) -> Result<Option<Expression>, String> {
    if let Some(paren_start) = expr_str.find('(') {
        if expr_str.ends_with(')') {
            let name = expr_str[..paren_start].trim();
            let args_str = &expr_str[paren_start + 1..expr_str.len() - 1];
            let arg_strings: Vec<&str> = args_str.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
            let mut args = Vec::new();
            for arg_str in arg_strings {
                let expr = parse_expression(arg_str)?;
                args.push(expr);
            }
            return Ok(Some(Expression::FunctionCall {
                name: name.to_string(),
                args,
            }));
        }
    }
    Ok(None)
}

///// Checks whether a string is a valid identifier (variable or function name).
fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_') && !s.chars().next().unwrap().is_numeric()
}
