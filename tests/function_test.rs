#[cfg(test)]
mod tests {
    use pseudo_interpreter::{constants::error_handler::PseudoError, Evaluator, Lexer, Parser};
    use std::fs;

    #[test]
    fn function_return()  {
        let testfile = "tests/test_codes/functions/function.ps";
        let input = fs::read_to_string(testfile).unwrap();
        let mut lex = Lexer::new(input.as_str());
        let tokens = lex.tokenize();
        assert_eq!(tokens.is_err(), false);
        let mut parser = Parser::new(tokens.unwrap());
        let tree = parser.parse();
        assert_eq!(tree.is_err(), false);
        let mut eval = Evaluator::new(tree.unwrap());
        assert_eq!(eval.evaluate().is_err(), false);        
    }
    #[test]
    fn arg_func() {
        let testfile = "tests/test_codes/functions/arg_func.ps";
        let input = fs::read_to_string(testfile).unwrap();
        let mut lex = Lexer::new(input.as_str());
        let tokens = lex.tokenize();
        assert_eq!(tokens.is_err(), false);
        let mut parser = Parser::new(tokens.unwrap());
        let tree = parser.parse();
        assert_eq!(tree.is_err(), false);
        let mut eval = Evaluator::new(tree.unwrap());
        assert_eq!(eval.evaluate().is_err(), false);
    }

    #[test]
    fn no_name_func(){
        let testfile = "tests/test_codes/functions/noname.ps";
        let input = fs::read_to_string(testfile).unwrap();
        let mut lex = Lexer::new(input.as_str());
        let tokens = lex.tokenize();
        assert_eq!(tokens.is_err(), false);
        let mut parser = Parser::new(tokens.unwrap());
        let tree = parser.parse();
        assert_eq!(tree.is_err(), true);
        // let mut eval = Evaluator::new(tree.unwrap());
        // assert_eq!(eval.evaluate().is_err(), false);
    }

    #[test]
    fn unmatched_args()  {
        let testfile = "tests/test_codes/functions/unmatch_arg.ps";
        let input = fs::read_to_string(testfile).unwrap();
        let mut lex = Lexer::new(input.as_str());
        let tokens = lex.tokenize();
        assert_eq!(tokens.is_err(), false);
        let mut parser = Parser::new(tokens.unwrap());
        let tree = parser.parse();
        assert_eq!(tree.is_ok(), false);
        let mut eval = Evaluator::new(tree.unwrap());
        assert_eq!(eval.evaluate().is_ok(), false);
    }
}
