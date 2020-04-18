use crate::parser::expression::{Expr, LiteralExpr};
use crate::parser::expression_parser::{primary, Parser};
use crate::scanner::tokens::{Literal, Token, TokenType};

fn match_literal(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok((&seed_vec[1..], Expr::Literal(LiteralExpr::new(token)),)),
        primary().parse(&seed_vec)
    );
}

#[test]
fn match_literals() {
    match_literal(Token::new(TokenType::True, Option::None));
    match_literal(Token::new(TokenType::False, Option::None));
    match_literal(Token::new(TokenType::Nil, Option::None));

    match_literal(Token::new(
        TokenType::Number,
        Option::Some(Literal::Number(123.0)),
    ));

    match_literal(Token::new(
        TokenType::Str,
        Option::Some(Literal::Str("Hello World".to_string())),
    ));
}
