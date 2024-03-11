use super::{ Statement, Token, PrintExpr, Assignment};
pub struct Parser {
    tokens: Vec<Token>,
    pub current_token: Option<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_token: None,
            position: 0,
        };
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = Some(self.tokens[self.position].clone());

            self.position += 1;
        } else {
            self.current_token = Some(Token::EOF);
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while let Some(token) = &self.current_token {
            match token {
                Token::Print => statements.push(self.parse_print()),
                Token::Ident(_) => statements.push(self.parse_assignment()),
                Token::EOL => self.next_token(),
                Token::EOF => break,
                _ => {
                    let expr = self.parse_expr();
                    statements.push(Statement::Expr(expr));
                }
            }
        }
        statements
    }

}
