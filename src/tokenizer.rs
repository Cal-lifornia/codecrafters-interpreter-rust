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
                    format!("Unexpected character: {value}"),
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

impl TryFrom<String> for MultiCharToken {
    type Error = std::io::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        use std::io::ErrorKind;
        use MultiCharToken::*;
        match value.as_str() {
            "==" => Ok(EqualEqual),
            "!=" => Ok(BangEqual),
            "<=" => Ok(LessEqual),
            ">=" => Ok(GreaterEqual),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected char: {value}"),
            )),
        }
    }
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
        use SingleCharToken::*;
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
                        _ => match SingleCharToken::try_from(c) {
                            Ok(token) => SingleChar(token),
                            Err(err) => {
                                if c.is_ascii_digit() {
                                    word.push(c);
                                    current_state = State::Num;
                                } else if c.is_alphanumeric() || c == '_' {
                                    word.push(c);
                                    current_state = State::Unquoted;
                                } else {
                                    errs.push(err);
                                }
                                continue;
                            }
                        },
                    };
                    if last_token == SingleChar(SingleCharToken::Slash)
                        && current_token == SingleChar(SingleCharToken::Slash)
                    {
                        let _ = tokens.pop();
                        break;
                    } else if matches!(
                        last_token,
                        SingleChar(Equal)
                            | SingleChar(Bang)
                            | SingleChar(Less)
                            | SingleChar(Greater)
                    ) && current_token == SingleChar(Equal)
                    {
                        let _ = tokens.pop();
                        match last_token {
                            SingleChar(Bang) => tokens.push(MultiChar(MultiCharToken::BangEqual)),
                            SingleChar(Equal) => tokens.push(MultiChar(MultiCharToken::EqualEqual)),
                            SingleChar(Less) => tokens.push(MultiChar(MultiCharToken::LessEqual)),
                            SingleChar(Greater) => {
                                tokens.push(MultiChar(MultiCharToken::GreaterEqual))
                            }
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
                State::Unquoted => match c {
                    Some(' ') => {
                        tokens.push(Identifier(std::mem::take(&mut word)));
                        current_state = State::Delimiter;
                    }
                    Some(ch) => {
                        if let Ok(token) = SingleCharToken::try_from(ch) {
                            let token = SingleChar(token);
                            last_token = token.clone();
                            tokens.push(Identifier(std::mem::take(&mut word)));
                            tokens.push(token);
                            current_state = State::Delimiter;
                        } else if ch.is_alphanumeric() || ch == '_' {
                            word.push(ch);
                        }
                    }
                    None => {
                        tokens.push(Identifier(std::mem::take(&mut word)));
                        break;
                    }
                },
                State::Num => unreachable!(),
            }
        }

        (tokens, errs)
    }
    pub fn lexeme(&self) -> String {
        use MultiCharToken::*;
        use SingleCharToken::*;
        use Token::*;
        let out = match self {
            SingleChar(LeftParen) => "(",
            SingleChar(RightParen) => ")",
            SingleChar(LeftBrace) => "{",
            SingleChar(RightBrace) => "}",
            SingleChar(Comma) => ",",
            SingleChar(Dot) => ".",
            SingleChar(Minus) => "-",
            SingleChar(Plus) => "+",
            SingleChar(Semicolon) => ";",
            SingleChar(Star) => "*",
            SingleChar(Equal) => "=",
            SingleChar(Bang) => "!",
            SingleChar(Less) => "<",
            SingleChar(Greater) => ">",
            SingleChar(Slash) => "/",
            MultiChar(EqualEqual) => "==",
            MultiChar(BangEqual) => "!=",
            MultiChar(GreaterEqual) => ">=",
            MultiChar(LessEqual) => "<=",
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
        use MultiCharToken::*;
        use SingleCharToken::*;
        use Token::*;
        match self {
            SingleChar(LeftParen) => "LEFT_PAREN",
            SingleChar(RightParen) => "RIGHT_PAREN",
            SingleChar(LeftBrace) => "LEFT_BRACE",
            SingleChar(RightBrace) => "RIGHT_BRACE",
            SingleChar(Comma) => "COMMA",
            SingleChar(Dot) => "DOT",
            SingleChar(Minus) => "MINUS",
            SingleChar(Plus) => "PLUS",
            SingleChar(Semicolon) => "SEMICOLON",
            SingleChar(Star) => "STAR",
            SingleChar(Equal) => "EQUAL",
            SingleChar(Bang) => "BANG",
            SingleChar(Less) => "LESS",
            SingleChar(Greater) => "GREATER",
            SingleChar(Slash) => "SLASH",
            MultiChar(EqualEqual) => "EQUAL_EQUAL",
            MultiChar(BangEqual) => "BANG_EQUAL",
            MultiChar(LessEqual) => "LESS_EQUAL",
            MultiChar(GreaterEqual) => "GREATER_EQUAL",
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
