use super::{
    super::{
        parser::Parser, ParseArray, ParseAssignment, ParseFor, ParseFunction, ParseInput,
        ParsePrintExpr, ParseWhile, Statement, Token,
    },
    token::ParseToken,
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
            while let Some(token) = &self.current_token {
                match token {
                    Token::Else => {
                        self.next_token();
                        alternative = self.parse_token(Token::EndIf);
                    }
                    Token::Print => consequence.push(self.parse_print()),
                    Token::Input => consequence.push(self.parse_input()),
                    Token::Ident(_) => consequence.push(self.parse_assignment()),
                    Token::Array(_, _) => consequence.push(self.parse_array()),
                    Token::While => consequence.push(self.parse_while()),
                    Token::If => consequence.push(self.parse_if()),
                    Token::For => consequence.push(self.parse_for()),
                    Token::Function => consequence.push(self.parse_function()),
                    Token::EOL => self.next_token(),
                    Token::EOF => break,
                    Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                        consequence.push(Statement::Expr(self.parse_expr()))
                    }
                    Token::EndIf => {
                        self.next_token();
                        break;
                    }
                    _ => panic!("Expected 'EndIf' keyword"),
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
