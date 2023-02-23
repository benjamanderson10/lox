#![allow(dead_code)]
#![allow(unused_imports)]

pub mod error;

pub mod token;

pub mod scanner;

pub mod parser;

pub mod expr;

use std::{io::Write, env, cell::{Cell, RefCell}};

use expr::Expr::*;

use crate::{scanner::Scanner, parser::Parser, token::Token, token::TokenType, error::ErrorHandler};

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
    let mut e = ErrorHandler::<String>::new();
    let mut s = Scanner::new(source, &mut e);
    s.scan();
    println!("num tokens: {}", s.tokens.len());
    for t in &s.tokens {
        print!("{} ", t);
    }
    println!();
    let p = Parser { errorhandler: s.errors, tokens: s.tokens, expressions: RefCell::new(vec![]) };
    p.parse_expr();
    p.pretty_print();
    e.report_errors();
}
