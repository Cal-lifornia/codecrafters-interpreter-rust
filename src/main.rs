use std::env;
use std::fs;
use std::process::exit;

use codecrafters_interpreter::ast::parse::token_stream::TokenStream;
use codecrafters_interpreter::ast::parse::Parser;
use codecrafters_interpreter::error::InterpreterError;
use codecrafters_interpreter::program::Program;
use codecrafters_interpreter::tokens::parse_tokens;
use codecrafters_interpreter::tokens::Lexer;
use codecrafters_interpreter::tokens::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    if let Err(err) = run_interpreter(command, filename) {
        eprintln!("{err}");
        exit(err.exit_code())
    }
}

fn run_interpreter(command: &str, filename: &str) -> Result<(), InterpreterError> {
    match command {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                let mut err_present = false;
                let (tokens, errs) = parse_tokens(&file_contents);
                if !errs.is_empty() {
                    err_present = true;
                    errs.iter().for_each(|err| {
                        eprintln!("{err}");
                    })
                }
                tokens.iter().for_each(|token| println!("{token}"));
                println!("{}", Token::EOF);
                if err_present {
                    exit(65)
                } else {
                    Ok(())
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
                Ok(())
            }
        }
        "parse" => {
            let mut lexer = Lexer::new(filename)?;
            let mut parser = Parser::new(TokenStream::direct_from_lexer(&mut lexer));
            println!("{}", parser.parse_expr(0)?);
            Ok(())
        }
        "evaluate" => {
            let mut lexer = Lexer::new(filename)?;
            let mut parser = Parser::new(TokenStream::direct_from_lexer(&mut lexer));
            let expr = parser.parse_expr(0)?;
            let mut program = Program::empty();
            println!("{}", expr.evaluate(&mut program)?);
            Ok(())
        }
        "run" => {
            let mut program = Program::new(filename)?;
            if let Err(err) = program.run() {
                eprintln!("{err}");
                exit(err.exit_code())
            } else {
                Ok(())
            }
        }
        _ => {
            Err(InterpreterError::Runtime("unknown command".to_string()))
            // return;
        }
    }
}
