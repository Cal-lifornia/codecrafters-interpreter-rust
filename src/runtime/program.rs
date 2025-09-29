use crate::{
    ast::parse::{token_stream::generate_token_stream, Parser},
    error::InterpreterError,
    runtime::interpreter::Interpreter,
    tokens::{Lexer, Token},
};

pub fn run_program(filename: &str) -> Result<(), InterpreterError> {
    let mut ast = vec![];
    let mut lexer = Lexer::new(filename)?;
    let stream = generate_token_stream(&mut lexer)?;
    let mut parser = Parser::new(stream);
    loop {
        if parser.current_token == Token::EOF {
            break;
        } else {
            ast.push(parser.parse_stmt()?);
        }
    }

    let mut interpreter = Interpreter::default();

    for stmt in ast.clone() {
        if matches!(
            stmt,
            crate::ast::stmt::Stmt::Item(_) | crate::ast::stmt::Stmt::Block(_)
        ) {
            interpreter.resolver.resolve_stmt(&stmt)?;
        }
    }
    interpreter.env.locals = interpreter.resolver.take_locals();
    for stmt in ast.clone() {
        interpreter.evaluate_stmt(&stmt)?;
    }
    Ok(())
}
