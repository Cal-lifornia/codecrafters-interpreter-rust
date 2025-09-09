use crate::{
    ast::{
        parse::Parser,
        stmt::{Block, Stmt},
    },
    error::InterpreterError,
    tokens::Token,
};

impl Parser {
    pub fn parse_block(&mut self) -> Result<Block, InterpreterError> {
        assert_eq!(self.current_token, Token::LeftBrace);
        self.bump();

        let mut stmts: Vec<Stmt> = vec![];
        loop {
            if self.current_token == Token::RightBrace {
                self.bump();
                break;
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(Block { stmts })
    }
    pub fn parse_stmt(&mut self) -> Result<Stmt, InterpreterError> {
        match self.current_token {
            Token::LeftBrace => Ok(Stmt::Block(self.parse_block()?)),
            _ => {
                let stmt = self.parse_expr(0)?;
                let next = self.look_ahead(1);
                if next == Token::Semicolon {
                    self.bump();
                    self.bump();
                    Ok(Stmt::Expr(stmt))
                } else {
                    panic!("no semicolon")
                }
            }
        }
    }
}
