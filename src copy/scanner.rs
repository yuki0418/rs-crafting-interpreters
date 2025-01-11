use crate::{
    error::Error,
    token::{Token, TokenType},
    Result,
};

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        let token = Token::new(TokenType::EOF, "".to_string(), None, self.line);
        self.tokens.push(token);
        Ok(self.tokens.clone())
    }

    pub fn scan_token(&mut self) -> Result<()> {
        let c: char = self.advance()?;

        match c {
            '(' => self.add_empty_token(TokenType::LEFT_PAREN),
            ')' => self.add_empty_token(TokenType::RIGHT_PAREN),
            '{' => self.add_empty_token(TokenType::LEFT_BRACE),
            '}' => self.add_empty_token(TokenType::RIGHT_BRACE),
            ',' => self.add_empty_token(TokenType::COMMA),
            '.' => self.add_empty_token(TokenType::DOT),
            '-' => self.add_empty_token(TokenType::MINUS),
            '+' => self.add_empty_token(TokenType::PLUS),
            ';' => self.add_empty_token(TokenType::SEMICOLON),
            '*' => self.add_empty_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_empty_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_empty_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_empty_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_empty_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_empty_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_empty_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_empty_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_empty_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                } else {
                    self.add_empty_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            _ => {
                if self.is_digit(c) {
                    self.number()?;
                } else if self.is_alpha(c) {
                    self.identifier()?;
                } else {
                    Err(Error::ScannerError(
                        self.line,
                        self.start,
                        format!("Unexpected character: {}", c),
                    ))?;
                }
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Returns the current character in the source code and advances the current pointer.
    fn advance(&mut self) -> Result<char> {
        let c = match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => {
                return Err(Error::ScannerError(
                    self.line,
                    self.current,
                    "Unexpected end of file".to_string(),
                ));
            }
        };
        self.current += 1;

        Ok(c)
    }

    fn add_empty_token(&mut self, toke_type: TokenType) {
        self.add_token(toke_type, None);
    }

    fn add_token(&mut self, toke_type: TokenType, literal: Option<String>) {
        // let text: String = self.source.substring(start, current);
        let text: String = self.source[self.start..self.current].to_string();
        let token = Token::new(toke_type, text, literal, self.line);
        self.tokens.push(token);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self
            .source
            .chars()
            .nth(self.current)
            .expect("No char found")
            != expected
        {
            return false;
        }

        self.current += 1;
        true
    }

    /// Returns the current character in the source code.
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current)
            .expect("No char found");
    }

    /// Returns the next character in the source code.
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .expect("No char found");
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(Error::ScannerError(
                self.line,
                self.current,
                "Unterminated string".to_string(),
            ));
        }

        // The closing ".
        self.advance()?;

        // Trim the surrounding quotes.
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::STRING, Some(value));

        Ok(())
    }

    fn is_digit(&mut self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) -> Result<()> {
        let peek = self.peek();
        while self.is_digit(peek) {
            self.advance()?;
        }

        // Look for a fractional part.
        let peek = self.peek();
        let next_peek = self.peek_next();
        if peek == '.' && self.is_digit(next_peek) {
            // Consume the "."
            self.advance()?;

            let peek = self.peek();
            while self.is_digit(peek) {
                self.advance()?;
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::NUMBER, Some(value.to_string()));

        Ok(())
    }

    fn identifier(&mut self) -> Result<()> {
        let mut peek = self.peek();
        while self.is_alpha_numeric(peek) {
            self.advance()?;
            peek = self.peek();
        }

        let text: String = self.source[self.start..self.current].to_string();
        let token_type: TokenType =
            TokenType::get_token_type(&text).map_or(TokenType::IDENTIFIER, |t| t);
        self.add_empty_token(token_type);

        Ok(())
    }

    fn is_alpha(&mut self, c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scan_tokens() {
        let source = "var a = 1;".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 6);

        let token: Token = Token::new(TokenType::IDENTIFIER, "var".to_string(), None, 1);
        assert_eq!(tokens[0], token);

        let token: Token = Token::new(TokenType::IDENTIFIER, "a".to_string(), None, 1);
        assert_eq!(tokens[1], token);

        let token: Token = Token::new(TokenType::EQUAL, "=".to_string(), None, 1);
        assert_eq!(tokens[2], token);

        let token: Token = Token::new(TokenType::NUMBER, "1".to_string(), Some("1".to_string()), 1);
        assert_eq!(tokens[3], token);

        let token: Token = Token::new(TokenType::SEMICOLON, ";".to_string(), None, 1);
        assert_eq!(tokens[4], token);

        let token: Token = Token::new(TokenType::EOF, "".to_string(), None, 1);
        assert_eq!(tokens[5], token);
    }
}
