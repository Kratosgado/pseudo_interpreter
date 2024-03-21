use crate::parser::ParseArray;

use super::{
    super::{
        Expr, ParseAssignment, ParseFunction, ParseIf, ParseInput, ParsePrintExpr, ParseWhile,
        Parser, Statement, Token,
    },
    token::ParseToken,
};

pub trait ParseFor {
    fn parse_for(&mut self) -> Statement;
}

impl ParseFor for Parser {
    fn parse_for(&mut self) -> Statement {
        self.next_token();
        let var = self.parse_expr();
        let mut start: Option<Expr> = None;
        let end: Expr;
        let mut step = Expr::Number(1);
        let mut fstatement = Vec::new();

        if let Some(Token::Assign) = &self.current_token {
            self.next_token();
            start = Some(self.parse_expr());
        }
        if let Some(Token::To) = &self.current_token {
            self.next_token();
            end = self.parse_expr();
            if let Some(Token::Step) = &self.current_token {
                self.next_token();
                step = self.parse_expr();
            }
            if let Some(Token::Do) = &self.current_token {
                self.next_token();
                 fstatement = self.parse_token(vec![Token::EndFor]);
            } else {
                panic!("Expected keyword 'Do' ")
            };
            Statement::For(var, start, end, step, Box::new(fstatement))
        } else {
            panic!("Expected 'To' keyword")
        }
    }
}
