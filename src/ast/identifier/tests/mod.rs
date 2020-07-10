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

#[test]
fn to_hash_returns_itself_if_variant_is_a_hash() {
    let id = Identifier::Hash(16183295663280961421);

    assert_eq!(Identifier::Hash(16183295663280961421), id.to_hash())
}

#[test]
fn to_hash_should_convert_an_id_to_a_matching_value() {
    let id = Identifier::Id("test".to_string());

    assert_eq!(Identifier::Hash(16183295663280961421), id.to_hash())
}

#[test]
fn two_id_identfiers_with_same_value_should_generate_the_same_hash() {
    let id_one = Identifier::Id("test".to_string());
    let id_two = Identifier::Id("test".to_string());

    assert_eq!(id_one.to_hash(), id_two.to_hash())
}
