use std::io::ErrorKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    Equal,
    EqualEqual,
    EOF,
}

impl Token {
    pub fn parse(input: &str) -> (Vec<Token>, Vec<std::io::Error>) {
        use Token::*;
        let mut chars = input.chars();
        let mut tokens = vec![];
        let mut errs = vec![];
        let mut last_token = Self::EOF;

        loop {
            match chars.next() {
                Some(c) => {
                    let token = match c {
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
                        _ => {
                            errs.push(std::io::Error::new(
                                ErrorKind::InvalidInput,
                                format!("Unexpected character: {c}"),
                            ));
                            last_token = EOF;
                            continue;
                        }
                    };
                    if last_token == Equal && token == Equal {
                        let _ = tokens.pop();
                        tokens.push(EqualEqual);
                        last_token = EOF;
                    } else {
                        last_token = token;
                        tokens.push(token);
                    }
                }
                None => {
                    tokens.push(EOF);
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
            Equal => "=",
            EqualEqual => "==",
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
            Equal => "EQUAL",
            EqualEqual => "EQUAL_EQUAL",
            EOF => "EOF",
        };
        write!(f, "{out}")
    }
}
