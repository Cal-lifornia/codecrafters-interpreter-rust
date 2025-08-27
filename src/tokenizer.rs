pub enum Token {
    LeftParen,
    RightParen,
    EOF,
}

impl Token {
    pub fn parse(input: &str) -> std::io::Result<Vec<Self>> {
        let mut chars = input.chars();
        let mut tokens: Vec<Token> = vec![];

        loop {
            match chars.next() {
                Some(c) => tokens.push(Token::try_from(c)?),
                None => {
                    tokens.push(Token::EOF);
                    break;
                }
            }
        }
        Ok(tokens)
    }
    pub fn to_char(&self) -> Option<char> {
        use Token::*;
        match self {
            LeftParen => Some('('),
            RightParen => Some(')'),
            EOF => None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let out = match self {
            LeftParen => "LEFT_PAREN",
            RightParen => "RIGHT_PAREN",
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
            _ => return Err(std::io::Error::new(ErrorKind::InvalidInput, "invalid char")),
        };
        Ok(result)
    }
}
