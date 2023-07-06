use std::fmt;
use std::io::{self, Read};
use std::iter;
use std::slice;
use std::str::{self, FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Integer(i64),

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,
}

impl Token {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '=' => Some(Self::Assign),
            '+' => Some(Self::Plus),
            ',' => Some(Self::Comma),
            ';' => Some(Self::Semicolon),
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            _ => None,
        }
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::TokenError(String::new()));
        }
        let first = s.as_bytes()[0] as char;
        if s.len() == 1 {
            if let Some(tok) = Self::from_char(first) {
                return Ok(tok);
            }
        }
        match s {
            "fn" => Ok(Self::Function),
            "let" => Ok(Self::Let),
            _ => {
                if first.is_ascii_digit() {
                    return i64::from_str(s)
                        .map(Self::Integer)
                        .map_err(|_| Error::TokenError(String::from(s)));
                }
                Ok(Self::Identifier(s.to_string()))
            }
        }
    }
}

pub struct Lexer<R> {
    input: io::BufReader<R>,
    token_buf: String,
    /// The latest character read but not yet parsed.
    latest_char: Option<char>,
}

impl<R: io::Read> Lexer<R> {
    pub fn new(inp: R) -> Self {
        Self {
            input: io::BufReader::new(inp),
            token_buf: String::with_capacity(32),
            latest_char: None,
        }
    }

    /// Returns the next character.
    fn read_char(&mut self) -> Result<char, Error> {
        let mut byte: u8 = 0;
        self.input
            .read_exact(slice::from_mut(&mut byte))
            .map_err(Error::from)?;
        char::from_u32(byte as u32).ok_or_else(|| Error::TokenError(format!("{:x}", byte)))
    }

    fn read_nonwhitespace_char(&mut self) -> Result<char, Error> {
        loop {
            let ch = self.read_char()?;
            if !ch.is_ascii_whitespace() {
                return Ok(ch);
            }
        }
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        let ch = match self.latest_char.filter(|c| !c.is_ascii_whitespace()) {
            Some(c) => c,
            None => self.read_nonwhitespace_char()?,
        };
        if let Some(tok) = Token::from_char(ch) {
            self.latest_char = None;
            return Ok(tok);
        }
        self.token_buf.clear();
        self.token_buf.push(ch);
        if is_identifier_char(ch) {
            self.fill_buffer_until(is_identifier_char)?;
        } else if ch.is_ascii_digit() {
            self.fill_buffer_until(|ch| ch.is_ascii_digit())?;
        }
        Token::from_str(&self.token_buf).map_err(|_| Error::TokenError(self.token_buf.clone()))
    }

    fn fill_buffer_until<F>(&mut self, cond: F) -> Result<(), Error>
    where
        F: Fn(char) -> bool,
    {
        loop {
            let ch = self.read_char()?;
            self.latest_char = Some(ch);
            if !cond(ch) {
                break;
            }
            self.token_buf.push(ch);
        }
        Ok(())
    }
}

impl<R: io::Read> iter::Iterator for Lexer<R> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_token() {
            Ok(tok) => Some(Ok(tok)),
            Err(err) => {
                if let Error::IO(ref e) = err {
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        return None;
                    }
                }
                Some(Err(err))
            }
        }
    }
}

#[derive(thiserror::Error, fmt::Debug)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] io::Error),

    #[error("invalid token \"{0}\"")]
    TokenError(String),
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
