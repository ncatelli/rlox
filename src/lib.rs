#[macro_use]
pub mod object;

#[macro_use]
pub mod ast;

pub mod environment;
pub mod functions;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;

#[cfg(test)]
mod tests;
