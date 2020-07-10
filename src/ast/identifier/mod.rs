use crate::ast::token;
use std::collections::hash_map::DefaultHasher;
use std::convert;
use std::fmt;
use std::hash::Hasher;

#[cfg(test)]
mod tests;

/// Identifier functions as a replacement for variable names, offering a raw Id and a Hash.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Identifier {
    Hash(u64), // todo change this to an actual hash
    Id(String),
}

impl Identifier {
    /// to_hash will always return a Hash variant. If the type is already a
    /// Hash, it will return itself. Otherwise the value of the Id variant will
    /// be hashed and returned via the Hash variant.
    pub fn to_hash(self) -> Self {
        match self {
            s @ Self::Hash(_) => s,
            Self::Id(id) => {
                let mut hasher = DefaultHasher::new();
                hasher.write(&id.into_bytes());
                Self::Hash(hasher.finish())
            }
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(ref s) => write!(f, "{}", s),
            Self::Hash(ref s) => write!(f, "{}", s),
        }
    }
}

impl convert::TryFrom<token::Token> for Identifier {
    type Error = &'static str;

    fn try_from(tok: token::Token) -> Result<Identifier, Self::Error> {
        match (tok.token_type, tok.lexeme) {
            (token::TokenType::Identifier, Some(lexeme)) => Ok(Identifier::Id(lexeme)),
            _ => Err("cannot convert token to identifier, lexeme not defined"),
        }
    }
}

impl From<&str> for Identifier {
    fn from(from: &str) -> Identifier {
        Identifier::Id(from.to_string())
    }
}

impl From<String> for Identifier {
    fn from(from: String) -> Identifier {
        Identifier::Id(from)
    }
}

#[allow(unused_macros)]
macro_rules! identifier_id {
    ($id:expr) => {
        $crate::ast::identifier::Identifier::Id($id.to_string())
    };
}
