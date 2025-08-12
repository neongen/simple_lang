// Enhanced main.rs for simple_lang_demo_runner with control flow support
use std::path::PathBuf;
use std::env;
use simple_lang::{
    parser::parse_program::parse_program,
    evaluator::evaluate_program::evaluate_program,
    source::read_source_file::read_source_file,
};

fn main() {
    println!("=== simple_lang Demo Runner ===");
    
    // Get current working directory
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    // List of demo programs to run
    let demo_programs = vec![
        "demo_program/hello_world.lang",
        "demo_program/nested_function_calls.lang",
        "demo_program/control_flow.lang",
    ];

    for program_path in demo_programs {
        println!("{}" , "=".repeat(60));
        println!("Running: {}", program_path);
        println!("{}", "=".repeat(60));

        // Create the full PathBuf
        let full_path: PathBuf = cwd.join(program_path);

        // Convert PathBuf to &str
        let file_path = match full_path.to_str() {
            Some(p) => p,
            None => {
                eprintln!("Failed to convert file path to UTF-8 string");
                continue;
            }
        };

        // Check if file exists
        if !full_path.exists() {
            println!("⚠️  Program file not found: {}", program_path);
        }

        // Read the source file
        let source_file = match read_source_file(file_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("❌ File read error: {}", err);
                continue;
            }
        };

        println!("📄 Source code:");
        println!("{}", "-".repeat(40));
        println!("{}", source_file.content);
        println!("{}", "-".repeat(40));

        // Parse the program
        let program = match parse_program(&source_file.content) {
            Ok(p) => {
                println!("✅ Parsing successful");
                p
            }
            Err(e) => {
                eprintln!("❌ Parse error: {}", e);
                continue;
            }
        };

        println!("📊 Program info:");
        println!("   Functions: {}", program.functions.len());
        for func in &program.functions {
            println!("   - {} (params: {}, return: {:?})", 
                func.name, 
                func.params.len(), 
                func.return_type
            );
        }

        println!("\n🚀 Execution output:");
        println!("{}", "-".repeat(40));

        // Evaluate the program
        match evaluate_program(&program) {
            Ok(exit_code) => {
                println!("{}", "-".repeat(40));
                if exit_code == 0 {
                    println!("✅ Program completed successfully (exit code: {})", exit_code);
                } else {
                    println!("✅ Program completed with exit code: {}", exit_code);
                }
            }
            Err(e) => {
                println!("{}", "-".repeat(40));
                eprintln!("❌ Runtime error: {}", e);
            }
        }
    }

    println!("{}", "=".repeat(60));
    println!("Demo runner completed!");
    println!("{}", "=".repeat(60));
}
