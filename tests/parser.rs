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

#[test]
fn parse_bad_let_statement() {
    let ast = Parser::new(
        vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::True,
            Token::Integer(5),
            Token::Semicolon,
        ]
        .iter(),
    )
    .parse();
    assert!(ast.is_err_and(|e| { matches!(e, parser::Error::ExpectedToken { expected: _, got: _ }) }));
}

#[test]
fn parse_return_literal_statement() -> Result<(), parser::Error> {
    let expected = AST::new(vec![Statement::Return(Expression::None)]);
    let ast = Parser::new(vec![Token::Return, Token::Integer(5), Token::Semicolon].iter()).parse()?;
    assert_eq!(ast, expected);
    Ok(())
}

#[test]
fn parse_return_expression_statement() -> Result<(), parser::Error> {
    let expected = AST::new(vec![Statement::Return(Expression::None)]);
    let ast = Parser::new(
        vec![
            Token::Return,
            Token::Integer(5),
            Token::Plus,
            Token::Integer(10),
            Token::Semicolon,
        ]
        .iter(),
    )
    .parse()?;
    assert_eq!(ast, expected);
    Ok(())
}

#[test]
fn parse_nonrecursive_expressions() -> Result<(), parser::Error> {
    let tests = vec![
        (
            vec![Token::Identifier("var".to_string()), Token::Semicolon],
            AST::new(vec![Statement::Expression(Expression::Identifier("var".to_string()))]),
        ),
        (
            vec![Token::Integer(65), Token::Semicolon],
            AST::new(vec![Statement::Expression(Expression::Integer(65))]),
        ),
    ];
    for test in tests {
        let ast = Parser::new(test.0.iter()).parse()?;
        assert_eq!(ast, test.1);
    }
    Ok(())
}
