use super::{super::Token, datatypes::Datatype};
use crate::lexer::lexer::Lexer;

pub trait Identifier {
    fn encode_identifier(&mut self) -> Token;
}

impl<'a> Identifier for Lexer<'a> {
    fn encode_identifier(&mut self) -> Token {
        let mut id = String::new();
        let mut size: Token = Token::Null;
        while let Some(ch) = self.current_char {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    id.push(ch);
                    self.next_char();
                }
                '[' => {
                    self.next_char();
                    size = self.number();
                }
                ']' => {
                    self.next_char();
                }
                '\t' | '\n' => {
                    self.next_char();
                }
                _ => break,
            }
        }
        match id.to_lowercase().as_str() {
            "print" | "output" | "display" => Token::Print,
            "input" | "get" | "read" | "recieve" => Token::Input,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),

            // comparison
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,

            // decision making
            "if" => Token::If,
            "then" => Token::Then,
            "endif" => Token::EndIf,
            "else" => Token::Else,

            // loop
            "do" => Token::Do,
            "while" => Token::While,
            "to" => Token::To,
            "endwhile" => Token::EndWhile,
            "for" => Token::For,
            "step" => Token::Step,
            "endfor" => Token::EndFor,

            "function" => Token::Function,
            "endfunction" => Token::EndFunction,
            "return" => Token::Return,
            "continue" => Token::Continue,
            "break" => Token::Break,

            _ => {
                if size != Token::Null {
                    Token::Array(id, Box::new(size))
                } else {
                    Token::Ident(id)
                }
            }
        }
    }
}
