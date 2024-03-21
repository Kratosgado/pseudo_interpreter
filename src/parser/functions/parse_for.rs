use crate::parser::ParseArray;

use super::super::{
    Expr, ParseAssignment, ParseFunction, ParseIf, ParseInput, ParsePrintExpr, ParseWhile, Parser,
    Statement, Token,
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
                while let Some(token) = &self.current_token {
                    match token {
                        Token::Print => fstatement.push(self.parse_print()),
                        Token::Input => fstatement.push(self.parse_input()),
                        Token::Ident(_) => fstatement.push(self.parse_assignment()),
                        Token::Array(_, _) => fstatement.push(self.parse_array()),
                        Token::While => fstatement.push(self.parse_while()),
                        Token::If => fstatement.push(self.parse_if()),
                        Token::For => fstatement.push(self.parse_for()),
                        Token::Function => fstatement.push(self.parse_function()),
                        Token::EOL => self.next_token(),
                        Token::EOF => break,
                        Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                            fstatement.push(Statement::Expr(self.parse_expr()))
                        }
                        Token::EndFor => {
                            self.next_token();
                            break;
                        }
                        _ => panic!("Expected 'EndFor' keyword"),
                    }
                }
            } else {
                panic!("Expected keyword 'Do' ")
            };
            Statement::For(var, start, end, step, Box::new(fstatement))
        } else {
            panic!("Expected 'To' keyword")
        }
    }
}
