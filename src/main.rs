#![feature(unicode)]
#![feature(old_io)]
use std::old_io;
use parser::Parser;
use context::Context;

mod common;
mod token;
mod lexer;
mod parser;
mod context;
mod printer;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut intr = Context::new();
    loop {
        print!("-> ");
        let input = old_io::stdin()
                        .read_line()
                        .ok()
                        .expect("Unpredictable I/O error.");
        let mut parser = Parser::new(input.chars());
        match parser.parse() {
            Ok(ref sexp) => {
                match intr.eval(sexp) {
                    Ok(ref res) => {
                        println!("{}", res);
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
