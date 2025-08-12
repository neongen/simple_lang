// Simple fix for parse_statement.rs to handle your specific if statement format

use crate::ast::statement_struct::Statement;
use crate::ast::expression_struct::Expression;
use crate::ast::type_struct::Type;
use crate::parser::parse_expression::parse_expression;

/// Parses a single statement line into a `Statement` AST node.
/// Enhanced to handle if statements properly.
pub fn parse_statement(line: &str) -> Result<Statement, String> {
    let trimmed = line.trim();

    // Handle if statements - check for the pattern that matches your code
    if trimmed.starts_with("if (") && trimmed.contains(") {") {
        return parse_if_statement_simple(trimmed);
    }

    // Must end with semicolon for non-if statements
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

/// Simple if statement parser for single-line format with opening brace
fn parse_if_statement_simple(line: &str) -> Result<Statement, String> {
    let trimmed = line.trim();
    
    // Find the condition part between "if (" and ") {"
    let condition_start = 4; // Length of "if ("
    let condition_end = trimmed.find(") {")
        .ok_or_else(|| "Invalid if statement format".to_string())?;
    
    let condition_str = &trimmed[condition_start..condition_end];
    let condition = parse_expression(condition_str)?;
    
    // For now, return an if statement with empty body
    // The actual body will be parsed separately by the function parser
    Ok(Statement::If {
        condition,
        body: Vec::new(), // Will be filled by the function parser
    })
}

/// Parse function that handles multi-line if statements by collecting the body
pub fn parse_if_statement_multiline(
    if_line: &str,
    remaining_lines: &[&str],
) -> Result<(Statement, usize), String> {
    // Parse the condition from the first line
    let trimmed = if_line.trim();
    
    if !trimmed.starts_with("if (") || !trimmed.contains(") {") {
        return Err("Invalid if statement format".to_string());
    }
    
    let condition_start = 4; // Length of "if ("
    let condition_end = trimmed.find(") {")
        .ok_or_else(|| "Invalid if statement format".to_string())?;
    
    let condition_str = &trimmed[condition_start..condition_end];
    let condition = parse_expression(condition_str)?;
    
    // Collect body lines until we find "};"
    let mut body_lines = Vec::new();
    let mut lines_consumed = 0;
    
    for &line in remaining_lines {
        lines_consumed += 1;
        let line_trimmed = line.trim();
        
        if line_trimmed == "};" {
            break;
        }
        
        if !line_trimmed.is_empty() {
            body_lines.push(line);
        }
    }
    
    // Parse body statements
    let mut body = Vec::new();
    for body_line in body_lines {
        let trimmed_body = body_line.trim();
        if !trimmed_body.is_empty() {
            let stmt = parse_statement(trimmed_body)?;
            body.push(stmt);
        }
    }
    
    Ok((Statement::If { condition, body }, lines_consumed))
}

/// Parses a comma-separated list of expressions.
fn parse_arguments(args_str: &str) -> Result<Vec<Expression>, String> {
    if args_str.trim().is_empty() {
        return Ok(vec![]);
    }
    args_str
        .split(',')
        .map(|arg| parse_expression(arg.trim()))
        .collect()
}

/// Parses a type string into a Type enum.
fn parse_type(type_str: &str) -> Result<Type, String> {
    match type_str {
        "i32" => Ok(Type::I32),
        "string" => Ok(Type::String),
        "void" => Ok(Type::Void),
        _ => Err(format!("Unknown type: {}", type_str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_if_statement_simple() {
        let line = "if (num > 0) {";
        let result = parse_if_statement_simple(line);
        assert!(result.is_ok());
        
        if let Ok(Statement::If { condition: _, body }) = result {
            assert_eq!(body.len(), 0); // Body will be empty initially
        }
    }

    #[test]
    fn test_parse_if_statement_multiline() {
        let if_line = "if (num > 0) {";
        let remaining_lines = vec![
            "    print(\"Number is positive\");",
            "    print(num);",
            "};"
        ];
        
        let result = parse_if_statement_multiline(if_line, &remaining_lines);
        assert!(result.is_ok());
        
        if let Ok((Statement::If { condition: _, body }, lines_consumed)) = result {
            assert_eq!(body.len(), 2);
            assert_eq!(lines_consumed, 3);
        }
    }
}