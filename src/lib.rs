#[macro_use]
pub mod object;

#[macro_use]
pub mod ast;

pub mod environment;
pub mod functions;
pub mod interpreter;
pub mod analyzer;
pub mod parser;
pub mod scanner;

#[cfg(test)]
mod tests;
