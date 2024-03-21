use super::{super::{
    parser::Parser, ParseArray, ParseAssignment, ParseFor, ParseFunction, ParseIf, ParseInput,
    ParsePrintExpr, Statement, Token,
}, token::ParseToken};

pub trait ParseWhile {
    fn parse_while(&mut self) -> Statement;
}

impl ParseWhile for Parser {
    fn parse_while(&mut self) -> Statement {
        self.next_token();
        let condition = self.parse_expr();
        let mut wstatement = Vec::new();

        if let Some(Token::Do) = &self.current_token {
            self.next_token();
            while &self.current_token != &Some(Token::EndWhile){
                wstatement.push(self.parse_token());
                if &self.current_token == &Some(Token::EOF) {
                    panic!("Expected 'EndWhile' keyword")
                }
            }
            Statement::While(condition, Box::new(wstatement))
        } else {
            panic!("Expected 'Do' keyword")
        }
    }
}
