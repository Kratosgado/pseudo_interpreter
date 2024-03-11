use std::str::Chars;

use super::token::Token;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            current_char: None,
        };
        lexer.next_char(); // Initialize the first character
        lexer
    }

    fn next_char(&mut self) {
        self.current_char = self.chars.next();
    }

    /// tokenize inputs
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()),
                '+' | '-' | '*' | '/' | '%' | '(' | ')' => tokens.push(self.operator()),
                '<' | '>' | '!' | '=' | '&' | '|' => tokens.push(self.comparison()),
                '\n' | ';' => {
                    tokens.push(Token::EOL);
                    self.next_char();
                }
                ' ' | '\t' | '\r' => self.next_char(),
                '"' => tokens.push(self.string()),
                _ => panic!("Invalid character: {}", ch),
            }
        }
        tokens
    }

    /// catch numbers
    fn number(&mut self) -> Token {
        let mut number = 0;
        while let Some('0'..='9') = self.current_char {
            number = number * 10 + self.current_char.unwrap().to_digit(10).unwrap() as i64;
            self.next_char();
        }
        Token::Number(number)
    }

    /// catch strings
    fn string(&mut self) -> Token {
        let mut string = String::new();
        self.next_char();
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.next_char();
                break;
            }
            string.push(ch);
            self.next_char();
        }
        Token::Str(string)
    }

    /// catch comparisons
    /// catch =, ==, <, >, <=, >=, !=
    /// catch &&, ||
    /// catch !
    fn comparison(&mut self) -> Token {
        match self.current_char {
            Some('<') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            Some('!') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            Some('=') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('&') => {
                self.next_char();
                if let Some('&') = self.current_char {
                    self.next_char();
                    Token::And
                } else {
                    panic!("Invalid character: &");
                }
            }
            Some('|') => {
                self.next_char();
                if let Some('|') = self.current_char {
                    self.next_char();
                    Token::Or
                } else {
                    panic!("Invalid character: |");
                }
            }
            _ => panic!("Invalid character"),
        }
    }

    /// catch operators
    fn operator(&mut self) -> Token {
        match self.current_char {
            Some('+') => {
                self.next_char();
                Token::Plus
            }
            Some('-') => {
                self.next_char();
                Token::Minus
            }
            Some('*') => {
                self.next_char();
                Token::Multiply
            }
            Some('/') => {
                self.next_char();
                Token::Divide
            }
            Some('%') => {
                self.next_char();
                Token::Modulo
            }
            Some('(') => {
                self.next_char();
                Token::LParen
            }
            Some(')') => {
                self.next_char();
                Token::RParen
            }
            _ => panic!("Invalid operator"),
        }
    }

    // catch variables
    fn identifier(&mut self) -> Token {
        let mut id = String::new();

        while let Some(ch) = self.current_char {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    id.push(ch);
                    self.next_char();
                }
                _ => break,
            }
        }
        match id.to_lowercase().as_str() {
            "print" | "output" | "display" => Token::Print,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),

            // comparison
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            
            // decision making
            "if" => Token::If,
            "then" => Token::Then, // "then" is not a keyword, but it is used in the parser
            "else" => Token::Else,
            "do" => Token::Do, // "do" is not a keyword, but it is used in the parser
            "while" => Token::While,
            "for" => Token::For,
            "function" => Token::Function,
            "return" => Token::Return,
            "continue" => Token::Continue,
            "break" => Token::Break,

            

            _ => Token::Ident(id),
        }
        
    }
}
