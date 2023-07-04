use std::io;

use monkeylang::lexer::*;

#[test]
fn iterate_single_char_tokens() -> Result<(), Error> {
    const CHARS: &str = "=+(){},";
    let tokens = &[
        Token::Assign,
        Token::Plus,
        Token::LeftParen,
        Token::RightParen,
        Token::LeftBrace,
        Token::RightBrace,
        Token::Comma,
    ];

    let lex = Lexer::new(io::Cursor::new(CHARS));
    for (i, tok) in lex.into_iter().enumerate() {
        let tok = tok?;
        assert_eq!(tok, tokens[i]);
        assert_eq!(tok, tokens[i]);
    }
    Ok(())
}
