#[cfg(test)]
mod tests;

/// SemanticAnalyzer provides a trait for performing transformations on an AST.
pub trait SemanticAnalyzer<A, B> {
    type Error;

    fn analyze(&self, input: A) -> Result<B, Self::Error>;
}

pub mod scope;
