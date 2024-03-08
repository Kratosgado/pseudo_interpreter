use super::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut number = 0;
                while let Some(&('0'..='9')) = chars.peek() {
                    number = number * 10 + chars.next().unwrap().to_digit(10).unwrap() as i64;
                }
                tokens.push(Token::Number(number));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Multiply);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Divide);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            _ => {
                chars.next();
            }
        }
    }
    tokens
}

