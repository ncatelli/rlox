use crate::ast::token;
use std::convert;
use std::fmt;

#[cfg(test)]
mod tests;

/// Identifier functions as a replacement for variable names, offering a raw
/// name corresponding to a variable name, and an Id functioning as a numeric
/// reference.
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Identifier {
    Name(String),
    Id(u64),
}

impl PartialEq<u64> for Identifier {
    fn eq(&self, other: &u64) -> bool {
        match self {
            Self::Id(h) => *h == *other,
            _ => false,
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(ref s) => write!(f, "{}", s),
            Self::Id(ref u) => write!(f, "{}", u),
        }
    }
}

impl convert::TryFrom<token::Token> for Identifier {
    type Error = &'static str;

    fn try_from(tok: token::Token) -> Result<Identifier, Self::Error> {
        match (tok.token_type, tok.lexeme) {
            (token::TokenType::Identifier, Some(lexeme)) => Ok(Identifier::Name(lexeme)),
            _ => Err("cannot convert token to identifier, lexeme not defined"),
        }
    }
}

impl From<&str> for Identifier {
    fn from(from: &str) -> Identifier {
        Identifier::Name(from.to_string())
    }
}

impl From<String> for Identifier {
    fn from(from: String) -> Identifier {
        Identifier::Name(from)
    }
}

#[allow(unused_macros)]
macro_rules! identifier_id {
    ($id:expr) => {
        $crate::ast::identifier::Identifier::Name($id.to_string())
    };
}
