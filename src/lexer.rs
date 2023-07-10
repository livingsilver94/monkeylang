use std::fmt;
use std::io;
use std::iter;
use std::str::{self, FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Single-char long.
    Assign,
    Asterisk,
    Bang,
    Comma,
    GreaterThan,
    LeftBrace,
    LeftParen,
    LessThan,
    Minus,
    Plus,
    RightBrace,
    RightParen,
    Semicolon,
    Slash,

    // Two-char long.
    Equal,
    NotEqual,

    // Multichar reserved words.
    Else,
    False,
    Function,
    If,
    Let,
    Return,
    True,

    // User-provided values.
    Identifier(String),
    Integer(i64),
}

impl Token {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '=' => Some(Self::Assign),
            '*' => Some(Self::Asterisk),
            '!' => Some(Self::Bang),
            ',' => Some(Self::Comma),
            '>' => Some(Self::GreaterThan),
            '{' => Some(Self::LeftBrace),
            '(' => Some(Self::LeftParen),
            '<' => Some(Self::LessThan),
            '-' => Some(Self::Minus),
            '+' => Some(Self::Plus),
            '}' => Some(Self::RightBrace),
            ')' => Some(Self::RightParen),
            ';' => Some(Self::Semicolon),
            '/' => Some(Self::Slash),
            _ => None,
        }
    }

    fn from_two_chars(chs: &[char; 2]) -> Option<Self> {
        if chs[1] != '=' {
            return None;
        }
        match chs[0] {
            '!' => Some(Self::NotEqual),
            '=' => Some(Self::Equal),
            _ => None,
        }
    }

    fn may_be_two_chars(ch: char) -> bool {
        ch == '!' || ch == '='
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
        if s.len() == 2 {
            let second = s.as_bytes()[1] as char;
            if let Some(tok) = Self::from_two_chars(&[first, second]) {
                return Ok(tok);
            }
        }
        match s {
            "else" => Ok(Self::Else),
            "false" => Ok(Self::False),
            "fn" => Ok(Self::Function),
            "if" => Ok(Self::If),
            "let" => Ok(Self::Let),
            "return" => Ok(Self::Return),
            "true" => Ok(Self::True),
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

pub struct Lexer<R: std::io::Read> {
    input: iter::Peekable<io::Bytes<R>>,
    token_buf: String,
}

impl<R: io::Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Self {
            input: input.bytes().peekable(),
            token_buf: String::with_capacity(32),
        }
    }

    /// Returns the next character.
    fn read_char(&mut self) -> Result<char, Error> {
        let byte = self
            .input
            .next()
            .unwrap_or(Err(io::Error::from(io::ErrorKind::UnexpectedEof)))?;
        char::from_u32(byte as u32).ok_or_else(|| Error::TokenError(format!("{:x}", byte)))
    }

    /// Peeks the next character. If it was able to read the character,
    fn peek_char(&mut self) -> Result<char, Error> {
        let byte = self
            .input
            .peek()
            .map(|result| match result {
                Ok(byte) => Ok(*byte),
                Err(e) => Err(io::Error::from(e.kind())),
            })
            .unwrap_or(Err(io::Error::from(io::ErrorKind::UnexpectedEof)))
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
        let ch = self.read_nonwhitespace_char()?;
        if Token::may_be_two_chars(ch) {
            let next = self.peek_char()?;
            if let Some(tok) = Token::from_two_chars(&[ch, next]) {
                let _ = self.read_char();
                return Ok(tok);
            }
        }
        if let Some(tok) = Token::from_char(ch) {
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
            let ch = self.peek_char()?;
            if !cond(ch) {
                break;
            }
            let _ = self.read_char();
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
