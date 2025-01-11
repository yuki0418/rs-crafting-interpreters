use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String, // The actual text of the token.
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"".to_string())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl TokenType {
    pub fn get_token_type(str: &str) -> Option<TokenType> {
        match str {
            "(" => Some(TokenType::LEFT_PAREN),
            ")" => Some(TokenType::RIGHT_PAREN),
            "{" => Some(TokenType::LEFT_BRACE),
            "}" => Some(TokenType::RIGHT_BRACE),
            "," => Some(TokenType::COMMA),
            "." => Some(TokenType::DOT),
            "-" => Some(TokenType::MINUS),
            "+" => Some(TokenType::PLUS),
            ";" => Some(TokenType::SEMICOLON),
            "*" => Some(TokenType::STAR),
            "!" => Some(TokenType::BANG),
            "!=" => Some(TokenType::BANG_EQUAL),
            "=" => Some(TokenType::EQUAL),
            "==" => Some(TokenType::EQUAL_EQUAL),
            ">" => Some(TokenType::GREATER),
            ">=" => Some(TokenType::GREATER_EQUAL),
            "<" => Some(TokenType::LESS),
            "<=" => Some(TokenType::LESS_EQUAL),
            _ => None,
        }
    }
}
