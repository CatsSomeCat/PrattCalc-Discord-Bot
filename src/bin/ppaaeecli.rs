use ppaaeedb::core::{evaluate, execute, SymbolTable, CalcError};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

// Define the allowed file extension
const ALLOWED_EXTENSION: &str = "pc";

fn print_usage(program_name: &str) {
    println!("Usage:");
    println!("  {} \"expression\"        Evaluate a single expression", program_name);
    println!("  {} --interactive | -i    Start interactive mode", program_name);
    println!("  {} --file | -f <path>    Evaluate expressions from .{} file line by line", program_name, ALLOWED_EXTENSION);
    println!("  {} --script | -s <path>  Evaluate .{} file as complete script", program_name, ALLOWED_EXTENSION);
    println!("  {} --help | -h           Show this help", program_name);
}

fn evaluate_expression(expression: &str, context: &mut SymbolTable<f32>) -> Result<Option<String>, String> {
    match evaluate(expression, context) {
        Ok(result) => Ok(Some(result.to_string())),
        Err(CalcError::Parse(err)) => Err(format!("SyntaxError: {}", err)),
        Err(CalcError::Eval(err)) => Err(format!("RuntimeError: {}", err)),
        Err(CalcError::Exec(err)) => Err(format!("ExecutionError: {}", err)),
    }
}

fn execute_statement(statement: &str, context: &mut SymbolTable<f32>) -> Result<Option<String>, String> {
    match execute(statement, context) {
        Ok(result) => Ok(result.map(|val| val.to_string())),
        Err(CalcError::Parse(err)) => Err(format!("SyntaxError: {}", err)),
        Err(CalcError::Eval(err)) => Err(format!("RuntimeError: {}", err)),
        Err(CalcError::Exec(err)) => Err(format!("ExecutionError: {}", err)),
    }
}

/// Displays all variables and their values from the context
fn list_variables(context: &SymbolTable<f32>) {
    // Sort variables by name for consistent display
    let mut vars: Vec<(&String, &f32)> = context.values.iter().collect();
    vars.sort_by(|a, b| a.0.cmp(b.0));
    
    if vars.is_empty() {
        println!("No variables defined.");
        return;
    }
    
    // Find the longest variable name for pretty formatting
    let max_name_len = vars.iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);
    
    // Print each variable with its value
    for (name, &value) in vars {
        let constant_marker = if context.is_constant(name) { " (constant)" } else { "" };
        println!("{:width$} = {}{}", name, value, constant_marker, width = max_name_len);
    }
}

fn interactive_mode(context: &mut SymbolTable<f32>) -> Result<(), Box<dyn Error>> {
    println!("Interactive calculator mode");
    println!("Type \"exit()\" or \"quit()\" to exit");
    println!("Type \"vars()\" to list all defined variables");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!(">>> ");
        stdout.flush()?;
        
        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        
        let input = input.trim();
        
        // Handle special commands
        match input.to_lowercase().as_str() {
            "exit()" | "quit()" => {
                break;
            }
            "vars()" => {
                list_variables(context);
                continue;
            }
            "" => continue,  // Skip empty lines
            _ => {}  // Continue with normal expression evaluation
        }
        
        // Try to execute as a statement first, then fall back to expression
        match execute_statement(input, context) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {}  // No output for statements with no return value
            Err(error) => eprintln!("{}", error),
        }
    }
    
    Ok(())
}

/// Check if the given file has the allowed extension
fn has_allowed_extension(file_path: &str) -> bool {
    Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| ext == ALLOWED_EXTENSION)
}

fn file_mode(file_path: &str, context: &mut SymbolTable<f32>, whole_script: bool) -> Result<(), Box<dyn Error>> {
    // Validate file extension
    if !has_allowed_extension(file_path) {
        return Err(format!("Error: File must have .{} extension", ALLOWED_EXTENSION).into());
    }

    println!("Executing file: {}", file_path);
    
    if whole_script {
        // Read the entire file at once and evaluate it as a single script
        let mut file = File::open(file_path)?;
        let mut script = String::new();
        file.read_to_string(&mut script)?;
        
        // Treat the entire file as a single script
        match execute_statement(&script, context) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {}  // No output for statements with no return value
            Err(error) => eprintln!("{}", error),
        }
    } else {
        // Line by line mode
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed = line.trim();
            
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
                continue; // Skip empty lines and comments (both // and Python-style #)
            }
            
            match execute_statement(trimmed, context) {
                Ok(Some(result)) => println!("{}", result),
                Ok(None) => {}  // No output for statements with no return value
                Err(error) => {
                    eprintln!("{}", error);
                    // Continue processing the file despite errors
                }
            }
        }
    }
    
    Ok(())
} 

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Create a context with pre-defined constants
    let mut context = SymbolTable::new();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(());
    }
    
    match args[1].as_str() {
        "--help" | "-h" => {
            print_usage(&args[0]);
        }
        "--interactive" | "-i" => {
            interactive_mode(&mut context)?;
        }
        "--file" | "-f" => {
            if args.len() < 3 {
                println!("Error: Missing file path");
                print_usage(&args[0]);
                return Ok(());
            }
            file_mode(&args[2], &mut context, false)?; // Line by line mode
        }
        "--script" | "-s" => {
            if args.len() < 3 {
                println!("Error: Missing file path");
                print_usage(&args[0]);
                return Ok(());
            }
            file_mode(&args[2], &mut context, true)?; // Whole script mode
        }
        _ => {
            // Treat as direct expression evaluation (original behavior)
            let expression = &args[1];
            match evaluate_expression(expression, &mut context) {
                Ok(Some(result)) => println!("{}", result),
                Ok(None) => {}  // No output for statements with no return value
                Err(error) => eprintln!("{}", error),
            }
        }
    }
    
    Ok(())
} 
