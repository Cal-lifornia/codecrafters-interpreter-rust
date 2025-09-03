use std::env;
use std::fs;
use std::process::exit;

use codecrafters_interpreter::expression::parse_tokens;
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
                file_contents.lines().enumerate().for_each(|(idx, line)| {
                    let (tokens, errs) = Token::parse(line);
                    if !errs.is_empty() {
                        err_present = true;
                        errs.iter().for_each(|err| {
                            let line_number = idx + 1;
                            eprintln!("[line {line_number}] Error: {err}");
                        })
                    }
                    tokens.iter().for_each(|token| println!("{token}"))
                });
                println!("{}", Token::EOF);
                if err_present {
                    exit(65)
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "parse" => match Lexer::new(filename) {
            Ok(mut lexer) => match parse_tokens(&mut lexer, 0) {
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
        "evaluate" => match Lexer::new(filename) {
            Ok(mut lexer) => match parse_tokens(&mut lexer, 0) {
                Ok(expr) => match expr.evaluate() {
                    Ok(eval) => println!("{eval}"),
                    Err(err) => {
                        eprintln!("{err}");
                        exit(err.exit_code())
                    }
                },
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

        _ => {
            eprintln!("Unknown command: {}", command);
            // return;
        }
    }
}
