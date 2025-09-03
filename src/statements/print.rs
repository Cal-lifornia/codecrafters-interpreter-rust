use crate::{expression::parse_tokens, statements::program::ProgramResult, tokens::Lexer};

pub fn print_stmt(lexer: &mut Lexer) -> ProgramResult {
    let _ = lexer.next_token();
    let expr = parse_tokens(lexer, 0)?;
    println!("{}", expr.evaluate()?);
    Ok(())
}
