#![feature(unicode)]
#![feature(old_io)]
use std::old_io;
use parser::Parser;
use context::Context;

mod token;
mod lexer;
mod parser;
mod ast;
mod context;
mod printer;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut ctx = Context::new();
    loop {
        print!("-> ");
        match old_io::stdin().read_line() {
            Ok(input) => {
                let mut parser = Parser::new(input.chars());
                match parser.parse() {
                    Ok(ref sexp) => {
                        match ctx.eval(sexp) {
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
            },
            Err(e) => {
                println!("{:?}", e);
                break
            }
        }
    }
}
