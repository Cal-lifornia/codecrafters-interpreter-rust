use std::io::ErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    SingleChar(SingleCharToken),
    MultiChar(MultiCharToken),
    StringLiteral(String),
    Number(String, f64),
    Identifier(String),
    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SingleCharToken {
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
    Slash,
    Less,
    Greater,
}

impl TryFrom<char> for SingleCharToken {
    type Error = std::io::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use SingleCharToken::*;
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
                    format!("Unexpected character {value}"),
                ))
            }
        };
        Ok(out)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MultiCharToken {
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(PartialEq, Eq)]
enum State {
    Delimiter,
    Quotes,
    Num,
    Unquoted,
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

        loop {
            let c = chars.next();
            if current_state == State::Num {
                if let Some(ch) = c {
                    if ch.is_ascii_digit() || ch == '.' {
                        word.push(ch);
                        continue;
                    } else {
                        match word.parse::<f64>() {
                            Ok(num) => {
                                tokens.push(Number(std::mem::take(&mut word), num));
                            }
                            Err(err) => {
                                errs.push(std::io::Error::new(ErrorKind::InvalidInput, err));
                            }
                        }
                        current_state = State::Delimiter;
                    }
                } else {
                    match word.parse::<f64>() {
                        Ok(num) => {
                            tokens.push(Number(std::mem::take(&mut word), num));
                        }
                        Err(err) => {
                            errs.push(std::io::Error::new(ErrorKind::InvalidInput, err));
                        }
                    }
                    break;
                }
            }

            match current_state {
                State::Delimiter => {
                    let c = match c {
                        Some(c) => c,
                        None => break,
                    };
                    let current_token = match c {
                        '"' => {
                            current_state = State::Quotes;
                            continue;
                        }
                        '\t' | ' ' => {
                            last_token = EOF;
                            continue;
                        }
                        _ => {
                            if let Ok()
                            if c.is_ascii_digit() {
                                word.push(c);
                                current_state = State::Num;
                            } else {
                                word.push(c);
                                current_state = State::Unquoted;
                            }
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
                State::Unquoted => {}
                State::Num => unreachable!(),
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
            Number(lex, _) => lex,
            Identifier(ident) => ident,
            EOF => "",
        };
        out.to_string()
    }

    pub fn literal(&self) -> String {
        use Token::*;
        match self {
            StringLiteral(val) => val.to_string(),
            Number(_, val) => format!("{val:?}"),
            _ => "null".to_string(),
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
            Number(_, _) => "NUMBER",
            Identifier(_) => "IDENTIFIER",
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
