use super::super::Token;
use crate::lexer::lexer::Lexer;

// catch variables
pub fn identifier(lexer: &mut Lexer) -> Token {
    let mut id = String::new();

    while let Some(ch) = lexer.current_char {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                id.push(ch);
                lexer.next_char();
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
        "do" => Token::Do,
        "while" => Token::While,
        "for" => Token::For,
        "function" => Token::Function,
        "return" => Token::Return,
        "continue" => Token::Continue,
        "break" => Token::Break,

        _ => Token::Ident(id),
    }
}
