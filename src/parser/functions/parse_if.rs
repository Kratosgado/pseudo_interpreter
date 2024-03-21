use super::{
    super::{parser::Parser, ParsePrintExpr, Statement, Token},
    parse_token::ParseToken,
};

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
            consequence.extend(self.parse_token(vec![Token::EndIf, Token::Else]));
            if let Some(Token::Else) = self.current_token {
                self.next_token();
                alternative.extend(self.parse_token(vec![Token::EndIf]));
            }
            Statement::If(
                condition,
                Box::new(consequence),
                Some(Box::new(alternative)),
            )
        } else {
            panic!("Expected 'then' after if condition");
        }
    }
}
