use std::io::ErrorKind;

mod reserved_words;
mod symbols;

pub use reserved_words::*;
pub use symbols::*;

trait TokenDisplay {
    fn lexeme(&self) -> String;
    fn literal(&self) -> String;
    fn type_str(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Symbol(SymbolToken),
    StringLiteral(String),
    Number(String, f64),
    Identifier(String),
    Reserved(ReservedWord),
    EOF,
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
        use SymbolToken::*;
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

                // Unquoted, so likely an identifier or reserved word
                State::Unquoted => match c {
                    Some(' ') => {
                        if let Some(reserved) = ReservedWord::get(&word) {
                            word.clear();
                            tokens.push(Reserved(reserved));
                        } else {
                            tokens.push(Identifier(std::mem::take(&mut word)));
                        }
                        current_state = State::Delimiter;
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
                            current_state = State::Delimiter;
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
        match self {
            Symbol(symbol) => symbol.lexeme(),
            StringLiteral(ident) => format!("\"{ident}\""),
            Number(lex, _) => lex.to_string(),
            Identifier(ident) => ident.to_string(),
            Reserved(word) => word.lexeme(),
            EOF => "".to_string(),
        }
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
            Symbol(symbol) => symbol.type_str(),
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
