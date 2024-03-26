#[cfg(test)]
mod tests {
    use pseudo_interpreter::{Evaluator, Lexer, Parser};
    use std::fs;

    #[test]
    fn if_not() {
        let testfile = "tests/test_codes/ifelse/ifnot.ps";
        let input = fs::read_to_string(testfile).unwrap();
        let mut lex = Lexer::new(input.as_str());
        let tokens = lex.tokenize();
        assert_eq!(tokens.is_ok(), true);
        let mut parser = Parser::new(tokens.unwrap());
        let tree = parser.parse();
        assert_eq!(tree.is_ok(), true);
        let mut eval = Evaluator::new(tree.unwrap());
        assert_eq!(eval.evaluate().is_ok(), true);
    }

    // #[test]
    // fn nested_if() {
    //     let testfile = "tests/test_codes/whileloop/nestedif.ps";
    //     let input = fs::read_to_string(testfile).unwrap();
    //     let mut lex = Lexer::new(input.as_str());
    //     let tokens = lex.tokenize();
    //     assert_eq!(tokens.is_ok(), true);
    //     let mut parser = Parser::new(tokens.unwrap());
    //     let tree = parser.parse();
    //     assert_eq!(tree.is_ok(), true);
    //     let mut eval = Evaluator::new(tree.unwrap());
    //     assert_eq!(eval.evaluate().is_ok(), true);
    // }
}
