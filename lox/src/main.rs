mod errors;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::process;
use errors::ParseError;

mod scanner;

#[cfg(test)]
mod tests;

type ParseResult<T> = Result<T, ParseError>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args_len == 2 {
        run_file(&args[1]).expect("Unable to parse file");
    } else {
        run_prompt();
    }
}

fn run_file(filename: &str) -> Result<(), String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => {
            run(contents).unwrap();
            Ok(())
        }
        Err(error) => Err(format!("error: {}", error)),
    }
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input).expect("execution error");
        run(input).unwrap();
    }
}

fn run(source: String) -> ParseResult<usize> {
    let mut s = scanner::Scanner::new(source);

    s.scan_tokens();
    Ok(0)
}
