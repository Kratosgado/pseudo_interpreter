use super::super::{parser::Parser, Expr, Operator, Token};
use super::{comparison::Comparison, print_expr::PrintExpr};

pub trait FactorTerm {
    fn parse_factor(&mut self) -> Expr;
    fn parse_term(&mut self) -> Expr;
}

impl FactorTerm for Parser {
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while let Some(token) = &self.current_token {
            match token {
                Token::Equal
                | Token::LessThan
                | Token::GreaterThan
                | Token::LessThanEqual
                | Token::GreaterThanEqual
                | Token::NotEqual => {
                    left = self.parse_comparison(left);
                }
                Token::Multiply | Token::Divide | Token::Modulo => {
                    let op = if matches!(token, Token::Multiply) {
                        Operator::Multiply
                    } else if matches!(token, Token::Modulo) {
                        Operator::Modulo
                    } else {
                        Operator::Divide
                    };
                    self.next_token();
                    let right = self.parse_factor();
                    left = Expr::BinOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current_token.take() {
            Some(Token::Number(value)) => {
                self.next_token();
                Expr::Number(value)
            }
            Some(Token::LParen) => {
                self.next_token();
                let expr = self.parse_expr();
                if let Some(Token::RParen) = self.current_token {
                    self.next_token();
                    expr
                } else {
                    panic!("Expected closing parenthesis");
                }
            }
            Some(Token::Str(value)) => {
                self.next_token();
                Expr::Str(value)
            }
            Some(Token::Ident(var)) => {
                self.next_token();
                Expr::Variable(var)
            }
            Some(Token::Boolean(value)) => {
                self.next_token();
                Expr::Boolean(value)
            }
            Some(token) => todo!("Implement parsing of {:?}", token),
            None => panic!("Expected a token"),
        }
    }
}
