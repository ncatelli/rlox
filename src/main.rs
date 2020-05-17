use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::process;

extern crate librlox;
extern crate parcel;
use librlox::environment::Environment;
use librlox::interpreter::statement::interpret;
use librlox::parser::statement_parser::statements;
use librlox::scanner;
use parcel::prelude::v1::*;

type RuntimeResult<T> = Result<T, String>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    match args_len {
        al if al > 2 => {
            println!("Usage: jlox [script]");
            process::exit(64);
        }
        2 => run_file(&args[1]).expect("Unable to parse file"),
        _ => run_prompt(),
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

#[allow(unused_must_use)]
fn run(source: String) -> RuntimeResult<usize> {
    let mut sym_table = Environment::new();
    let token_iter = scanner::Scanner::new(source).scan_tokens().into_iter();
    let token_count = token_iter.len();

    let tokens: Vec<scanner::Token> = token_iter
        .map(|tok| match tok {
            Ok(tok) => tok,
            Err(e) => panic!("{}", e),
        })
        .collect();

    match statements().parse(&tokens) {
        Ok(parcel::MatchStatus::Match((_, stmt))) => match interpret(sym_table, stmt) {
            Ok(s) => sym_table = s,
            Err(e) => panic!(e),
        },
        Ok(parcel::MatchStatus::NoMatch(_)) => println!("No match found"),
        Err(e) => println!("{:?}", e),
    };

    Ok(token_count)
}
