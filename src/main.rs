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
            run_file(file_path).unwrap_or_else(report_errors);
        }
        _ => {
            run_prompt().unwrap_or_else(report_errors);
        }
    }
}

fn report_errors(errors: Vec<RLoxError>) {
    for error in errors {
        eprintln!("{}", error);
    }
    std::process::exit(65);
}

fn run_file(file_path: &str) -> Result<(), Vec<RLoxError>> {
    let file = std::fs::read_to_string(file_path)
        .map_err(|e| vec![RLoxError::FailedToReadFile(e.to_string())])?;

    run(file)?;

    Ok(())
}

// Run command line prompt
fn run_prompt() -> Result<(), Vec<RLoxError>> {
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        run(input)?;
    }
}

fn run(source: String) -> Result<(), Vec<RLoxError>> {
    println!("Run source: {:?}", source);
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().map_err(|scan_errors| {
        let rlox_error: Vec<RLoxError> = scan_errors
            .into_iter()
            .map(RLoxError::ScannerError)
            .collect();
        rlox_error
    })?;

    println!("Tokens: {:?}", tokens);

    Ok(())
}
