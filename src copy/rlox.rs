use crate::{scanner::Scanner, token::Token};

pub struct RLox {
    pub had_error: bool,
}

impl RLox {
    pub fn new() -> RLox {
        RLox { had_error: false }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("{:?}", e.to_string());
                return;
            }
        };

        for token in tokens {
            println!("token: {:?}", token);
        }
    }

    pub fn run_file(&mut self, path: String) {
        // Read the file
        let source = std::fs::read_to_string(path).expect("Failed to read file");
        self.run(source);
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            self.run(line);
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}

impl Default for RLox {
    fn default() -> Self {
        Self::new()
    }
}
