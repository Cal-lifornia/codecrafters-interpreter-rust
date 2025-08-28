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
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    EOF,
}

impl Token {
    pub fn parse(input: &str) -> (Vec<Token>, Vec<std::io::Error>) {
        use Token::*;
        let chars = input.chars();
        let mut tokens = vec![];
        let mut errs = vec![];
        let mut last_token = Self::EOF;

        for c in chars {
            let current_token = match c {
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
                '\t' | ' ' => {
                    last_token = EOF;
                    continue;
                }
                _ => {
                    errs.push(std::io::Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unexpected character: {c}"),
                    ));
                    last_token = EOF;
                    continue;
                }
            };
            if last_token == Slash && current_token == Slash {
                let _ = tokens.pop();
                break;
            } else if matches!(last_token, Equal | Bang | Less | Greater) && current_token == Equal
            {
                let _ = tokens.pop();
                match last_token {
                    Bang => tokens.push(BangEqual),
                    Equal => tokens.push(EqualEqual),
                    Less => tokens.push(LessEqual),
                    Greater => tokens.push(GreaterEqual),
                    _ => unreachable!(),
                }
                last_token = EOF;
            } else {
                last_token = current_token;
                tokens.push(current_token);
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
            Bang => "!",
            BangEqual => "!=",
            Less => "<",
            LessEqual => "<=",
            Greater => ">",
            GreaterEqual => ">=",
            Slash => "/",
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
            Bang => "BANG",
            BangEqual => "BANG_EQUAL",
            Less => "LESS",
            LessEqual => "LESS_EQUAL",
            Greater => "GREATER",
            GreaterEqual => "GREATER_EQUAL",
            Slash => "SLASH",
            EOF => "EOF",
        };
        write!(f, "{out}")
    }
}
