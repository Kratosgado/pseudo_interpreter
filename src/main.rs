use pseudo_interpreter::{Evaluator, Lexer, Parser};

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: pseudo_interpreter <file_path>");
        std::process::exit(1);
    }
    let filename = &args[1];
    let input = fs::read_to_string(filename);
    match &input {
        Ok(inputs) => {
            let mut lexer = Lexer::new(inputs);
            let tokens = lexer.tokenize();
            // println!("Tokens: {:?}", tokens); // debug

            let mut parser = Parser::new(tokens);
            let parsed_token = parser.parse();
            // println!("Parse tree: {:?}", parsed_token); // debug

            let mut evaluator = Evaluator::new(parsed_token);
            evaluator.evaluate();
            println!("execution done!");
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        }
    }
}
