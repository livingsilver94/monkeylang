use std::io;

use monkeylang::lexer::*;

#[test]
fn iterate_single_char_tokens() -> Result<(), Error> {
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
fn iterate_multi_char_tokens() -> Result<(), Error> {
    const CHARS: &str = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);"#;
    let tokens = &[
        Token::Let,
        Token::Identifier("five".to_string()),
        Token::Assign,
        Token::Integer(5),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("ten".to_string()),
        Token::Assign,
        Token::Integer(10),
        Token::Semicolon,
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
