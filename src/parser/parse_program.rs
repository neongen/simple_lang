use crate::data_struct::program_struct::Program;
use crate::data_struct::function_struct::Function;
use crate::parser::parse_function::parse_function;
use std::string::String;
use std::vec::Vec;

/// Parses a complete program from the source string.
/// Fixed to properly handle multi-line if statements and other block constructs.
pub fn parse_program(source: &str) -> Result<Program, String> {
    let mut functions: Vec<Function> = Vec::new();

    // Normalize line endings and preserve the normalized string
    let normalized = source.replace("\r\n", "\n");
    
    // Filter and collect lines, preserving empty lines within functions
    let all_lines: Vec<&str> = normalized.lines().collect();
    let mut processed_lines = Vec::new();
    
    for line in all_lines {
        let trimmed = line.trim();
        // Keep all lines that aren't purely comment lines
        if !trimmed.starts_with("//") {
            processed_lines.push(line);
        }
    }

    // Parse functions by identifying complete blocks with proper brace tracking
    let mut current_block: Vec<&str> = Vec::new();
    let mut brace_depth = 0;
    let mut in_function = false;
    let mut i = 0;

    while i < processed_lines.len() {
        let line = processed_lines[i];
        let trimmed = line.trim();
        
        // Skip empty lines outside functions
        if !in_function && trimmed.is_empty() {
            i += 1;
            continue;
        }

        current_block.push(line);

        // Detect function start
        if trimmed.contains(": function(") && trimmed.ends_with("{") {
            in_function = true;
            brace_depth = 1;
        } else if in_function {
            // Count braces more carefully
            brace_depth += count_net_braces(trimmed);

            // Function complete when we return to brace_depth 0 and see };
            if brace_depth == 0 && trimmed.ends_with("};") {
                let func = parse_function_block(&current_block)?;
                functions.push(func);
                current_block.clear();
                in_function = false;
            } else if brace_depth < 0 {
                return Err("Unmatched closing brace in function".to_string());
            }
        }
        
        i += 1;
    }

    if in_function {
        return Err("Incomplete function block found at end of source.".to_string());
    }

    if !current_block.is_empty() {
        return Err("Unexpected content found outside function blocks.".to_string());
    }

    Ok(Program { functions })
}

/// Counts the net brace difference in a line (opening braces - closing braces)
fn count_net_braces(line: &str) -> i32 {
    let mut net_braces = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for ch in line.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => net_braces += 1,
            '}' if !in_string => net_braces -= 1,
            _ => {}
        }
    }

    net_braces
}

/// Parses a function block from lines of source code.
/// Enhanced to handle multi-line constructs properly.
fn parse_function_block(lines: &[&str]) -> Result<Function, String> {
    if lines.is_empty() {
        return Err("Empty function block".to_string());
    }
    
    // Use the enhanced function parser
    parse_function(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_net_braces() {
        assert_eq!(count_net_braces("if (x > 0) {"), 1);
        assert_eq!(count_net_braces("    };"), -1);
        assert_eq!(count_net_braces("{ print(\"hello\"); }"), 0);
        assert_eq!(count_net_braces("print(\"test { } test\");"), 0); // Braces in strings don't count
    }

    #[test]
    fn test_parse_program_with_multiline_if() {
        let source = r#"
check_value: function(num: i32) -> i32 {
    if (num > 0) {
        print("Number is positive");
        print(num);
    };
    
    return num;
};

main: function() -> i32 {
    result: i32 = check_value(42);
    return 0;
};
"#;
        let result = parse_program(source);
        assert!(result.is_ok(), "Parse should succeed: {:?}", result.err());
        
        if let Ok(program) = result {
            assert_eq!(program.functions.len(), 2);
            assert_eq!(program.functions[0].name, "check_value");
            assert_eq!(program.functions[1].name, "main");
        }
    }

    #[test]
    fn test_parse_program_with_complex_example() {
        let source = r#"
add_numbers: function(a: i32, b: i32) -> i32 {
    return a + b;
};

check_value: function(num: i32) -> i32 {
    if (num > 0) {
        print("Number is positive");
        print(num);
    };
    
    return num;
};

main: function() -> i32 {
    message: string = "Hello, World! Your code belongs to the Entity!";
    count: i32 = 42;
    
    print(message);
    
    result: i32 = add_numbers(count, 8);
    print(result);
    
    result_checked: i32 = check_value(result);
    return 0;
};
"#;
        let result = parse_program(source);
        assert!(result.is_ok(), "Parse should succeed: {:?}", result.err());
        
        if let Ok(program) = result {
            assert_eq!(program.functions.len(), 3);
        }
    }
}