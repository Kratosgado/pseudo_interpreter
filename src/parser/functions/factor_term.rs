use crate::constants::error_handler::PseudoError;

use super::super::{Parser, Expr, Operator, Token, ParseComparison, ParsePrintExpr};

pub trait ParseFactorTerm {
    fn parse_factor(&mut self) -> Result<Expr, PseudoError>;
    fn parse_term(&mut self) -> Result<Expr, PseudoError>;
}

impl ParseFactorTerm for Parser {
    fn parse_term(&mut self) -> Result<Expr, PseudoError> {
        let mut left = self.parse_factor()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Equal
                | Token::LessThan
                | Token::GreaterThan
                | Token::LessThanEqual
                | Token::GreaterThanEqual
                | Token::NotEqual => {
                    left = self.parse_comparison(left)?;
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
                    let right = self.parse_factor()?;
                    left = Expr::BinOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, PseudoError> {
        match self.current_token.take() {
            Some(Token::Number(value)) => {
                self.next_token();
                Ok(Expr::Number(value))
            },
            Some(Token::Float(value)) => {
                self.next_token();
                Ok(Expr::Float(value))
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
                Ok(Expr::Str(value))
            }
            Some(Token::Ident(var)) => {
                self.next_token();
                if let Some(Token::LParen) = self.current_token {
                    self.next_token();
                    if let Some(Token::RParen) = self.current_token {
                        self.next_token();
                        Ok(Expr::FunctionCall(var, Box::new(Some(Expr::Multi(vec![])))))
                    } else {
                        let args = self.parse_expr()?;
                        if let Some(Token::RParen) = self.current_token {
                            self.next_token();
                            Ok(Expr::FunctionCall(var, Box::new(Some(args))))
                        } else {
                            panic!("Expected closing parenthesis");
                        }
                    }
                    // while let Some(token ) = &self.current_token {
                    //     match token {
                    //         Token::RParen => {
                    //             self.next_token();
                    //             break;
                    //         }
                    //         _ => args.push(self.parse_expr())
                    //     }
                    // }
                }else {
                    Ok(Expr::Variable(var))
                }
            }
            Some(Token::Boolean(value)) => {
                self.next_token();
                Ok(Expr::Boolean(value))
            }
            Some(Token::Array(var, index)) => {
                self.next_token();
                let index = if let Token::Number(val) = index.as_ref() {
                    Expr::Number(*val)
                } else if let Token::Ident(var) = index.as_ref() {
                    Expr::Variable(var.clone())
                }
                else  {
                    panic!("Invalid array index")
                };
                Ok(Expr::ArrayVariable(var, Box::new(index)))
            }
            Some(token) => todo!("Implement parsing of {:?}", token),
            None => return Err(PseudoError::UnexpectedEOF),
        }
    }
}
