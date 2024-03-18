use super::super::{ParsePrintExpr, Statement,Expr, Token, Parser};

pub trait ParseAssignment {
    fn parse_assignment(&mut self) -> Statement;
}

impl ParseAssignment for Parser {
    fn parse_assignment(&mut self) -> Statement {
        match &self.current_token {
            Some(Token::Ident(var)) => {
                let var = var.clone();
                self.next_token();
                if let Some(Token::Assign) = self.current_token {
                    self.next_token();
                    let expr = self.parse_expr();
                    Statement::Assignment(var, expr)
                }else if let Some(Token::LParen) = self.current_token {
                    self.next_token();
                    let args = self.parse_expr();
                    // while let Some(token ) = &self.current_token {
                    //     match token {
                    //         Token::RParen => {
                    //             self.next_token();
                    //             break;
                    //         }
                    //         _ => args.push(self.parse_expr())
                    //     }
                    // }
                    Statement::Expr(Expr::FunctionCall(var, Box::new(args)))
                }
                 else {
                    panic!("Expected assignment operator");
                }
            }
            _ => todo!("Implement for other types of assignments"),
        }
    }
}