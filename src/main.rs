use pseudo_interpreter::evaluator::evaluator::Evaluator;
use pseudo_interpreter::lexer::lexer::Lexer;
use pseudo_interpreter::parser::arithmetic::Parser;


fn main(){
    let input = "(3 + 5) * (10 - 2)";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let parsed_token = parser.parse();
    println!("Parse tree: {:?}", parsed_token);

    let result = Evaluator::evaluate(&parsed_token);
    println!("Result of the calculation: {}", result)
}