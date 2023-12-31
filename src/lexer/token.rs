use std::fmt;
use std::str::FromStr;

use super::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn from_char(ch: char) -> Option<Self> {
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

    pub fn from_two_chars(chs: &[char; 2]) -> Option<Self> {
        if chs[1] != '=' {
            return None;
        }
        match chs[0] {
            '!' => Some(Self::NotEqual),
            '=' => Some(Self::Equal),
            _ => None,
        }
    }

    pub fn may_be_two_chars(ch: char) -> bool {
        ch == '!' || ch == '='
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::Token(String::new()));
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
                        .map_err(|_| Error::Token(String::from(s)));
                }
                Ok(Self::Identifier(s.to_string()))
            }
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Assign => write!(f, "="),
            Token::Asterisk => write!(f, "*"),
            Token::Bang => write!(f, "!"),
            Token::Comma => write!(f, ","),
            Token::Else => write!(f, "else"),
            Token::Equal => write!(f, "=="),
            Token::False => write!(f, "false"),
            Token::Function => write!(f, "fn"),
            Token::GreaterThan => write!(f, ">"),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::If => write!(f, "if"),
            Token::Integer(int) => write!(f, "{}", int),
            Token::LeftBrace => write!(f, "{{"),
            Token::LeftParen => write!(f, "("),
            Token::LessThan => write!(f, "<"),
            Token::Let => write!(f, "let"),
            Token::Minus => write!(f, "-"),
            Token::NotEqual => write!(f, "!="),
            Token::Plus => write!(f, "+"),
            Token::Return => write!(f, "return"),
            Token::RightBrace => write!(f, "}}"),
            Token::RightParen => write!(f, ")"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::True => write!(f, "true"),
        }
    }
}
