use crate::tokens::Token;

pub type TokenStream = Vec<TokenTree>;

pub enum TokenTree {
    Token(Token),
    Delimited(Delimiter, TokenStream),
}

pub enum Delimiter {
    Parenthesis,
    Brace,
}
