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
        let statements = self.parse_statements().map_err(|partials| partials.error)?;
        Ok(AST::new(statements))
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, PartialStatements> {
        let mut statements = Vec::new();
        while let Some(tok) = self.tokens.peek() {
            let statement = match tok {
                Token::Let => self.parse_let(),
                Token::Return => self.parse_return(),
                _ => self.parse_expression_statement(),
            };
            match statement {
                Ok(stat) => statements.push(stat),
                Err(partial) => {
                    return Err(PartialStatements {
                        statements,
                        error: partial,
                    })
                }
            }
        }
        Ok(statements)
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

    /// Parses an expression using the Pratt parsing algorithm.
    fn parse_expression(&mut self, priority: Priority) -> Result<Expression, Error> {
        let mut exp = self.parse_prefix()?;
        while let Some(tok) = self.tokens.peek() {
            if tok != &&Token::Semicolon && priority < Priority::from_token(tok) {
                exp = self.parse_infix(exp)?;
            } else {
                break;
            }
        }
        Ok(exp)
    }

    fn parse_prefix(&mut self) -> Result<Expression, Error> {
        let tok = self.next_token()?;
        match tok {
            Token::True | Token::False => Ok(Expression::Boolean(tok == &Token::True)),
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
            Token::LeftParen => {
                // Reset the priority as if we were parsing an expression
                // from the beginning.
                let expr = self.parse_expression(Priority::Lowest)?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            }
            Token::If => {
                self.expect_token(Token::LeftParen)?;
                let cond = self.parse_expression(Priority::Lowest)?;
                self.expect_token(Token::RightParen)?;
                self.expect_token(Token::LeftBrace)?;
                let conseq: Vec<Statement> = match self.parse_statements() {
                    Ok(stats) => stats,
                    Err(e) => {
                        if let Error::ExpectedToken { expected: _, got } = &e.error {
                            if got == &Token::RightBrace {
                                e.statements
                            } else {
                                return Err(e.error);
                            }
                        } else {
                            return Err(e.error);
                        }
                    }
                };
                Ok(Expression::If {
                    cond: Box::new(cond),
                    conseq,
                    altern: None,
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_infix(&mut self, left_expr: Expression) -> Result<Expression, Error> {
        let tok = self.next_token()?;
        match tok {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::GreaterThan
            | Token::LessThan
            | Token::Equal
            | Token::NotEqual => {
                let tok = tok.clone();
                let right_exp = self.parse_expression(Priority::from_token(&tok))?;
                Ok(Expression::Binary {
                    left: Box::new(left_expr),
                    operator: tok,
                    right: Box::new(right_exp),
                })
            }
            _ => Ok(left_expr),
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

/// Contains the statements parsed before the error occurred, along
/// with the error.
struct PartialStatements {
    statements: Vec<Statement>,
    error: Error,
}
