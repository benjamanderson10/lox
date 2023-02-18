#![feature(associated_type_bounds)]
#![allow(dead_code)]

use std::{io::Write};

use expr::Expression::*;

use crate::{scanner::Scanner, parser::Parser, token::Token, token::TokenType, error::ErrorHandler};

fn main() {
    let mut e = ErrorHandler::<String>::new();
    let mut p = Parser { errorhandler: &mut e, tokens: vec![Token { tokentype: TokenType::Number(6.0), src: "6"}, Token {tokentype: TokenType::Star, src: "*"}, Token {tokentype: TokenType::Number(6.0), src: "7"}, Token { tokentype: TokenType::Number(6.0), src: "6"}, Token {tokentype: TokenType::Star, src: "*"}, Token {tokentype: TokenType::Number(6.0), src: "7"}], expressions: vec![Operation(Box::new(Literal(Token { tokentype: TokenType::Number(6.0), src: "6"})), Token {tokentype: TokenType::Star, src: "*"}, Box::new(Literal(Token {tokentype: TokenType::Number(6.0), src: "7"})))]};
    p.pretty_print();
    p.parse();
    /*

    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    match path {
        Some(n) => {
            runfile(&n);
        }
        None => {
            repl();
        }
        Some(n) => {
            runfile(&n);
        }
        None => {
            repl();
        }
    }
    */
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
    for token in s.tokens {
        println!("{:?}", token);
    }
    e.report_errors();
}

pub mod error;

pub mod token;

pub mod scanner;

pub mod parser;

pub mod expr;
