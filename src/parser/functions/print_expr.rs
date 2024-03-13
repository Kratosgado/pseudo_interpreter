use super::super::{Expr,Statement, Token, ParseFactorTerm, Parser, Operator};
pub trait ParsePrintExpr {
    fn parse_print(&mut self) -> Statement;
    fn parse_expr(&mut self) -> Expr;
}

impl ParsePrintExpr for Parser {
    fn parse_print(&mut self) -> Statement {
        self.next_token();
        match &self.current_token {
            Some(Token::Ident(var)) => {
                let var = var.clone();
                self.next_token();
                Statement::Print(Expr::Variable(var))
            }
            _ => {
                let expr = self.parse_expr();
                Statement::Print(expr)
            }
        }
    }

     fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus | Token::Minus => {
                    let op = if matches!(token, Token::Plus) {
                        Operator::Add
                    } else {
                        Operator::Subtract
                    };
                    self.next_token();
                    let right = self.parse_term();
                    left = Expr::BinOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            };
        }
        left
    }
}