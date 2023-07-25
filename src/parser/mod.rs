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
        Self { tokens: it.peekable() }
    }

    pub fn parse(&mut self) -> Result<AST, Error> {
        let mut tree = AST::default();
        while let Some(tok) = self.tokens.peek() {
            let statement = match tok {
                Token::Let => self.parse_let(),
                Token::Return => self.parse_return(),
                _ => self.parse_expression_statement(),
            }?;
            tree.push(statement);
        }
        Ok(tree)
    }

    fn parse_let(&mut self) -> Result<Statement, Error> {
        self.expect_token(Token::Let)?;
        let identifier = self
            .expect_token(Token::Identifier(String::default()))
            .map(|tok| match tok {
                Token::Identifier(s) => s.to_string(),
                _ => unreachable!(),
            })?;
        self.expect_token(Token::Assign)?;
        while self.next_token()? != &Token::Semicolon {}
        Ok(Statement::Let {
            identifier,
            expression: Expression::None,
        })
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        self.expect_token(Token::Return)?;
        while self.next_token()? != &Token::Semicolon {}
        Ok(Statement::Return(Expression::None))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let exp = self.parse_expression(Priority::Lowest)?;
        let _ = self.expect_token(Token::Semicolon); // Semicolon is optional.
        Ok(Statement::Expression(exp))
    }

    fn parse_expression(&mut self, _priority: Priority) -> Result<Expression, Error> {
        self.parse_prefix()
    }

    fn parse_prefix(&mut self) -> Result<Expression, Error> {
        let tok = self.next_token()?;
        match tok {
            Token::Identifier(s) => Ok(Expression::Identifier(s.to_string())),
            Token::Integer(int) => Ok(Expression::Integer(*int)),
            Token::Bang | Token::Minus => {
                let tok = tok.clone();
                let expr = self.parse_expression(Priority::Unary)?;
                Ok(Expression::Unary {
                    operator: tok,
                    expression: Box::new(expr),
                })
            }
            _ => unreachable!(),
        }
    }

    fn next_token(&mut self) -> Result<&Token, Error> {
        self.tokens.next().ok_or(Error::EOF)
    }

    fn expect_token(&mut self, token: Token) -> Result<&Token, Error> {
        let tok = self.next_token()?;
        if mem::discriminant(tok) != mem::discriminant(&token) {
            return Err(Error::ExpectedToken {
                expected: token,
                got: (*tok).clone(),
            });
        }
        Ok(tok)
    }
}
