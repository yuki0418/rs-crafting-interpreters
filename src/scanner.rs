use crate::token::{Literal, Token, TokenTrait, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ScannerError {
    #[error("unexpected character at line: {0}, current: {1}, character: {2}")]
    UnexpectedCharacter(u32, usize, char),
    #[error("next character not found: {0}")]
    NextCharacterNotFound(String),
    #[error("unterminated string at line: {0}, current: {1}")]
    UnterminatedString(u32, usize),
    #[error("failed to parse number at line: {0}, current: {1}")]
    FailedToParseNumber(u32, usize),
}

pub trait ScannerTrait {
    fn new(source: String) -> Self;
    fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScannerError>>;
    fn scan_token(&mut self) -> Result<(), ScannerError>;
    /// Add a token with a literal
    fn add_token(&mut self, token_type: TokenType, literal: Literal);
    /// Add a token with a null literal
    fn add_null_token(&mut self, token_type: TokenType);
    fn add_string(&mut self) -> Result<(), ScannerError>;
    fn add_number(&mut self) -> Result<(), ScannerError>;
    fn add_identifier(&mut self) -> Result<(), ScannerError>;
    fn is_at_end(&self) -> bool;
    fn is_alpha(&self, c: char) -> bool;
    fn is_alpha_numeric(&self, c: char) -> bool;
    fn is_digit(&self, c: char) -> bool;
    /// Advance the current character and return the character
    fn advance(&mut self) -> Option<char>;
    /// Peek the current character, if it not exists, return \0
    fn peek(&self) -> char;
    /// Peek the next character, if it not exists, return \0
    fn peek_next(&self) -> char;
    /// Check if the next character matches the expected character
    fn char_match(&mut self, expected: char) -> Result<bool, ScannerError>;
}

impl ScannerTrait for Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        let mut errors: Vec<ScannerError> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }
        }

        let end_token = Token::new(TokenType::Eof, "".to_string(), Literal::Null, self.line);
        self.tokens.push(end_token);

        if !errors.is_empty() {
            return Err(errors.clone());
        }

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        let char = self.advance();
        let char = match char {
            Some(c) => c,
            None => {
                return Err(ScannerError::NextCharacterNotFound(format!(
                    "line: {}, current: {}",
                    self.line, self.current,
                )))
            }
        };

        match char {
            '(' => self.add_null_token(TokenType::LeftParen),
            ')' => self.add_null_token(TokenType::RightParen),
            '{' => self.add_null_token(TokenType::LeftBrace),
            '}' => self.add_null_token(TokenType::RightBrace),
            ',' => self.add_null_token(TokenType::Comma),
            '.' => self.add_null_token(TokenType::Dot),
            '-' => self.add_null_token(TokenType::Minus),
            '+' => self.add_null_token(TokenType::Plus),
            '*' => self.add_null_token(TokenType::Star),
            ';' => self.add_null_token(TokenType::Semicolon),
            '!' => {
                let token_type = if self.char_match('=')? {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_null_token(token_type);
            }
            '=' => {
                let token_type = if self.char_match('=')? {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_null_token(token_type);
            }
            '<' => {
                let token_type = if self.char_match('=')? {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_null_token(token_type);
            }
            '>' => {
                let token_type = if self.char_match('=')? {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_null_token(token_type);
            }
            '/' => {
                if self.char_match('/')? {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_null_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => self.add_string()?,
            _ => {
                if self.is_digit(char) {
                    self.add_number()?;
                } else if self.is_alpha(char) {
                    self.add_identifier()?;
                } else {
                    return Err(ScannerError::UnexpectedCharacter(
                        self.line,
                        self.current,
                        char,
                    ));
                }
            }
        };

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }

    fn add_null_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::Null);
    }

    /// Add a string token
    /// Iterate through the source until the closing " is found
    fn add_string(&mut self) -> Result<(), ScannerError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString(self.line, self.current));
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Literal::String(value));

        Ok(())
    }

    fn add_number(&mut self) -> Result<(), ScannerError> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Literal::Number(
                self.source[self.start..self.current]
                    .parse()
                    .map_err(|_| ScannerError::FailedToParseNumber(self.line, self.current))?,
            ),
        );

        Ok(())
    }

    fn add_identifier(&mut self) -> Result<(), ScannerError> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "null" => TokenType::Null,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_null_token(token_type);

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn char_match(&mut self, expected: char) -> Result<bool, ScannerError> {
        if self.is_at_end() {
            return Ok(false);
        }

        let char = match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => {
                return Err(ScannerError::NextCharacterNotFound(format!(
                    "line: {}, current: {}",
                    self.line, self.current,
                )))
            }
        };

        if char != expected {
            return Ok(false);
        }

        self.current += 1;
        Ok(true)
    }
}
