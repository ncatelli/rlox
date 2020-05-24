use crate::ast::token::TokenType;
use crate::object;

use super::helpers::compare_single_token_source_with_literal_helper;

#[test]
fn scan_tokens_should_lex_full_string() {
    compare_single_token_source_with_literal_helper(
        "\"test\"",
        object::Object::Literal(object::Literal::Str("test".to_string())),
        TokenType::Literal,
    )
}
