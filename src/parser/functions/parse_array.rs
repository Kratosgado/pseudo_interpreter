use crate::{evaluator::Expr, lexer::enums::token::Token, parser::ParsePrintExpr};

use super::super::{Parser, Statement};

pub trait ParseArray {
    fn parse_array(&mut self) -> Statement;
}

impl ParseArray for Parser {
    fn parse_array(&mut self) -> Statement {
        if let Some(Token::Array(var, size)) = self.current_token.clone() {
            let size = match size.as_ref() {
                Token::Number(val) => Expr::Number(*val),
                Token::Ident(var) => Expr::Variable(var.clone()),
                _ => panic!("Invalid array size"),
            };
            self.next_token();
            if let Some(Token::Assign) = &self.current_token {
                self.next_token();
                if let Some(Token::LBracket) = &self.current_token {
                    let mut indices = Vec::new();
                    self.next_token();
                    while let Some(token) = &self.current_token {
                        match token {
                            Token::Number(_) => indices.push(self.parse_expr()),
                            Token::Str(_) => indices.push(self.parse_expr()),
                            Token::Boolean(_) => indices.push(self.parse_expr()),
                            Token::Ident(_) => indices.push(self.parse_expr()),
                            Token::RBracket => {
                                self.next_token();
                                break;
                            }
                            _ => panic!("Invalid array value"),
                        }
                    }
                    Statement::AssignArray(var.clone(), size, Box::new(indices))
                } else {
                    Statement::AssignIndex(var.clone(), size, self.parse_expr())
                }
            } else {
                Statement::DeclareArray(var.clone(), size)
            }
        } else {
            panic!("Invalid array declaration")
        }
    }
}
