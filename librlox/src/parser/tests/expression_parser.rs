use crate::parser::expression::{Expr, LiteralExpr, UnaryExpr};
use crate::parser::expression_parser::*;
use crate::scanner::tokens::{Literal, Token, TokenType};

fn match_literal_helper(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok((&seed_vec[1..], Expr::Literal(LiteralExpr::new(token)),)),
        primary().parse(&seed_vec)
    );
}

#[test]
fn match_literals() {
    match_literal_helper(Token::new(TokenType::True, Option::None));
    match_literal_helper(Token::new(TokenType::False, Option::None));
    match_literal_helper(Token::new(TokenType::Nil, Option::None));

    match_literal_helper(Token::new(
        TokenType::Number,
        Option::Some(Literal::Number(123.0)),
    ));

    match_literal_helper(Token::new(
        TokenType::Str,
        Option::Some(Literal::Str("Hello World".to_string())),
    ));
}

#[test]
fn match_unary() {
    let op_token = Token::new(TokenType::Bang, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![op_token.clone(), literal_token.clone()];

    assert_eq!(
        Ok((
            &seed_vec[2..],
            Expr::Unary(UnaryExpr::new(
                op_token,
                Box::new(Expr::Literal(LiteralExpr::new(literal_token)))
            ))
        )),
        unary().parse(&seed_vec)
    );
}
