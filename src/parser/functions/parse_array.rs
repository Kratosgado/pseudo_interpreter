use crate::{evaluator::Expr, lexer::enums::token::Token, parser::ParsePrintExpr};

use super::super::{Parser, Statement};

pub trait ParseArray {
    fn parse_array(&mut self) -> Statement;
}

impl ParseArray for Parser {
    fn parse_array(&mut self) -> Statement {
        if let Some(Token::Array(var, size)) = self.current_token.clone() {
            let size = if let Token::Number(val) = size.as_ref() {
                *val
            } else {
                panic!("Invalid array size")
            };
            self.next_token();
            if let Some(Token::Assign) = &self.current_token {
                self.next_token();
                let mut indices = Vec::new();
                self.next_token();
                while let Some(token) = &self.current_token {
                    match token {
                        Token::Number(_) => indices.push(self.parse_expr()),
                        Token::Str(_) => indices.push(self.parse_expr()),
                        Token::Boolean(_) => indices.push(self.parse_expr()),
                        Token::RBracket => {
                            self.next_token();
                            break;
                        }
                        _ => panic!("Invalid array value"),
                    }
                }
                Statement::AssignArray(var.clone(), Expr::Number(size), Box::new(indices))
            } else {
                panic!("Invalid array assignment")
            }
        } else {
            panic!("Invalid array declaration")
        }
    }
}
