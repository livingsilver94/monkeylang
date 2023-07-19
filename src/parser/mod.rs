mod ast;
pub use ast::*;

mod error;
pub use error::*;

use std::iter::Peekable;
use std::mem;

use crate::lexer::Token;

pub struct Parser<'a, T: Iterator<Item = &'a Token>> {
    tokens: Peekable<T>,
}

impl<'a, T: Iterator<Item = &'a Token>> Parser<'a, T> {
    pub fn new(it: T) -> Self {
        Self {
            tokens: it.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<AST, Error> {
        let mut tree = AST::default();
        while let Some(st) = self.parse_statement() {
            tree.push(st?);
        }
        Ok(tree)
    }

    fn parse_statement(&mut self) -> Option<Result<Statement, Error>> {
        match self.tokens.next()? {
            Token::Let => Some(self.parse_let()),
            Token::Return => Some(self.parse_return()),
            _ => None,
        }
    }

    fn parse_let(&mut self) -> Result<Statement, Error> {
        let identifier = self
            .expect_token(Token::Identifier(String::default()))
            .map(|tok| match tok {
                Token::Identifier(s) => s.to_string(),
                _ => unreachable!(),
            })?;
        self.expect_token(Token::Assign)?;
        while self.tokens.next().ok_or(Error::EOF)? != &Token::Semicolon {}
        Ok(Statement::Let {
            identifier,
            expression: Expression::None,
        })
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        while self.tokens.next().ok_or(Error::EOF)? != &Token::Semicolon {}
        Ok(Statement::Return(Expression::None))
    }

    fn expect_token(&mut self, token: Token) -> Result<&Token, Error> {
        match self.tokens.peek() {
            Some(tok) => {
                if mem::discriminant(*tok) != mem::discriminant(&token) {
                    return Err(Error::ExpectedToken {
                        expected: token,
                        got: (**tok).clone(),
                    });
                }
                Ok(self.tokens.next().unwrap())
            }
            None => Err(Error::EOF),
        }
    }
}
