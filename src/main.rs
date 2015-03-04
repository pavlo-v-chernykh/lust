#![feature(core)]
#![feature(unicode)]
#![feature(old_io)]
#[macro_use]
mod lexer;
#[macro_use]
mod parser;
#[macro_use]
mod interpreter;

use std::old_io;
use parser::Parser;
use interpreter::Context;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut ctx = Context::new();
    loop {
        print!("-> ");
        match old_io::stdin().read_line() {
            Ok(input) => {
                match Parser::new(input.chars()).parse() {
                    Ok(ref expr) => {
                        match ctx.eval(expr) {
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
