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
fn parse_return_statement() -> Result<(), parser::Error> {
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
fn parse_literal_expressions() -> Result<(), parser::Error> {
    let tests = vec![
        (
            vec![Token::False, Token::Semicolon],
            AST::new(vec![Statement::Expression(Expression::Boolean(false))]),
        ),
        (
            vec![Token::True, Token::Semicolon],
            AST::new(vec![Statement::Expression(Expression::Boolean(true))]),
        ),
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

#[test]
fn parse_unary_expressions() -> Result<(), parser::Error> {
    let tests = vec![
        (
            vec![Token::Bang, Token::Integer(5)], // !5.
            AST::new(vec![Statement::Expression(Expression::Unary {
                operator: Token::Bang,
                expression: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Minus, Token::Integer(5)], // -5.
            AST::new(vec![Statement::Expression(Expression::Unary {
                operator: Token::Minus,
                expression: boxx(Expression::Integer(5)),
            })]),
        ),
    ];
    for test in tests {
        let ast = Parser::new(test.0.iter()).parse()?;
        assert_eq!(ast, test.1);
    }
    Ok(())
}

#[test]
fn parse_binary_expressions() -> Result<(), parser::Error> {
    let tests = vec![
        (
            vec![Token::Integer(5), Token::Plus, Token::Integer(5)], // 5 + 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Plus,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::Minus, Token::Integer(5)], // 5 - 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Minus,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::Asterisk, Token::Integer(5)], // 5 * 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Asterisk,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::Slash, Token::Integer(5)], // 5 / 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Slash,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::GreaterThan, Token::Integer(5)], // 5 > 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::GreaterThan,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::LessThan, Token::Integer(5)], // 5 < 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::LessThan,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::Equal, Token::Integer(5)], // 5 == 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Equal,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
        (
            vec![Token::Integer(5), Token::NotEqual, Token::Integer(5)], // 5 != 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::NotEqual,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
    ];
    for test in tests {
        let ast = Parser::new(test.0.iter()).parse()?;
        assert_eq!(ast, test.1);
    }
    Ok(())
}

#[test]
fn parse_nested_binary_expressions() -> Result<(), parser::Error> {
    let tests = vec![(
        vec![
            Token::Integer(5),
            Token::Plus,
            Token::Integer(5),
            Token::Plus,
            Token::Integer(5),
        ], // 5 + 5 + 5.
        AST::new(vec![Statement::Expression(Expression::Binary {
            left: boxx(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Plus,
                right: boxx(Expression::Integer(5)),
            }),
            operator: Token::Plus,
            right: boxx(Expression::Integer(5)),
        })]),
    )];
    for test in tests {
        let ast = Parser::new(test.0.iter()).parse()?;
        assert_eq!(ast, test.1);
    }
    Ok(())
}

/// Tests expressions with unobvious priority, e.g. with a multiplication
/// on the right or parentheses to override the default priority.
#[test]
fn parse_expressions_with_priority() -> Result<(), parser::Error> {
    let tests = vec![
        (
            vec![
                Token::Integer(5),
                Token::Plus,
                Token::Integer(5),
                Token::Asterisk,
                Token::Integer(5),
            ], // 5 + 5 * 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Integer(5)),
                operator: Token::Plus,
                right: boxx(Expression::Binary {
                    left: boxx(Expression::Integer(5)),
                    operator: Token::Asterisk,
                    right: boxx(Expression::Integer(5)),
                }),
            })]),
        ),
        (
            vec![
                Token::LeftParen,
                Token::Integer(10),
                Token::Plus,
                Token::Integer(5),
                Token::RightParen,
                Token::Asterisk,
                Token::Integer(5),
            ], // (10 + 5) * 5.
            AST::new(vec![Statement::Expression(Expression::Binary {
                left: boxx(Expression::Binary {
                    left: boxx(Expression::Integer(10)),
                    operator: Token::Plus,
                    right: boxx(Expression::Integer(5)),
                }),
                operator: Token::Asterisk,
                right: boxx(Expression::Integer(5)),
            })]),
        ),
    ];
    for test in tests {
        let ast = Parser::new(test.0.iter()).parse()?;
        assert_eq!(ast, test.1);
    }
    Ok(())
}

/// Just an abbreviated Box::new(T).
#[inline(always)]
fn boxx<T>(val: T) -> Box<T> {
    Box::new(val)
}
