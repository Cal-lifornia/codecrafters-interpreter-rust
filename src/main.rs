use std::env;
use std::fs;
use std::process::exit;

use codecrafters_interpreter::ast::Expression;
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
            // Uncomment this block to pass the first stage
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
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut all_tokens = vec![];

            if !file_contents.is_empty() {
                file_contents.lines().for_each(|line| {
                    let (mut tokens, errs) = Token::parse(line);
                    if !errs.is_empty() {
                        panic!("no errs")
                    } else {
                        all_tokens.append(&mut tokens);
                    }
                });
                let exprs = Expression::parse_tokens(&all_tokens);
                for expr in exprs {
                    println!("{expr}");
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            // return;
        }
    }
}
