use super::super::{parser::Parser, ParseAssignment, ParsePrintExpr, ParseWhile, Statement, Token};

pub trait ParseIf {
    fn parse_if(&mut self) -> Statement;
}

impl ParseIf for Parser {
    fn parse_if(&mut self) -> Statement {
        self.next_token();
        let condition = self.parse_expr();
        let mut consequence: Vec<Statement> = Vec::new();
        let mut alternative: Vec<Statement> = Vec::new();

        if let Some(Token::Then) = self.current_token {
            self.next_token();
            while let Some(token) = &self.current_token {
                match token {
                    Token::EndIf => {
                        // self.if_stack.pop().expect("Unmatched endif");
                        self.next_token();
                        break;
                    }
                    Token::Else => {
                        self.next_token();
                        while let Some(token) = &self.current_token {
                            match token {
                                Token::EndIf => {
                                    // self.next_token();
                                    break;
                                }
                                Token::Print => alternative.push(self.parse_print()),
                                Token::Ident(_) => alternative.push(self.parse_assignment()),
                                Token::If => {
                                    // let new_if = self.parse_if();
                                    alternative.push(self.parse_if());
                                    // self.if_stack.push(new_if);
                                }
                                Token::EOL => self.next_token(),
                                Token::EOF => break,
                                _ => {
                                    let expr = self.parse_expr();
                                    alternative.push(Statement::Expr(expr));
                                }
                            }
                        }
                    }
                    Token::Print => consequence.push(self.parse_print()),
                    Token::Ident(_) => consequence.push(self.parse_assignment()),
                    Token::While => consequence.push(self.parse_while()),
                    Token::If => {
                        // let new_if = self.parse_if();
                        consequence.push(self.parse_if());
                        // self.if_stack.push(new_if);
                    }
                    Token::EOL => self.next_token(),
                    Token::EOF => break,
                    _ => {
                        let expr = self.parse_expr();
                        consequence.push(Statement::Expr(expr));
                    } // _ => {
                      //     consequence = self.parse();
                      //     panic!("Expected 'EndIf' keyword");
                      // }
                }
            }
        } else {
            panic!("Expected 'then' after if condition");
        }

        Statement::If(
            condition,
            Box::new(consequence),
            Some(Box::new(alternative)),
        )
    }
}
