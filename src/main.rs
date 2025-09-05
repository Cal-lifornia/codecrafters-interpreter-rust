use std::env;
use std::fs;
use std::process::exit;

use codecrafters_interpreter::ast::parse_exprs;
use codecrafters_interpreter::context::Context;
use codecrafters_interpreter::program::Program;
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

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                let mut err_present = false;
                let (tokens, errs) = Token::tokenize(&file_contents);
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
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "parse" => match Lexer::new(filename) {
            Ok(lexer) => match parse_exprs(&mut Context::new(lexer), 0) {
                Ok(expr) => println!("{expr}"),
                Err(err) => {
                    eprintln!("Error {err}");
                    exit(65)
                }
            },
            Err(err) => {
                eprintln!("Error {err}");
                exit(65)
            }
        },
        "evaluate" => match Program::new(filename) {
            Ok(mut program) => match parse_exprs(&mut Context::new(program.lexer()), 0) {
                Ok(expr) => match expr.evaluate(&mut program) {
                    Ok(eval) => println!("{eval}"),
                    Err(err) => {
                        eprintln!("{err}");
                        exit(err.exit_code())
                    }
                },
                Err(err) => {
                    eprintln!("Error {err}");
                    exit(err.exit_code())
                }
            },
            Err(err) => {
                eprintln!("Error {err}");
                exit(err.exit_code())
            }
        },
        "run" => match Program::new(filename) {
            Ok(mut program) => {
                if let Err(err) = program.run() {
                    eprintln!("{err}");
                    exit(err.exit_code())
                }
            }
            Err(err) => {
                eprintln!("{err}");
                exit(err.exit_code())
            }
        },

        _ => {
            eprintln!("Unknown command: {}", command);
            // return;
        }
    }
}
