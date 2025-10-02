use lox_shared::error::LoxError;

use crate::{
    ast::{Attribute, Class, FunSig, Function, Ident, Item, ItemKind},
    parser::{
        Parser,
        token::{ReservedWord, TokenKind},
    },
};

impl Parser {
    pub fn parse_item(&mut self) -> Result<Item, LoxError> {
        let attr = Attribute::new(self.new_node_id(), self.current_token.span().clone());
        match self.current_token.kind() {
            TokenKind::Reserved(ReservedWord::Class) => {
                self.bump();
                let Some(ident) = Ident::from_token(&self.current_token) else {
                    return Err(LoxError::Syntax(format!(
                        "Expecting class identifier, got {}",
                        self.current_token
                    )));
                };
                self.bump();
                assert_eq!(self.current_token.kind(), &TokenKind::LeftBrace);
                // Bumped past left brace for Class contents
                self.bump();
                let mut methods = Vec::new();
                loop {
                    if matches!(self.look_ahead(1).kind(), TokenKind::LeftParen)
                        && matches!(self.current_token.kind(), TokenKind::Identifier(_))
                    {
                        methods.push(self.parse_function()?);
                    } else if self.current_token == TokenKind::RightBrace {
                        break;
                    }
                }

                assert_eq!(self.current_token.kind(), &TokenKind::RightBrace);
                self.bump();
                Ok(Item::new(ItemKind::Class(Class::new(ident, methods)), attr))
            }
            TokenKind::Reserved(ReservedWord::Fun) => {
                self.bump();
                Ok(Item::new(
                    crate::ast::ItemKind::Fun(self.parse_function()?),
                    attr,
                ))
            }
            _ => unreachable!(),
        }
    }
    pub fn parse_fun_sig(&mut self, ident: Ident) -> Result<FunSig, LoxError> {
        assert_eq!(self.current_token, TokenKind::LeftParen);
        self.bump();
        let mut inputs = vec![];
        loop {
            if let Some(ident) = Ident::from_token(&self.current_token) {
                inputs.push(ident);
                self.bump();
            } else {
                if self.prev_token == TokenKind::Comma {
                    return Err(LoxError::Syntax("Missing parameter after comma".into()));
                }
                break;
            }
            if self.current_token == TokenKind::Comma {
                self.bump();
                continue;
            } else {
                break;
            }
        }
        if self.current_token != TokenKind::RightParen {
            Err(LoxError::Syntax("missing ')'".into()))
        } else {
            Ok(FunSig { ident, inputs })
        }
    }

    pub fn parse_function(&mut self) -> Result<Function, LoxError> {
        let Some(ident) = Ident::from_token(&self.current_token) else {
            return Err(LoxError::Syntax(format!(
                "Expected identifier, got {}",
                self.current_token
            )));
        };
        self.bump();
        let sig = self.parse_fun_sig(ident)?;
        self.bump();

        let body = self.parse_block()?;

        Ok(Function { sig, body })
    }
}
