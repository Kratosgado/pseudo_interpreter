use pseudo_interpreter::evaluator::evaluator::Evaluator;
use pseudo_interpreter::lexer::lexer::Lexer;
use pseudo_interpreter::parser::parser::Parser;

use  std::fs;

fn main() {
    let filename = "code.ps";
    let input = fs::read_to_string(filename).expect("Something went to wrong reading file.");
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let parsed_token = parser.parse();
    println!("Parse tree: {:?}", parsed_token);

    let mut evaluator = Evaluator::new(parsed_token);
    evaluator.evaluate();
}
