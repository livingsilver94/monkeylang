use std::iter::Peekable;
use std::mem;

use crate::lexer::Token;
use ast::{Statement, AST};

use self::ast::Expression;

pub mod ast;

pub struct Parser<'a, T: Iterator<Item = &'a Token>> {
    tokens: Peekable<T>,
}

impl<'a, T: Iterator<Item = &'a Token>> Parser<'a, T> {
    pub fn new(it: T) -> Self {
        Self {
            tokens: it.peekable(),
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut tree = AST::default();
        while let Some(st) = self.parse_statement() {
            tree.push(st);
        }
        tree
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.tokens.next()? {
            Token::Let => self.parse_let(),
            _ => None,
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        let identifier = match self.tokens.peek()? {
            Token::Identifier(s) => {
                self.tokens.next();
                s.to_string()
            }
            _ => return None,
        };
        if !self.peek_expected(Token::Assign) {
            return None;
        }
        while self.tokens.next()? != &Token::Semicolon {}
        Some(Statement::Let {
            identifier,
            expression: Expression::None,
        })
    }

    fn peek_expected(&mut self, token: Token) -> bool {
        match self.tokens.peek() {
            Some(tok) => {
                if mem::discriminant(*tok) != mem::discriminant(&token) {
                    return false;
                }
                let _ = self.tokens.next();
                true
            }
            None => false,
        }
    }
}
