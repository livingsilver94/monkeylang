use std::io;

use monkeylang::lexer::*;

#[test]
fn skip_whitespaces() {
    const CHARS: &str = " \t\n\r";

    let lex = Lexer::new(io::Cursor::new(CHARS));
    assert!(lex.into_iter().next().is_none());
}

#[test]
fn detect_single_chars_only() -> Result<(), Error> {
    const CHARS: &str = "=*!,>{(<-+});/";
    let tokens = &[
        Token::Assign,
        Token::Asterisk,
        Token::Bang,
        Token::Comma,
        Token::GreaterThan,
        Token::LeftBrace,
        Token::LeftParen,
        Token::LessThan,
        Token::Minus,
        Token::Plus,
        Token::RightBrace,
        Token::RightParen,
        Token::Semicolon,
        Token::Slash,
    ];

    let lex = Lexer::new(io::Cursor::new(CHARS));
    for (i, tok) in lex.into_iter().enumerate() {
        let tok = tok?;
        assert_eq!(tok, tokens[i]);
        assert_eq!(tok, tokens[i]);
    }
    Ok(())
}

#[test]
fn detect_reserved_words_only() -> Result<(), Error> {
    const CHARS: &str = "fn let";
    let tokens = &[Token::Function, Token::Let];

    let lex = Lexer::new(io::Cursor::new(CHARS));
    for (i, tok) in lex.into_iter().enumerate() {
        let tok = tok?;
        assert_eq!(tok, tokens[i]);
        assert_eq!(tok, tokens[i]);
    }
    Ok(())
}

#[test]
fn detect_integer_assignments() -> Result<(), Error> {
    const CHARS: &str = "\
let one_digit = 5;
let multidigit = 10;";
    let tokens = &[
        Token::Let,
        Token::Identifier("one_digit".to_string()),
        Token::Assign,
        Token::Integer(5),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("multidigit".to_string()),
        Token::Assign,
        Token::Integer(10),
        Token::Semicolon,
    ];

    let lex = Lexer::new(io::Cursor::new(CHARS));
    for (i, tok) in lex.into_iter().enumerate() {
        let tok = tok?;
        assert_eq!(tok, tokens[i]);
        assert_eq!(tok, tokens[i]);
    }
    Ok(())
}

#[test]
fn detect_function_assignments() -> Result<(), Error> {
    const CHARS: &str = "\
let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";
    let tokens = &[
        Token::Let,
        Token::Identifier("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LeftParen,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::RightParen,
        Token::LeftBrace,
        Token::Identifier("x".to_string()),
        Token::Plus,
        Token::Identifier("y".to_string()),
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier("result".to_string()),
        Token::Assign,
        Token::Identifier("add".to_string()),
        Token::LeftParen,
        Token::Identifier("five".to_string()),
        Token::Comma,
        Token::Identifier("ten".to_string()),
        Token::RightParen,
        Token::Semicolon,
    ];
    let lex = Lexer::new(io::Cursor::new(CHARS));
    for (i, tok) in lex.into_iter().enumerate() {
        let tok = tok?;
        assert_eq!(tok, tokens[i]);
        assert_eq!(tok, tokens[i]);
    }
    Ok(())
}
