use crate::{
    expression::parse_tokens,
    statements::program::{Program, ProgramResult},
};

pub fn print_stmt(program: &mut Program) -> ProgramResult {
    let expr = parse_tokens(&mut program.lexer, 0)?;
    println!("{}", expr.evaluate()?);
    Ok(())
}
