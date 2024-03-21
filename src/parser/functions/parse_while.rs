use super::{super::{
    parser::Parser,
    ParsePrintExpr, Statement, Token,
}, token::ParseToken};

pub trait ParseWhile {
    fn parse_while(&mut self) -> Statement;
}

impl ParseWhile for Parser {
    fn parse_while(&mut self) -> Statement {
        self.next_token();
        let condition = self.parse_expr();

        if let Some(Token::Do) = &self.current_token {
            self.next_token();
            let wstatement = self.parse_token(vec![Token::EndWhile]);
            Statement::While(condition, Box::new(wstatement))
        } else {
            panic!("Expected 'Do' keyword")
        }
    }
}
