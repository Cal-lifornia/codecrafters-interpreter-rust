use crate::tokens::Token;

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(file: &str) -> Result<Self, std::io::Error> {
        let file_contents = std::fs::read_to_string(file)?;
        let mut all_tokens = vec![];

        if !file_contents.is_empty() {
            for line in file_contents.lines() {
                let (mut tokens, errs) = Token::parse(line);
                if !errs.is_empty() {
                    return Err(std::io::Error::other(errs[0].to_string()));
                } else {
                    all_tokens.append(&mut tokens);
                }
            }
        }
        all_tokens.reverse();

        Ok(Self { tokens: all_tokens })
    }

    pub fn next_token(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EOF)
    }
    pub fn peek_next(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::EOF)
    }
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
