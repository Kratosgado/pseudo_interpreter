use crate::lexer::expr::Expr;
use crate::lexer::operator::Operator;
use crate::lexer::statement::Statement;
use crate::lexer::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Option<Token>,
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

    fn next_token(&mut self) {
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
                Token::Print => {
                    self.next_token();
                    let expr = self.parse_expr();
                    statements.push(Statement::Print(expr));
                    if matches!(self.current_token, Some(Token::EOL)) {
                        self.next_token();
                    }
                }
                Token::EOF => break,
                _ => {
                    let expr = self.parse_expr();
                    println!("{:?}", expr);
                    statements.push(Statement::Expr(expr));
                    if matches!(self.current_token, Some(Token::EOL)) {
                        self.next_token();
                    }
                }
            }
        }
        statements
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus | Token::Minus => {
                    let op = if matches!(token, Token::Plus) {
                        Operator::Add
                    } else {
                        Operator::Subtract
                    };
                    self.next_token();
                    let right = self.parse_term();
                    left = Expr::BinOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            };
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while let Some(token) = &self.current_token {
            match token {
                Token::Multiply | Token::Divide => {
                    let op = if matches!(token, Token::Multiply) {
                        Operator::Multiply
                    } else {
                        Operator::Divide
                    };
                    self.next_token();
                    let right = self.parse_factor();
                    left = Expr::BinOp(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current_token.take() {
            Some(Token::Number(value)) => {
                self.next_token();
                Expr::Number(value)
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
                Expr::Str(value)
            }
            _ => todo!(
                "Implement parsing of strings and variables, {:?}",
                self.current_token
            ),
        }
    }
}
