use super::super::{
    Expr, ParseArray, ParseAssignment, ParseFor, ParseIf, ParseInput, ParsePrintExpr, ParseWhile,
    Parser, Statement, Token,
};

pub trait ParseFunction {
    fn parse_function(&mut self) -> Statement;
}

impl ParseFunction for Parser {
    fn parse_function(&mut self) -> Statement {
        self.next_token();
        if let Some(Token::Ident(name)) = self.current_token.clone() {
            self.next_token();
            if let Some(Token::LParen) = self.current_token {
                self.next_token();
                let mut params: Vec<Expr> = Vec::new();
                while self.current_token != Some(Token::RParen) {
                    match &self.current_token {
                        Some(Token::Ident(param)) => {
                            params.push(Expr::Param(param.clone()));
                            self.next_token();
                        }
                        _ => todo!("Invalid  function parameter"),
                    }
                }
                self.next_token();
                let mut fstatements: Vec<Statement> = Vec::new();
                let mut ret_ment: Option<Expr> = None;
                while let Some(token) = &self.current_token {
                    match token {
                        Token::Print => fstatements.push(self.parse_print()),
                        Token::Input => fstatements.push(self.parse_input()),
                        Token::Ident(_) => fstatements.push(self.parse_assignment()),
                        Token::Array(_, _) => fstatements.push(self.parse_array()),
                        Token::While => fstatements.push(self.parse_while()),
                        Token::If => fstatements.push(self.parse_if()),
                        Token::For => fstatements.push(self.parse_for()),
                        Token::Function => fstatements.push(self.parse_function()),
                        Token::EOL => self.next_token(),
                        Token::EOF => break,
                        Token::Return => {
                            self.next_token();
                            ret_ment = Some(self.parse_expr())
                        }
                        Token::EndFunction => {
                            self.next_token();
                            break;
                        }
                        Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                            fstatements.push(Statement::Expr(self.parse_expr()))
                        }
                        _ => panic!("Unexpected token: {:?}", token),
                    }
                }
                Statement::Function(name.clone(), params, Box::new(fstatements), ret_ment)
            } else {
                panic!("missing opening parenthesis")
            }
        } else {
            panic!("missing function name")
        }
    }
}
