use super::super::tokens::token::Token;

pub fn parse(tokens: &[Token]) -> i64 {
    // implement parser and evaluator
    if tokens.len() == 3 {
        match tokens[1] {
            Token::Number(_) => todo!(),
            Token::Plus => {
               if let Token::Number(n1) = tokens[0] {
                   if let Token::Number(n2) = tokens[2] {
                       return n1 + n2;
                   }
               }
               panic!("Invalid syntax");
            },
            Token::Minus => todo!(),
            Token::Multiply => todo!(),
            Token::Divide => todo!(),
            Token::LParen => todo!(),
            Token::RParen => todo!(),
        }
    }
    

    if tokens.len() == 1 {
        if let Token::Number(number) = tokens[0] {
            return number;
        }
    }
    panic!("Unexpected token sequence");
}