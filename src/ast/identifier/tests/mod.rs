use crate::ast::identifier::Identifier;
use crate::ast::token;
use std::convert::TryFrom;

#[test]
fn should_convert_identfier_token_with_lexeme_to_identfier() {
    let tok = token::Token::new(
        token::TokenType::Identifier,
        0,
        Some("test".to_string()),
        None,
    );

    assert_eq!(
        Ok(Identifier::Id("test".to_string())),
        Identifier::try_from(tok)
    )
}

#[test]
fn should_throw_an_error_if_token_not_an_identifer_on_conversion() {
    let tok = token::Token::new(
        token::TokenType::LeftParen,
        0,
        Some("test".to_string()),
        None,
    );

    assert!(Identifier::try_from(tok).is_err())
}

#[test]
fn should_throw_an_error_if_token_has_no_lexeme() {
    let tok = token::Token::new(token::TokenType::Identifier, 0, None, None);

    assert!(Identifier::try_from(tok).is_err())
}
