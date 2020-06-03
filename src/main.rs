use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::process;

extern crate parcel;
use parcel::prelude::v1::*;
use rlox::ast::token;
use rlox::interpreter::Interpreter;
use rlox::interpreter::StatefulInterpreter;
use rlox::parser::statement_parser::statements;
use rlox::scanner;

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
    let mut interpreter = StatefulInterpreter::new();
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => {
            run(&mut interpreter, contents).unwrap();
            Ok(())
        }
        Err(error) => Err(format!("error: {}", error)),
    }
}

fn run_prompt() {
    let mut interpreter = StatefulInterpreter::new();
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input).expect("execution error");
        run(&mut interpreter, input).unwrap();
    }
}

fn run(interpreter: &mut StatefulInterpreter, source: String) -> RuntimeResult<usize> {
    let token_iter = scanner::Scanner::new(source).scan_tokens().into_iter();
    let token_count = token_iter.len();

    let tokens: Vec<token::Token> = token_iter
        .map(|tok| match tok {
            Ok(tok) => tok,
            Err(e) => panic!("{}", e),
        })
        .collect();

    match statements().parse(&tokens) {
        Ok(parcel::MatchStatus::Match((_, stmt))) => interpreter.interpret(stmt).unwrap(),
        Ok(parcel::MatchStatus::NoMatch(_)) => println!("No match found"),
        Err(e) => println!("{:?}", e),
    };

    Ok(token_count)
}
