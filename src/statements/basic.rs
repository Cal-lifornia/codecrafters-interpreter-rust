use crate::{
    context::Context,
    expression::parse_tokens,
    statements::program::{Program, ProgramResult},
};

pub fn print_stmt(program: &mut Program, ctx: &mut Context) -> ProgramResult {
    let expr = parse_tokens(ctx, 0)?;
    println!("{}", expr.evaluate(program)?);
    Ok(())
}
