use std::fmt;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum PassErr {
    TypeErr(String),
}

impl fmt::Display for PassErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeErr(e) => write!(f, "invalid type: {}", e),
        }
    }
}

pub trait Pass<A, B> {
    type Error;

    fn tree_pass(&self, input: A) -> Result<B, Self::Error>;
}

pub trait PassMut<A, B> {
    type Error;

    fn tree_pass_mut(&mut self, input: A) -> Result<B, Self::Error>;
}
