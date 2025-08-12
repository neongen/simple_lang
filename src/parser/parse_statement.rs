// Enhanced parse_statement.rs to handle if-else statements

use crate::ast::expression_struct::Expression;
use crate::ast::statement_struct::Statement;
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
    let condition_end = trimmed
        .find(") {")
        .ok_or_else(|| "Invalid if statement format".to_string())?;

    let condition_str = &trimmed[condition_start..condition_end];
    let condition = parse_expression(condition_str)?;

    // For now, return an if statement with empty body and no else
    // The actual body will be parsed separately by the function parser
    Ok(Statement::If {
        condition,
        body: Vec::new(), // Will be filled by the function parser
        else_body: None,  // Will be filled by the function parser if else exists
    })
}

/// Parse function that handles multi-line if statements by collecting the body
/// Enhanced to support else blocks
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
    let condition_end = trimmed
        .find(") {")
        .ok_or_else(|| "Invalid if statement format".to_string())?;

    let condition_str = &trimmed[condition_start..condition_end];
    let condition = parse_expression(condition_str)?;

    // Collect body lines until we find "} else {" or "};"
    let mut body_lines = Vec::new();
    let mut lines_consumed = 0;
    let mut found_else = false;
    let mut else_block_start = 0;

    for (i, &line) in remaining_lines.iter().enumerate() {
        lines_consumed += 1;
        let line_trimmed = line.trim();

        if line_trimmed == "};" {
            break;
        } else if line_trimmed == "} else {" {
            found_else = true;
            else_block_start = i + 1;
            break;
        }

        if !line_trimmed.is_empty() {
            body_lines.push(line);
        }
    }

    // Parse if body statements
    let mut body = Vec::new();
    for body_line in body_lines {
        let trimmed_body = body_line.trim();
        if !trimmed_body.is_empty() {
            let stmt = parse_statement(trimmed_body)?;
            body.push(stmt);
        }
    }

    // Parse else body if it exists
    let mut else_body = None;
    if found_else {
        let mut else_body_lines = Vec::new();

        // Continue from where we left off to find the end of else block
        for &line in &remaining_lines[else_block_start..] {
            lines_consumed += 1;
            let line_trimmed = line.trim();

            if line_trimmed == "};" {
                break;
            }

            if !line_trimmed.is_empty() {
                else_body_lines.push(line);
            }
        }

        // Parse else body statements
        let mut else_statements = Vec::new();
        for else_line in else_body_lines {
            let trimmed_else = else_line.trim();
            if !trimmed_else.is_empty() {
                let stmt = parse_statement(trimmed_else)?;
                else_statements.push(stmt);
            }
        }

        if !else_statements.is_empty() {
            else_body = Some(else_statements);
        }
    }

    Ok((
        Statement::If {
            condition,
            body,
            else_body,
        },
        lines_consumed,
    ))
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

        if let Ok(Statement::If {
            condition: _,
            body,
            else_body,
        }) = result
        {
            assert_eq!(body.len(), 0); // Body will be empty initially
            assert!(else_body.is_none()); // No else body initially
        }
    }

    #[test]
    fn test_parse_if_else_statement_multiline() {
        let if_line = "if (num > 0) {";
        let remaining_lines = vec![
            "    print(\"Number is positive\");",
            "    print(num);",
            "} else {",
            "    print(\"Number is not positive\");",
            "};",
        ];

        let result = parse_if_statement_multiline(if_line, &remaining_lines);
        assert!(result.is_ok());

        if let Ok((
            Statement::If {
                condition: _,
                body,
                else_body,
            },
            lines_consumed,
        )) = result
        {
            assert_eq!(body.len(), 2); // Two statements in if body
            assert!(else_body.is_some()); // Should have else body
            if let Some(else_stmts) = else_body {
                assert_eq!(else_stmts.len(), 1); // One statement in else body
            }
            assert_eq!(lines_consumed, 5);
        }
    }

    #[test]
    fn test_parse_if_statement_multiline_without_else() {
        let if_line = "if (num > 0) {";
        let remaining_lines = vec![
            "    print(\"Number is positive\");",
            "    print(num);",
            "};",
        ];

        let result = parse_if_statement_multiline(if_line, &remaining_lines);
        assert!(result.is_ok());

        if let Ok((
            Statement::If {
                condition: _,
                body,
                else_body,
            },
            lines_consumed,
        )) = result
        {
            assert_eq!(body.len(), 2);
            assert!(else_body.is_none()); // Should not have else body
            assert_eq!(lines_consumed, 3);
        }
    }
}
