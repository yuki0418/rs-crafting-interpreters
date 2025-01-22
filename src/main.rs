use rs_crafting_interpreters::scanner::{Scanner, ScannerError, ScannerTrait};
use std::env;

#[derive(Debug, thiserror::Error)]
enum RLoxError {
    #[error("failed to read file: {0}")]
    FailedToReadFile(String),

    #[error("scanner error: {0}")]
    ScannerError(#[from] ScannerError),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        i if i > 2 => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
        2 => {
            let file_path = &args[1];
            run_file(file_path).unwrap_or_else(report_error);
        }
        _ => {
            run_prompt().unwrap_or_else(report_error);
        }
    }
}

fn report_error(error: RLoxError) {
    eprintln!("{}", error);
}

fn run_file(file_path: &str) -> Result<(), RLoxError> {
    let file = std::fs::read_to_string(file_path)
        .map_err(|e| RLoxError::FailedToReadFile(e.to_string()))?;

    run(file)?;

    Ok(())
}

// Run command line prompt
fn run_prompt() -> Result<(), RLoxError> {
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        run(input)?;
    }
}

fn run(source: String) -> Result<(), RLoxError> {
    println!("Run source: {:?}", source);
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    println!("Tokens: {:?}", tokens);

    Ok(())
}
