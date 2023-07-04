use std::io;

use monkeylang::lexer::*;

#[test]
fn iterate_single_chars() -> Result<(), Error> {
    const CHARS: &str = "=+(){},";
    let tokens = &[
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Plus,
            value: "+".to_string(),
        },
        Token {
            kind: TokenKind::LeftParen,
            value: "(".to_string(),
        },
        Token {
            kind: TokenKind::RightParen,
            value: ")".to_string(),
        },
        Token {
            kind: TokenKind::LeftBrace,
            value: "{".to_string(),
        },
        Token {
            kind: TokenKind::RightBrace,
            value: "}".to_string(),
        },
        Token {
            kind: TokenKind::Comma,
            value: ",".to_string(),
        },
    ];

    let lex = Lexer::new(io::Cursor::new(CHARS));
    for (i, tok) in lex.into_iter().enumerate() {
        let tok = tok?;
        assert_eq!(tok.kind, tokens[i].kind);
        assert_eq!(tok.value, tokens[i].value);
    }
    Ok(())
}
