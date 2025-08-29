use std::io::ErrorKind;

use crate::tokens::TokenDisplay;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolToken {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Equal,
    Bang,
    Less,
    Greater,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
    Slash,
}

impl TryFrom<char> for SymbolToken {
    type Error = std::io::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use SymbolToken::*;
        let out = match value {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            ';' => Semicolon,
            '*' => Star,
            '=' => Equal,
            '!' => Bang,
            '<' => Less,
            '>' => Greater,
            '/' => Slash,
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unexpected character: {value}"),
                ))
            }
        };
        Ok(out)
    }
}

impl TokenDisplay for SymbolToken {
    fn lexeme(&self) -> String {
        use SymbolToken::*;
        let out = match self {
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            Semicolon => ";",
            Star => "*",
            Equal => "=",
            Bang => "!",
            Less => "<",
            Greater => ">",
            Slash => "/",
            EqualEqual => "==",
            BangEqual => "!=",
            GreaterEqual => ">=",
            LessEqual => "<=",
        };
        out.to_string()
    }
    fn literal(&self) -> String {
        "null".to_string()
    }

    fn type_str(&self) -> &str {
        use SymbolToken::*;
        match self {
            LeftParen => "LEFT_PAREN",
            RightParen => "RIGHT_PAREN",
            LeftBrace => "LEFT_BRACE",
            RightBrace => "RIGHT_BRACE",
            Comma => "COMMA",
            Dot => "DOT",
            Minus => "MINUS",
            Plus => "PLUS",
            Semicolon => "SEMICOLON",
            Star => "STAR",
            Equal => "EQUAL",
            Bang => "BANG",
            Less => "LESS",
            Greater => "GREATER",
            Slash => "SLASH",
            EqualEqual => "EQUAL_EQUAL",
            BangEqual => "BANG_EQUAL",
            LessEqual => "LESS_EQUAL",
            GreaterEqual => "GREATER_EQUAL",
        }
    }
}
