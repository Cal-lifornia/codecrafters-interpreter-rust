use crate::{
    ast::{
        parse::Parser,
        stmt::{Block, ControlFlowStmt, Stmt},
    },
    error::InterpreterError,
    tokens::{ReservedWord, Token},
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
            Token::Reserved(ReservedWord::If) => self.parse_if(),
            Token::Reserved(ReservedWord::While) => self.parse_while_loop(),
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
    pub fn parse_control_flow_stmt(&mut self) -> Result<ControlFlowStmt, InterpreterError> {
        if matches!(self.current_token, Token::LeftBrace) {
            Ok(ControlFlowStmt::Block(self.parse_block()?))
        } else {
            Ok(ControlFlowStmt::Stmt(Box::new(self.parse_stmt()?)))
        }
    }
    pub fn parse_if(&mut self) -> Result<Stmt, InterpreterError> {
        assert_eq!(self.current_token, Token::Reserved(ReservedWord::If));
        self.bump();

        let group = self.parse_group()?;
        self.bump();

        let kind = self.parse_control_flow_stmt()?;
        let if_else = if self.current_token == Token::Reserved(ReservedWord::Else) {
            self.bump();
            Some(self.parse_control_flow_stmt()?)
        } else {
            None
        };
        Ok(Stmt::If(group, kind, if_else))
    }

    pub fn parse_while_loop(&mut self) -> Result<Stmt, InterpreterError> {
        assert_eq!(self.current_token, Token::Reserved(ReservedWord::While));
        self.bump();

        let group = self.parse_group()?;
        self.bump();

        let kind = self.parse_control_flow_stmt()?;

        Ok(Stmt::WhileLoop(group, kind))
    }
}
