use crate::token::Token;

#[derive(Debug)]
pub struct Scanner {
    source: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ScannerError {
    #[error("unexpected character")]
    UnexpectedCharacter,
}

pub trait ScannerTrait {
    fn new(source: String) -> Self;
    fn scan_tokens(self) -> Result<Vec<Token>, ScannerError>;
}

impl ScannerTrait for Scanner {
    fn new(source: String) -> Scanner {
        Scanner { source }
    }

    fn scan_tokens(self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens: Vec<Token> = Vec::new();
        Ok(tokens)
    }
}
