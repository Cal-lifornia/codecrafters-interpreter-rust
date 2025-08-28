use std::io::ErrorKind;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    StringLiteral(String),
    EOF,
}

enum State {
    Delimiter,
    Quotes,
}

impl Token {
    pub fn parse(input: &str) -> (Vec<Token>, Vec<std::io::Error>) {
        use Token::*;
        let mut chars = input.chars();
        let mut word = String::new();
        let mut tokens = vec![];
        let mut errs = vec![];
        let mut last_token = Self::EOF;
        let mut current_state = State::Delimiter;
        #[allow(clippy::while_let_loop)]
        loop {
            let c = chars.next();
            match current_state {
                State::Delimiter => {
                    let c = match c {
                        Some(c) => c,
                        None => break,
                    };
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
                        '"' => {
                            current_state = State::Quotes;
                            continue;
                        }
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
                    } else if matches!(last_token, Equal | Bang | Less | Greater)
                        && current_token == Equal
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
                        last_token = current_token.clone();
                        tokens.push(current_token);
                    }
                }
                State::Quotes => match c {
                    Some('"') => {
                        let token = StringLiteral(std::mem::take(&mut word));
                        tokens.push(token);
                        current_state = State::Delimiter;
                    }
                    Some(ch) => word.push(ch),
                    None => {
                        errs.push(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            "Unterminated string.",
                        ));
                        break;
                    }
                },
            }
        }

        (tokens, errs)
    }
    pub fn lexeme(&self) -> String {
        use Token::*;
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
            EqualEqual => "==",
            Bang => "!",
            BangEqual => "!=",
            Less => "<",
            LessEqual => "<=",
            Greater => ">",
            GreaterEqual => ">=",
            Slash => "/",
            StringLiteral(ident) => return format!("\"{ident}\""),
            EOF => "",
        };
        out.to_string()
    }

    pub fn literal(&self) -> &str {
        if let Self::StringLiteral(value) = self {
            value
        } else {
            "null"
        }
    }

    pub fn type_str(&self) -> &str {
        use Token::*;
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
            EqualEqual => "EQUAL_EQUAL",
            Bang => "BANG",
            BangEqual => "BANG_EQUAL",
            Less => "LESS",
            LessEqual => "LESS_EQUAL",
            Greater => "GREATER",
            GreaterEqual => "GREATER_EQUAL",
            Slash => "SLASH",
            StringLiteral(_) => "STRING",
            EOF => "EOF",
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.type_str(),
            self.lexeme(),
            self.literal()
        )
    }
}
