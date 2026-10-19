pub mod  lexer;
pub mod parser;
pub mod evaluator;
pub mod  constants;
pub mod std;
// pub mod constants;

pub use lexer::lexer::Lexer;
pub use parser::parser::Parser;
pub use evaluator::evaluator::Evaluator;
pub use evaluator::output::SharedBuffer;

use constants::error_handler::PseudoError;

/// Lex, parse and run `source`, printing to stdout.
pub fn run(source: &str) -> Result<(), PseudoError> {
    let statements = Lexer::new(source).tokenize().and_then(|tokens| Parser::new(tokens).parse())?;
    Evaluator::new(statements).evaluate()
}

/// Lex, parse and run `source`, returning everything it printed.
pub fn run_capture(source: &str) -> Result<String, PseudoError> {
    let statements = Lexer::new(source).tokenize().and_then(|tokens| Parser::new(tokens).parse())?;
    let buffer = SharedBuffer::new();
    Evaluator::with_writer(statements, Box::new(buffer.clone())).evaluate()?;
    Ok(buffer.contents())
}
