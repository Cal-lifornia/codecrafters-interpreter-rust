use std::collections::VecDeque;

pub enum Token {
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
    EOF,
}

impl Token {
    pub fn parse(input: &str) -> (Vec<Token>, Vec<std::io::Error>) {
        let mut chars = input.chars();
        let mut tokens = vec![];
        let mut errs = vec![];

        loop {
            match chars.next() {
                Some(c) => match Token::try_from(c) {
                    Ok(token) => tokens.push(token),
                    Err(err) => errs.push(err),
                },
                None => {
                    tokens.push(Token::EOF);
                    break;
                }
            }
        }
        (tokens, errs)
    }
    pub fn symbol_str(&self) -> &str {
        use Token::*;
        match self {
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
            EOF => "",
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let out = match self {
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
            EOF => "EOF",
        };
        write!(f, "{out}")
    }
}

impl TryFrom<char> for Token {
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;
        let result = match value {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            ',' => Self::Comma,
            '.' => Self::Dot,
            '-' => Self::Minus,
            '+' => Self::Plus,
            ';' => Self::Semicolon,
            '*' => Self::Star,
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unexpected character: {value}"),
                ))
            }
        };
        Ok(result)
    }
}
