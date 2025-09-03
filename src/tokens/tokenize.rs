use crate::{error::InterpreterError, tokens::reserved_words::ReservedWord};

pub trait TokenDisplay {
    fn lexeme(&self) -> String;
    fn literal(&self) -> String;
    fn type_str(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
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
    Bang,
    Less,
    Greater,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
    Slash,
    StringLiteral(String),
    Number(String, f64),
    Identifier(String),
    Reserved(ReservedWord),
    EOF,
}

#[derive(PartialEq, Eq)]
enum State {
    Symbol,
    Quotes,
    Num,
    Unquoted,
}

impl Token {
    fn single_char(value: char) -> Option<Token> {
        use Token::*;
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
            _ => return None,
        };
        Some(out)
    }

    fn check_double_token(token: &Self, ch: char) -> bool {
        if let Some(other) = Token::single_char(ch) {
            match (token, other) {
                (Token::Slash, Token::Slash) => true,
                (token, Token::Equal) => matches!(
                    token,
                    Token::Equal | Token::Bang | Token::Less | Token::Greater
                ),
                _ => false,
            }
        } else {
            false
        }
    }

    fn new_double_token(first: &Self, second: &Self) -> Option<Token> {
        if second == &Token::Equal {
            match first {
                Self::Equal => Some(Self::EqualEqual),
                Self::Bang => Some(Self::BangEqual),
                Self::Less => Some(Self::LessEqual),
                Self::Greater => Some(Self::GreaterEqual),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn tokenize(input: &str) -> (Vec<Token>, Vec<InterpreterError>) {
        use Token::*;

        let mut chars = input.chars().peekable();
        let mut word = String::new();
        let mut tokens = vec![];
        let mut errs = vec![];

        let mut state = State::Symbol;

        loop {
            let c = chars.next();
            state = match state {
                State::Symbol => match c {
                    Some('"') => State::Quotes,
                    Some('\t' | ' ') => {
                        continue;
                    }
                    Some(ch) => {
                        if let Some(token) = Token::single_char(ch) {
                            let next_ch = chars
                                .next_if(|&val| Token::check_double_token(&token, val))
                                .unwrap_or('c');
                            if let Some(next_token) = Token::single_char(next_ch) {
                                if token == Slash && next_token == Slash {
                                    break;
                                }
                                if let Some(double_token) =
                                    Self::new_double_token(&token, &next_token)
                                {
                                    tokens.push(double_token);
                                    continue;
                                }
                            }
                            tokens.push(token);
                            State::Symbol
                        } else if ch.is_ascii_digit() {
                            word.push(ch);

                            let next_is_number = match chars.peek() {
                                Some(val) => val.is_ascii_digit() || *val == '.',
                                None => false,
                            };

                            if next_is_number {
                                State::Num
                            } else {
                                match word.parse::<f64>() {
                                    Ok(num) => {
                                        tokens.push(Number(std::mem::take(&mut word), num));
                                    }
                                    Err(err) => {
                                        errs.push(InterpreterError::Syntax(err.to_string()));
                                    }
                                }
                                State::Symbol
                            }
                        } else if ch.is_alphanumeric() || ch == '_' {
                            word.push(ch);
                            if let Some(next_ch) = chars.peek() {
                                if Token::single_char(*next_ch).is_some() {
                                    if let Some(reserved) = ReservedWord::get(&word) {
                                        word.clear();
                                        tokens.push(Reserved(reserved));
                                    } else {
                                        tokens.push(Identifier(std::mem::take(&mut word)));
                                    }
                                    state = State::Symbol;
                                    continue;
                                }
                            }
                            State::Unquoted
                        } else {
                            errs.push(InterpreterError::Syntax(format!(
                                "Unexpected character: {ch}"
                            )));
                            State::Symbol
                        }
                    }
                    None => {
                        break;
                    }
                },
                State::Quotes => match c {
                    Some('"') => {
                        let token = StringLiteral(std::mem::take(&mut word));
                        tokens.push(token);
                        State::Symbol
                    }
                    Some(ch) => {
                        word.push(ch);
                        State::Quotes
                    }
                    None => {
                        errs.push(InterpreterError::Syntax("Unterminated string.".to_string()));
                        break;
                    }
                },
                State::Num => match c {
                    Some(ch) => {
                        if ch.is_ascii_digit() || ch == '.' {
                            word.push(ch);
                        }
                        let next_is_number = match chars.peek() {
                            Some(val) => val.is_ascii_digit() || *val == '.',
                            None => false,
                        };

                        if next_is_number {
                            State::Num
                        } else {
                            match word.parse::<f64>() {
                                Ok(num) => {
                                    tokens.push(Number(std::mem::take(&mut word), num));
                                }
                                Err(err) => {
                                    errs.push(InterpreterError::Syntax(err.to_string()));
                                }
                            }
                            State::Symbol
                        }
                    }
                    None => break,
                },
                State::Unquoted => match c {
                    Some(' ') | None => {
                        if let Some(reserved) = ReservedWord::get(&word) {
                            word.clear();
                            tokens.push(Reserved(reserved));
                        } else {
                            tokens.push(Identifier(std::mem::take(&mut word)));
                        }
                        State::Symbol
                    }
                    Some(ch) => {
                        if ch.is_alphanumeric() || ch == '_' {
                            word.push(ch);
                        } else {
                            errs.push(InterpreterError::Syntax(format!(
                                "Unexpected character: {ch}"
                            )));
                            state = State::Symbol;
                            continue;
                        }
                        if let Some(next_ch) = chars.peek() {
                            if Token::single_char(*next_ch).is_some() {
                                if let Some(reserved) = ReservedWord::get(&word) {
                                    word.clear();
                                    tokens.push(Reserved(reserved));
                                } else {
                                    tokens.push(Identifier(std::mem::take(&mut word)));
                                }
                                state = State::Symbol;
                                continue;
                            }
                        }
                        State::Unquoted
                    }
                },
            };
        }

        (tokens, errs)
    }
}

impl TokenDisplay for Token {
    fn lexeme(&self) -> String {
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
            Bang => "!",
            Less => "<",
            Greater => ">",
            Slash => "/",
            EqualEqual => "==",
            BangEqual => "!=",
            GreaterEqual => ">=",
            LessEqual => "<=",
            StringLiteral(ident) => return format!("\"{ident}\""),
            Number(lex, _) => lex,
            Identifier(ident) => ident,
            Reserved(word) => return word.lexeme(),
            EOF => "",
        };
        out.to_string()
    }

    fn literal(&self) -> String {
        use Token::*;
        match self {
            StringLiteral(val) => val.to_string(),
            Number(_, val) => format!("{val:?}"),
            _ => "null".to_string(),
        }
    }

    fn type_str(&self) -> &str {
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
            Bang => "BANG",
            Less => "LESS",
            Greater => "GREATER",
            Slash => "SLASH",
            EqualEqual => "EQUAL_EQUAL",
            BangEqual => "BANG_EQUAL",
            LessEqual => "LESS_EQUAL",
            GreaterEqual => "GREATER_EQUAL",
            StringLiteral(_) => "STRING",
            Number(_, _) => "NUMBER",
            Identifier(_) => "IDENTIFIER",
            Reserved(word) => word.type_str(),
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
