use std::str::Chars;
use std::iter::Peekable;

use pseudo_interpreter::tokens::lexer::lex;
use pseudo_interpreter::parser::arithmetic;


fn main(){
    let input = "2 + 2";
    let tokens = lex(input);
    println!("Tokens: {:?}", tokens);

    let results = arithmetic::parse(&tokens);
    println!("Results: {}", results);
}