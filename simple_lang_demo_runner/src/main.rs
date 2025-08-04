use std::path::PathBuf;
use std::env;
use simple_lang::{
    parser::parse_program::parse_program,
    evaluator::evaluate_program::evaluate_program,
    source::read_source_file::read_source_file,
};

fn main() {
    // Get current working directory
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    // Create the full PathBuf first to ensure it lives long enough
    let full_path: PathBuf = cwd.join("demo_program/nested_function_calls.lang");

    // Convert PathBuf to &str
    let file_path = match full_path.to_str() {
        Some(p) => p,
        None => {
            eprintln!("Failed to convert file path to UTF-8 string");
            return;
        }
    };

    // Read the source file
    let source_file = match read_source_file(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("File read error: {}", err);
            return;
        }
    };

    // Parse the program
    let program = match parse_program(&source_file.content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            return;
        }
    };

    // Evaluate the program
    match evaluate_program(&program) {
        Ok(exit_code) => {
            println!("Program exited with code: {}", exit_code);
        }
        Err(e) => {
            eprintln!("Runtime error: {}", e);
        }
    }
}
