use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args_len == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(filename: &str) -> Result<(), String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => {
            run(&contents);
            Ok(())
        }
        Err(error) => Err(format!("error: {}", error))
    }
}

fn run_prompt() {
    while true {
        let mut input = String::new();
        print!("> ");
        stdin().read_line(&mut input).expect("execution error");
        run(&input);
    }
}

fn run(source: &str) {

}