pub enum TokenType {
    // Single-character tokens
    Left_Paren,
    Right_Paren,
    Left_Brace,
    Right_Brace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star

    // One or two character tokens
    Bang,
    Bang_Equal,
    Equal,
    Equal_Equal,
    Greater,
    Greater_Equal,
    Less,
    Less_Equal,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u64
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u64) -> Token {
        Token {
            TokenType: token_type,
            lexeme: lexeme.clone(),
            line: line
        }
    }
}

impl ToString for Token {
    pub fn to_string(&self) -> String {
        format!("{} {} {}\n", self.token_type, self.lexeme, self.line)
    }    
}