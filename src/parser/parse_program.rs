use crate::ast::program_struct::Program;
use crate::ast::function_struct::Function;
use std::string::String;
use std::vec::Vec;

/// Parses a complete program from the source string.
/// Assumes each function is separated by double newlines or a terminating semicolon.
/// Returns a `Program` containing all parsed functions or an error string.
pub fn parse_program(source: &str) -> Result<Program, String> {
    let mut functions: Vec<Function> = Vec::new();

    // Normalize line endings and bind to variable to ensure lifetime is preserved
    let normalized = source.replace("\r\n", "\n");

    let lines: Vec<&str> = normalized
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect();

    // Identify function blocks based on trailing semicolons
    let mut current_block: Vec<&str> = Vec::new();
    for line in lines {
        current_block.push(line);

        if line.ends_with("};") {
            let func = parse_function_block(&current_block)?;
            functions.push(func);
            current_block.clear();
        }
    }

    if !current_block.is_empty() {
        return Err("Incomplete function block found at end of source.".to_string());
    }

    Ok(Program { functions })
}

/// Parses a function block from lines of source code.
/// Expects the lines to represent a complete function ending in `};`.
fn parse_function_block(lines: &[&str]) -> Result<Function, String> {
    if lines.is_empty() {
        return Err("Empty function block".to_string());
    }
    crate::parser::parse_function::parse_function(lines)
}
