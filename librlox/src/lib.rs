#[macro_use]
pub mod object;

pub mod ast;
pub mod environment;
pub mod interpreter;
pub mod parser;
pub mod scanner;

#[cfg(test)]
mod tests;
