use crate::{
    constants::error_handler::{KeywordError, PseudoError},
    lexer::enums::statement::{IfCond, IfStructure},
};

use super::{
    super::{parser::Parser, ParsePrintExpr, Statement, Token},
    parse_token::ParseToken,
};

pub trait ParseIf {
    fn parse_if(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseIf for Parser {
    fn parse_if(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        let condition = self.parse_expr()?;
        let mut consequence: Vec<Statement> = Vec::new();
        let mut elseifs: Vec<IfCond> = vec![];
        let mut alternative: Vec<Statement> = Vec::new();

        if let Some(Token::Then) = self.current_token {
            self.next_token();
            consequence.extend(self.parse_token(vec![Token::EndIf, Token::Else, Token::ElseIf])?);
            while Some(Token::Else) == self.current_token
                || Some(Token::ElseIf) == self.current_token
            {
                match self.current_token {
                    Some(Token::Else) => {
                        self.next_token();
                        alternative.extend(self.parse_token(vec![Token::EndIf])?);
                    }
                    Some(Token::ElseIf) => {
                        self.next_token();
                        let cond = self.parse_expr()?;

                        if Some(Token::Then) == self.current_token {
                            self.next_token();
                            self.next_token();
                            self.next_token();
                            let conseq =
                                self.parse_token(vec![Token::EndIf, Token::Else, Token::ElseIf])?;
                            elseifs.push(IfCond {
                                cond,
                                consequence: conseq,
                            })
                        }
                    }
                    _ => break,
                }
            }
            let ifstruct = IfStructure {
                ifcond: IfCond {
                    cond: condition,
                    consequence,
                },
                elseifs: Some(elseifs),
                alternative: Some(alternative),
            };

            Ok(Statement::If(ifstruct))
        } else {
            return Err(PseudoError::keyword(
                vec![Token::Then],
                &self.current_token.as_ref().unwrap(),
            ));
        }
    }
}
