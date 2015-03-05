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
use interpreter::{Context, Scope};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let ctx = &mut Context::new();
    let root_scope = &mut Scope::new_std();
    loop {
        print!("-> ");
        match old_io::stdin().read_line() {
            Ok(input) => {
                match Parser::new(input.chars()).parse() {
                    Ok(ref expr) => {
                        match ctx.eval(root_scope, expr) {
                            Ok(ref res) => {
                                println!("{}", res);
                            },
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                break
            }
        }
    }
}
