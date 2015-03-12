#![feature(core)]
#![feature(old_io)]
#[macro_use]
mod lexer;
#[macro_use]
mod expr;
#[macro_use]
mod parser;
mod scope;

use std::old_io;
use parser::Parser;
use scope::Scope;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let ref mut root_scope = Scope::new_std();
    loop {
        print!("-> ");
        match old_io::stdin().read_line() {
            Ok(input) => {
                match Parser::new(input.chars()).parse() {
                    Ok(ref expr) => {
                        match expr.expand(root_scope) {
                            Ok(ref expr) => {
                                match expr.eval(root_scope) {
                                    Ok(ref res) => {
                                        println!("{}", res);
                                    },
                                    Err(e) => {
                                        println!("Whoops, error detected.\n{}.\n\
                                                  Please, try again...", e);
                                    }
                                }
                            },
                            Err(e) => {
                                println!("Whoops, error detected.\n{}.\n\
                                          Please, try again...", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Whoops, error detected.\n{}.\n\
                                  Please, try again...", e);
                    }
                }
            },
            Err(e) => {
                println!("Whoops, error detected.\n{}.\n\
                          Please, try again...", e);
                break
            }
        }
    }
}
