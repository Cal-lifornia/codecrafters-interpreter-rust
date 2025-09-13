use crate::{
    ast::{ident::Ident, item::FunSig, parse::Parser},
    error::InterpreterError,
    tokens::Token,
};

impl Parser {
    pub fn parse_fun_sig(&mut self, ident: Ident) -> Result<FunSig, InterpreterError> {
        assert_eq!(self.current_token, Token::LeftParen);
        self.bump();
        let mut inputs = vec![];
        loop {
            if let Some(ident) = Ident::from_token(&self.current_token) {
                inputs.push(ident);
                self.bump();
            } else {
                if self.prev_token == Token::Comma {
                    return Err(InterpreterError::Syntax(
                        "Missing parameter after comma".to_string(),
                    ));
                }
                break;
            }
            if self.current_token == Token::Comma {
                self.bump();
                continue;
            } else {
                break;
            }
        }
        assert_eq!(self.current_token, Token::RightParen);
        Ok(FunSig { ident, inputs })
    }
}
