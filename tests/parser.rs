use monkeylang::lexer::*;
use monkeylang::parser::{self, *};

#[test]
fn parse_let_statement() -> Result<(), parser::Error> {
    let expected = AST::new(vec![Statement::Let {
        identifier: "x".to_string(),
        expression: Expression::None,
    }]);
    let ast = Parser::new(
        vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
        ]
        .iter(),
    )
    .parse()?;
    assert_eq!(ast, expected);
    Ok(())
}
