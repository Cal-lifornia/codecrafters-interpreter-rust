use std::io::ErrorKind;

mod reserved_words;

pub use reserved_words::*;

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
    DoubleSymbol,
    Quotes,
    Num,
    Unquoted,
    Skip,
}

impl Token {
    pub fn single_char(value: char) -> Option<Token> {
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

    pub fn check_combo(current: &Self, next: &Self) -> bool {
        use Token::*;
        match (current, next) {
            (Slash, Slash) => true,
            (Equal | Bang | Less | Greater, Equal) => true,
            _ => false,
        }
    }

    pub fn parse(input: &str) -> (Vec<Token>, Vec<std::io::Error>) {
        use Token::*;
        let mut chars = input.chars().peekable();
        let mut word = String::new();
        let mut tokens = vec![];
        let mut errs = vec![];
        let mut last_token = Self::EOF;
        let mut current_state = State::Symbol;

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
                        current_state = State::Symbol;
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
                State::Symbol => {
                    let c = match c {
                        Some(c) => c,
                        None => break,
                    };
                    let current_token = match c {
                        '"' => {
                            current_state = State::Quotes;
                            continue;
                        }
                        // Ignore tabs or whitespace
                        '\t' | ' ' => {
                            last_token = EOF;
                            continue;
                        }
                        _ => match SymbolToken::try_from(c) {
                            Ok(token) => Symbol(token),
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
                                last_token = EOF;
                                continue;
                            }
                        },
                    };

                    // Check for comments
                    if last_token == Symbol(Slash) && current_token == Symbol(Slash) {
                        let _ = tokens.pop();
                        break;
                    // Check for comparison operators
                    } else if matches!(
                        last_token,
                        Symbol(Equal) | Symbol(Bang) | Symbol(Less) | Symbol(Greater)
                    ) && current_token == Symbol(Equal)
                    {
                        let _ = tokens.pop();
                        match last_token {
                            Symbol(Bang) => tokens.push(Symbol(BangEqual)),
                            Symbol(Equal) => tokens.push(Symbol(EqualEqual)),
                            Symbol(Less) => tokens.push(Symbol(LessEqual)),
                            Symbol(Greater) => tokens.push(Symbol(GreaterEqual)),
                            _ => unreachable!(),
                        }
                        last_token = EOF;
                    // Normal token
                    } else {
                        last_token = current_token.clone();
                        tokens.push(current_token);
                    }
                }

                // Capturing any that is a string
                State::Quotes => match c {
                    Some('"') => {
                        let token = StringLiteral(std::mem::take(&mut word));
                        tokens.push(token);
                        current_state = State::Symbol;
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

                // Unquoted, so likely an identifier or reserved word
                State::Unquoted => match c {
                    Some(' ') => {
                        if let Some(reserved) = ReservedWord::get(&word) {
                            word.clear();
                            tokens.push(Reserved(reserved));
                        } else {
                            tokens.push(Identifier(std::mem::take(&mut word)));
                        }
                        current_state = State::Symbol;
                    }
                    Some(ch) => {
                        if let Ok(token) = SymbolToken::try_from(ch) {
                            let token = Symbol(token);
                            if let Some(reserved) = ReservedWord::get(&word) {
                                word.clear();
                                tokens.push(Reserved(reserved));
                            } else {
                                tokens.push(Identifier(std::mem::take(&mut word)));
                            }
                            last_token = token.clone();
                            tokens.push(token);
                            current_state = State::Symbol;
                        } else if ch.is_alphanumeric() || ch == '_' {
                            word.push(ch);
                            continue;
                        }
                    }
                    None => {
                        if let Some(reserved) = ReservedWord::get(&word) {
                            word.clear();
                            tokens.push(Reserved(reserved));
                        } else {
                            tokens.push(Identifier(std::mem::take(&mut word)));
                        }
                        break;
                    }
                },
                State::Num => unreachable!(),
            }
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
