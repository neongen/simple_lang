// Final integrated parse_function.rs that works with your exact code format

use crate::ast::function_struct::Function;
use crate::ast::parameter_struct::Parameter;
use crate::ast::statement_struct::Statement;
use crate::ast::type_struct::Type;
use crate::parser::parse_statement::{parse_if_statement_multiline, parse_statement};

/// Parses a function definition from a slice of input lines.
/// Works with your exact code format including multi-line if statements.
pub fn parse_function(lines: &[&str]) -> Result<Function, String> {
    if lines.is_empty() {
        return Err("Empty function input.".to_string());
    }

    // Parse function header
    let header = lines[0].trim();
    if !header.ends_with('{') {
        return Err("Function header must end with '{'.".to_string());
    }

    let header_clean = &header[..header.len() - 1].trim_end();
    let (name, params, return_type) = parse_function_signature(header_clean)?;

    // Verify function ends properly
    if !lines.last().unwrap().trim().eq("};") {
        return Err("Function must end with '};'".to_string());
    }

    // Parse the function body
    let body_lines = &lines[1..lines.len() - 1];
    let body = parse_function_body_integrated(body_lines)?;

    Ok(Function {
        name,
        params,
        return_type,
        body,
    })
}

/// Parse function body with integrated multi-line if statement handling
fn parse_function_body_integrated(lines: &[&str]) -> Result<Vec<Statement>, String> {
    let mut body = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if this is the start of a multi-line if statement
        if line.starts_with("if (") && line.contains(") {") && !line.contains("};") {
            // This is a multi-line if statement
            let remaining_lines = &lines[i + 1..];
            let (if_statement, lines_consumed) =
                parse_if_statement_multiline(line, remaining_lines)?;
            body.push(if_statement);
            i += lines_consumed + 1; // +1 for the if line itself
        } else {
            // Regular single-line statement
            let stmt = parse_statement(line)?;
            body.push(stmt);
            i += 1;
        }
    }

    Ok(body)
}

/// Parse function signature from header
fn parse_function_signature(header: &str) -> Result<(String, Vec<Parameter>, Type), String> {
    let parts: Vec<&str> = header.splitn(2, ": function").collect();
    if parts.len() != 2 {
        return Err("Invalid function declaration syntax.".to_string());
    }

    let name = parts[0].trim().to_string();
    let signature = parts[1].trim();

    let open_paren = signature
        .find('(')
        .ok_or("Missing '(' in function signature.")?;
    let close_paren = signature
        .find(')')
        .ok_or("Missing ')' in function signature.")?;
    let params_str = &signature[open_paren + 1..close_paren];
    let return_str = signature[close_paren + 1..].trim();

    let return_type = if return_str.starts_with("->") {
        parse_type(return_str.trim_start_matches("->").trim())?
    } else {
        return Err("Missing return type in function signature.".to_string());
    };

    let mut params = Vec::new();
    if !params_str.is_empty() {
        for param in params_str.split(',') {
            let param = param.trim();
            let parts: Vec<&str> = param.split(':').map(str::trim).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid parameter syntax: '{}'", param));
            }
            params.push(Parameter {
                name: parts[0].to_string(),
                param_type: parse_type(parts[1])?,
            });
        }
    }

    Ok((name, params, return_type))
}

/// Parse type string
fn parse_type(s: &str) -> Result<Type, String> {
    match s {
        "i32" => Ok(Type::I32),
        "string" => Ok(Type::String),
        "void" => Ok(Type::Void),
        _ => Err(format!("Unknown type: '{}'", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function_with_multiline_if() {
        let lines = vec![
            "check_value: function(num: i32) -> i32 {",
            "    if (num > 0) {",
            "        print(\"Number is positive\");",
            "        print(num);",
            "    };",
            "    ",
            "    return num;",
            "};",
        ];

        let result = parse_function(&lines);
        assert!(
            result.is_ok(),
            "Should parse successfully: {:?}",
            result.err()
        );

        if let Ok(function) = result {
            assert_eq!(function.name, "check_value");
            assert_eq!(function.body.len(), 2); // if statement and return

            // Check that first statement is an if statement with 2 body statements
            if let Statement::If {
                condition: _,
                body,
                else_body,
            } = &function.body[0]
            {
                assert_eq!(body.len(), 2); // print statements
            } else {
                panic!("First statement should be an if statement");
            }
        }
    }

    #[test]
    fn test_parse_your_exact_code() {
        let lines = vec![
            "check_value: function(num: i32) -> i32 {",
            "    if (num > 0) {",
            "        print(\"Number is positive\");",
            "        print(num);",
            "    };",
            "    ",
            "    return num;",
            "};",
        ];

        let result = parse_function(&lines);
        assert!(result.is_ok(), "Your code should parse: {:?}", result.err());
    }
}
