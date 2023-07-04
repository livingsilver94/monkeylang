use std::fmt;
use std::io::{self, Read};
use std::iter;
use std::slice;
use std::str::{self, FromStr};

#[derive(strum::Display, strum::EnumString, Debug, PartialEq, Eq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Token {
    #[strum(serialize = "")]
    EOF,

    Identifier(String),
    Integer(String),

    #[strum(serialize = "=")]
    Assign,
    #[strum(serialize = "+")]
    Plus,

    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ";")]
    Semicolon,

    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,

    #[strum(serialize = "fn")]
    Function,
    #[strum(serialize = "let")]
    Let,
}

pub struct Lexer<R> {
    input: io::BufReader<R>,
}

impl<T: io::Read> Lexer<T> {
    pub fn new(inp: T) -> Self {
        Self {
            input: io::BufReader::new(inp),
        }
    }

    fn read_char(&mut self) -> Result<u8, Error> {
        let mut ch: u8 = 0;
        self.input
            .read_exact(slice::from_mut(&mut ch))
            .map_err(Error::from)?;
        Ok(ch)
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        let s = &[self.read_char()?];
        str::from_utf8(s)
            .ok()
            .and_then(|s| Token::from_str(s).ok())
            .ok_or_else(|| Error::TokenError(format!("{:x}", s[0])))
    }
}

impl<T: io::Read> iter::Iterator for Lexer<T> {
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

    #[error("unknown token string {0}")]
    TokenError(String),
}
