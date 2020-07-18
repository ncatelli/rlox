#[macro_use]
pub mod object;

#[macro_use]
pub mod ast;

pub mod analyzer;
pub mod environment;
pub mod functions;
pub mod interpreter;
pub mod parser;
pub mod pass;
pub mod scanner;
pub mod statics;

#[cfg(test)]
mod tests;
