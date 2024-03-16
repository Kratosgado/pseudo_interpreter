use super::{ParseToken, Statement, Token};
pub struct Parser {
    tokens: Vec<Token>,
    pub current_token: Option<Token>,
    position: usize,
    pub if_stack: Vec<Statement>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_token: None,
            position: 0,
            if_stack: Vec::new(),
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
        let statements = self.parse_token();
        statements
    }
}
