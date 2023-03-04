#![allow(dead_code)]
#![allow(unused_imports)]

pub mod error;

pub mod token;

pub mod scanner;

pub mod parser;

pub mod expr;

use std::{
    cell::{Cell, RefCell},
    env,
    io::Write,
};

use expr::Expr::*;

use crate::{error::ErrorHandler, parser::Parser, scanner::Scanner, token::Token, token::TokenType};

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
    let mut e = RefCell::new(ErrorHandler::<String>::new());
    let mut tokens = {
    let mut s = Scanner::new(source, e.borrow_mut());
    s.scan();
    println!("num tokens: {}", s.tokens.len());
    for t in &s.tokens {
        print!("{} ", t);
    }
    println!();
    s.tokens
    };
    /*let mut p = RefCell::new(Parser {
        errorhandler: e.borrow_mut(),
        tokens: tokens,
        expressions: RefCell::new(vec![]),
    });
    let mut binding = p.borrow_mut();
    binding.parse_expr();
    binding.pretty_print();
    
    binding.errorhandler.report_errors();*/
}
