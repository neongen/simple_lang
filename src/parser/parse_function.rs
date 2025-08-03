use crate::ast::type_struct::Type;
use crate::ast::function_struct::Function;
use crate::ast::parameter_struct::Parameter;
use crate::parser::parse_statement::parse_statement;

///// Parses a function definition from a slice of input lines.
/////
///// Expects well-formed syntax like:
///// `my_func: function(a: i32, b: i32) -> i32 {` followed by body lines and closing `};`
///// Returns a `Function` AST node or an error message.
pub fn parse_function(lines: &[&str]) -> Result<Function, String> {
    if lines.is_empty() {
        return Err("Empty function input.".to_string());
    }

    let header = lines[0].trim();
    if !header.ends_with('{') {
        return Err("Function header must end with '{'.".to_string());
    }

    // Remove trailing '{' and parse the function signature
    let header_clean = &header[..header.len() - 1].trim_end();
    let (name, params, return_type) = parse_function_signature(header_clean)?;

    // Find the closing "};" line
    if !lines.last().unwrap().trim().eq("};") {
        return Err("Function must end with '};'".to_string());
    }

    let body_lines = &lines[1..lines.len() - 1];
    let mut body = Vec::new();
    for &line in body_lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stmt = parse_statement(trimmed)?;
        body.push(stmt);
    }

    Ok(Function {
        name,
        params,
        return_type,
        body,
    })
}

///// Parses the function signature from a header line like:
///// `add_numbers: function(a: i32, b: i32) -> i32`
fn parse_function_signature(header: &str) -> Result<(String, Vec<Parameter>, Type), String> {
    // Split into name and rest
    let parts: Vec<&str> = header.splitn(2, ": function").collect();
    if parts.len() != 2 {
        return Err("Invalid function declaration syntax.".to_string());
    }

    let name = parts[0].trim().to_string();
    let signature = parts[1].trim();

    // Extract parameters and return type
    let open_paren = signature.find('(').ok_or("Missing '(' in function signature.")?;
    let close_paren = signature.find(')').ok_or("Missing ')' in function signature.")?;
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

///// Parses a type string like "i32", "string", or "void" into a `Type`.
fn parse_type(s: &str) -> Result<Type, String> {
    match s {
        "i32" => Ok(Type::I32),
        "string" => Ok(Type::String),
        "void" => Ok(Type::Void),
        _ => Err(format!("Unknown type: '{}'", s)),
    }
}
