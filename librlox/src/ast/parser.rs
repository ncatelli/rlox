use crate::scanner::tokens::Token;

/// Parser wraps the functionality of converting the tokens from the scanner
/// into a corresponding AST.
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens }
    }
}
