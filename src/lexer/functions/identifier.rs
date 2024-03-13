use super::super::Token;
use crate::lexer::lexer::Lexer;

pub trait Identifier {
    fn encode_identifier(&mut self) -> Token;
}

impl<'a> Identifier for Lexer<'a> {
    fn encode_identifier(&mut self) -> Token {
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

            _ => Token::Ident(id),
        }
    }
}
