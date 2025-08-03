use crate::ast::statement_struct::Statement;
use crate::ast::expression_struct::Expression;
use crate::ast::type_struct::Type;
use crate::parser::parse_expression::parse_expression;

/// Parses a single statement line into a `Statement` AST node.
/// Returns an error if the syntax is invalid.
pub fn parse_statement(line: &str) -> Result<Statement, String> {
    let trimmed = line.trim();

    // Must end with semicolon
    if !trimmed.ends_with(';') {
        return Err(String::from("Statement must end with a semicolon ';'"));
    }

    let content = &trimmed[..trimmed.len() - 1]; // remove semicolon

    // Return statement
    if content.starts_with("return ") {
        let expr_str = content["return ".len()..].trim();
        let expr = parse_expression(expr_str)?;
        return Ok(Statement::Return { value: expr });
    }

    // If statement (e.g., if (x > 0) { ... }; handled elsewhere, not here)
    if content.starts_with("if ") {
        return Err(String::from("If statement must be parsed at block level"));
    }

    // Variable declaration (e.g., name: type = expression)
    if let Some(idx_eq) = content.find('=') {
        let (left, right) = content.split_at(idx_eq);
        let right_expr_str = right[1..].trim(); // Skip '='

        let left = left.trim();
        let parts: Vec<&str> = left.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(String::from("Invalid variable declaration syntax"));
        }
        let name = parts[0].to_string();
        let var_type_str = parts[1];
        let var_type = parse_type(var_type_str)?;
        let expr = parse_expression(right_expr_str)?;

        return Ok(Statement::VariableDeclaration {
            name,
            var_type,
            value: expr,
        });
    }

    // Function call (e.g., print("hello"))
    if let Some(idx_paren) = content.find('(') {
        if content.ends_with(')') {
            let name = content[..idx_paren].trim().to_string();
            let args_str = &content[idx_paren + 1..content.len() - 1];
            let args = parse_arguments(args_str)?;
            return Ok(Statement::FunctionCall { name, args });
        }
    }

    Err(String::from("Unrecognized statement syntax"))
}

/// Parses a comma-separated list of expressions.
/// Used for function call arguments.
fn parse_arguments(args_str: &str) -> Result<Vec<Expression>, String> {
    if args_str.trim().is_empty() {
        return Ok(vec![]);
    }
    args_str
        .split(',')
        .map(|arg| parse_expression(arg.trim()))
        .collect()
}

/// Parses a type string like "i32" or "string" into a `Type` enum.
fn parse_type(type_str: &str) -> Result<Type, String> {
    match type_str {
        "i32" => Ok(Type::I32),
        "string" => Ok(Type::String),
        "void" => Ok(Type::Void),
        _ => Err(format!("Unknown type: {}", type_str)),
    }
}
