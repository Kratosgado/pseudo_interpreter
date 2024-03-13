use super::super::{
    parser::Parser, ParseAssignment, ParseFor, ParseIf, ParsePrintExpr, Statement, Token,
};

pub trait ParseWhile {
    fn parse_while(&mut self) -> Statement;
}

impl ParseWhile for Parser {
    fn parse_while(&mut self) -> Statement {
        self.next_token();
        let condition = self.parse_expr();
        let mut wstatement = Vec::new();

        if let Some(Token::Do) = &self.current_token {
            self.next_token();
            while let Some(token) = &self.current_token {
                match token {
                    Token::Print => wstatement.push(self.parse_print()),
                    Token::Ident(_) => wstatement.push(self.parse_assignment()),
                    Token::While => wstatement.push(self.parse_while()),
                    Token::For => wstatement.push(self.parse_for()),
                    Token::If => wstatement.push(self.parse_if()),
                    Token::EOL => self.next_token(),
                    Token::EOF => break,
                    Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                        wstatement.push(Statement::Expr(self.parse_expr()))
                    }
                    Token::EndWhile => {
                        self.next_token();
                        break;
                    }
                    _ => panic!("Expected 'EndWhile' keyword"),
                }
            }
        }
        Statement::While(condition, Box::new(wstatement))
    }
}
