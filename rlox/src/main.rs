use std::{env, io::Write};

use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    match path {
        Some(n) => {
            runfile(&n);
        }
        None => {
            repl();
        }
    }
}

fn runfile(file: &String) {
    let source = std::fs::read_to_string(file).expect(format!("{}", file).as_str());
    run(&source);
}

fn repl() {
    println!();
    loop {
        let mut input = String::new();
        let mut s = std::io::stdout();
        let mut h = s.lock();
        h.write_all(b"> ").unwrap();
        s.flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        run(&input);
    }
}

fn run(source: &String) {
    let mut s = Scanner::new(source);
    s.scan();
    for error in s.errors {
        error.report();
    }
}

pub mod error;

pub mod scanner;

pub mod token;
