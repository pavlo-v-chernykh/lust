#![feature(core)]
#![feature(unicode)]
#![feature(old_io)]
#[macro_use]
mod lexer;
#[macro_use]
mod expr;
#[macro_use]
mod parser;
#[macro_use]
mod interpreter;

use std::old_io;
use parser::Parser;
use interpreter::{Context, Scope};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let ref mut ctx = Context::new();
    let ref mut root_scope = Scope::new_std();
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
                                println!("Whoops, error detected.\n{}.\nPlease, try again...", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Whoops, error detected.\n{}.\nPlease, try again...", e);
                    }
                }
            },
            Err(e) => {
                println!("Whoops, error detected.\n{}.\nPlease, try again...", e);
                break
            }
        }
    }
}
