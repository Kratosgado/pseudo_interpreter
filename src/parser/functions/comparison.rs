use super::super::{Expr, Operator, Parser, Token, ParsePrintExpr};

pub trait ParseComparison {
    fn parse_comparison(&mut self, left: Expr) -> Expr;
}

impl ParseComparison for Parser {
    fn parse_comparison(&mut self, left: Expr) -> Expr {
        if let Some(token) = &self.current_token {
            let op = match token {
                Token::Equal => Operator::Equal,
                Token::LessThan => Operator::LessThan,
                Token::GreaterThan => Operator::GreaterThan,
                Token::LessThanEqual => Operator::LessThanEqual,
                Token::GreaterThanEqual => Operator::GreaterThanEqual,
                Token::NotEqual => Operator::NotEqual,
                _ => panic!("Expected comparison operator"),
            };
            self.next_token();
            let right = self.parse_expr();
            Expr::Comparison(Box::new(left), op, Box::new(right))
        } else {
            left
        }
    }
}