use crate::constants::error_handler::PseudoError;

use super::{ParseToken, Statement, Token};
pub struct Parser {
    tokens: Vec<Token>,
    pub current_token: Option<Token>,
    position: usize,
    pub if_stack: Vec<Statement>,
    pub line: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            current_token: None,
            position: 0,
            if_stack: Vec::new(),
            line: 1,
        };
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            if token == Token::EOL {
                self.line += 1;
            }
            self.current_token = Some(token);
            self.position += 1;
        } else {
            self.current_token = Some(Token::EOF);
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, PseudoError> {
        self.parse_token(vec![Token::EOF])
    }
}
